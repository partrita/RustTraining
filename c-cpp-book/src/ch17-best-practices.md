# Rust 모범 사례(Best Practices) 요약

> **학습 내용:** 관용적인(idiomatic) Rust 코드를 작성하기 위한 실무 가이드라인을 배웁니다. 코드 구조화, 명명 규칙, 에러 처리 패턴 및 문서화 방법을 다룹니다. 자주 찾아보게 될 요약 챕터입니다.

## 코드 구조화 (Code Organization)
- **작은 함수를 선호하십시오**: 테스트하기 쉽고 논리적으로 이해하기 좋습니다.
- **설명적인 이름을 사용하십시오**: `calc()`보다는 `calculate_total_price()`가 좋습니다.
- **관련 기능을 그룹화하십시오**: 모듈과 별도의 파일을 활용하십시오.
- **문서 주석을 작성하십시오**: 공개 API에는 `///`를 사용하십시오.

## 에러 처리 (Error Handling)
- **실패하지 않는다는 확신이 없다면 `unwrap()`을 피하십시오**: 절대 패닉이 발생하지 않을 것이라고 100% 확신할 때만 사용하십시오.
```rust
// 나쁜 예: 패닉이 발생할 수 있음
let value = some_option.unwrap();

// 좋은 예: None 케이스를 처리함
let value = some_option.unwrap_or(default_value);
let value = some_option.unwrap_or_else(|| expensive_computation());
let value = some_option.unwrap_or_default(); // Default 트레이트 사용

// Result<T, E>의 경우
let value = some_result.unwrap_or(fallback_value);
let value = some_result.unwrap_or_else(|err| {
    eprintln!("에러 발생: {err}");
    default_value
});
```
- **상세한 메시지와 함께 `expect()`를 사용하십시오**: unwrap을 사용하는 것이 정당하다면, 왜 그런지 이유를 설명하십시오.
```rust
let config = std::env::var("CONFIG_PATH")
    .expect("CONFIG_PATH 환경 변수가 반드시 설정되어야 합니다");
```
- **실패 가능성이 있는 연산에는 `Result<T, E>`를 반환하십시오**: 에러를 어떻게 처리할지는 호출자가 결정하게 하십시오.
- **커스텀 에러 타입에는 `thiserror`를 사용하십시오**: 수동으로 구현하는 것보다 더 편리합니다.
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO 에러: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("파싱 에러: {message}")]
    Parse { message: String },
    
    #[error("값 {value}이 범위를 벗어났습니다")]
    OutOfRange { value: i32 },
}
```
- **`?` 연산자로 에러를 체이닝하십시오**: 에러를 호출 스택 위로 전파하십시오.
- **`anyhow`보다 `thiserror`를 선호하십시오**: 우리 팀의 컨벤션은 호출자가 특정 변형(variant)을 매칭할 수 있도록 `#[derive(thiserror::Error)]`를 사용하여 명시적인 에러 열거형을 정의하는 것입니다. `anyhow::Error`는 빠른 프로토타이핑에는 편리하지만 에러 타입을 지워버리기 때문에 호출자가 특정 실패 상황을 처리하기 어렵게 만듭니다. 라이브러리와 실제 운영 코드에는 `thiserror`를 사용하고, `anyhow`는 일회성 스크립트나 에러 출력만 필요한 최상위 바이너리로 제한하십시오.
- **`unwrap()`이 허용되는 경우**:
  - **단위 테스트(Unit tests)**: `assert_eq!(result.unwrap(), expected)`
  - **프로토타이핑**: 나중에 교체할 임시 코드
  - **실패할 리 없는 연산**: 실패하지 않음을 증명할 수 있을 때
```rust
let numbers = vec![1, 2, 3];
let first = numbers.get(0).unwrap(); // 안전함: 방금 요소가 있는 벡터를 생성했기 때문

// 더 나은 방법: 설명을 포함한 expect() 사용
let first = numbers.get(0).expect("벡터가 비어 있지 않음이 보장됨");
```
- **빨리 실패하기(Fail fast)**: 전제 조건을 조기에 확인하고 즉시 에러를 반환하십시오.

