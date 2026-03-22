from __future__ import annotations
from ..models import ApiEndpointTest, TestContext, ApiCallResult

CATEGORY = "schedules"
LABEL = "⏰ 스케줄 (Schedules)"


def _list(ctx: TestContext) -> ApiCallResult:
    iid = ctx.store.get("installedAppId")
    if not iid:
        raise ValueError("installedAppId 없음 — 설치된 앱 필요")
    return ctx.client.request("GET", f"/installedapps/{iid}/schedules")


tests: list[ApiEndpointTest] = [
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "스케줄 목록 조회",
        "GET /installedapps/{id}/schedules — 예약 실행 목록",
        _list,
        needs_setup="설치된 앱(installedApp)이 필요합니다",
    ),
]
