## C# 개발자를 위한 권장 사례 (Best Practices for C# Developers)

> **학습 내용:** 다섯 가지 핵심 사고방식의 전환(GC→소유권, 예외→Result, 상속→구성),
> 관용적인 프로젝트 구조화, 에러 처리 전략, 테스트 패턴, 그리고 C# 개발자가 Rust에서 저지르기 쉬운 실수들을 살펴봅니다.
>
> **난이도:** 🟡 중급

### 1. **사고방식의 전환 (Mindset Shifts)**
- **GC에서 소유권으로**: 누가 데이터를 소유하고 언제 해제되는지 생각하세요.
- **예외에서 Result로**: 에러 처리를 명시적이고 가시적으로 만드세요.
- **상속에서 구성으로**: 트레이트(traits)를 사용하여 기능을 조합하세요.
- **Null에서 Option으로**: 값이 없음을 타입 시스템에서 명시적으로 표현하세요.

### 2. **코드 구조화 (Code Organization)**
```rust
// C# 솔루션과 유사한 프로젝트 구조
src/
├── main.rs          // Program.cs와 유사
├── lib.rs           // 라이브러리 엔트리 포인트
├── models/          // C#의 Models/ 폴더와 유사
│   ├── mod.rs
│   ├── user.rs
│   └── product.rs
├── services/        // C#의 Services/ 폴더와 유사
│   ├── mod.rs
│   ├── user_service.rs
│   └── product_service.rs
├── controllers/     // 웹 앱의 Controllers/와 유사
├── repositories/    // Repositories/와 유사
└── utils/           // Utilities/와 유사
```

### 3. **에러 처리 전략 (Error Handling Strategy)**
```rust
// 애플리케이션 공용 Result 타입 정의
pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("데이터베이스 에러: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("HTTP 에러: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("유효성 검사 에러: {message}")]
    Validation { message: String },
    
    #[error("비즈니스 로직 에러: {message}")]
    Business { message: String },
}

// 애플리케이션 전체에서 활용
pub async fn create_user(data: CreateUserRequest) -> AppResult<User> {
    validate_user_data(&data)?;  // AppError::Validation 반환
    let user = repository.create_user(data).await?;  // AppError::Database 반환
    Ok(user)
}
```

### 4. **테스트 패턴 (Testing Patterns)**
```rust
// C# 단위 테스트와 유사한 구조
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;  // C#의 [Theory]와 같은 매개변수화된 테스트용
    
    #[test]
    fn test_basic_functionality() {
        // Arrange (준비)
        let input = "test data";
        
        // Act (실행)
        let result = process_data(input);
        
        // Assert (단언)
        assert_eq!(result, "expected output");
    }
    
    #[rstest]
    #[case(1, 2, 3)]
    #[case(5, 5, 10)]
    #[case(0, 0, 0)]
    fn test_addition(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
        assert_eq!(add(a, b), expected);
    }
    
    #[tokio::test]  // 비동기 테스트용
    async fn test_async_functionality() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### 5. **피해야 할 흔한 실수 (Common Mistakes to Avoid)**
```rust
// [오류] 상속을 구현하려고 하지 마세요
// 다음과 같은 방식은 Rust에 존재하지 않습니다:
// struct Manager : Employee

// [권장] 트레이트를 통한 구성(composition) 사용
trait Employee {
    fn get_salary(&self) -> u32;
}

trait Manager: Employee {
    fn get_team_size(&self) -> usize;
}

// [오류] 모든 곳에 unwrap()을 사용하지 마세요 (예외 무시와 유사함)
let value = might_fail().unwrap();  // 패닉(panic) 발생 가능!

// [권장] 에러를 적절히 처리하세요
let value = match might_fail() {
    Ok(v) => v,
    Err(e) => {
        log::error!("작업 실패: {}", e);
        return Err(e.into());
    }
};

// [오류] 모든 것을 clone() 하지 마세요 (불필요한 객체 복사와 유사함)
let data = expensive_data.clone();  // 비용이 많이 듭니다!

