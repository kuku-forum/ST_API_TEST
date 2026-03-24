# 01. Rust 기초 (Basics)

이 문서에서는 Python 개발자가 Rust의 기본적인 문법(변수, 타입, 함수, 제어 흐름)을 이해할 수 있도록 돕습니다. `smartthings_tools_rs/src/schemas/` 디렉토리의 코드를 예시로 사용합니다.

## 1. 변수와 가변성 (Variables and Mutability)

Python에서 변수는 기본적으로 가변적(mutable)입니다. 하지만 Rust에서 변수는 기본적으로 **불변(immutable)**입니다. 값을 변경하려면 `mut` 키워드를 명시해야 합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/common_tools.rs`)

```rust
// 53번 라인 부근
let mut params = std::collections::HashMap::new();
if let Some(loc) = &input.location_id {
    params.insert("locationId".to_string(), loc.clone());
}
```

*   `let mut params`: `params`라는 변수를 선언하고, 나중에 `insert`를 통해 내용을 변경할 것이므로 `mut`를 붙였습니다.
*   `let input`: (위쪽 코드에서) `input`은 선언 후 변경되지 않으므로 `mut` 없이 선언되었습니다.

### Python과의 비교

| 개념 | Python | Rust |
| :--- | :--- | :--- |
| 변수 선언 | `x = 5` | `let x = 5;` (불변) |
| 가변 변수 | (기본값) | `let mut x = 5;` (가변) |
| 상수 | `X = 5` (관례상) | `const X: i32 = 5;` |

---

## 2. 데이터 타입 (Data Types)

Rust는 정적 타입 언어입니다. 컴파일 타임에 모든 변수의 타입이 결정되어야 합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/schemas/common.rs`)

```rust
pub struct ListDevicesInput {
    pub location_id: Option<String>,
    pub capability: Option<String>,
}
```

*   `String`: 힙(heap)에 저장되는 가변 길이 문자열입니다. Python의 `str`과 유사합니다.
*   `Option<String>`: 값이 있을 수도(`Some(String)`), 없을 수도(`None`) 있음을 나타내는 타입입니다. Python의 `Optional[str]`과 같습니다.

### 주요 타입 비교

| Rust 타입 | Python 타입 | 설명 |
| :--- | :--- | :--- |
| `i32`, `i64` | `int` | 정수형 (32비트, 64비트 등 명시) |
| `f64` | `float` | 부동 소수점 (64비트) |
| `bool` | `bool` | 논리형 (`true`, `false`) |
| `String` | `str` | 문자열 |
| `Vec<T>` | `list` | 동적 배열 (벡터) |
| `HashMap<K, V>` | `dict` | 해시 맵 (딕셔너리) |

---

## 3. 함수 (Functions)

함수는 `fn` 키워드로 정의하며, 매개변수와 반환값의 타입을 명시해야 합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/client.rs`)

```rust
pub fn new(token: impl Into<String>) -> Self {
    Self {
        token: token.into(),
        agent: ureq::AgentBuilder::new().build(),
    }
}
```

*   `fn new(...) -> Self`: `new`라는 함수를 정의하며, `Self`(여기서는 `SmartThingsClient`) 타입을 반환합니다.
*   **세미콜론(`;`) 없는 마지막 표현식**: Rust 함수에서 마지막 줄에 세미콜론을 붙이지 않으면 그 값이 반환값(return value)이 됩니다. `return` 키워드를 생략하는 것이 관례입니다.

---

## 4. 제어 흐름 (Control Flow)

`if`, `match`, `loop`, `while`, `for` 등이 있습니다. 특히 `match`는 Python 3.10의 `match-case`와 유사하지만 훨씬 강력합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/common_tools.rs`)

```rust
// 48번 라인 부근
let input: ListDevicesInput = match serde_json::from_value(args) {
    Ok(v) => v,
    Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
};
```

*   `match`: `serde_json::from_value(args)`의 결과가 성공(`Ok`)인지 실패(`Err`)인지에 따라 다른 동작을 수행합니다.
*   **표현식으로서의 제어문**: Rust의 `if`나 `match`는 값을 반환할 수 있습니다. 위 예시에서 `match`의 결과값이 `input` 변수에 할당됩니다.

### Python과의 비교

```python
# Python (3.10+)
match result:
    case Ok(v):
        input = v
    case Err(e):
        return ToolResult.fail(f"ValidationError: {e}")
```

---

## 5. 구조체 (Structs)

데이터를 그룹화하는 방식입니다. Python의 `dataclass`나 Pydantic 모델과 유사합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/schemas/common.rs`)

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SendCommandInput {
    pub device_id: String,
    pub capability: String,
    pub command: String,
    #[serde(default)]
    pub arguments: Vec<serde_json::Value>,
    #[serde(default = "default_component")]
    pub component: String,
}
```

*   `pub struct`: 외부에서 접근 가능한 구조체를 정의합니다.
*   `#[derive(...)]`: 매크로를 사용하여 `Debug`(출력용), `Deserialize`(JSON 읽기용) 등의 기능을 자동으로 구현합니다. Python의 데코레이터와 비슷한 역할을 합니다.

---

## 요약

1.  변수는 기본적으로 **불변**이며, 변경하려면 `mut`가 필요합니다.
2.  모든 변수와 함수 매개변수에는 **타입**이 명시되어야 합니다.
3.  함수의 마지막 표현식(세미콜론 없음)은 자동으로 **반환**됩니다.
4.  `match`는 강력한 패턴 매칭 도구이며, 제어문 자체가 **값**을 가질 수 있습니다.

다음 장에서는 Rust의 가장 중요한 개념인 **소유권과 빌림**에 대해 알아보겠습니다.
