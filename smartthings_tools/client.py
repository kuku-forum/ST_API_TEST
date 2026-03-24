"""SmartThings REST API HTTP 클라이언트.

SmartThings API v1에 대한 동기식 HTTP 래퍼.
requests.Session 기반이며, Bearer 토큰 인증을 자동 처리한다.
"""

from __future__ import annotations

from typing import Any

import requests

BASE_URL = "https://api.smartthings.com/v1"


class SmartThingsClient:
    """SmartThings REST API 클라이언트.

    Args:
        token: SmartThings Personal Access Token (PAT).

    사용 예시::

        client = SmartThingsClient(token="your-pat-token")
        devices = client.get("/devices")
        client.post("/devices/{id}/commands", body={...})
    """

    def __init__(self, token: str) -> None:
        self._session = requests.Session()
        self._session.headers.update(
            {
                "Authorization": f"Bearer {token}",
                "Accept": "application/json",
                "Content-Type": "application/json",
            }
        )

    def request(
        self,
        method: str,
        path: str,
        *,
        body: dict[str, Any] | None = None,
        params: dict[str, str] | None = None,
    ) -> dict[str, Any]:
        """HTTP 요청을 전송하고 JSON 응답을 반환한다.

        API 에러 시 예외를 발생시키지 않고, 에러 응답 body를 그대로 반환한다.
        호출자(Tool)가 응답을 해석하여 ToolResult로 변환해야 한다.

        Args:
            method: HTTP 메서드 (GET, POST, PUT, DELETE).
            path: API 경로 (예: "/devices", "/devices/{id}/commands").
            body: POST/PUT 요청 본문.
            params: URL 쿼리 파라미터.

        Returns:
            API 응답 JSON을 dict로 변환한 값.
            파싱 실패 시 ``{"error": "...", "status_code": N}`` 형태.
        """
        url = f"{BASE_URL}{path}"

        kwargs: dict[str, Any] = {}
        if params:
            kwargs["params"] = params
        if body is not None:
            kwargs["json"] = body

        try:
            resp = self._session.request(method, url, **kwargs)
        except requests.RequestException as e:
            return {"error": str(e), "status_code": 0}

        try:
            data = resp.json()
        except (ValueError, requests.JSONDecodeError):
            data = {"raw": resp.text or None}

        if not resp.ok:
            data["status_code"] = resp.status_code

        return data

    def get(
        self,
        path: str,
        *,
        params: dict[str, str] | None = None,
    ) -> dict[str, Any]:
        """GET 요청."""
        return self.request("GET", path, params=params)

    def post(
        self,
        path: str,
        *,
        body: dict[str, Any] | None = None,
    ) -> dict[str, Any]:
        """POST 요청."""
        return self.request("POST", path, body=body)

    def command(
        self,
        device_id: str,
        capability: str,
        command: str,
        arguments: list[Any] | None = None,
        *,
        component: str = "main",
    ) -> dict[str, Any]:
        """디바이스 커맨드 전송 shortcut.

        ``POST /devices/{device_id}/commands`` 를 간편하게 호출한다.

        Args:
            device_id: 대상 디바이스 UUID.
            capability: capability 이름 (예: "switch", "switchLevel").
            command: 명령 이름 (예: "on", "off", "setLevel").
            arguments: 명령 인자 리스트 (예: [75]).
            component: 디바이스 컴포넌트 (기본값 "main").
        """
        return self.post(
            f"/devices/{device_id}/commands",
            body={
                "commands": [
                    {
                        "component": component,
                        "capability": capability,
                        "command": command,
                        "arguments": arguments or [],
                    }
                ]
            },
        )
