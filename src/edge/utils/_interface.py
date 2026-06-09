"""Implements C#-style interfaces using a custom metaclass.

The metaclass checks if a subclass is an interface or implementation,
and validates an implementation's correctness.
"""

# TODO Look into static error checking.

from abc import ABCMeta
from collections.abc import Callable
from inspect import Signature, getattr_static, signature
from typing import Any, NoReturn


_ABSTRACTMETHOD_DUNDER = "__isabstractmethod__"
"""The dunder used by abc for defining abstract methods."""

_INTERFACE_ATTR = "_is_interface_"
"""The class attribute used to identify it as an interface."""


def _getattr(
	obj: object,
	/,
	attr_name: str = _ABSTRACTMETHOD_DUNDER,
	default: bool | None = False,
) -> Any | bool | None:
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


def _is_interface_attribute_valid(name: str, value: Any) -> bool:
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


def _evaluate_if_interface(bases: tuple[type, ...], namespace: dict[str, Any]) -> bool:
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

	# Object level field.
	cls_name: str = cls.__name__

	def _raise_signature_violation(
		reason: str, attr_name: str, expected: Any, provided: Any
	) -> NoReturn:
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
		intrfc_sig: Signature, impl_sig: Signature, attr_name: str
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
				attr_name=attr_name,
				expected=intrfc_sig.return_annotation,
				provided=impl_sig.return_annotation,
			)

		# Compare total parameter counts.
		if len(intrfc_sig.parameters) != len(impl_sig.parameters):
			_raise_signature_violation(
				reason="Parameter count mismatch",
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
					attr_name=attr_name,
					expected=intrfc_p.annotation,
					provided=impl_p.annotation,
				)

			# Prevent changing a positional argument into a keyword-only argument.
			if intrfc_p.kind != impl_p.kind:
				_raise_signature_violation(
					reason=f"Parameter '{intrfc_p.name}' kind mismatch",
					attr_name=attr_name,
					expected=intrfc_p.kind,
					provided=impl_p.kind,
				)

	def _validate_property_signatures(
		intrfc_prop: property, impl_prop: property, attr_name: str
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
			intrfc_access_func: Any, impl_access_func: Any, accessor_type: str
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
				int_sig: Signature = signature(obj=intrfc_access_func)
				imp_sig: Signature = signature(obj=impl_access_func)
			except (ValueError, TypeError) as e:
				raise TypeError(
					f"Un-inspectable property {accessor_type} "
					f"in '{cls_name}.{attr_name}': {e}"
				)

			_compare_signatures(
				intrfc_sig=int_sig,
				impl_sig=imp_sig,
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

	def _get_safe_signature(func: Any, error_msg: str) -> Signature:
		"""Safely attempts to extract an inspect.Signature."""

		try:
			return signature(func)
		except (ValueError, TypeError) as e:
			raise TypeError(f"{error_msg} {e}")

	def _unwrap_descriptor_callables(
		attr_name: str, intrfc_val: Any, impl_val: Any
	) -> tuple[Any, Any]:
		"""Ensures that the implementation respects the @classmethod or @staticmethod
		mandates of the interface, and returns the unwrapped underlying functions.
		"""

		if isinstance(intrfc_val, classmethod):
			if not isinstance(impl_val, classmethod):
				raise TypeError(
					f"Implementation error in '{cls_name}.{attr_name}': "
					f"Interface mandates a classmethod, but implementor provided "
					f"'{type(impl_val).__name__}'."
				)
			return intrfc_val.__func__, impl_val.__func__

		if isinstance(intrfc_val, staticmethod):
			if not isinstance(impl_val, staticmethod):
				raise TypeError(
					f"Implementation error in '{cls_name}.{attr_name}': "
					f"Interface mandates a staticmethod, but implementor provided "
					f"'{type(impl_val).__name__}'."
				)
			return intrfc_val.__func__, impl_val.__func__

		# If it is a standard method, return them as-is.
		return intrfc_val, impl_val

	def _validate_attribute(
		cls: type, intrfc_attr_name: str, intrfc_attr_val: Any, intrfc_name: str
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

		try:
			# Bypass the descriptor protocol (__get__) to get raw attributes.
			impl_attr_raw = getattr_static(cls, intrfc_attr_name)
		except AttributeError:
			# Missing entirely, ABC Meta will block instantiation later.
			return

		# If the implementer hasn't overridden it yet, it remains abstract.
		# Will also be blocked by ABC Meta.
		if _getattr(impl_attr_raw):
			return

		# Handle properties.
		if isinstance(intrfc_attr_val, property):
			if not isinstance(impl_attr_raw, property):
				raise TypeError(
					f"Implementation error in '{cls_name}.{intrfc_attr_name}': "
					f"Interface mandates a property, but implementor provided "
					f"'{type(impl_attr_raw).__name__}'."
				)

			_validate_property_signatures(
				intrfc_prop=intrfc_attr_val,
				impl_prop=impl_attr_raw,
				attr_name=intrfc_attr_name,
			)
			return

		intrfc_func: Callable
		impl_func: Callable
		intrfc_func, impl_func = _unwrap_descriptor_callables(
			attr_name=intrfc_attr_name,
			intrfc_val=intrfc_attr_val,
			impl_val=impl_attr_raw,
		)

		# Validate the Interface Signature.
		intrfc_sig: Signature = _get_safe_signature(
			func=intrfc_func,
			error_msg=f"Interface definition error: '{intrfc_name}.{intrfc_attr_name}' "
			f"is un-inspectable.",
		)

		# Validate the Implementation Signature.
		impl_sig: Signature = _get_safe_signature(
			func=impl_func,
			error_msg=f"Implementation error: '{cls_name}.{intrfc_attr_name}': "
			f"is un-inspectable and cannot be validated against the interface.\n"
			f"Ensure you are not directly assigning C-bindings "
			f"or using broken decorators.",
		)

		_compare_signatures(
			intrfc_sig=intrfc_sig,
			impl_sig=impl_sig,
			attr_name=intrfc_attr_name,
		)

	# ===================================
	# _validate_implementation execution.
	# ===================================
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


class _InterfaceMeta(ABCMeta):
	"""Metaclass to provide the expected functionality of the interfaces.

	Please do not create your own interface with this as the metaclass.
	Doing so will make it incompatible with other interfaces.

	Prefer to subclass :func:`BaseInterface`.
	"""

	# Cannot use typing.Self in a metaclass.

	def __new__(
		mcls, name: str, bases: tuple[type, ...], namespace: dict[str, Any], **kwargs
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

	def __call__(cls, *args, **kwargs) -> Any:
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
