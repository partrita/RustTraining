# 권장 사례: C# 개발자를 위한 Rust 핵심 가이드

> **학습 목표:** C#에서 Rust로 넘어올 때 반드시 챙겨야 할 **네 가지 사고방식의 전환**과 실무에서 바로 쓸 수 있는 프로젝트 구조화, 에러 처리, 테스트 패턴을 익힙니다. 또한 자기도 모르게 저지르는 C#식 습관(과도한 clone, unwrap 남용 등)을 교정하여 더 Rust다운 코드를 작성하는 법을 배웁니다.

---

### 1. 네 가지 핵심 사고방식의 전환

| **구분** | **C# 방식** | **Rust 방식** | **이점** |
| :--- | :--- | :--- | :--- |
| **메모리** | 가비지 컬렉터(GC)에 의존 | **소유권(Ownership)**과 빌림 | 런타임 오버헤드 제거, 성능 극대화 |
| **에러** | 예외(Exception) 던지기 | **Result<T, E>** 반환 | 에러 발생 가능성을 명시적으로 관리 |
| **설계** | 클래스 상속 위주의 계층 구조 | **트레이트(Traits)**와 구성(Composition) | 유연한 기능 조합, 다중 상속 효과 |
| **안전** | 런타임 지연 바인딩, Null 허용 | **타임 시스템**과 Option<T> | Null 참조 에러(NRE) 원천 차단 |

---

### 2. 피해야 할 C#식 습관 (Anti-Patterns)

#### 🚫 모든 곳에 `.clone()` 사용하기
C#은 객체 참조를 넘길 때 비용이 거의 없지만, Rust에서 `.clone()`은 명시적인 데이터 복사(Deep Copy)를 유발합니다.
- **해결책**: 가급적 **빌림(`&`)**을 선호하세요. 소유권이 정말로 필요한 경우에만 복제합니다.

#### 🚫 운영 코드에서 `.unwrap()` 남용하기
C#에서 예외 처리를 잊는 것과 같습니다. 프로그램이 예기치 않게 종료될 수 있습니다.
- **해결책**: **`?` 연산자**를 사용하여 에러를 상위 호출자에게 전파하세요.

#### 🚫 상속 구조 몰입하기
`struct Manager : Employee`와 같은 방식은 Rust에 없습니다.
- **해결책**: 공통 기능을 **트레이트(Trait)**로 정의하고, 구조체가 해당 트레이트를 구현하게 하세요.

---

### 3. 실무 에러 처리 전략
C#의 `try-catch` 대신, 프로젝트 전체의 에러를 관리하는 열거형(Enum)을 정의하고 `thiserror` 크레이트를 활용하는 것이 표준입니다.

```rust
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("데이터베이스 연결 실패: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("유효하지 않은 입력: {0}")]
    InvalidInput(String),
}

pub type AppResult<T> = Result<T, AppError>;
```

---

### 💡 실무 팁: '평평한' 코드 작성하기
중첩된 `if`나 `match` 대신 콤비네이터(`map`, `and_then`, `filter`)를 사용하세요. 코드가 훨씬 읽기 쉬워지고 논리적 흐름이 한눈에 들어옵니다.

```rust
// [C# 스타일의 중첩 체크]
if let Some(user) = get_user() {
    if user.is_active() {
        process(user);
    }
}

// [Rust 스타일의 체이닝]
get_user()
    .filter(|u| u.is_active())
    .map(|u| process(u));
```

