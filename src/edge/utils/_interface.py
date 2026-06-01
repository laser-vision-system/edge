"""Implements C#-style interfaces using a custom metaclass.

The metaclass checks if a subclass is an interface or implementation,
and validates an implementation's correctness.
"""

# TODO Look into static error checking.

from abc import ABCMeta as _ABCMeta
from inspect import Signature as _Signature, signature as _signature
from typing import Any as _Any, NoReturn as _NoReturn


# Cannot use typing.Self in a metaclass.


_ABSTRACTMETHOD_DUNDER = "__isabstractmethod__"
"""The dunder used by abc for defining abstract methods."""

_INTERFACE_ATTR = "_is_interface_"
"""The class attribute used to identify it as an interface."""


def _getattr(
	obj: object,
	/,
	attr_name: str = _ABSTRACTMETHOD_DUNDER,
	default: bool | None = False,
) -> _Any | bool | None:
	"""Wrapper for `getattr` providing easy shorthand in most circumstances.

	In most cases, it will check if the attribute has the value of true.

	Parameters
	----------
	obj : object
		The object to be inspected.
	attr_name : str, default=:func:`_ABSTRACTMETHOD_DUNDER`
		The name of the attribute.
	default : bool or None, default=False
		The value to return of the attribute doesn't exist.

	Returns
	-------
	Any
		The attribute value.
	bool, None
		`default`, if the attribute doesn't exist.
	"""

	return getattr(obj, attr_name, default)


def _is_interface_attribute_valid(name: str, value: _Any) -> bool:
	"""Evaluates if an attribute is allowed within an interface.

	Allow internal dunders, static methods, and abstract methods.

	Parameters
	----------
	name : str
		The name of the attribute.
	value : Any
		The value of the attribute.

	Returns
	-------
	bool
		Whether the provided attribute is valid.
	"""

	return bool(
		(name.startswith("__") and name.endswith("__"))
		or isinstance(value, staticmethod)
		or _getattr(value, _ABSTRACTMETHOD_DUNDER)
	)


def _evaluate_if_interface(bases: tuple[type, ...], namespace: dict[str, _Any]) -> bool:
	"""Determines if a class qualifies as an interface.

	The rules for whether a class qualifies as an interface are:

	- It does not inherit from any non-interface (excluding 'object').
	- It does not define concrete methods or data fields.

	Parameters
	----------
	bases : tuple of class types
		The classes provided as the base for this class.
	namespace : dictionary of attribute names and values
		The namespace elements provided.

	Returns
	-------
	bool
		If the details provided constitute a valid interface.
	"""

	# Check if it inherits from any non-interface (excluding 'object').
	if not all(
		_getattr(base, attr_name=_INTERFACE_ATTR)
		for base in bases
		if base is not object
	):
		return False

	# Check if it defines concrete methods or data fields.
	return all(
		_is_interface_attribute_valid(name=key, value=value)
		for key, value in namespace.items()
	)


