"""SmartThings LLM Tool Package.

LLM agent가 SmartThings 디바이스를 제어하기 위한 도구 모음.
Pydantic BaseModel 기반 스키마 + class 기반 도구 구조.
프레임워크 비종속 — OpenAI, Anthropic, LangChain 등 어디서든 사용 가능.

사용 예시::

    from tools import SmartThingsToolkit

    toolkit = SmartThingsToolkit(token="your-pat-token")

    # OpenAI function calling 형식
    schemas = toolkit.to_openai_tools()

    # 도구 실행
    result = toolkit.execute("switch_power", device_id="xxx", state="on")
"""

from __future__ import annotations

from .base import BaseTool, ToolResult
from .client import SmartThingsClient

__all__ = [
    "SmartThingsToolkit",
    "BaseTool",
    "ToolResult",
    "SmartThingsClient",
]


class SmartThingsToolkit:
    """SmartThings 도구 모음. 외부 프로젝트에서 이것만 import 하면 된다.

    Args:
        token: SmartThings PAT 토큰.
        include_extended: True 시 사용자 미보유 디바이스 도구도 포함.
    """

    def __init__(self, token: str, *, include_extended: bool = False) -> None:
        self.client = SmartThingsClient(token)
        self._tools: list[BaseTool] = []
        self._register_tools(include_extended)

    def _register_tools(self, include_extended: bool) -> None:
        from .common_tools import get_common_tools
        from .my_devices import get_my_device_tools

        for tool_cls in get_common_tools():
            self._tools.append(tool_cls(self.client))

        for tool_cls in get_my_device_tools():
            self._tools.append(tool_cls(self.client))

        if include_extended:
            from .extended_devices import get_extended_device_tools

            for tool_cls in get_extended_device_tools():
                self._tools.append(tool_cls(self.client))

    def get_tools(self) -> list[BaseTool]:
        return list(self._tools)

    def get_tool(self, name: str) -> BaseTool | None:
        return next((t for t in self._tools if t.name == name), None)

    def list_tool_names(self) -> list[str]:
        return [t.name for t in self._tools]

    def execute(self, tool_name: str, **kwargs) -> ToolResult:
        """도구를 이름으로 찾아 실행한다."""
        tool = self.get_tool(tool_name)
        if tool is None:
            return ToolResult(success=False, error=f"Unknown tool: {tool_name}")
        return tool.validate_and_execute(kwargs)

    def to_openai_tools(self) -> list[dict]:
        return [t.to_openai_tool() for t in self._tools]

    def to_anthropic_tools(self) -> list[dict]:
        return [t.to_anthropic_tool() for t in self._tools]

    def to_function_schemas(self) -> list[dict]:
        return [t.to_function_schema() for t in self._tools]
