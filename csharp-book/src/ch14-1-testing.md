## Rust와 C#의 테스트 비교

> **학습 내용:** 내장된 `#[test]`와 xUnit 비교, `rstest`를 사용한 매개변수화된 테스트(`[Theory]`와 유사), `proptest`를 사용한 속성 기반 테스트(property testing), `mockall`을 사용한 모킹(mocking), 그리고 비동기 테스트 패턴.
>
> **난이도:** 🟡 중급

### 단위 테스트 (Unit Tests)
```csharp
// C# — xUnit
using Xunit;

public class CalculatorTests
{
    [Fact]
    public void Add_ReturnsSum()
    {
        var calc = new Calculator();
        Assert.Equal(5, calc.Add(2, 3));
    }

    [Theory]
    [InlineData(1, 2, 3)]
    [InlineData(0, 0, 0)]
    [InlineData(-1, 1, 0)]
    public void Add_Theory(int a, int b, int expected)
    {
        Assert.Equal(expected, new Calculator().Add(a, b));
    }
}
```

```rust
// Rust — 내장 테스트 기능 제공, 외부 프레임워크 불필요
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]  // `cargo test` 실행 시에만 컴파일됨
mod tests {
    use super::*;  // 부모 모듈로부터 아이템 가져오기

    #[test]
    fn add_returns_sum() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_negative_numbers() {
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn add_overflow_panics() {
        let _ = add(i32::MAX, 1); // 디버그 모드에서 패닉 발생
    }
}
```

### 매개변수화된 테스트 (`[Theory]`와 유사)
```rust
// 매개변수화된 테스트를 위해 `rstest` 크레이트를 사용합니다.
use rstest::rstest;

#[rstest]
#[case(1, 2, 3)]
#[case(0, 0, 0)]
#[case(-1, 1, 0)]
fn test_add(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
    assert_eq!(add(a, b), expected);
}

// Fixtures — 테스트 설정(setup) 메서드와 유사함
#[rstest]
fn test_with_fixture(#[values(1, 2, 3)] x: i32) {
    assert!(x > 0);
}
```

### 단언문(Assertions) 비교

| C# (xUnit) | Rust | 비고 |
|-------------|------|-------|
| `Assert.Equal(expected, actual)` | `assert_eq!(expected, actual)` | 실패 시 차이점(diff) 출력 |
| `Assert.NotEqual(a, b)` | `assert_ne!(a, b)` | |
| `Assert.True(condition)` | `assert!(condition)` | |
| `Assert.Contains("sub", str)` | `assert!(str.contains("sub"))` | |
| `Assert.Throws<T>(() => ...)` | `#[should_panic]` | 또는 `std::panic::catch_unwind` 사용 |
| `Assert.Null(obj)` | `assert!(option.is_none())` | null 없음 — `Option` 사용 |

### 테스트 조직화

```text
my_crate/
├── src/
│   ├── lib.rs          # #[cfg(test)] mod tests { } 내부의 단위 테스트
│   └── parser.rs       # 각 모듈은 자신만의 테스트 모듈을 가질 수 있음
├── tests/              # 통합 테스트 (각 파일은 별도의 크레이트로 간주됨)
│   ├── parser_test.rs  # 외부 소비자 입장에서 공개 API 테스트
│   └── api_test.rs
└── benches/            # 벤치마크 (criterion 크레이트 사용)
    └── my_benchmark.rs
```

```rust
// tests/parser_test.rs — 통합 테스트
// 공개 API(PUBLIC API)에만 접근 가능 (외부 어셈블리에서 테스트하는 것과 같음)
use my_crate::parser;

#[test]
fn test_parse_valid_input() {
    let result = parser::parse("valid input");
    assert!(result.is_ok());
}
```

### 비동기 테스트
```csharp
// C# — xUnit을 사용한 비동기 테스트
[Fact]
public async Task GetUser_ReturnsUser()
{
    var service = new UserService();
    var user = await service.GetUserAsync(1);
    Assert.Equal("Alice", user.Name);
}
```

```rust
// Rust — tokio를 사용한 비동기 테스트
#[tokio::test]
async fn get_user_returns_user() {
    let service = UserService::new();
    let user = service.get_user(1).await.unwrap();
    assert_eq!(user.name, "Alice");
}
```

### mockall을 사용한 모킹(Mocking)
```rust
use mockall::automock;

#[automock]                         // MockUserRepo 구조체 생성
trait UserRepo {
    fn find_by_id(&self, id: u32) -> Option<User>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_returns_user_from_repo() {
        let mut mock = MockUserRepo::new();
        mock.expect_find_by_id()
            .with(mockall::predicate::eq(1))
            .returning(|_| Some(User { name: "Alice".into() }));

        let service = UserService::new(mock);
        let user = service.get_user(1).unwrap();
        assert_eq!(user.name, "Alice");
    }
}
```

```csharp
// C# — Moq의 경우
var mock = new Mock<IUserRepo>();
mock.Setup(r => r.FindById(1)).Returns(new User { Name = "Alice" });
var service = new UserService(mock.Object);
Assert.Equal("Alice", service.GetUser(1).Name);
```

