# 08. 프로젝트 전체 구조 분석 (Walkthrough)

이 문서에서는 `smartthings_tools_rs` 프로젝트의 전체적인 코드 흐름과 파일별 역할을 종합적으로 살펴봅니다. 데이터가 어떻게 흐르고, 각 모듈이 어떻게 협력하는지 이해하는 것이 목표입니다.

## 1. 전체 구조도

```text
[사용자/LLM]
      ↓
[SmartThingsToolkit] (lib.rs)
      ↓
      ├─ [SmartThingsClient] (client.rs) ──→ [SmartThings API]
      │
      └─ [Tools] (common_tools.rs, my_devices/*.rs, ...)
            ↓
            ├─ [Tool Trait] (tool.rs)
            └─ [Schemas] (schemas/*.rs)
```

---

## 2. 파일별 역할 상세

### 1) `src/lib.rs` (진입점)
프로젝트의 메인 인터페이스인 `SmartThingsToolkit` 구조체를 정의합니다.
*   **역할**: 도구들을 생성하고 관리하며, 외부(LLM 등)에서 도구를 실행하거나 스키마를 조회할 수 있는 통합 창구 역할을 합니다.
*   **핵심 메서드**:
    *   `new()`: 클라이언트를 초기화하고 툴킷을 생성합니다.
    *   `get_tools()`: 등록된 모든 도구 인스턴스를 생성하여 반환합니다. 이 메서드 내부에서 `Box::new(...)`를 통해 각 도구 객체를 힙에 할당하고 트레이트 객체로 변환합니다.
    *   `execute()`: 도구 이름을 받아 해당 도구의 `execute` 메서드를 호출합니다. 내부적으로 `find`를 사용하여 이름이 일치하는 도구를 찾습니다.
    *   `to_openai_tools()`: 모든 도구의 스키마를 OpenAI Function Calling 형식으로 변환하여 반환합니다.

### 2) `src/tool.rs` (추상화)
도구의 공통 규격인 `Tool` 트레이트와 실행 결과인 `ToolResult`를 정의합니다.
*   **역할**: 모든 도구가 동일한 인터페이스를 가지도록 강제하며, 성공/실패 시 일관된 응답 형식을 보장합니다.
*   **핵심**: `Tool` 트레이트 덕분에 `SmartThingsToolkit`은 구체적인 도구 타입(예: `ListDevicesTool`)을 몰라도 `dyn Tool`이라는 추상화된 타입으로 도구를 다룰 수 있습니다. 이는 객체 지향 언어의 인터페이스 다형성과 같습니다.

### 3) `src/client.rs` (통신)
SmartThings API와 실제로 통신하는 `SmartThingsClient`를 정의합니다.
*   **역할**: HTTP 요청(GET, POST)을 보내고, 인증 헤더를 설정하며, 응답 JSON을 파싱합니다.
*   **핵심**: `ureq` 라이브러리를 사용하여 동기식으로 통신합니다. `request` 메서드 하나에서 모든 HTTP 메서드를 처리하며, `parse_response`를 통해 일관된 `HashMap<String, Value>` 형태로 결과를 반환합니다.

### 4) `src/schemas/` (데이터 모델)
도구의 입력값 형식을 정의하는 구조체들이 모여 있습니다.
*   **역할**: `serde`를 통해 JSON 데이터를 Rust 구조체로 변환하고, `schemars`를 통해 LLM용 JSON 스키마를 생성합니다.
*   **파일 구성**:
    *   `common.rs`: 공통 도구(목록 조회, 상태 조회 등)용 스키마.
    *   `my_devices.rs`: 조명, 스위치, TV 등 사용자 디바이스 제어용 스키마.
    *   `extended_devices.rs`: 로봇청소기, 도어록 등 확장 디바이스용 스키마.

### 5) `src/common_tools.rs`, `src/my_devices/`, `src/extended_devices/` (구현체)
실제 SmartThings 기능을 수행하는 개별 도구들이 구현되어 있습니다.
*   **역할**: `Tool` 트레이트를 구현하며, 각 도구만의 고유한 로직(API 경로 설정, 데이터 가공 등)을 담고 있습니다.
*   **특징**: 모든 도구는 `SmartThingsClient`를 참조(`&'a SmartThingsClient`)로 받아 사용합니다. 이는 클라이언트 객체를 매번 생성하지 않고 하나를 공유하여 효율적으로 사용하기 위함입니다.

---

## 3. 데이터 흐름 예시: `list_devices` 실행

