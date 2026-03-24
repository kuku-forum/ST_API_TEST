# 04. 에러 처리 (Error Handling)

Rust의 안전하고 명시적인 에러 처리 방식인 **Result**와 **Option** 타입을 알아봅니다. Python의 `try-except`나 `None` 체크와는 다른, Rust만의 독특한 방식을 프로젝트 코드를 통해 살펴봅니다.

## 1. Option 타입 (값이 있거나 없거나)

값이 존재하지 않을 수 있는 상황을 나타냅니다. Python의 `Optional[T]` 또는 `None`과 유사합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/schemas/common.rs`)

```rust
// 5번 라인 부근
pub struct ListDevicesInput {
    pub location_id: Option<String>,
    pub capability: Option<String>,
}
```

*   `Option<String>`: `location_id`는 `Some(String)`이거나 `None`일 수 있습니다.
*   **안전성**: Rust에서는 `Option` 타입의 값을 사용하려면 반드시 `None`인 경우를 처리해야 합니다. 이를 통해 널 포인터 예외(Null Pointer Exception)를 원천적으로 방지합니다.

---

## 2. Result 타입 (성공하거나 실패하거나)

작업의 성공 여부와 결과값(또는 에러값)을 함께 담는 타입입니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/client.rs`)

```rust
// 46번 라인 부근
match resp_result {
    Ok(resp) => Self::parse_response(resp),
    Err(ureq::Error::Status(code, resp)) => {
        // HTTP 에러 상태 코드 처리...
    }
    Err(ureq::Error::Transport(err)) => {
        // 네트워크 연결 에러 처리...
    }
}
```

*   `Ok(T)`: 작업이 성공했을 때의 결과값입니다.
*   `Err(E)`: 작업이 실패했을 때의 에러 정보입니다.
*   **명시적 처리**: `Result`를 반환하는 함수를 호출하면, 호출자는 반드시 성공과 실패 케이스를 모두 다루어야 합니다.

---

## 3. 패턴 매칭 (Pattern Matching)

`match` 구문을 사용하여 `Option`이나 `Result`의 내부 값을 안전하게 꺼내옵니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/common_tools.rs`)

```rust
// 48번 라인 부근
let input: ListDevicesInput = match serde_json::from_value(args) {
    Ok(v) => v,
    Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
};
```

*   `serde_json::from_value(args)`는 `Result<ListDevicesInput, Error>`를 반환합니다.
*   `Ok(v)`인 경우: 내부 값 `v`를 꺼내어 `input` 변수에 할당합니다.
*   `Err(e)`인 경우: 에러 메시지를 담아 즉시 함수를 종료(`return`)합니다.

---

## 4. if let 구문 (간결한 매칭)

특정 케이스 하나만 처리하고 싶을 때 사용합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/client.rs`)

```rust
// 34번 라인 부근
if let Some(ref query) = params {
    for (k, v) in query {
        req = req.query(k, v);
    }
}
```

*   `params`가 `Some`인 경우에만 내부의 `query` 값을 꺼내어 블록 내부를 실행합니다.
*   `None`인 경우에는 아무 일도 일어나지 않습니다.

---

## 5. unwrap과 expect (주의해서 사용)

에러 처리를 건너뛰고 강제로 값을 꺼내올 때 사용합니다. 만약 값이 없거나 에러가 발생하면 프로그램이 즉시 종료(Panic)됩니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/client.rs`)

```rust
// 63번 라인 부근
let text = resp.into_string().unwrap_or_default();
```

*   `unwrap_or_default()`: 에러가 발생하면 패닉을 일으키는 대신, 해당 타입의 기본값(빈 문자열 등)을 반환합니다.
*   **권장 사항**: 실제 서비스 코드에서는 `unwrap()`보다는 `match`나 `if let`, 또는 `unwrap_or` 계열의 함수를 사용하여 안전하게 처리하는 것이 좋습니다.

---

## 6. Python과의 비교

### Python (try-except)

```python
# Python
try:
    input = parse_json(args)
    # 성공 로직...
except ValidationError as e:
    return ToolResult.fail(f"ValidationError: {e}")
```

### Rust (Result + match)

```rust
// Rust
let input = match parse_json(args) {
    Ok(v) => v,
    Err(e) => return ToolResult::fail(format!("ValidationError: {e}")),
};
// 성공 로직...
```

*   **차이점**: Python은 예외가 발생하면 호출 스택을 따라 올라가며 처리되지만, Rust는 에러를 **값**으로 취급하여 함수 간에 명시적으로 전달합니다.

---

## 7. 왜 이렇게 할까?

1.  **예측 가능성**: 어떤 함수가 에러를 발생시킬 수 있는지 함수의 시그니처(`-> Result<...>`)만 보고도 알 수 있습니다.
2.  **안전성**: 실수로 에러 처리를 빠뜨리는 것을 컴파일러가 방지해 줍니다.
3.  **가독성**: 복잡한 중첩 `if` 문 대신 `match`나 `?` 연산자(이 프로젝트에서는 다루지 않지만)를 사용하여 깔끔하게 에러를 처리할 수 있습니다.

### 프로젝트에서의 활용

이 프로젝트에서는 `ToolResult`라는 구조체를 정의하여 성공 여부와 데이터, 에러 메시지를 일관된 방식으로 반환합니다.

```rust
// tool.rs
pub struct ToolResult {
    pub success: bool,
    pub data: Option<Value>,
    pub error: Option<String>,
}
```

---

## 요약

1.  **Option**: 값이 없을 수 있는 상황 (`Some`, `None`).
2.  **Result**: 실패할 수 있는 작업 (`Ok`, `Err`).
3.  **match**: 모든 케이스를 명시적으로 처리하는 강력한 도구.
4.  **if let**: 특정 케이스만 간결하게 처리.
5.  **안전성**: 런타임 에러를 컴파일 타임에 미리 방지하는 Rust의 핵심 철학.

다음 장에서는 Rust에서 JSON 데이터를 다루는 **serde** 라이브러리에 대해 알아보겠습니다.
