# 03. 트레이트와 제네릭 (Traits and Generics)

Rust에서 다형성(Polymorphism)을 구현하는 핵심 도구인 **트레이트(Trait)**와 **제네릭(Generics)**을 알아봅니다. Python의 추상 베이스 클래스(ABC)나 인터페이스와 유사한 개념입니다.

## 1. 트레이트 (Traits)

트레이트는 특정 타입이 가져야 할 **공통된 동작**을 정의합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/tool.rs`)

```rust
// 40번 라인 부근
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn parameters_schema(&self) -> RootSchema;
    fn execute(&self, args: Value) -> ToolResult;

    // 기본 구현 (Default Implementation)
    fn to_openai_tool(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": self.description(),
                "parameters": self.parameters_schema(),
            }
        })
    }
    // ...
}
```

*   `pub trait Tool`: `Tool`이라는 트레이트를 정의합니다.
*   `fn name(&self) -> ...`: 이 트레이트를 구현하는 모든 타입은 `name` 함수를 반드시 구현해야 합니다.
*   `to_openai_tool`: 트레이트 내에서 **기본 구현**을 제공할 수도 있습니다. 구현하는 쪽에서 재정의(Override)하지 않으면 이 기본 구현이 사용됩니다.

---

## 2. 트레이트 구현 (Implementing Traits)

구조체에 트레이트를 적용할 때는 `impl [Trait] for [Struct]` 구문을 사용합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/common_tools.rs`)

```rust
// 34번 라인 부근
impl Tool for ListDevicesTool<'_> {
    fn name(&self) -> &'static str {
        "list_devices"
    }

    fn description(&self) -> &'static str {
        "등록된 모든 SmartThings 디바이스 목록을 조회합니다..."
    }

    fn parameters_schema(&self) -> RootSchema {
        schema_for!(ListDevicesInput)
    }

    fn execute(&self, args: Value) -> ToolResult {
        // 실제 실행 로직...
    }
}
```

*   `ListDevicesTool` 구조체가 `Tool` 트레이트의 모든 요구사항을 충족하도록 구현되었습니다.
*   이제 `ListDevicesTool`은 `Tool`로서 취급될 수 있습니다.

---

## 3. 트레이트 객체 (Trait Objects)

서로 다른 타입들을 하나의 리스트에 담고 싶을 때 **트레이트 객체**를 사용합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/lib.rs`)

```rust
// 10번 라인 부근
pub struct SmartThingsToolkit<'a> {
    pub client: SmartThingsClient,
    pub tools: Vec<Box<dyn Tool + 'a>>,
}
```

*   `dyn Tool`: `Tool` 트레이트를 구현하는 **어떤 타입**이든 올 수 있음을 의미합니다.
*   `Box<dyn Tool>`: 트레이트 객체는 크기를 미리 알 수 없으므로, 힙(Heap)에 저장하고 포인터(`Box`)로 가리킵니다.
*   **다형성**: `tools` 벡터에는 `ListDevicesTool`, `GetDeviceStatusTool`, `SendCommandTool` 등 서로 다른 구조체들이 섞여서 들어갈 수 있습니다.

---

## 4. Python과의 비교

Python의 추상 베이스 클래스(ABC)와 매우 유사합니다.

### Python (ABC)

```python
# Python
from abc import ABC, abstractmethod

class Tool(ABC):
    @abstractmethod
    def name(self) -> str:
        pass

    def to_openai_tool(self):
        return {"name": self.name(), ...}

class ListDevicesTool(Tool):
    def name(self):
        return "list_devices"
```

### Rust (Trait)

```rust
// Rust
pub trait Tool {
    fn name(&self) -> &'static str;
    fn to_openai_tool(&self) -> Value { ... }
}

pub struct ListDevicesTool;
impl Tool for ListDevicesTool {
    fn name(&self) -> &'static str { "list_devices" }
}
```

---

## 5. 제네릭 (Generics)

타입을 매개변수화하여 코드 중복을 줄이는 기능입니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/client.rs`)

```rust
// 12번 라인 부근
pub fn new(token: impl Into<String>) -> Self { ... }
```

*   `impl Into<String>`: `String`으로 변환될 수 있는 **어떤 타입**이든 인자로 받을 수 있다는 뜻입니다.
*   `&str`이나 `String` 모두 이 함수의 인자로 사용될 수 있어 편리합니다.

---

## 6. 왜 트레이트를 쓸까?

1.  **인터페이스 보장**: 특정 기능을 수행하기 위해 필요한 메서드들이 모두 구현되어 있음을 컴파일 타임에 보장합니다.
2.  **코드 재사용**: `to_openai_tool`처럼 공통된 로직을 트레이트에 한 번만 작성하고 모든 도구에서 공유할 수 있습니다.
3.  **유연한 확장**: 새로운 도구를 추가할 때 `Tool` 트레이트만 구현하면 기존 시스템(`SmartThingsToolkit`)에 즉시 통합됩니다.

### 프로젝트에서의 활용

이 프로젝트의 핵심은 **"모든 SmartThings 기능을 하나의 `Tool` 인터페이스로 통일하는 것"**입니다. LLM(Large Language Model)은 이 인터페이스를 통해 어떤 도구가 있는지 파악하고 실행할 수 있게 됩니다.

---

## 요약

1.  **트레이트**: 공통된 동작(메서드)의 집합을 정의합니다.
2.  **구현**: `impl Trait for Struct`를 통해 구조체에 기능을 부여합니다.
3.  **트레이트 객체**: `Box<dyn Trait>`를 사용하여 서로 다른 타입들을 하나의 컬렉션에 담을 수 있습니다.
4.  **다형성**: 트레이트를 통해 런타임에 적절한 메서드가 호출되도록 합니다.

다음 장에서는 Rust의 안전한 에러 처리 방식인 **Result와 Option**에 대해 알아보겠습니다.
