"""SmartThings LLM Tool 기반 클래스.

LLM agent의 tool calling을 위한 추상 기반 클래스와 결과 타입을 정의한다.
프레임워크 비종속 — OpenAI, Anthropic, LangChain 등 어디서든 사용 가능.
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import asdict, dataclass, field
from typing import Any, ClassVar

from pydantic import BaseModel


@dataclass
class ToolResult:
    """도구 실행 결과.

    모든 도구는 성공/실패에 관계없이 이 타입을 반환한다.
    JSON 직렬화 가능하며, LLM에게 결과를 전달하기 위한 표준 포맷이다.

    Attributes:
        success: 실행 성공 여부.
        data: 성공 시 반환 데이터. dict, list, str 등 JSON 직렬화 가능한 값.
        error: 실패 시 에러 메시지.
    """

    success: bool
    data: Any = None
    error: str | None = None

    def to_dict(self) -> dict[str, Any]:
        """LLM에게 전달하기 위한 dict 변환."""
        return asdict(self)


class BaseTool(ABC):
    """모든 SmartThings 도구의 추상 기반 클래스.

    외부 프로젝트에서는 이 인터페이스만 참조하면 된다:
        - name: 도구 고유 이름 (LLM이 호출 시 사용)
        - description: 도구 설명 (LLM이 도구 선택 시 참고)
        - args_schema: Pydantic BaseModel — 입력 파라미터 스키마
        - execute(**kwargs) -> ToolResult: 도구 실행
        - to_openai_tool() -> dict: OpenAI function calling 형식 변환
        - to_anthropic_tool() -> dict: Anthropic tool use 형식 변환

    사용 예시::

        class MyTool(BaseTool):
            name = "my_tool"
            description = "설명"
            args_schema = MyInput

            def execute(self, **kwargs: Any) -> ToolResult:
                data = self.client.get("/some/path")
                return ToolResult(success=True, data=data)

        tool = MyTool(client)
        result = tool.validate_and_execute({"param": "value"})
    """

    name: ClassVar[str]
    description: ClassVar[str]
    args_schema: ClassVar[type[BaseModel]]

    def __init__(self, client: Any) -> None:
        """도구를 초기화한다.

        Args:
            client: SmartThingsClient 인스턴스.
        """
        self.client = client

    @abstractmethod
    def execute(self, **kwargs: Any) -> ToolResult:
        """도구를 실행한다.

        Args:
            **kwargs: args_schema의 필드와 1:1 대응하는 키워드 인자.

        Returns:
            ToolResult: 실행 결과.
        """
        ...

    def validate_and_execute(self, raw_args: dict[str, Any]) -> ToolResult:
        """LLM이 전달한 raw JSON을 Pydantic으로 검증 후 실행한다.

        Args:
            raw_args: LLM이 생성한 도구 호출 인자 (미검증 dict).

        Returns:
            ToolResult: 검증 실패 시 error가 포함된 결과, 성공 시 execute() 결과.
        """
        try:
            validated = self.args_schema.model_validate(raw_args)
        except Exception as e:
            return ToolResult(success=False, error=f"ValidationError: {e}")
        try:
            return self.execute(**validated.model_dump())
        except TypeError:
            return self.execute(validated)

    def to_function_schema(self) -> dict[str, Any]:
        """범용 function schema. OpenAI/Anthropic 공통 기반.

        Returns:
            name, description, parameters를 포함하는 dict.
        """
        return {
            "name": self.name,
            "description": self.description,
            "parameters": self.args_schema.model_json_schema(),
        }

    def to_openai_tool(self) -> dict[str, Any]:
        """OpenAI function calling 형식으로 변환.

        Returns:
            ``{"type": "function", "function": {...}}`` 형식의 dict.
        """
        return {
            "type": "function",
            "function": self.to_function_schema(),
        }

    def to_anthropic_tool(self) -> dict[str, Any]:
        """Anthropic tool use 형식으로 변환.

        Returns:
            ``{"name": ..., "description": ..., "input_schema": {...}}`` 형식의 dict.
        """
        return {
            "name": self.name,
            "description": self.description,
            "input_schema": self.args_schema.model_json_schema(),
        }

    def __repr__(self) -> str:
        return f"<{type(self).__name__} name={self.name!r}>"
