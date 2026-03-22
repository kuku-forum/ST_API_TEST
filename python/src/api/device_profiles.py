from __future__ import annotations
from ..models import ApiEndpointTest, TestContext, ApiCallResult

CATEGORY = "deviceProfiles"
LABEL = "🔧 디바이스 프로필 (Device Profiles)"


def _list(ctx: TestContext) -> ApiCallResult:
    result = ctx.client.request("GET", "/deviceprofiles")
    if result.response["status"] == 200:
        items = (result.response["body"] or {}).get("items", [])
        if items:
            ctx.store.set("deviceProfileId", items[0]["id"])
    return result


def _get(ctx: TestContext) -> ApiCallResult:
    pid = ctx.store.get("deviceProfileId")
    if not pid:
        raise ValueError("등록된 프로필 없음 — 목록이 비어있습니다")
    return ctx.client.request("GET", f"/deviceprofiles/{pid}")


tests: list[ApiEndpointTest] = [
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "디바이스 프로필 목록 조회",
        "GET /deviceprofiles — 등록된 프로필 목록",
        _list,
    ),
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "특정 디바이스 프로필 조회",
        "GET /deviceprofiles/{id} — 프로필 상세 정보",
        _get,
        needs_setup="등록된 디바이스 프로필이 필요합니다",
    ),
]
