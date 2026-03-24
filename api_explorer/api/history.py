from __future__ import annotations
from ..models import ApiEndpointTest, TestContext, ApiCallResult

CATEGORY = "history"
LABEL = "📜 이벤트 기록 (History)"


def _device_history(ctx: TestContext) -> ApiCallResult:
    did = ctx.store.get("deviceId")
    lid = ctx.store.get("locationId")
    if not did or not lid:
        raise ValueError("deviceId 또는 locationId 없음")
    return ctx.client.request(
        "GET", "/history/devices", query={"locationId": lid, "deviceId": did}
    )


tests: list[ApiEndpointTest] = [
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "디바이스 이벤트 기록 조회",
        "GET /history/devices — 최근 디바이스 이벤트",
        _device_history,
    ),
]