## 메모리 관리 (Memory Management)
- **복제(cloning)보다 빌림(borrowing)을 선호하십시오**: 가능하면 `clone()` 대신 `&T`를 사용하십시오.
- **`Rc<T>` 사용을 절제하십시오**: 공유 소유권이 정말로 필요한 경우에만 사용하십시오.
- **수명을 제한하십시오**: 값이 드롭되는 시점을 제어하기 위해 스코프 `{}`를 활용하십시오.
- **공개 API에서 `RefCell<T>`를 피하십시오**: 내부 가변성(interior mutability)은 내부적으로만 유지하십시오.

## 성능 (Performance)
- **최적화 전 프로파일링을 수행하십시오**: `cargo bench`와 프로파일링 도구를 사용하십시오.
- **루프보다 반복자(iterator)를 선호하십시오**: 가독성이 더 좋고 대개 더 빠릅니다.
- **소유권이 필요 없을 때는 `String`보다 `&str`를 사용하십시오**: 
- **커다란 스택 객체에는 `Box<T>`를 고려하십시오**: 필요하다면 힙으로 이동시키십시오.

## 반드시 구현해야 할 핵심 트레이트들 (Essential Traits)

### 모든 타입이 고려해야 할 기본 트레이트

커스텀 타입을 생성할 때, Rust의 기본 타입처럼 느껴지도록 다음 기본 트레이트들을 구현하는 것을 고려하십시오.

#### **Debug 및 Display**
```rust
use std::fmt;

#[derive(Debug)]  // 디버깅을 위한 자동 구현
struct Person {
    name: String,
    age: u32,
}

// 사용자에게 보여줄 출력을 위한 수동 Display 구현
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (나이: {})", self.name, self.age)
    }
}

// 사용 예시:
let person = Person { name: "Alice".to_string(), age: 30 };
println!("{:?}", person);  // Debug: Person { name: "Alice", age: 30 }
println!("{}", person);    // Display: Alice (나이: 30)
```

#### **Clone 및 Copy**
```rust
// Copy: 작고 단순한 타입의 암시적 복사
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// Clone: 복잡한 타입의 명시적 복제
#[derive(Debug, Clone)]
struct Person {
    name: String,  // String은 Copy를 구현하지 않음
    age: u32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1;  // Copy 발생 (암시적)

let person1 = Person { name: "Bob".to_string(), age: 25 };
let person2 = person1.clone();  // Clone 발생 (명시적)
```

#### **PartialEq 및 Eq**
```rust
#[derive(Debug, PartialEq, Eq)]
struct UserId(u64);

#[derive(Debug, PartialEq)]
struct Temperature {
    celsius: f64,  // f64는 (NaN 때문에) Eq를 구현하지 않음
}

let id1 = UserId(123);
let id2 = UserId(123);
assert_eq!(id1, id2);  // PartialEq 덕분에 작동함

let temp1 = Temperature { celsius: 20.0 };
let temp2 = Temperature { celsius: 20.0 };
assert_eq!(temp1, temp2);  // PartialEq로 작동함
```

#### **PartialOrd 및 Ord**
```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Priority(u8);

let high = Priority(1);
let low = Priority(10);
assert!(high < low);  // 숫자가 낮을수록 높은 우선순위

// 컬렉션에서의 사용
let mut priorities = vec![Priority(5), Priority(1), Priority(8)];
priorities.sort();  // Priority가 Ord를 구현하므로 작동함
```

#### **Default**
```rust
#[derive(Debug, Default)]
struct Config {
    debug: bool,           // false (기본값)
    max_connections: u32,  // 0 (기본값)
    timeout: Option<u64>,  // None (기본값)
}

// 수동 Default 구현
impl Default for Config {
    fn default() -> Self {
        Config {
            debug: false,
            max_connections: 100,  // 커스텀 기본값
            timeout: Some(30),     // 커스텀 기본값
        }
    }
}

let config = Config::default();
let config = Config { debug: true, ..Default::default() };  // 일부 필드만 덮어쓰기
```

#### **From 및 Into**
```rust
struct UserId(u64);
struct UserName(String);

// From을 구현하면 Into는 자동으로 제공됩니다.
impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        UserId(id)
    }
}

impl From<String> for UserName {
    fn from(name: String) -> Self {
        UserName(name)
    }
}

impl From<&str> for UserName {
    fn from(name: &str) -> Self {
        UserName(name.to_string())
    }
}

// 사용 예시:
let user_id: UserId = 123u64.into();         // Into 사용
let user_id = UserId::from(123u64);          // From 사용
let username = UserName::from("alice");      // &str -> UserName
let username: UserName = "bob".into();       // Into 사용
```