def _validate_implementation(cls: type, /) -> None:
	"""Validate against all interfaces in the mro.

	This function is treated like a single use class.

	Parameters
	----------
	cls : type
		This is the implementation being created that needs validation.

	Raises
	------
	TypeError
		If any signature detail does not match.
	"""

	# TODO Define object level fields.

	def _raise_signature_violation(
		reason: str, cls_name: str, attr_name: str, expected: _Any, provided: _Any
	) -> _NoReturn:
		"""Raise a TypeError with a predefined structure.

		Parameters
		----------
		reason : str
			Short explanation of why this was raised.
		expected : Any
			The value that was expected.
		provided : Any
			The value that was provided instead.
		cls_name : str
			The name of the class
			(for debug reasons).
		attr_name : str
			The name of the attribute
			(for debug reasons).

		Raises
		------
		TypeError
		"""

		raise TypeError(
			f"{reason} in '{cls_name}.{attr_name}'\n"
			f"Interface mandates: {expected}\n"
			f"Implementer provided: {provided}"
		)

	def _compare_signatures(
		intrfc_sig: _Signature, impl_sig: _Signature, cls_name: str, attr_name: str
	) -> None:
		"""Validates that the implementer matches the interface mandate exactly.

		Parameters
		----------
		intrfc_sig : Signature
			The signature of an interface within the mro.
		impl_sig : Signature
			The signature of the implementation being tested.
		cls_name : str
			The name of the class
			(for debug reasons).
		attr_name : str
			The name of the attribute
			(for debug reasons).

		Raises
		------
		TypeError
			If any signature detail does not match.
		"""

		# Compare return type hints.
		if intrfc_sig.return_annotation != impl_sig.return_annotation:
			_raise_signature_violation(
				reason="Return type mismatch",
				cls_name=cls_name,
				attr_name=attr_name,
				expected=intrfc_sig.return_annotation,
				provided=impl_sig.return_annotation,
			)

		# Compare total parameter counts.
		if len(intrfc_sig.parameters) != len(impl_sig.parameters):
			_raise_signature_violation(
				reason="Parameter count mismatch",
				cls_name=cls_name,
				attr_name=attr_name,
				expected=len(intrfc_sig.parameters),
				provided=len(impl_sig.parameters),
			)

		# Compare sequential parameter type hints, ignoring names.
		for intrfc_p, impl_p in zip(
			intrfc_sig.parameters.values(), impl_sig.parameters.values()
		):
			if intrfc_p.annotation != impl_p.annotation:
				_raise_signature_violation(
					reason=f"Parameter '{intrfc_p.name}' type mismatch",
					cls_name=cls_name,
					attr_name=attr_name,
					expected=intrfc_p.annotation,
					provided=impl_p.annotation,
				)

			# Prevent changing a positional argument into a keyword-only argument.
			if intrfc_p.kind != impl_p.kind:
				_raise_signature_violation(
					reason=f"Parameter '{intrfc_p.name}' kind mismatch",
					cls_name=cls_name,
					attr_name=attr_name,
					expected=intrfc_p.kind,
					provided=impl_p.kind,
				)

	def _validate_property_signatures(
		intrfc_prop: property, impl_prop: property, cls_name: str, attr_name: str
	) -> None:
		"""Validate an implementation's property.

		Validates that it strictly matches the mandated getter,
		setter and deleter accessors defined by the interface.

		Parameters
		----------
		intrfc_prop, impl_prop : property
			Matching base property attributes on the interface and implementation.
		cls_name : str
			The name of the class
			(for debug reasons).
		attr_name : str
			The name of the attribute
			(for debug reasons).

		Raises
		------
		TypeError
			If any signature detail does not match.
		"""

		def _validate_accessor(
			intrfc_access_func: _Any, impl_access_func: _Any, accessor_type: str
		) -> None:
			"""Validate a single accessor function for an implementation.

			Parameters
			----------
			intrfc_access_func, impl_access_func : Any
				Matching accessor functions for the interface and implementer.
			accessor_type: str
				The named type of the accessor
				(for debug reasons).

			Raises
			------
			TypeError
				If any signature detail does not match.
			"""

			# If the interface mandates this accessor, the implementor must provide it.
			if intrfc_access_func is None:
				return

			if impl_access_func is None:
				raise TypeError(
					f"Implementation error in '{cls_name}.{attr_name}': "
					f"Missing property {accessor_type}."
				)

			try:
				int_sig: _Signature = _signature(obj=intrfc_access_func)
				imp_sig: _Signature = _signature(obj=impl_access_func)
			except (ValueError, TypeError) as e:
				raise TypeError(
					f"Un-inspectable property {accessor_type} "
					f"in '{cls_name}.{attr_name}': {e}"
				)

			_compare_signatures(
				intrfc_sig=int_sig,
				impl_sig=imp_sig,
				cls_name=cls_name,
				attr_name=f"{attr_name} ({accessor_type})",
			)

		# Evaluate all three descriptor accessors.
		_validate_accessor(
			intrfc_access_func=intrfc_prop.fget,
			impl_access_func=impl_prop.fget,
			accessor_type="getter",
		)
		_validate_accessor(
			intrfc_access_func=intrfc_prop.fset,
			impl_access_func=impl_prop.fset,
			accessor_type="setter",
		)
		_validate_accessor(
			intrfc_access_func=intrfc_prop.fdel,
			impl_access_func=impl_prop.fdel,
			accessor_type="deleter",
		)

	def _validate_attribute(
		cls: type, intrfc_attr_name: str, intrfc_attr_val: _Any, intrfc_name: str
	) -> None:
		"""Validate an interface attribute against the implementation.

		First check if the implementation has overridden the attribute,
		then compare the signatures.

		Parameters
		----------
		cls : type
			This is the implementation being created that needs validation.
		intrfc_attr_name : str
			The name of the attribute on the interface being checked.
		intrfc_attr_val : Any
			The value of the attribute on the interface being checked.
		intrfc_name : str
			The name of the interface currently being validated against
			(for debug reasons).

		Raises
		------
		TypeError
			If any signature detail does not match.
		"""

		if not _getattr(intrfc_attr_val):
			return

		impl_attr_val: _Any | None = _getattr(
			cls, attr_name=intrfc_attr_name, default=None
		)

		# If the implementer hasn't overridden it yet, it remains abstract.
		# Skip signature validation and let ABC block instantiation later.
		if impl_attr_val is None or _getattr(impl_attr_val):
			return

		# Handle properties explicitly.
		if isinstance(intrfc_attr_val, property):
			if not isinstance(impl_attr_val, property):
				raise TypeError(
					f"Implementation error in '{cls.__name__}.{intrfc_attr_name}': "
					f"Interface mandates a property, but implementor provided "
					f"'{type(impl_attr_val).__name__}'."
				)

			_validate_property_signatures(
				intrfc_prop=intrfc_attr_val,
				impl_prop=impl_attr_val,
				cls_name=cls.__name__,
				attr_name=intrfc_attr_name,
			)
			return

		# Validate the Interface Signature.
		try:
			intrfc_sig: _Signature = _signature(intrfc_attr_val)
		except (ValueError, TypeError) as e:
			raise TypeError(
				f"Interface definition error: '{intrfc_name}.{intrfc_attr_name}' "
				f"is un-inspectable. {e}"
			)

		# Validate the Implementation Signature.
		try:
			impl_sig: _Signature = _signature(impl_attr_val)
		except (ValueError, TypeError) as e:
			raise TypeError(
				f"Implementation error: '{cls.__name__}.{intrfc_attr_name}': "
				f"is un-inspectable and cannot be validated against the interface.\n"
				f"Ensure you are not directly assigning C-bindings "
				f"or using broken decorators. {e}"
			)

		_compare_signatures(
			intrfc_sig=intrfc_sig,
			impl_sig=impl_sig,
			cls_name=cls.__name__,
			attr_name=intrfc_attr_name,
		)

	# _validate_implementation execution.
	for interface in [
		base for base in cls.__mro__ if _getattr(base, attr_name=_INTERFACE_ATTR)
	]:
		for intrfc_attr_name, intrfc_attr_val in interface.__dict__.items():
			_validate_attribute(
				cls=cls,
				intrfc_attr_name=intrfc_attr_name,
				intrfc_attr_val=intrfc_attr_val,
				intrfc_name=interface.__name__,
			)


