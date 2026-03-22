from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING, Any, Callable

if TYPE_CHECKING:
    from .client import ApiClient


@dataclass
class ApiCallResult:
    request: dict[str, Any]
    response: dict[str, Any]
    duration: int


@dataclass
class ApiEndpointTest:
    category: str
    category_label: str
    name: str
    description: str
    test: Callable[[TestContext], ApiCallResult | list[ApiCallResult]]
    has_side_effect: bool = False
    needs_setup: str | None = None


class DataStore:
    def __init__(self) -> None:
        self._data: dict[str, Any] = {}

    def get(self, key: str, default: Any = None) -> Any:
        return self._data.get(key, default)

    def set(self, key: str, value: Any) -> None:
        self._data[key] = value

    def has(self, key: str) -> bool:
        return key in self._data


@dataclass
class TestContext:
    client: ApiClient
    store: DataStore
    run_side_effects: bool


@dataclass
class EndpointTestResult:
    endpoint: ApiEndpointTest
    calls: list[ApiCallResult]
    success: bool
    skipped: bool
    error: str | None = None


@dataclass
class TestSuiteResult:
    total: int
    passed: int
    failed: int
    skipped: int
    duration: int
    results: list[EndpointTestResult]
