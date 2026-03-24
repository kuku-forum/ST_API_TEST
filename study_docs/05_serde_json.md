# 05. JSON 직렬화와 역직렬화 (Serde)

Rust에서 JSON 데이터를 다루는 표준 라이브러리인 **serde**와 **serde_json**을 알아봅니다. Python의 `json` 모듈이나 Pydantic과 유사한 역할을 수행합니다.

## 1. Serde란?

**Serde**는 **Ser**ialization(직렬화)과 **De**serialization(역직렬화)의 약자입니다. Rust의 데이터 구조를 JSON, YAML, TOML 등 다양한 형식으로 변환하거나 그 반대로 변환하는 기능을 제공합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/schemas/common.rs`)

```rust
// 4번 라인 부근
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListDevicesInput {
    pub location_id: Option<String>,
    pub capability: Option<String>,
}
```

*   `#[derive(Deserialize)]`: JSON 데이터를 `ListDevicesInput` 구조체로 변환할 수 있게 해줍니다.
*   `#[derive(Serialize)]`: (다른 파일 예시) 구조체 데이터를 JSON으로 변환할 수 있게 해줍니다.

---

## 2. JSON 역직렬화 (JSON -> Struct)

JSON 문자열이나 `serde_json::Value`를 Rust 구조체로 변환합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/common_tools.rs`)

```rust
// 48번 라인 부근
let input: ListDevicesInput = match serde_json::from_value(args) {
    Ok(v) => v,
    Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
};
```

*   `serde_json::from_value(args)`: `args`라는 `Value`(동적 JSON 객체)를 `ListDevicesInput` 타입으로 변환을 시도합니다.
*   **타입 안전성**: 필드 이름이 다르거나 타입이 맞지 않으면 `Err`를 반환합니다.

---

## 3. JSON 직렬화 (Struct -> JSON)

Rust 구조체를 JSON 데이터로 변환합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/common_tools.rs`)

```rust
// 71번 라인 부근
serde_json::to_value(&response).unwrap_or_default()
```

*   `serde_json::to_value(&response)`: `HashMap`이나 구조체를 `serde_json::Value`로 변환합니다.

---

## 4. json! 매크로 (동적 JSON 생성)

코드 내에서 JSON 객체를 직관적으로 생성할 수 있는 매크로입니다. Python의 딕셔너리 리터럴과 매우 비슷합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/tool.rs`)

```rust
// 47번 라인 부근
serde_json::json!({
    "type": "function",
    "function": {
        "name": self.name(),
        "description": self.description(),
        "parameters": self.parameters_schema(),
    }
})
```

*   `json!({ ... })`: 중괄호와 키-값 쌍을 사용하여 JSON 객체를 만듭니다.
*   **변수 삽입**: `self.name()`처럼 Rust 표현식을 값으로 직접 넣을 수 있습니다.

---

## 5. Serde 속성 (Attributes)

필드 이름을 변경하거나 특정 조건에서만 직렬화하는 등 세밀한 제어가 가능합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/tool.rs`)

```rust
// 8번 라인 부근
#[derive(Debug, Clone, Serialize)]
pub struct ToolResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
```

*   `#[serde(skip_serializing_if = "Option::is_none")]`: 값이 `None`인 경우 JSON 결과에서 해당 필드를 아예 제외합니다.
*   `#[serde(rename = "newName")]`: (예시) JSON 필드 이름과 Rust 필드 이름을 다르게 설정할 때 사용합니다.

---

## 6. Python과의 비교

### Python (json 모듈)

```python
# Python
import json

data = {"name": "test", "value": 123}
json_str = json.dumps(data) # 직렬화
obj = json.loads(json_str)  # 역직렬화
```

### Rust (Serde)

```rust
// Rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyData {
    name: String,
    value: i32,
}

let data = MyData { name: "test".to_string(), value: 123 };
let json_str = serde_json::to_string(&data).unwrap(); // 직렬화
let obj: MyData = serde_json::from_str(&json_str).unwrap(); // 역직렬화
```

---

## 7. 왜 Serde를 쓸까?

1.  **성능**: 컴파일 타임에 직렬화/역직렬화 코드를 생성하므로 런타임 오버헤드가 매우 적습니다.
2.  **타입 안전성**: 잘못된 형식의 JSON 데이터가 들어오면 즉시 에러를 발생시켜 프로그램의 안정성을 높입니다.
3.  **유연성**: 수많은 데이터 형식을 지원하며, 복잡한 데이터 구조도 쉽게 다룰 수 있습니다.

### 프로젝트에서의 활용

이 프로젝트는 SmartThings API와 통신하며 수많은 JSON 데이터를 주고받습니다. `serde`는 이 데이터를 Rust의 타입 시스템 안에서 안전하게 다룰 수 있게 해주는 핵심적인 역할을 합니다.

---

## 요약

1.  **Serde**: Rust의 표준 직렬화/역직렬화 프레임워크.
2.  **derive**: `Serialize`, `Deserialize` 매크로를 통해 기능을 자동으로 부여.
3.  **json! 매크로**: 동적 JSON 데이터를 편리하게 생성.
4.  **속성**: `#[serde(...)]`를 통해 세밀한 제어 가능.
5.  **안전성**: 타입 시스템과 결합하여 견고한 데이터 처리를 보장.

다음 장에서는 HTTP 요청을 처리하는 **ureq** 라이브러리에 대해 알아보겠습니다.
