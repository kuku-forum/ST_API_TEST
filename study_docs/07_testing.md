# 07. 테스트 (Testing)

Rust의 내장 테스트 프레임워크와 통합 테스트 작성법을 알아봅니다. Python의 `pytest`나 `unittest`와 유사하지만, 언어 차원에서 더 긴밀하게 통합되어 있습니다.

## 1. Rust 테스트의 종류

1.  **단위 테스트 (Unit Tests)**: 소스 코드 파일(`src/*.rs`) 내부에 작성하며, 비공개(private) 함수도 테스트할 수 있습니다.
2.  **통합 테스트 (Integration Tests)**: `tests/` 디렉토리에 작성하며, 라이브러리의 공개(public) API만 테스트할 수 있습니다.

---

## 2. 테스트 함수 정의

`#[test]` 속성(Attribute)을 함수 위에 붙여 테스트임을 명시합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/tests/integration_test.rs`)

```rust
// 36번 라인 부근
#[test]
fn test_tool_registration() {
    // ... 테스트 로직 ...
    assert_eq!(names.len(), 38); // 단언문 (Assertion)
}
```

*   `#[test]`: 이 함수를 테스트 케이스로 등록합니다.
*   `assert_eq!(a, b)`: `a`와 `b`가 같지 않으면 테스트가 실패하고 패닉을 일으킵니다.
*   `assert!(condition)`: 조건이 참이 아니면 실패합니다.

---

## 3. 테스트 실행 (Cargo Test)

터미널에서 `cargo test` 명령어를 사용하여 테스트를 실행합니다.

```bash
# 모든 테스트 실행
cargo test

# 특정 테스트만 실행
cargo test test_tool_registration

# 테스트 출력 결과 확인 (--nocapture)
cargo test -- --nocapture
```

*   `--nocapture`: 기본적으로 Rust는 성공한 테스트의 표준 출력(`println!`)을 숨깁니다. 이 옵션을 사용하면 성공한 테스트의 출력도 볼 수 있습니다.

---

## 4. 통합 테스트 구조

`tests/` 디렉토리의 파일들은 각각 독립적인 크레이트(Crate)로 컴파일됩니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/tests/integration_test.rs`)

```rust
// 1번 라인 부근
use smartthings_tools_rs::tool::ToolResult;
use smartthings_tools_rs::SmartThingsToolkit;

#[test]
fn test_api_calls() {
    let token = match get_token() {
        Some(t) => t,
        None => return, // 토큰이 없으면 테스트 건너뜀
    };

    let toolkit = SmartThingsToolkit::new(&token, true);
    let result = toolkit.execute("list_devices", serde_json::json!({}));
    
    assert!(result.success); // 실행 성공 확인
}
```

*   `use smartthings_tools_rs::...`: 라이브러리 이름을 통해 공개된 기능을 가져옵니다.
*   **실제 API 호출**: 이 프로젝트의 통합 테스트는 실제 SmartThings API를 호출하여 도구들이 정상적으로 작동하는지 확인합니다.

---

## 5. 테스트 헬퍼 함수

테스트 코드 내에서 반복되는 로직을 별도의 함수로 분리할 수 있습니다. `#[test]`가 붙지 않은 일반 함수는 테스트 케이스로 간주되지 않습니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/tests/integration_test.rs`)

```rust
// 8번 라인 부근
fn print_result(test_name: &str, result: &ToolResult) {
    let status = if result.success { "PASS" } else { "FAIL" };
    println!("  [{status}] {test_name}");
    // ... 상세 결과 출력 ...
}
```

---

## 6. Python과의 비교

### Python (pytest)

```python
# Python
def test_tool_registration():
    toolkit = SmartThingsToolkit(token="...", is_rust=True)
    names = toolkit.list_tool_names()
    assert len(names) == 38
```

### Rust (cargo test)

```rust
// Rust
#[test]
fn test_tool_registration() {
    let toolkit = SmartThingsToolkit::new("...", true);
    let names = toolkit.list_tool_names();
    assert_eq!(names.len(), 38);
}
```

*   **차이점**: Rust는 별도의 테스트 라이브러리 설치 없이도 `cargo` 도구만으로 강력한 테스트 환경을 제공합니다. 또한 컴파일 타임에 많은 오류를 잡아주므로, 런타임 테스트는 주로 로직의 올바름을 검증하는 데 집중하게 됩니다.

---

## 7. 왜 테스트가 중요할까?

1.  **회귀 방지**: 코드를 수정했을 때 기존 기능이 망가지지 않았는지 즉시 확인할 수 있습니다.
2.  **문서화 역할**: 테스트 코드는 라이브러리를 어떻게 사용하는지 보여주는 가장 정확한 예시가 됩니다.
3.  **신뢰성**: 특히 이 프로젝트처럼 외부 API(SmartThings)와 연동되는 경우, API 변경이나 네트워크 문제를 빠르게 감지할 수 있습니다.

### 프로젝트에서의 활용

이 프로젝트의 통합 테스트는 38개의 도구가 모두 올바르게 등록되었는지, OpenAI/Anthropic 형식으로 잘 변환되는지, 그리고 실제 API 호출 시 데이터가 정상적으로 반환되는지를 꼼꼼하게 검증합니다.

---

## 요약

1.  **#[test]**: 테스트 함수를 지정하는 속성.
2.  **assert! / assert_eq!**: 결과 검증을 위한 단언문.
3.  **cargo test**: 테스트 실행 도구.
4.  **통합 테스트**: `tests/` 디렉토리에서 라이브러리 외부 인터페이스 검증.
5.  **--nocapture**: 테스트 출력 내용을 확인하기 위한 옵션.

마지막 장에서는 프로젝트의 전체적인 코드 흐름을 따라가며 총정리하는 시간을 갖겠습니다.
