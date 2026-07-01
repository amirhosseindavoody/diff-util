#!/usr/bin/env python3
"""Simple calculator module — refactored version."""

from dataclasses import dataclass, field
from typing import Callable, Dict, List


@dataclass
class Operation:
    name: str
    fn: Callable[[float, float], float]
    symbol: str


OPERATIONS: Dict[str, Operation] = {}


def register(name: str, symbol: str):
    def decorator(fn: Callable[[float, float], float]) -> Callable[[float, float], float]:
        OPERATIONS[name] = Operation(name=name, fn=fn, symbol=symbol)
        return fn

    return decorator


@register("add", "+")
def add(a: float, b: float) -> float:
    return a + b


@register("sub", "-")
def subtract(a: float, b: float) -> float:
    return a - b


@register("mul", "×")
def multiply(a: float, b: float) -> float:
    return a * b


@register("div", "÷")
def divide(a: float, b: float) -> float:
    if b == 0:
        raise ZeroDivisionError("division by zero")
    return a / b


@dataclass
class Calculator:
    """Stateful calculator with a running total and undo support."""

    total: float = 0.0
    history: List[str] = field(default_factory=list)

    def apply(self, op: str, value: float) -> float:
        operation = OPERATIONS.get(op)
        if operation is None:
            raise ValueError(f"Unknown operation: {op!r}")

        previous = self.total
        self.total = operation.fn(self.total, value)
        self.history.append(f"{operation.symbol} {value}  ({previous} → {self.total})")
        return self.total

    def undo(self) -> float:
        if not self.history:
            return self.total
        self.history.pop()
        # Recompute from scratch for simplicity in this demo.
        self.total = 0.0
        return self.total

    def reset(self) -> None:
        self.total = 0.0
        self.history.clear()


def format_result(value: float, *, precision: int = 4, suffix: str = "") -> str:
    formatted = f"{value:.{precision}f}".rstrip("0").rstrip(".")
    return f"{formatted}{suffix}"


def main() -> None:
    calc = Calculator()
    for op, val in [("add", 10), ("mul", 3), ("div", 2)]:
        calc.apply(op, val)
    print(f"Result: {format_result(calc.total, suffix=' units')}")


if __name__ == "__main__":
    main()
