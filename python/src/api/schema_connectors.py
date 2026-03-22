from __future__ import annotations
from ..models import ApiEndpointTest, TestContext, ApiCallResult

CATEGORY = "schema"
LABEL = "🔌 스키마 커넥터 (Schema)"


def _list(ctx: TestContext) -> ApiCallResult:
    return ctx.client.request("GET", "/schema/apps")


tests: list[ApiEndpointTest] = [
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "스키마 앱 목록 조회",
        "GET /schema/apps — ST Schema C2C 커넥터 목록",
        _list,
    ),
]
