from tkinter import ttk as ttk
from typing import Self as Self

from edge.settings import SETTINGS
from edge.typing import ButtonCommand


class Button:
	"""A wrapper for the tkinter button to provide extra functionality and control.

	Should only be created using the class factory methods :func:`simple_button`
	and :func:`complex_button`.
	"""

	@property
	def label(self) -> str:
		return self._label

	@label.setter
	def label(self, val: str) -> None:
		self._label = val
		self._button.configure(text=val)

	@property
	def order(self) -> int:
		return self._order

	@order.setter
	def order(self, val: int) -> None:
		self._order = val
		self._button.grid(row=val, column=0, sticky="news", padx=5, pady=5)

	@property
	def button(self) -> ttk.Button:
		return self._button

	def __init__(
		self, input_panel: ttk.Frame, label: str, order: int, command: ButtonCommand
	) -> None:
		self._label: str = label
		self._button: ttk.Button = ttk.Button(
			master=input_panel,
			text=label,
			command=command,
			width=SETTINGS.INPUT_BUTTON_WIDTH,
		)
		self.order = order
		input_panel.rowconfigure(index=order, weight=0)