// [권장] 가능하다면 빌림(borrowing)을 사용하세요
let data = &expensive_data;  // 단순 참조

// [오류] 모든 곳에 RefCell을 사용하지 마세요 (모든 것을 가변으로 만드는 것과 유사함)
struct Data {
    value: RefCell<i32>,  // 내부 가변성 - 절제해서 사용하세요
}

// [권장] 소유하거나 빌린 데이터를 선호하세요
struct Data {
    value: i32,  // 단순하고 명확함
}
```

이 가이드는 C# 개발자들이 기존 지식을 Rust로 어떻게 전환할 수 있는지에 대한 포괄적인 이해를 돕습니다. 소유권과 같은 Rust의 제약 사항은 초기 복잡성을 수반하지만, C#에서 발생할 수 있는 여러 클래스의 버그를 원천적으로 방지하도록 설계되었습니다.

---

### 6. **과도한 `clone()` 피하기** 🟡

C# 개발자들은 가비지 컬렉터가 비용을 처리해주기 때문에 본능적으로 데이터를 복제하곤 합니다. 하지만 Rust에서 모든 `.clone()`은 명시적인 할당을 의미합니다. 빌림을 통해 대부분의 복제를 제거할 수 있습니다.

```rust
// [오류] C# 습관: 문자열을 전달할 때마다 복제함
fn greet(name: String) {
    println!("안녕하세요, {name}");
}

let user_name = String::from("Alice");
greet(user_name.clone());  // 불필요한 할당
greet(user_name.clone());  // 또다시 할당

// [권장] 빌림 사용 — 할당 발생 안 함
fn greet(name: &str) {
    println!("안녕하세요, {name}");
}

let user_name = String::from("Alice");
greet(&user_name);  // 빌림
greet(&user_name);  // 다시 빌림 — 비용 없음
```

**clone이 적절한 경우:**
- 데이터를 스레드나 `'static` 클로저로 이동시킬 때 (`Arc::clone`은 카운터만 증가시키므로 저렴함)
- 캐싱: 독립적인 복사본이 진정으로 필요한 경우
- 프로토타이핑: 일단 작동하게 만든 후, 나중에 clone을 제거하며 최적화할 때

**결정 체크리스트:**
1. `&T`나 `&str`을 대신 전달할 수 있는가? → 그렇게 하세요.
2. 호출된 함수가 소유권을 필요로 하는가? → clone이 아닌 이동(move)으로 전달하세요.
3. 스레드 간에 공유되는가? → `Arc<T>`를 사용하세요 (clone은 참조 횟수만 증가시킵니다).
4. 위 경우에 해당하지 않는가? → 이때는 `clone()`이 정당화될 수 있습니다.

---

### 7. **운영 코드에서 `unwrap()` 피하기** 🟡

예외를 무시하는 습관이 있는 C# 개발자들은 Rust 코드 모든 곳에 `.unwrap()`을 작성하곤 합니다. 둘 다 똑같이 위험합니다.

```rust
// [오류] "나중에 고치겠지"라는 함정
let config = std::fs::read_to_string("config.toml").unwrap();
let port: u16 = config_value.parse().unwrap();
let conn = db_pool.get().await.unwrap();

// [권장] 애플리케이션 코드에서 ?로 전파하기
let config = std::fs::read_to_string("config.toml")?;
let port: u16 = config_value.parse()?;
let conn = db_pool.get().await?;

// [권장] 실패가 진정한 버그인 경우에만 expect() 사용
let home = std::env::var("HOME")
    .expect("HOME 환경 변수가 반드시 설정되어 있어야 합니다");  // 불변 조건을 문서화함
```

