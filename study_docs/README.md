# SmartThings Tools Rust 학습 가이드

이 문서는 Python 개발자가 `smartthings_tools_rs` 프로젝트의 Rust 코드를 이해하고 학습할 수 있도록 돕기 위해 작성되었습니다. Rust의 핵심 개념을 실제 프로젝트 코드를 통해 설명하며, Python과의 비교를 통해 더 쉽게 이해할 수 있도록 구성했습니다.

## 학습 목표

이 가이드를 모두 읽고 나면 다음과 같은 능력을 갖추게 됩니다.

1.  **Rust 코드 읽기**: `smartthings_tools_rs` 프로젝트의 전체적인 흐름과 개별 도구의 구현 방식을 이해할 수 있습니다.
2.  **Rust 개념 이해**: 소유권, 빌림, 트레이트, 에러 처리 등 Rust의 핵심 개념이 실제 프로젝트에서 어떻게 활용되는지 파악할 수 있습니다.
3.  **도구 추가 및 수정**: 기존 도구의 로직을 수정하거나, 새로운 SmartThings 기능을 위한 도구를 직접 추가할 수 있는 기초 지식을 쌓습니다.
4.  **Python과의 차이점 파악**: Python의 동적 타이핑과 가비지 컬렉션 방식이 Rust의 정적 타이핑과 소유권 시스템으로 어떻게 전환되는지 이해합니다.

## 사전 준비 (Prerequisites)

학습을 시작하기 전에 다음 사항들이 준비되어 있는지 확인하세요.