class _InterfaceMeta(_ABCMeta):
	"""Metaclass to provide the expected functionality of the interfaces.

	Please do not create your own interface with this as the metaclass.
	Doing so will make it incompatible with other interfaces.

	Prefer to subclass :func:`BaseInterface`.
	"""

	def __new__(
		mcls, name: str, bases: tuple[type, ...], namespace: dict[str, _Any], **kwargs
	):
		# Determine if this new class conforms to interface rules BEFORE creation.
		is_interface: bool = _evaluate_if_interface(bases=bases, namespace=namespace)
		namespace[_INTERFACE_ATTR] = is_interface
		# Construct the class standardly.
		cls = super().__new__(mcls, name, bases, namespace, **kwargs)
		# If it has devolved into an implementation, validate its concrete methods.
		if not is_interface:
			_validate_implementation(cls)
		return cls

	def __call__(cls, *args, **kwargs) -> _Any:
		# Explicitly deny instantiation of any interface, even if it is entirely empty.
		if _getattr(cls, attr_name=_INTERFACE_ATTR):
			raise TypeError(f"Cannot instantiate pure interface '{cls.__name__}'.")
		return super().__call__(*args, **kwargs)

	def __subclasscheck__(cls, subclass: type) -> bool:
		# Disallow duck typing; the subclass must explicitly inherit from the interface.
		if cls not in subclass.__mro__:
			return False

		# If the subclass is itself an interface, standard inheritance applies.
		if _getattr(subclass, attr_name=_INTERFACE_ATTR):
			return True

		def _is_abstract_implemented(attr_name: str, /) -> bool:
			"""Check if the attribute is an abstract method and has been implemented.

			Parameters
			----------
			attr_name : str
				The name of the attribute in the base class to check.

			Returns
			-------
			False
				If an abstract method has not been implemented.
			True
				In all other cases.
			"""
			# Check if the class attribute is an abstract method.
			if not _getattr(_getattr(cls, attr_name=attr_name, default=None)):
				return True
			# Check if the matching subclass attribute is not abstract.
			return not _getattr(_getattr(subclass, attr_name=attr_name, default=None))

		# If subclass is a concrete implementation, it must fully override all
		# abstract methods mandated by all interfaces in the mro.
		return all(_is_abstract_implemented(attr_name) for attr_name in dir(cls))


class BaseInterface(metaclass=_InterfaceMeta):
	"""The root node for all strict C#-like interfaces.

	Inherit from this class to define an interface, interfaces can only contain
	abstract methods, abstract properties, and static methods. Interfaces can
	inherit from as many interfaces to combine attributes.

	An implementation is a class that inherits from one or interface and either defines
	non-abstract methods, non-abstract properties, or non-static methods.
	An implementation is allowed to be a pure-abstract class. An implementation can
	also inherit from other class types.

	Examples
	--------
	>>> # Define your interface using this as the base.
	>>> class IMyInterface(BaseInterface):
	... 	@abstractmethod
	... 	def my_method(self): raise NotImplementedError
	>>>
	>>> # Then apply it to an implementation.
	>>> class MyImplementation(IMyInterface):
	... 	def my_method(self):
	... 		# Implementation here
	... 		pass
	"""
