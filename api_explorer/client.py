from __future__ import annotations

import time
import requests as req

from .models import ApiCallResult

BASE_URL = "https://api.smartthings.com/v1"


class ApiClient:
    def __init__(self, token: str) -> None:
        self.session = req.Session()
        self.session.headers.update(
            {"Authorization": f"Bearer {token}", "Accept": "application/json"}
        )

    def request(
        self,
        method: str,
        path: str,
        *,
        body: object = None,
        query: dict[str, str] | None = None,
    ) -> ApiCallResult:
        url = f"{BASE_URL}{path}"

        kwargs: dict = {"params": query}
        if body is not None:
            kwargs["json"] = body

        start = time.perf_counter()
        resp = self.session.request(method, url, **kwargs)
        duration = round((time.perf_counter() - start) * 1000)

        try:
            resp_body = resp.json()
        except (ValueError, req.JSONDecodeError):
            resp_body = resp.text or None

        return ApiCallResult(
            request={"method": method, "url": str(resp.url), "body": body},
            response={
                "status": resp.status_code,
                "status_text": resp.reason,
                "body": resp_body,
            },
            duration=duration,
        )