**경험 법칙:**
| 메서드 | 사용 시점 |
|--------|------------|
| `?` | 애플리케이션/라이브러리 코드 — 호출자에게 에러 전파 |
| `expect("이유")` | 시작 시점의 단언(assertions), 반드시 유지되어야 하는 불변 조건 |
| `unwrap()` | 테스트 코드, 또는 `is_some()`/`is_ok()` 체크 직후 |
| `unwrap_or(기본값)` | 적절한 대체값이 있는 경우 |
| `unwrap_or_else(|| ...)` | 대체값을 계산하는 비용이 큰 경우 |

---

### 8. **빌림 검사기(Borrow Checker)와의 싸움 (그리고 대처법)** 🟡

모든 C# 개발자는 빌림 검사기가 유효해 보이는 코드를 거부하는 단계를 겪게 됩니다. 해결책은 대개 임시방편이 아니라 구조적인 변경입니다.

```rust
// [오류] 순회 중에 수정을 시도함 (C#의 foreach + 수정 패턴)
let mut items = vec![1, 2, 3, 4, 5];
for item in &items {
    if *item > 3 {
        items.push(*item * 2);  // 에러: items를 가변으로 빌릴 수 없음
    }
}

// [권장] 먼저 수집한 후 수정함
let extras: Vec<i32> = items.iter()
    .filter(|&&x| x > 3)
    .map(|&x| x * 2)
    .collect();
items.extend(extras);
```

```rust
// [오류] 지역 변수에 대한 참조를 반환함 (C#은 GC를 통해 자유롭게 참조 반환 가능)
fn get_greeting() -> &str {
    let s = String::from("안녕");
    &s  // 에러: 함수 끝에서 s가 드롭(drop)됨
}

// [권장] 소유한 데이터를 반환함
fn get_greeting() -> String {
    String::from("안녕")  // 호출자가 소유권을 가짐
}
```

**빌림 검사기 갈등을 해결하는 일반적인 패턴:**

| C# 습관 | Rust 해결책 |
|----------|--------------|
| 구조체에 참조 저장하기 | 소유한 데이터를 사용하거나 수명(lifetime) 매개변수 추가 |
| 공유 상태를 자유롭게 수정하기 | `Arc<Mutex<T>>`를 사용하거나 공유를 피하도록 재구조화 |
| 지역 변수에 대한 참조 반환하기 | 소유한 값 반환 |
| 순회 중에 컬렉션 수정하기 | 변경 사항을 별도로 수집한 후 적용 |
| 다중 가변 참조 시도 | 구조체를 독립적인 부분들로 분리 |

---

### 9. **할당 피라미드 축소하기** 🟢

C# 개발자들은 `if (x != null) { if (x.Value > 0) { ... } }`와 같은 중첩된 구조를 작성하곤 합니다. Rust의 `match`, `if let`, 그리고 `?` 연산자는 이를 평평하게 펴줍니다.

```rust
// [오류] C# 스타일의 중첩된 널 체크 방식
fn process(input: Option<String>) -> Option<usize> {
    match input {
        Some(s) => {
            if !s.is_empty() {
                match s.parse::<usize>() {
                    Ok(n) => {
                        if n > 0 {
                            Some(n * 2)
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                }
            } else {
                None
            }
        }
        None => None,
    }
}

// [권장] 콤비네이터(combinators)를 사용한 평탄화
fn process(input: Option<String>) -> Option<usize> {
    input
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|&n| n > 0)
        .map(|n| n * 2)
}
```

**모든 C# 개발자가 알아야 할 핵심 콤비네이터:**

| 콤비네이터 | 역할 | C# 대응 개념 |
|-----------|-------------|---------------|
| `map` | 내부 값 변환 | `Select` / 널 조건 연산자 `?.` |
| `and_then` | Option/Result를 반환하는 작업 체이닝 | `SelectMany` / `?.Method()` |
| `filter` | 조건에 맞는 값만 유지 | `Where` |
| `unwrap_or` | 기본값 제공 | `??` (Null 병합 연산자) |
| `ok()` | `Result`를 `Option`으로 변환 (에러 무시) | — |
| `transpose` | `Option<Result>`를 `Result<Option>`으로 전환 | — |



***
