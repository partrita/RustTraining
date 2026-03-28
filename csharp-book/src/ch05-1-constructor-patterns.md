# 생성자 패턴: 유연하고 안전한 객체 생성

> **학습 목표:** C#의 클래스 생성자 대신 Rust에서 관용적으로 사용하는 다양한 초기화 패턴을 배웁니다. `new()` 관례부터 `Default` 트레이트 활용, 그리고 복잡한 설정을 위한 빌더(Builder) 패턴까지 마스터합니다.

---

### 1. 관용적인 `new()`와 팩토리 메서드
Rust에는 별도의 `constructor` 키워드가 없습니다. 대신 `new`라는 이름의 **연관 함수(Static Method)**를 정의하는 것이 표준 관례입니다.

```rust
struct Config {
    db_url: String,
    timeout: u32,
}

impl Config {
    // 1. 기본 생성 관례
    pub fn new(url: &str) -> Self {
        Self {
            db_url: url.to_string(),
            timeout: 30, // 기본값 설정
        }
    }

    // 2. 명명된 팩토리 메서드 (C#의 팩토리 패턴)
    pub fn for_local() -> Self {
        Self {
            db_url: "localhost".to_string(),
            timeout: 5,
        }
    }
}
```

---

### 2. `Default` 트레이트: 표준적인 기본값 설정
C#의 매개변수 없는 생성자와 유사한 역할을 수행합니다. `#[derive(Default)]`를 사용하거나 직접 구현하여 `Config::default()` 형태로 호출할 수 있습니다.

```rust
impl Default for Config {
    fn default() -> Self {
        Self {
            db_url: "localhost".to_string(),
            timeout: 30,
        }
    }
}

// 활용: 특정 필드만 바꾸고 나머지는 기본값으로 채우기
let custom_config = Config {
    timeout: 10,
    ..Config::default() // 구조체 업데이트 구문
};
```

---

### 3. 빌더(Builder) 패턴
설정할 인자가 많거나 복잡한 유효성 검사가 필요한 경우, 빌더 패턴을 사용하여 가독성과 안전성을 높입니다.

```rust
// [빌더 사용 예시]
let server = ServerBuilder::new()
    .host("127.0.0.1")
    .port(8080)
    .max_connections(100)
    .build()?; // 마지막에 유효성 검사 후 인스턴스 반환
```

- **장점**: 인자 순서를 헷갈릴 일이 없으며, 필수 인자가 누락되었을 때 컴파일 타임 혹은 런타임(`Result`) 에러로 안전하게 처리할 수 있습니다.

---

### 💡 C# 개발자를 위한 팁: `impl Into<String>` 활용하기
함수나 빌더에서 문자열을 받을 때 `&str` 대신 `impl Into<String>`을 인자로 받아보세요. 이렇게 하면 호출자가 `"literal"`이나 `String` 객체 중 무엇을 전달하든 컴파일러가 알아서 적절히 처리해주어 호출부 코드가 훨씬 깔끔해집니다.

