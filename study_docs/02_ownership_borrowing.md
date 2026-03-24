# 02. 소유권과 빌림 (Ownership and Borrowing)

Rust의 가장 독특하고 강력한 개념인 **소유권(Ownership)**과 **빌림(Borrowing)**, 그리고 **수명(Lifetimes)**을 알아봅니다. 이 개념들은 메모리 안전성을 보장하면서도 가비지 컬렉터(GC) 없이 높은 성능을 낼 수 있게 해줍니다.

## 1. 소유권 (Ownership)

Python에서는 객체를 변수에 할당하면 참조가 복사됩니다. 하지만 Rust에서는 **소유권**이 이동(Move)합니다.

### 소유권의 세 가지 규칙

1.  Rust의 각 값은 **소유자(Owner)**라고 불리는 변수를 가집니다.
2.  한 번에 **단 하나의 소유자**만 존재할 수 있습니다.
3.  소유자가 스코프(Scope) 밖으로 벗어나면, 값은 자동으로 **해제(Drop)**됩니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/client.rs`)

```rust
// 12번 라인 부근
pub fn new(token: impl Into<String>) -> Self {
    Self {
        token: token.into(), // token의 소유권이 Self(SmartThingsClient)로 이동합니다.
        agent: ureq::AgentBuilder::new().build(),
    }
}
```

*   `token.into()`: `token`이라는 변수가 가지고 있던 문자열의 소유권이 `SmartThingsClient` 구조체의 `token` 필드로 넘어갑니다. 이제 원래의 `token` 변수는 더 이상 사용할 수 없습니다.

---

## 2. 빌림 (Borrowing)

소유권을 넘기지 않고 값을 사용하고 싶을 때 **참조(Reference)**를 사용합니다. 이를 **빌림**이라고 합니다.

### 참조의 종류

1.  **불변 참조 (`&T`)**: 값을 읽을 수만 있습니다. 여러 개가 동시에 존재할 수 있습니다.
2.  **가변 참조 (`&mut T`)**: 값을 수정할 수 있습니다. 한 번에 단 하나만 존재할 수 있습니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/common_tools.rs`)

```rust
// 22번 라인 부근
pub fn extract_error_msg(response: &std::collections::HashMap<String, Value>) -> String {
    extract_error(response).unwrap_or_else(|| "Unknown error".to_string())
}
```

*   `response: &std::collections::HashMap<String, Value>`: `response`의 소유권을 가져오지 않고, **불변 참조**로 빌려옵니다. 함수가 끝나도 호출한 쪽에서는 `response`를 계속 사용할 수 있습니다.

---

## 3. 수명 (Lifetimes)

참조가 유효한 기간을 **수명**이라고 합니다. Rust 컴파일러는 참조가 가리키는 값이 참조보다 먼저 사라지지 않도록 보장합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/common_tools.rs`)

```rust
// 30번 라인 부근
pub struct ListDevicesTool<'a> {
    pub client: &'a SmartThingsClient,
}
```

*   `'a`: 이것이 수명 매개변수입니다.
*   `client: &'a SmartThingsClient`: `ListDevicesTool` 구조체가 `SmartThingsClient`를 참조로 가지고 있음을 의미합니다.
*   **의미**: `ListDevicesTool` 인스턴스는 자신이 참조하는 `SmartThingsClient`보다 더 오래 살 수 없습니다. 컴파일러는 이 관계를 체크하여 댕글링 포인터(Dangling Pointer) 에러를 방지합니다.

---

## 4. Python과의 비교

Python 개발자에게 이 개념은 생소할 수 있습니다. Python은 모든 것을 참조로 다루고 가비지 컬렉터가 메모리를 관리하기 때문입니다.

### Python (참조 복사)

```python
# Python
client = SmartThingsClient(token="...")
tool = ListDevicesTool(client=client)
# client와 tool.client는 동일한 객체를 가리킵니다.
# client가 사라져도 tool.client는 계속 유효합니다 (GC가 관리).
```

### Rust (소유권/빌림)

```rust
// Rust
let client = SmartThingsClient::new("...");
let tool = ListDevicesTool { client: &client };
// tool은 client를 '빌려' 쓰고 있습니다.
// 만약 client가 스코프를 벗어나 사라지면, tool도 더 이상 사용할 수 없습니다.
// 컴파일러가 이를 미리 체크하여 에러를 냅니다.
```

---

## 5. 왜 이렇게 복잡할까?

처음에는 수명(`'a`)이나 참조(`&`)가 번거롭게 느껴질 수 있습니다. 하지만 이 덕분에 Rust는 다음과 같은 이점을 얻습니다.

1.  **메모리 안전성**: 널 포인터(Null Pointer)나 댕글링 포인터 에러가 발생하지 않습니다.
2.  **데이터 경합 방지**: 가변 참조(`&mut`) 규칙 덕분에 멀티스레드 환경에서 안전하게 데이터를 공유할 수 있습니다.
3.  **성능**: 가비지 컬렉터가 없으므로 실행 시 오버헤드가 적고 예측 가능한 성능을 제공합니다.

### 프로젝트에서의 활용

이 프로젝트에서는 `SmartThingsClient` 하나를 생성하여 여러 `Tool`들이 공유해서 사용합니다. 각 `Tool`은 클라이언트를 소유하지 않고 **빌려서** 사용하므로 메모리 낭비가 없고 안전합니다.

```rust
// lib.rs의 구조
pub struct SmartThingsToolkit<'a> {
    pub client: SmartThingsClient,
    pub tools: Vec<Box<dyn Tool + 'a>>, // 'a 수명을 가진 도구들을 담습니다.
}
```

---

## 요약

1.  **소유권**: 값은 단 하나의 소유자만 가집니다.
2.  **이동(Move)**: 값을 다른 변수에 할당하면 소유권이 넘어갑니다.
3.  **빌림(Borrowing)**: `&`를 사용하여 소유권을 넘기지 않고 참조할 수 있습니다.
4.  **수명(Lifetimes)**: 참조가 유효한 기간을 명시하여 안전성을 보장합니다.

다음 장에서는 Python의 ABC와 유사한 **트레이트(Traits)**에 대해 알아보겠습니다.
