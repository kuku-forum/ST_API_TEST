# 06. HTTP 클라이언트 (ureq)

Rust에서 HTTP 요청을 처리하는 가볍고 직관적인 라이브러리인 **ureq**를 알아봅니다. Python의 `requests` 라이브러리와 유사한 사용법을 제공합니다.

## 1. ureq란?

**ureq**는 동기식(Blocking) HTTP 클라이언트 라이브러리입니다. 비동기(Async) 라이브러리인 `reqwest`보다 설정이 간단하고 사용하기 쉬워, CLI 도구나 간단한 라이브러리 제작에 적합합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/client.rs`)

```rust
// 6번 라인 부근
pub struct SmartThingsClient {
    token: String,
    agent: ureq::Agent,
}
```

*   `ureq::Agent`: HTTP 연결을 관리하는 객체입니다. 쿠키나 커넥션 풀링 등을 처리합니다.
*   `token`: SmartThings API 인증을 위한 PAT(Personal Access Token)를 저장합니다.

---

## 2. HTTP 요청 생성 (GET, POST)

`ureq`를 사용하여 서버에 데이터를 요청하거나 전송합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/client.rs`)

```rust
// 27번 라인 부근
let mut req = self
    .agent
    .request(method, &url)
    .set("Authorization", &format!("Bearer {}", self.token))
    .set("Accept", "application/json")
    .set("Content-Type", "application/json");
```

*   `.request(method, &url)`: 요청 메서드(GET, POST 등)와 URL을 설정합니다.
*   `.set(key, value)`: HTTP 헤더를 설정합니다. SmartThings API는 `Authorization` 헤더가 필수입니다.

---

## 3. 쿼리 파라미터와 JSON 본문 (Query & Body)

요청에 추가적인 데이터를 포함시킵니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/client.rs`)

```rust
// 34번 라인 부근
if let Some(ref query) = params {
    for (k, v) in query {
        req = req.query(k, v); // 쿼리 파라미터 추가 (?key=value)
    }
}

// 40번 라인 부근
let resp_result = if let Some(payload) = body {
    req.send_json(payload) // JSON 본문 전송
} else {
    req.call() // 본문 없이 요청 실행
};
```

*   `.query(k, v)`: URL 뒤에 `?locationId=...`와 같은 쿼리 스트링을 붙입니다.
*   `.send_json(payload)`: `serde_json::Value`를 JSON으로 직렬화하여 요청 본문에 담아 보냅니다.

---

## 4. 응답 처리 (Response Handling)

서버로부터 받은 응답을 해석합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/client.rs`)

```rust
// 62번 라인 부근
fn parse_response(resp: ureq::Response) -> HashMap<String, Value> {
    let text = resp.into_string().unwrap_or_default();
    if text.trim().is_empty() {
        return HashMap::new();
    }
    match serde_json::from_str::<HashMap<String, Value>>(&text) {
        Ok(map) => map,
        Err(_) => {
            let mut m = HashMap::new();
            m.insert("raw".to_string(), Value::String(text));
            m
        }
    }
}
```

*   `resp.into_string()`: 응답 본문을 문자열로 읽어옵니다.
*   `serde_json::from_str`: 문자열을 `HashMap`으로 변환합니다.

---

## 5. 에러 처리 (Error Handling)

네트워크 에러나 HTTP 에러 상태 코드를 처리합니다.

### 프로젝트 코드 예시 (`smartthings_tools_rs/src/client.rs`)

```rust
// 46번 라인 부근
match resp_result {
    Ok(resp) => Self::parse_response(resp),
    Err(ureq::Error::Status(code, resp)) => {
        // 400, 404, 500 등 에러 상태 코드 처리
        let mut data = Self::parse_response(resp);
        data.insert("status_code".to_string(), Value::Number(code.into()));
        data
    }
    Err(ureq::Error::Transport(err)) => {
        // 네트워크 연결 실패 등 전송 에러 처리
        let mut m = HashMap::new();
        m.insert("error".to_string(), Value::String(err.to_string()));
        m
    }
}
```

*   `ureq::Error::Status`: 서버가 에러 상태 코드를 반환한 경우입니다.
*   `ureq::Error::Transport`: 네트워크 연결 자체가 실패한 경우입니다.

---

## 6. Python과의 비교

### Python (requests)

```python
# Python
import requests

resp = requests.get(
    "https://api.smartthings.com/v1/devices",
    headers={"Authorization": "Bearer ..."},
    params={"locationId": "..."}
)
data = resp.json()
```

### Rust (ureq)

```rust
// Rust
let resp = ureq::get("https://api.smartthings.com/v1/devices")
    .set("Authorization", "Bearer ...")
    .query("locationId", "...")
    .call();

let data: serde_json::Value = resp.unwrap().into_json().unwrap();
```

---

## 7. 왜 ureq를 쓸까?

1.  **단순함**: 비동기(Async/Await) 개념 없이도 직관적으로 HTTP 요청을 보낼 수 있습니다.
2.  **가벼움**: 의존성이 적고 컴파일 속도가 빠릅니다.
3.  **안전성**: Rust의 타입 시스템과 결합하여 에러 처리를 명확하게 할 수 있습니다.

### 프로젝트에서의 활용

이 프로젝트의 `SmartThingsClient`는 `ureq`를 감싸서(Wrapping) SmartThings API 전용 클라이언트를 만듭니다. 모든 도구(`Tool`)들은 이 클라이언트를 통해 일관된 방식으로 API를 호출합니다.

---

## 요약

1.  **ureq**: 동기식 HTTP 클라이언트 라이브러리.
2.  **Agent**: 연결 관리 및 설정 공유.
3.  **메서드 체이닝**: `.set()`, `.query()` 등을 연결하여 요청 설정.
4.  **JSON 지원**: `send_json()`, `into_json()` 등을 통해 편리하게 JSON 처리.
5.  **에러 처리**: `Result` 타입을 통해 네트워크 및 HTTP 에러를 명확히 구분.

다음 장에서는 Rust의 테스트 시스템에 대해 알아보겠습니다.