*   **Rust 설치**: [rustup.rs](https://rustup.rs/)를 통해 Rust 툴체인을 설치해야 합니다.
*   **IDE 설정**: VS Code를 사용한다면 `rust-analyzer` 확장을 설치하는 것을 강력히 권장합니다.
*   **SmartThings PAT**: 실제 API 호출 테스트를 위해 [SmartThings 토큰](https://account.smartthings.com/tokens)이 필요합니다.
*   **기본 지식**: Python의 클래스, 함수, 딕셔너리, 리스트 등 기본적인 프로그래밍 개념을 알고 있어야 합니다.

## 학습 순서

가장 권장하는 학습 순서는 다음과 같습니다. 각 문서는 이전 문서의 내용을 바탕으로 구성되어 있으므로 순서대로 읽는 것이 좋습니다.

1.  **[01_rust_basics.md](./01_rust_basics.md) — Rust 기초**
    *   변수, 타입, 함수, 제어 흐름 등 기본적인 문법을 다룹니다.
    *   참조 파일: `smartthings_tools_rs/src/schemas/*.rs`
    *   내용: 변수 가변성(`mut`), 정적 타입 시스템, 표현식 기반 함수 반환, 강력한 `match` 구문.

2.  **[02_ownership_borrowing.md](./02_ownership_borrowing.md) — 소유권과 빌림**
    *   Rust의 가장 독특한 개념인 소유권(Ownership)과 참조(References), 수명(Lifetimes)을 설명합니다.
    *   참조 파일: `smartthings_tools_rs/src/common_tools.rs` (Tool 구조체의 `&'a SmartThingsClient`)
    *   내용: 소유권 이동(Move), 불변/가변 참조, 댕글링 포인터 방지를 위한 수명 명시.

3.  **[03_traits_generics.md](./03_traits_generics.md) — 트레이트와 제네릭**
    *   Python의 추상 베이스 클래스(ABC)와 유사한 트레이트(Trait) 개념을 배웁니다.
    *   참조 파일: `smartthings_tools_rs/src/tool.rs` (`Tool` 트레이트)
    *   내용: 인터페이스 정의, 트레이트 구현(`impl`), 다형성을 위한 트레이트 객체(`Box<dyn Tool>`).

4.  **[04_error_handling.md](./04_error_handling.md) — 에러 처리**
    *   `Result`, `Option` 타입과 패턴 매칭을 통한 안전한 에러 처리 방식을 다룹니다.
    *   참조 파일: `smartthings_tools_rs/src/client.rs`, `common_tools.rs`
    *   내용: 값이 없을 때의 `Option`, 실패할 수 있는 작업의 `Result`, 명시적인 에러 처리의 중요성.

5.  **[05_serde_json.md](./05_serde_json.md) — JSON 직렬화/역직렬화**
    *   Rust에서 JSON 데이터를 다루는 표준 라이브러리인 `serde`와 `serde_json` 사용법을 익힙니다.
    *   참조 파일: `smartthings_tools_rs/src/schemas/*.rs`, `tool.rs`
    *   내용: `#[derive(Serialize, Deserialize)]` 매크로, 동적 JSON 생성을 위한 `json!` 매크로.

6.  **[06_http_client.md](./06_http_client.md) — HTTP 클라이언트 (ureq)**
    *   `ureq` 라이브러리를 사용한 HTTP 요청 처리 방식을 설명합니다.
    *   참조 파일: `smartthings_tools_rs/src/client.rs`
    *   내용: 동기식 HTTP 요청, 헤더 및 쿼리 파라미터 설정, 응답 데이터 파싱 및 에러 캡처.

7.  **[07_testing.md](./07_testing.md) — 테스트**
    *   Rust의 내장 테스트 프레임워크와 통합 테스트 작성법을 배웁니다.
    *   참조 파일: `smartthings_tools_rs/tests/integration_test.rs`
    *   내용: `#[test]` 속성, `cargo test` 실행 방법, 실제 API 연동을 통한 통합 테스트 전략.

8.  **[08_project_walkthrough.md](./08_project_walkthrough.md) — 프로젝트 전체 구조 분석**
    *   코드베이스의 전체적인 흐름과 파일별 역할을 종합적으로 살펴봅니다.
    *   내용: `lib.rs` 진입점 분석, 데이터 흐름 추적, 프로젝트 설계 철학 및 확장 방법.

## 자주 묻는 질문 (FAQ)

**Q: Rust는 왜 이렇게 컴파일 에러가 많이 나나요?**
A: Rust 컴파일러는 런타임에 발생할 수 있는 수많은 잠재적 버그(메모리 누수, 데이터 경합 등)를 컴파일 타임에 미리 찾아냅니다. 에러가 많이 난다는 것은 그만큼 안전한 프로그램을 만들고 있다는 증거입니다.

**Q: Python의 `None`과 Rust의 `Option::None`은 무엇이 다른가요?**
A: Python의 `None`은 어떤 타입의 변수에도 할당될 수 있어 런타임에 `AttributeError`를 일으킬 위험이 있습니다. 반면 Rust의 `Option`은 타입 시스템의 일부이며, 명시적으로 `Some`인지 `None`인지 체크하지 않으면 컴파일이 되지 않습니다.

**Q: `Box<dyn Tool>`에서 `dyn`은 무엇을 의미하나요?**
A: `dyn`은 'Dynamic'의 약자로, 런타임에 다형성을 수행함을 의미합니다. 컴파일 타임에 크기를 알 수 없는 트레이트 객체를 다룰 때 사용하며, 보통 `Box`나 참조(`&`)와 함께 쓰입니다.

**Q: 비동기(Async) 코드는 왜 사용하지 않았나요?**
A: 이 프로젝트는 CLI 도구 및 LLM 도구로서의 실용성에 집중했습니다. 복잡한 비동기 런타임(Tokio 등)을 도입하는 대신, 단순하고 직관적인 동기식 코드를 사용하여 학습 곡선을 낮추고 유지보수성을 높였습니다.

## 기여 가이드 (Contribution Guide)

이 프로젝트에 새로운 도구를 추가하거나 개선하고 싶다면 다음 절차를 권장합니다.

1.  **이슈 확인**: 먼저 기존에 유사한 도구가 있는지 확인합니다.
2.  **브랜치 생성**: 새로운 기능을 위한 브랜치를 생성합니다.
3.  **코드 작성**: `08_project_walkthrough.md`의 "새로운 도구 추가하기" 섹션을 참고하여 코드를 작성합니다.
4.  **테스트 실행**: `cargo test`를 통해 모든 테스트가 통과하는지 확인합니다.
5.  **PR 제출**: 변경 사항을 설명하는 Pull Request를 제출합니다.

## 다음 단계 (Next Steps)

이 가이드를 모두 마쳤다면 다음 단계로 나아가 보세요.

*   **비동기 전환**: `ureq` 대신 `reqwest`와 `tokio`를 사용하여 프로젝트를 비동기 방식으로 전환해 보세요.
*   **추가 도구 구현**: SmartThings의 더 복잡한 기능(규칙 생성, 구독 관리 등)을 도구로 만들어 보세요.
*   **MCP 서버 구축**: 이 라이브러리를 기반으로 실제 MCP(Model Context Protocol) 서버를 구축하여 LLM과 연동해 보세요.

## 학습 팁

*   **실제 코드를 곁에 두세요**: 각 문서에서 인용하는 코드 스니펫의 원본 파일을 직접 열어보며 문맥을 파악하는 것이 좋습니다. VS Code나 IntelliJ 같은 IDE에서 코드를 열어두면 타입 힌트와 정의 이동 기능을 활용할 수 있어 훨씬 편리합니다.
*   **Python과 비교해보세요**: "Python에서는 이렇게 했을 텐데, Rust에서는 왜 이렇게 할까?"라는 질문을 던지며 읽어보세요. Rust의 제약 사항들이 어떻게 런타임 에러를 방지하는지 깨닫는 순간이 Rust 학습의 가장 큰 즐거움입니다.
*   **컴파일러의 메시지를 믿으세요**: Rust 학습 과정에서 가장 큰 스승은 Rust 컴파일러(`rustc`)입니다. 에러 메시지가 매우 친절하고 해결 방법까지 제시해주는 경우가 많으므로, 이를 꼼꼼히 읽는 습관을 들이면 좋습니다.
*   **직접 수정해보세요**: 눈으로만 읽지 말고, 간단한 출력문을 추가하거나 기존 도구의 이름을 바꿔보는 등 코드를 직접 만져보세요. `cargo test`를 실행하여 자신의 수정이 어떤 영향을 주는지 확인하는 과정이 실력 향상에 큰 도움이 됩니다.
*   **공식 문서를 활용하세요**: 이 가이드는 프로젝트 코드 이해에 초점을 맞추고 있습니다. 더 깊은 언어적 이해가 필요하다면 [Rust 공식 가이드(The Book)](https://doc.rust-lang.org/book/)를 병행해서 읽는 것을 추천합니다.

준비가 되셨다면 [01_rust_basics.md](./01_rust_basics.md)부터 시작해 보세요!