#### **TryFrom 및 TryInto**
```rust
use std::convert::TryFrom;

struct PositiveNumber(u32);

#[derive(Debug)]
struct NegativeNumberError;

impl TryFrom<i32> for PositiveNumber {
    type Error = NegativeNumberError;
    
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(PositiveNumber(value as u32))
        } else {
            Err(NegativeNumberError)
        }
    }
}

// 사용 예시:
let positive = PositiveNumber::try_from(42)?;     // Ok(PositiveNumber(42))
let error = PositiveNumber::try_from(-5);         // Err(NegativeNumberError)
```

#### **Serde (직렬화용)**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// 자동 JSON 직렬화/역직렬화
let user = User {
    id: 1,
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
};

let json = serde_json::to_string(&user)?;
let deserialized: User = serde_json::from_str(&json)?;
```

### 트레이트 구현 체크리스트

새로운 타입을 만들 때마다 다음 체크리스트를 고려하십시오.

```rust
#[derive(
    Debug,          // [확인] 디버깅을 위해 항상 구현하십시오.
    Clone,          // [확인] 타입을 복제할 수 있어야 한다면 구현하십시오.
    PartialEq,      // [확인] 타입을 비교할 수 있어야 한다면 구현하십시오.
    Eq,             // [확인] 동등 비교가 반사적/이행적일 때 구현하십시오.
    PartialOrd,     // [확인] 타입에 순서가 있을 때 구현하십시오.
    Ord,            // [확인] 전체 순서(total ordering)가 있을 때 구현하십시오.
    Hash,           // [확인] HashMap의 키로 사용될 타입이라면 구현하십시오.
    Default,        // [확인] 합리적인 기본값이 있을 때 구현하십시오.
)]
struct MyType {
    // 필드들...
}

// 고려해볼 만한 수동 구현:
impl Display for MyType { /* 사용자에게 보여줄 표현 */ }
impl From<OtherType> for MyType { /* 편리한 변환 */ }
impl TryFrom<FallibleType> for MyType { /* 실패 가능한 변환 */ }
```

### 트레이트를 구현하지 말아야 할 때

- **힙 데이터를 가진 타입에는 Copy를 구현하지 마십시오**: `String`, `Vec`, `HashMap` 등.
- **값이 NaN이 될 수 있다면 Eq를 구현하지 마십시오**: `f32`/`f64`를 포함하는 타입들.
- **합리적인 기본값이 없다면 Default를 구현하지 마십시오**: 파일 핸들, 네트워크 연결 등.
- **복제 비용이 크다면 Clone을 구현하지 마십시오**: 커다란 데이터 구조체 (대신 `Rc<T>` 고려).

### 요약: 트레이트의 장점

| 트레이트 | 장점 | 사용 시점 |
|-------|---------|-------------|
| `Debug` | `println!("{:?}", value)` 가능 | 거의 항상 (드문 예외 제외) |
| `Display` | `println!("{}", value)` 가능 | 사용자에게 보여지는 타입 |
| `Clone` | `value.clone()` 가능 | 명시적인 복제가 의미 있을 때 |
| `Copy` | 암시적 복사 가능 | 작고 단순한 타입 |
| `PartialEq` | `==` 및 `!=` 연산자 사용 가능 | 대부분의 타입 |
| `Eq` | 반사적 동등성 보장 | 수학적으로 동등성이 성립할 때 |
| `PartialOrd` | `<`, `>`, `<=`, `>=` 연산자 가능 | 자연스러운 순서가 있는 타입 |
| `Ord` | `sort()`, `BinaryHeap` 사용 가능 | 전체 순서가 성립할 때 |
| `Hash` | `HashMap` 키 사용 가능 | 맵의 키로 사용될 타입 |
| `Default` | `Default::default()` 가능 | 명확한 기본값이 있는 타입 |
| `From/Into` | 편리한 변환 가능 | 일반적인 타입 변환 시 |
| `TryFrom/TryInto` | 실패 가능한 변환 가능 | 실패할 수 있는 타입 변환 시 |

----
