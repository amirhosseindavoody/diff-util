#!/usr/bin/env python3
"""Simple calculator module — original version."""

from typing import List, Optional


def add(a: float, b: float) -> float:
    return a + b


def subtract(a: float, b: float) -> float:
    return a - b


def multiply(a: float, b: float) -> float:
    return a * b


def divide(a: float, b: float) -> float:
    if b == 0:
        raise ValueError("Cannot divide by zero")
    return a / b


class Calculator:
    """Stateful calculator with a running total."""

    def __init__(self, initial: float = 0.0) -> None:
        self.total = initial
        self.history: List[str] = []

    def apply(self, op: str, value: float) -> float:
        if op == "add":
            self.total = add(self.total, value)
        elif op == "sub":
            self.total = subtract(self.total, value)
        elif op == "mul":
            self.total = multiply(self.total, value)
        elif op == "div":
            self.total = divide(self.total, value)
        else:
            raise ValueError(f"Unknown operation: {op}")

        self.history.append(f"{op}({value}) -> {self.total}")
        return self.total

    def reset(self) -> None:
        self.total = 0.0
        self.history.clear()


def format_result(value: float, precision: int = 2) -> str:
    return f"{value:.{precision}f}"


def main() -> None:
    calc = Calculator()
    calc.apply("add", 10)
    calc.apply("mul", 3)
    print(f"Result: {format_result(calc.total)}")


if __name__ == "__main__":
    main()
