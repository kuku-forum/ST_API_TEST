from __future__ import annotations
from ..models import ApiEndpointTest, TestContext, ApiCallResult

CATEGORY = "subscriptions"
LABEL = "🔔 구독 (Subscriptions)"


def _list(ctx: TestContext) -> ApiCallResult:
    iid = ctx.store.get("installedAppId")
    if not iid:
        raise ValueError("installedAppId 없음 — 설치된 앱 필요")
    return ctx.client.request("GET", f"/installedapps/{iid}/subscriptions")


tests: list[ApiEndpointTest] = [
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "구독 목록 조회",
        "GET /installedapps/{id}/subscriptions — 이벤트 구독 목록",
        _list,
        needs_setup="설치된 앱(installedApp)이 필요합니다",
    ),
]