<details>
<summary><strong>🏋️ 연습 문제: 포괄적인 테스트 작성하기</strong> (클릭하여 펼치기)</summary>

**도전 과제**: 주어진 함수에 대해 정상적인 경로(happy path), 빈 입력, 숫자 문자열, 그리고 유니코드 문자를 포함하는 테스트를 작성해 보세요.

```rust
pub fn title_case(input: &str) -> String {
    input.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => format!("{}{}", c.to_uppercase(), chars.as_str().to_lowercase()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
```

<details>
<summary>🔑 정답</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(title_case("hello world"), "Hello World");
    }

    #[test]
    fn empty_input() {
        assert_eq!(title_case(""), "");
    }

    #[test]
    fn single_word() {
        assert_eq!(title_case("rust"), "Rust");
    }

    #[test]
    fn already_title_case() {
        assert_eq!(title_case("Hello World"), "Hello World");
    }

    #[test]
    fn all_caps() {
        assert_eq!(title_case("HELLO WORLD"), "Hello World");
    }

    #[test]
    fn extra_whitespace() {
        // split_whitespace는 여러 개의 공백을 처리합니다.
        assert_eq!(title_case("  hello   world  "), "Hello World");
    }

    #[test]
    fn unicode() {
        assert_eq!(title_case("café résumé"), "Café Résumé");
    }

    #[test]
    fn numeric_words() {
        assert_eq!(title_case("hello 42 world"), "Hello 42 World");
    }
}
```

**핵심 요점**: Rust의 내장 테스트 프레임워크는 대부분의 단위 테스트 요구사항을 충족합니다. 매개변수화된 테스트에는 `rstest`를, 모킹에는 `mockall`을 사용하세요. xUnit과 같은 대규모 테스트 프레임워크는 별도로 필요하지 않습니다.

</details>
</details>


<!-- ch14a.1: Property Testing with proptest -->
## 속성 기반 테스트: 대규모 정확성 검증 (Property Testing)

**FsCheck**가 익숙한 C# 개발자라면 속성 기반 테스트를 쉽게 이해하실 것입니다. 개별 테스트 케이스를 작성하는 대신, **모든 가능한 입력**에 대해 유지되어야 하는 *속성(property)*을 기술하면, 프레임워크가 수천 개의 무작위 입력을 생성하여 해당 속성을 깨뜨리려고 시도합니다.

### 속성 기반 테스트가 중요한 이유
```csharp
// C# — 직접 작성한 단위 테스트는 특정 케이스만 확인합니다.
[Fact]
public void Reverse_Twice_Returns_Original()
{
    var list = new List<int> { 1, 2, 3 };
    list.Reverse();
    list.Reverse();
    Assert.Equal(new[] { 1, 2, 3 }, list);
}
// 하지만 빈 리스트는 어떨까요? 요소가 하나뿐인 경우는? 요소가 10,000개인 경우는? 음수는?
// 수십 개의 테스트 케이스를 직접 작성해야 할 것입니다.
```

```rust
// Rust — proptest가 수천 개의 입력을 자동으로 생성합니다.
use proptest::prelude::*;

fn reverse<T: Clone>(v: &[T]) -> Vec<T> {
    v.iter().rev().cloned().collect()
}

proptest! {
    #[test]
    fn reverse_twice_is_identity(ref v in prop::collection::vec(any::<i32>(), 0..1000)) {
        let reversed_twice = reverse(&reverse(v));
        prop_assert_eq!(v, &reversed_twice);
    }
    // proptest는 수백 개의 무작위 Vec<i32> 값을 사용하여 이 테스트를 실행합니다:
    // [], [0], [i32::MIN, i32::MAX], [42; 999], 무작위 시퀀스 등...
    // 만약 실패한다면, 실패를 유발하는 가장 작은 입력으로 축소(SHRINK)합니다!
}
```

### proptest 시작하기
```toml
# Cargo.toml
[dev-dependencies]
proptest = "1.4"
```

### C# 개발자를 위한 공통 패턴

```rust
use proptest::prelude::*;

// 1. 왕복(Roundtrip) 속성: 직렬화 → 역직렬화 = 동일함
// (JsonSerializer.Serialize → Deserialize 테스트와 유사)
proptest! {
    #[test]
    fn json_roundtrip(name in "[a-zA-Z]{1,50}", age in 0u32..150) {
        let user = User { name: name.clone(), age };
        let json = serde_json::to_string(&user).unwrap();
        let parsed: User = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(user, parsed);
    }
}

// 2. 불변성(Invariant) 속성: 출력 결과가 항상 특정 조건을 만족함
proptest! {
    #[test]
    fn sort_output_is_sorted(ref v in prop::collection::vec(any::<i32>(), 0..500)) {
        let mut sorted = v.clone();
        sorted.sort();
        // 모든 인접한 쌍이 순서대로 정렬되어 있어야 함
        for window in sorted.windows(2) {
            prop_assert!(window[0] <= window[1]);
        }
    }
}

