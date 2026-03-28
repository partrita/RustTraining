# 테스트: 내장 프레임워크와 견고한 검증

> **학습 목표:** 별도의 프레임워크 설치 없이 바로 사용할 수 있는 Rust의 내장 테스트 시스템을 배웁니다. C#의 xUnit/NUnit과 비교하며 단위 테스트, 통합 테스트, 그리고 무작위 입력을 통해 버그를 찾아내는 **속성 기반 테스트(Property Testing)** 기법을 익힙니다.

---

### 1. 단위 테스트 (Unit Tests)
Rust는 소스 코드 파일 안에 테스트 코드를 함께 작성하는 독특한 문법을 가지고 있습니다. 이를 통해 비공개(private) 함수도 쉽게 테스트할 수 있습니다.

```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)] // 테스트 실행 시에만 컴파일
mod tests {
    use super::*; // 부모 모듈의 add 함수 가져오기

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

---

### 2. 단언문(Assertions) 비교

| **C# (xUnit)** | **Rust** | **비고** |
| :--- | :--- | :--- |
| `Assert.Equal(exp, act)` | `assert_eq!(exp, act)` | 두 값이 같은지 검사 |
| `Assert.True(cond)` | `assert!(cond)` | 조건이 참인지 검사 |
| `Assert.NotNull(obj)` | `assert!(opt.is_some())` | 값이 존재하는지 (`Some`) 검사 |
| `Throws<Exception>(...)` | `#[should_panic]` | 패닉이 발생하는지 검사 |

---

### 3. 통합 테스트 (Integration Tests)
프로젝트 루트의 `tests/` 디렉토리에 작성합니다. 이곳의 테스트는 라이브러리의 사용자 입장에서 **공개(public) API**만 접근할 수 있습니다. C#에서 테스트 프로젝트를 따로 분리하는 것과 유사한 개념입니다.

```text
my_project/
├── src/
│   └── lib.rs (단위 테스트 포함)
└── tests/
    └── api_test.rs (통합 테스트)
```

---

### 4. 속성 기반 테스트 (Property Testing)
특정 입력값 하나만 테스트하는 대신, "어떤 입력이 들어와도 이 규칙은 지켜져야 한다"는 **속성**을 테스트합니다. `proptest` 크레이트를 사용하면 수만 개의 무작위 입력을 자동으로 생성해 엣지 케이스를 찾아냅니다.

```rust
proptest! {
    #[test]
    fn test_string_reverse(s in "\\PC*") {
        let reversed = reverse(&s);
        let double_reversed = reverse(&reversed);
        // "어떤 문자열이든 두 번 뒤집으면 원래대로 돌아와야 한다"는 속성 검증
        prop_assert_eq!(s, double_reversed);
    }
}
```

---

### 💡 실무 팁: 비동기 테스트와 모킹
- **`#[tokio::test]`**: 비동기 함수를 테스트할 때 사용합니다.
- **`mockall`**: 인터페이스(트레이트)를 모킹하여 외부 의존성 없이 로직을 검증할 때 필수적인 도구입니다. C#의 Moq나 NSubstitute와 같은 역할을 합니다.

