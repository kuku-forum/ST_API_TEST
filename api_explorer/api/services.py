from __future__ import annotations
from ..models import ApiEndpointTest, TestContext, ApiCallResult

CATEGORY = "services"
LABEL = "🌤️ 서비스 (Services)"


def _weather(ctx: TestContext) -> ApiCallResult | list[ApiCallResult]:
    lid = ctx.store.get("locationId")
    if not lid:
        raise ValueError("locationId 없음")
    result = ctx.client.request("GET", f"/services/coordinate/locations/{lid}/weather")
    if result.response["status"] == 404:
        alt = ctx.client.request("GET", f"/locations/{lid}/weather")
        return [result, alt]
    return result


tests: list[ApiEndpointTest] = [
    ApiEndpointTest(
        CATEGORY,
        LABEL,
        "날씨 정보 조회",
        "GET /services/coordinate/locations/{id}/weather — 현재 날씨",
        _weather,
    ),
]