// 3. 오라클(Oracle) 속성: 두 가지 구현 방식을 비교함
proptest! {
    #[test]
    fn fast_path_matches_slow_path(input in "[0-9a-f]{1,100}") {
        let result_fast = parse_hex_fast(&input);
        let result_slow = parse_hex_slow(&input);
        prop_assert_eq!(result_fast, result_slow);
    }
}

// 4. 커스텀 전략(Custom strategies): 도메인 특화 테스트 데이터 생성
fn valid_email() -> impl Strategy<Value = String> {
    ("[a-z]{1,20}", "[a-z]{1,10}", prop::sample::select(vec!["com", "org", "io"]))
        .prop_map(|(user, domain, tld)| format!("{}@{}.{}", user, domain, tld))
}

proptest! {
    #[test]
    fn email_parsing_accepts_valid_emails(email in valid_email()) {
        let result = Email::new(&email);
        prop_assert!(result.is_ok(), "이메일 파싱 실패: {}", email);
    }
}
```

### proptest vs FsCheck 비교

| 기능 | C# FsCheck | Rust proptest |
|---------|-----------|---------------|
| 무작위 입력 생성 | `Arb.Generate<T>()` | `any::<T>()` |
| 커스텀 생성기 | `Arb.Register<T>()` | `impl Strategy<Value = T>` |
| 실패 시 축소 (Shrinking) | 자동 | 자동 |
| 문자열 패턴 | 수동 작성 | `"[regex]"` 전략 |
| 컬렉션 생성 | `Gen.ListOf` | `prop::collection::vec(strategy, range)` |
| 생성기 조합 | `Gen.Select` | `.prop_map()`, `.prop_flat_map()` |
| 설정 (케이스 수) | `Config.MaxTest` | `proptest!` 블록 내부에 `#![proptest_config(ProptestConfig::with_cases(10000))]` 사용 |

### 속성 기반 테스트 vs 단위 테스트 사용 시점

| **단위 테스트**를 사용하는 경우 | **proptest**를 사용하는 경우 |
|------------------------|----------------------|
| 특정 엣지 케이스를 테스트할 때 | 모든 입력에 대한 불변성을 검증할 때 |
| 에러 메시지/코드를 테스트할 때 | 왕복 속성(파싱 ↔ 포맷팅)을 확인할 때 |
| 통합/모의(mock) 테스트를 수행할 때 | 두 가지 다른 구현 방식을 비교할 때 |
| 동작이 정확한 값에 의존할 때 | "모든 X에 대해, 속성 P가 성립한다"를 보장할 때 |

---

## 통합 테스트: `tests/` 디렉토리

단위 테스트는 `src/` 내부에 `#[cfg(test)]`와 함께 위치합니다. 통합 테스트는 별도의 `tests/` 디렉토리에 위치하며, 마치 C# 통합 테스트가 프로젝트를 외부 어셈블리로 참조하는 것처럼 크레이트의 **공개 API**를 테스트합니다.

```
my_crate/
├── src/
│   ├── lib.rs          // 공개 API
│   └── internal.rs     // 비공개 구현
├── tests/
│   ├── smoke.rs        // 각 파일은 별도의 테스트 바이너리로 컴파일됨
│   ├── api_tests.rs
│   └── common/
│       └── mod.rs      // 공유 테스트 헬퍼
└── Cargo.toml
```

### 통합 테스트 작성하기

`tests/` 내의 각 파일은 라이브러리에 의존하는 별도의 크레이트로 컴파일됩니다.

```rust
// tests/smoke.rs — my_crate의 pub 아이템에만 접근 가능
use my_crate::{process_order, Order, OrderResult};

#[test]
fn process_valid_order_returns_confirmation() {
    let order = Order::new("SKU-001", 3);
    let result = process_order(order);
    assert!(matches!(result, OrderResult::Confirmed { .. }));
}
```

### 공유 테스트 헬퍼 (Shared Test Helpers)

공유 설정 코드는 `tests/common/mod.rs`에 두어야 합니다. (`tests/common.rs`로 만들면 그 자체가 하나의 테스트 파일로 취급되므로 주의하세요.)

```rust
// tests/common/mod.rs
use my_crate::Config;

pub fn test_config() -> Config {
    Config::builder()
        .database_url("sqlite::memory:")
        .build()
        .expect("테스트 설정이 유효해야 함")
}
```

```rust
// tests/api_tests.rs
mod common;

use my_crate::App;

#[test]
fn app_starts_with_test_config() {
    let config = common::test_config();
    let app = App::new(config);
    assert!(app.is_healthy());
}
```

### 특정 유형의 테스트 실행하기

```bash
cargo test                  # 모든 테스트 실행 (단위 + 통합)
cargo test --lib            # 단위 테스트만 실행 (dotnet test --filter Category=Unit과 유사)
cargo test --test smoke     # tests/smoke.rs만 실행
cargo test --test api_tests # tests/api_tests.rs만 실행
```

**C#과의 주요 차이점:** 통합 테스트 파일은 크레이트의 `pub` API에만 접근할 수 있습니다. 비공개 함수는 보이지 않으므로, 자연스럽게 공개 인터페이스를 통해 테스트하도록 유도됩니다. 이는 일반적으로 더 나은 테스트 설계를 지향하게 합니다.

***
