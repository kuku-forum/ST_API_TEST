from __future__ import annotations
from ..models import ApiEndpointTest, TestContext, ApiCallResult

CATEGORY = "installedApps"
LABEL = "📲 설치된 앱 (Installed Apps)"


def _list(ctx: TestContext) -> ApiCallResult:
    result = ctx.client.request("GET", "/installedapps")
    if result.response["status"] == 200:
        items = (result.response["body"] or {}).get("items", [])
        if items:
            ctx.store.set("installedAppId", items[0]["installedAppId"])
    return result


def _get(ctx: TestContext) -> ApiCallResult:
    iid = ctx.store.get("installedAppId")
    if not iid:
        raise ValueError("installedAppId 없음")
    return ctx.client.request("GET", f"/installedapps/{iid}")


tests: list[ApiEndpointTest] = [
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "설치된 앱 목록 조회",
        "GET /installedapps — 설치된 앱 목록",
        _list,
    ),
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "특정 설치된 앱 상세 조회",
        "GET /installedapps/{id} — 설치된 앱 상세 정보",
        _get,
    ),
]
