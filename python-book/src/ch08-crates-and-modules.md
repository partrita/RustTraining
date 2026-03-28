# 크레이트와 모듈: 코드 조직화 및 패키지 관리

> **학습 목표:** 파이썬의 패키지/모듈 시스템이 Rust에서는 어떻게 대응되는지 배웁니다. `mod`와 `use`의 차이점, `pub` 키워드를 통한 엄격한 가시성 제어, 그리고 `pip`/`PyPI`를 대체하는 `Cargo`/`crates.io` 생태계를 익힙니다.

---

### 1. 모듈 시스템: 파이썬 vs Rust
파이썬은 파일 자체가 자동으로 모듈이 되지만, Rust는 명시적인 선언이 필요합니다.

| **개념** | **Python** | **Rust** | **비고** |
| :--- | :--- | :--- | :--- |
| **패키지 선언** | `__init__.py`가 있는 디렉터리 | **`mod.rs`** 또는 **`mod` 선언** | Rust는 `src/main.rs`가 루트 |
| **모듈 불러오기** | `import x`, `from x import y` | **`mod x;`**, **`use x::y;`** | `mod`는 선언, `use`는 사용 |
| **가시성 제어** | `_` 접두사 (관례) | **`pub` 키워드** (강제) | Rust는 기본적으로 모든 게 비공개 |
| **상위 모듈 접근** | `from .. import sibling` | **`use super::sibling;`** | `super` 키워드 사용 |

---

### 2. 가시성 (Visibility): "우리는 모두 성인이다" vs "컴파일러의 감시"
파이썬은 `_`를 붙여도 외부에서 접근할 수 있지만, Rust는 `pub`이 없으면 절대 접근할 수 없습니다.

```rust
pub struct User {
    pub name: String, // 공개 필드
    age: i32,         // 비공개 필드 (모듈 내부에서만 접근 가능)
}

impl User {
    pub fn new(name: &str, age: i32) -> Self {
        User { name: name.into(), age }
    }
}
```

---

### 3. 필수 크레이트 (Essential Crates)
파이썬 표준 라이브러리나 유명 패키지들에 대응하는 Rust 크레이트 목록입니다.

| **Python 라이브러리** | **Rust 크레이트** | **용도** |
| :--- | :--- | :--- |
| **`requests`** | **`reqwest`** | HTTP 클라이언트 |
| **`json`** | **`serde_json`** | JSON 직렬화/역직렬화 |
| **`pydantic`** | **`serde`** | 데이터 검증 및 모델링 |
| **`fastapi`** | **`axum`** / **`actix-web`** | 웹 프레임워크 |
| **`logging`** | **`tracing`** | 로그 및 추적 |
| **`argparse`** / **`click`** | **`clap`** | CLI 인자 파싱 |
| **`pytest`** | **`cargo test`** (내장) | 테스트 러너 |

---

### 4. 워크스페이스 (Workspaces)
파이썬의 모노레포(Monorepo) 구성을 Rust에서는 **워크스페이스** 기능을 통해 공식적으로 지원합니다. 하나의 `Cargo.lock` 파일을 공유하여 의존성 버전을 통일하고 여러 크레이트를 한 번에 관리할 수 있습니다.

---

### 💡 실무 팁: `mod.rs` vs 파일 이름 모듈
최근 Rust 트렌드는 `utils/mod.rs` 대신 `utils.rs` 파일과 `utils/` 디렉터리를 같은 계층에 두는 방식을 선호합니다. 프로젝트 구조를 잡을 때 참고하세요.

