from __future__ import annotations
from ..models import ApiEndpointTest, TestContext, ApiCallResult

CATEGORY = "devices"
LABEL = "📱 디바이스 (Devices)"


def _list(ctx: TestContext) -> ApiCallResult:
    result = ctx.client.request("GET", "/devices")
    if result.response["status"] == 200:
        items = (result.response["body"] or {}).get("items", [])
        if items:
            dev = items[0]
            ctx.store.set("deviceId", dev["deviceId"])
            ctx.store.set("deviceLabel", dev.get("label", dev["deviceId"]))
            caps = [
                c["id"]
                for c in (dev.get("components") or [{}])[0].get("capabilities", [])
            ]
            ctx.store.set("deviceCapabilities", caps)
    return result


def _get(ctx: TestContext) -> ApiCallResult:
    did = ctx.store.get("deviceId")
    if not did:
        raise ValueError("deviceId 없음")
    return ctx.client.request("GET", f"/devices/{did}")


def _status(ctx: TestContext) -> ApiCallResult:
    did = ctx.store.get("deviceId")
    if not did:
        raise ValueError("deviceId 없음")
    return ctx.client.request("GET", f"/devices/{did}/status")


def _health(ctx: TestContext) -> ApiCallResult:
    did = ctx.store.get("deviceId")
    if not did:
        raise ValueError("deviceId 없음")
    return ctx.client.request("GET", f"/devices/{did}/health")


def _refresh(ctx: TestContext) -> ApiCallResult:
    did = ctx.store.get("deviceId")
    if not did:
        raise ValueError("deviceId 없음")
    return ctx.client.request(
        "POST",
        f"/devices/{did}/commands",
        body={
            "commands": [
                {"component": "main", "capability": "refresh", "command": "refresh"}
            ]
        },
    )


tests: list[ApiEndpointTest] = [
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "디바이스 목록 조회",
        "GET /devices — 모든 디바이스 목록",
        _list,
    ),
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "특정 디바이스 상세 조회",
        "GET /devices/{id} — 디바이스 상세 정보",
        _get,
    ),
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "디바이스 전체 상태 조회",
        "GET /devices/{id}/status — 모든 컴포넌트/기능 상태",
        _status,
    ),
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "디바이스 연결 상태 조회",
        "GET /devices/{id}/health — ONLINE/OFFLINE 상태",
        _health,
    ),
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "디바이스 명령 실행 (refresh)",
        "POST /devices/{id}/commands — refresh 명령 (안전)",
        _refresh,
        has_side_effect=True,
    ),
]