1.  **호출**: 사용자가 `toolkit.execute("list_devices", json!({}))`를 호출합니다.
2.  **도구 찾기**: `SmartThingsToolkit`은 `get_tools()`를 호출하여 모든 도구 인스턴스를 생성하고, 그 중에서 이름이 "list_devices"인 도구를 찾습니다.
3.  **실행**: 찾은 도구의 `execute(args)` 메서드를 호출합니다.
4.  **검증**: 도구 내부에서 `serde_json::from_value(args)`를 호출합니다. 이때 입력값이 `ListDevicesInput` 구조체의 정의와 맞지 않으면 즉시 에러를 반환합니다.
5.  **API 요청**: 검증된 입력값을 바탕으로 `client.get("/devices", params)`를 호출합니다. 클라이언트는 PAT 토큰을 헤더에 실어 SmartThings 서버로 요청을 보냅니다.
6.  **데이터 가공**: 서버로부터 받은 전체 디바이스 목록에서 LLM에게 필요한 핵심 정보(ID, 라벨, 기능 목록 등)만 추출하여 정제된 JSON 데이터를 만듭니다.
7.  **결과 반환**: 정제된 데이터를 `ToolResult::ok(data)`에 담아 최종적으로 반환합니다.

---

## 4. 새로운 도구 추가하기 (How to add a new tool)

새로운 SmartThings 기능을 도구로 추가하고 싶다면 다음 단계를 따르세요.

1.  **스키마 정의**: `src/schemas/` 아래 적절한 파일에 입력값 구조체를 정의합니다. `#[derive(JsonSchema, Deserialize)]`를 잊지 마세요.
2.  **도구 구현**: `src/my_devices/` 등에 새로운 파일을 만들거나 기존 파일에 구조체를 추가합니다. `Tool` 트레이트를 구현(`impl Tool for ...`)하고 `name`, `description`, `execute` 등을 작성합니다.
3.  **툴킷 등록**: `src/lib.rs`의 `get_tools()` 메서드에 새로 만든 도구를 `tools.push(Box::new(...))`를 통해 등록합니다.
4.  **테스트**: `tests/integration_test.rs`에 새로운 테스트 케이스를 추가하여 정상 작동 여부를 확인합니다.

---

## 5. 프로젝트의 설계 철학

1.  **타입 안전성**: 모든 입력과 출력은 Rust의 타입 시스템을 통해 검증됩니다. 잘못된 데이터가 들어오면 런타임 에러가 발생하기 전에 컴파일 타임이나 입력 검증 단계에서 걸러집니다.
2.  **일관성**: 38개의 도구가 모두 동일한 `Tool` 인터페이스를 따릅니다. 새로운 도구를 추가할 때도 이 인터페이스만 맞추면 기존 시스템에 즉시 통합됩니다.
3.  **실용성**: 복잡한 비동기(Async) 로직 대신 단순한 동기식(Blocking) 코드를 선택했습니다. 이는 코드의 흐름을 파악하기 쉽게 만들고, 디버깅을 용이하게 합니다.
4.  **LLM 친화적**: OpenAI, Anthropic 등 주요 LLM 플랫폼의 도구 호출 규격을 모두 지원합니다. `schemars`를 통해 자동으로 생성되는 JSON 스키마는 LLM이 도구를 정확하게 이해하고 호출할 수 있게 돕습니다.

---

## 6. 마무리하며

지금까지 Rust의 기초부터 소유권, 트레이트, 에러 처리, 그리고 실제 프로젝트 구조까지 살펴보았습니다. Rust는 처음에는 배우기 까다로울 수 있지만, 한 번 익숙해지면 **"컴파일만 되면 버그가 거의 없다"**는 강력한 신뢰를 주는 언어입니다.

이 가이드가 여러분의 Rust 학습에 좋은 출발점이 되었기를 바랍니다. 이제 실제 코드를 수정해보거나 새로운 도구를 추가해보면서 Rust의 매력을 직접 느껴보세요! 궁금한 점이 있다면 언제든 코드를 다시 읽어보거나 Rust 공식 문서를 참고하시기 바랍니다.

---

## 학습 완료 체크리스트

*   [ ] Rust의 변수 선언과 가변성(`mut`)을 이해했나요?
*   [ ] 소유권과 빌림(`&`)의 차이를 설명할 수 있나요?
*   [ ] 트레이트가 Python의 ABC와 어떻게 다른지 이해했나요?
*   [ ] `Result`와 `Option`을 사용하여 에러를 처리할 수 있나요?
*   [ ] `serde`를 사용하여 JSON 데이터를 다룰 수 있나요?
*   [ ] `ureq`를 통한 HTTP 요청 과정을 이해했나요?
*   [ ] `cargo test`를 통해 테스트를 실행해 보았나요?
*   [ ] 프로젝트의 전체적인 데이터 흐름을 파악했나요?

**수고하셨습니다! 이제 여러분은 Rust로 SmartThings 도구를 다룰 준비가 되었습니다.**
