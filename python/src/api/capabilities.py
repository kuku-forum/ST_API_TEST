from __future__ import annotations
from ..models import ApiEndpointTest, TestContext, ApiCallResult

CATEGORY = "capabilities"
LABEL = "⚡ 기능 (Capabilities)"


def _namespaces(ctx: TestContext) -> ApiCallResult:
    return ctx.client.request("GET", "/capabilities/namespaces")


def _switch(ctx: TestContext) -> ApiCallResult:
    return ctx.client.request("GET", "/capabilities/switch/1")


def _device_cap(ctx: TestContext) -> ApiCallResult:
    caps = ctx.store.get("deviceCapabilities", [])
    cap_id = caps[0] if caps else "temperatureMeasurement"
    return ctx.client.request("GET", f"/capabilities/{cap_id}/1")


tests: list[ApiEndpointTest] = [
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "기능 네임스페이스 목록 조회",
        "GET /capabilities/namespaces — 기능 네임스페이스 목록",
        _namespaces,
    ),
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "switch 기능 상세 조회",
        "GET /capabilities/switch/1 — 표준 switch 기능 정의",
        _switch,
    ),
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "디바이스 기반 기능 조회",
        "첫 번째 디바이스의 첫 기능 상세 조회",
        _device_cap,
    ),
]
