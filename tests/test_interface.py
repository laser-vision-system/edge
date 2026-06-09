"""In-depth tests for the interface module."""

from abc import abstractmethod

import pytest

from edge.utils import BaseInterface


# === Interface & Implementation Definitions ===


# 1. Base Interfaces
class IRoot(BaseInterface):
	@abstractmethod
	def root_method(self) -> str:
		raise NotImplementedError


class IOther(BaseInterface):
	@abstractmethod
	def other_method(self, flag: bool, /) -> None:
		raise NotImplementedError


# 2. Sub-Interfaces
class IBranch(IRoot):
	"""An interface extending another interface."""

	@abstractmethod
	def branch_method(self, val: int, /) -> int:
		raise NotImplementedError


class IMultiBranch(IBranch, IOther):
	"""An interface extending multiple interfaces."""


# 3. Sub-Interface Implementers
class ValidBranchImplementer(IBranch):
	"""Successfully implements all methods in the interface chain."""

	def root_method(self) -> str:
		return "root"

	def branch_method(self, val: int, /) -> int:
		return val * 2


class ValidMultiImplementer(IMultiBranch):
	"""Successfully implements a complex multiple inheritance matrix."""

	def root_method(self) -> str:
		return "root"

	def branch_method(self, val: int, /) -> int:
		return val

	def other_method(self, flag: bool, /) -> None:
		pass


# 4. Partial & Fake Implementers
class FakeImplementer(IBranch):
	"""Inherits the interface but provides no implementations."""

	def random_method(self) -> None:
		pass


class PartialImplementer(IBranch):
	"""Implements the root interface method, but misses the branch interface method."""

	def root_method(self) -> str:
		return "root"


# 5. Abstract Non-Interface & Implementation
class AbstractNonInterface(IRoot):
	"""Breaks interface rules by adding a concrete method, becoming a standard ABC.
	It does not implement root_method, remaining abstract."""

	def concrete_helper(self) -> str:
		return "helper"


class ConcreteNonInterface(AbstractNonInterface):
	"""Successfully implements the interface mandated by its abstract parent."""

	def root_method(self) -> str:
		return "concrete root"


# 6. Edge Cases (Properties and Static Methods)
class IEdgeCases(BaseInterface):
	"""An interface defining a static method and
	a static property with all accessors."""

	@staticmethod
	@abstractmethod
	def abstract_static(x: int, /) -> int:
		raise NotImplementedError

	@classmethod
	@abstractmethod
	def abstract_class(cls, x: int, /) -> int:
		raise NotImplementedError

	@property
	@abstractmethod
	def abstract_prop(self) -> str:
		raise NotImplementedError

	@abstract_prop.setter
	@abstractmethod
	def abstract_prop(self, val: str) -> None:
		raise NotImplementedError

	@abstract_prop.deleter
	@abstractmethod
	def abstract_proper(self) -> None:
		raise NotImplementedError


class EdgeCaseImplementer(IEdgeCases):
	"""Successfully implements the interface mandated by its abstract parent."""

	def __init__(self) -> None:
		self._prop: str = "property"

	@staticmethod
	def abstract_static(x: int, /) -> int:
		return x

	@classmethod
	def abstract_class(cls, x: int, /) -> int:
		return x

	@property
	def abstract_prop(self) -> str:
		return self._prop

	@abstract_prop.setter
	def abstract_prop(self, val: str) -> None:
		self._prop = val

	@abstract_prop.deleter
	def abstract_proper(self) -> None:
		del self._prop


# === Tests ===


class TestInterfaceArchitecture:
	"""Full test suite for the interface component."""

	# --- Sub-Interfaces & Hierarchies ---

	def test_interface_hierarchy_subclassing(self):
		"""Validates that interfaces recognise their own inheritance chains."""
		assert issubclass(IBranch, IRoot)
		assert issubclass(IMultiBranch, IRoot)
		assert issubclass(IMultiBranch, IOther)

	def test_sub_interface_implementer_subclassing(self):
		"""Validates a complete implementer resolves
		as a subclass to the entire chain."""
		assert issubclass(ValidBranchImplementer, IBranch)
		assert issubclass(ValidBranchImplementer, IRoot)

	def test_sub_interface_implementer_instantiation(self):
		"""Validates a complete implementer instantiates
		and resolves types correctly."""
		assert isinstance(imp := ValidBranchImplementer(), ValidBranchImplementer)
		assert isinstance(imp, IBranch)
		assert isinstance(imp, IRoot)

	# --- Multiple Inheritance ---

	def test_multiple_interface_implementer(self):
		"""Validates that an implementer fulfilling
		multiple interfaces resolves all types."""
		assert isinstance(imp := ValidMultiImplementer(), IMultiBranch)
		assert isinstance(imp, IBranch)
		assert isinstance(imp, IRoot)
		assert isinstance(imp, IOther)
		assert issubclass(ValidMultiImplementer, IOther)

	# --- Partial & Fake Implementers ---

	def test_fake_implementer_subclassing(self):
		"""An explicitly inheriting class that implements
		nothing is actively denied subclass status."""
		assert not issubclass(FakeImplementer, IBranch)
		assert not issubclass(FakeImplementer, IRoot)

	def test_partial_implementer_subclassing(self):
		"""A class that partially fulfils the interface is still
		denied subclass status for the interfaces it misses."""
		assert issubclass(PartialImplementer, IRoot)
		# It implemented IRoot, but it is evaluated against IBranch's requirements.
		assert not issubclass(PartialImplementer, IBranch)

	def test_incomplete_instantiation_blocked(self):
		"""Validates standard ABC protection against
		instantiating incomplete classes."""
		with pytest.raises(expected_exception=TypeError):
			_ = FakeImplementer()  # type: ignore

		with pytest.raises(expected_exception=TypeError):
			_ = PartialImplementer()  # type: ignore

	# --- Abstract Non-Interfaces ---

	def test_abstract_non_interface_classification(self):
		"""Validates that introducing concrete states
		strips the interface classification."""
		assert getattr(AbstractNonInterface, "_is_interface_") is False

	def test_abstract_non_interface_subclassing(self):
		"""An abstract non-interface is incomplete, and therefore
		not a valid subclass of the interface."""
		assert not issubclass(AbstractNonInterface, IRoot)

	def test_concrete_non_interface_subclassing(self):
		"""When a standard class finally fulfils the interface mandated by its
		abstract parent, it is rightfully granted subclass status."""
		# Note: I love this docstring a lot.
		assert issubclass(ConcreteNonInterface, IRoot)

	def test_concrete_non_interface_instantiation(self):
		assert isinstance(imp := ConcreteNonInterface(), ConcreteNonInterface)
		assert isinstance(imp, IRoot)
		assert imp.concrete_helper() == "helper"

	# --- Signature & Structure Violations ---

	def test_rogue_implementer_signature_validation(self):
		"""Validates that altering a signature parameter
		type immediately crashes compilation."""
		with pytest.raises(expected_exception=TypeError):

			class Rogueimplementer(IBranch):
				def root_method(self) -> str:
					return "root"

				def branch_method(self, val: str, /) -> int:  # type: ignore
					return 0

	def test_rogue_implementer_return_validation(self):
		"""Validates that altering a return type immediately crashes compilation."""
		with pytest.raises(expected_exception=TypeError):

			class RogueReturn(IRoot):
				def root_method(self) -> int:  # type: ignore
					return 1

	# --- Duck Typing (Explicitly Denied) ---

	def test_duck_typing_disallowed(self):
		"""Structurally identical classes that do not explicitly inherit from
		the interface must actively fail isinstance and issubclass checks."""

		class DuckDuckGoose:
			def root_method(self) -> str:
				return "quack"

		duck = DuckDuckGoose()
		assert not isinstance(duck, IRoot)
		assert not issubclass(DuckDuckGoose, IRoot)

	# --- Edge Cases (Properties and Static Methods) ---

	def test_edge_case_implementer_subclassing(self):
		"""Validates properties and static methods are correctly
		processed without signature evaluation crashing."""
		assert issubclass(EdgeCaseImplementer, IEdgeCases)

	def test_edge_case_implementer_instantiation(self):
		"""Validates properties and static methods
		function correctly under the interface."""
		assert isinstance(imp := EdgeCaseImplementer(), IEdgeCases)
		assert EdgeCaseImplementer.abstract_static(x := 5) == x
		assert EdgeCaseImplementer.abstract_class(x := 5) == x
		assert imp.abstract_prop == "property"
		imp.abstract_prop = (new_text := "other")
		assert imp.abstract_prop == new_text
