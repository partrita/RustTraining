# 철저한 패턴 매칭과 Null 안전성

> **학습 목표:** C#의 `switch` 표현식이 왜 잠재적인 런타임 에러를 가질 수 있는지 분석하고, Rust의 `match`가 어떻게 컴파일 타임에 모든 경로를 검증하는지 배웁니다. 또한 `Option<T>`와 `Result<T, E>`를 통해 '10억 달러짜리 실수(Null)'와 '예외(Exception)'를 어떻게 우아하게 대체하는지 익힙니다.

---

### 1. 철저한 매칭: 컴파일러가 지키는 마지막 방어선
C#의 `switch`는 새로운 열거형 변형이 추가되어도 경고만 줄 뿐 컴파일은 성공합니다. 반면 Rust는 **모든 경우의 수**를 처리하지 않으면 빌드조차 허용하지 않습니다.

```rust
enum Status { Pending, Approved, Rejected, OnHold }

fn handle(status: Status) {
    match status {
        Status::Pending => println!("대기 중"),
        Status::Approved => println!("승인됨"),
        Status::Rejected => println!("거절됨"),
        // [위험!] OnHold 케이스를 누락하면 컴파일 에러 발생!
        // C#에서는 런타임에 SwitchExpressionException이 발생할 수 있는 지점입니다.
    }
}
```

---

### 2. Null 안전성: `Option<T>` 시스템
C#은 `string?` 등을 통해 Null 안전성을 개선했지만, 여전히 런타임 예외의 가능성이 남아 있습니다. Rust는 `Option<T>`라는 열거형을 통해 값이 '있음(`Some`)과 '없음(`None`)을 명시적으로 구분합니다.

| **C# (Reference Type)** | **Rust (`Option<T>`)** | **의미** |
| :--- | :--- | :--- |
| `string name = null;` | `let name: Option<String> = None;` | 값이 없을 수 있음을 타입으로 표현 |
| `name.Length` (위험) | `match name { ... }` (필수) | 값 추출 전 반드시 존재 여부 확인 강제 |
| `name?.Length` | `name.map(|s| s.len())` | 안전한 체이닝 (콤비네이터 활용) |

---

### 3. 예외 대신 `Result<T, E>`
Rust는 `try-catch` 대신 `Result` 타입을 사용하여 에러 발생 가능성을 함수 시그니처에 명시합니다.

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("0으로 나눌 수 없습니다.".to_string())
    } else {
        Ok(a / b)
    }
}

// 물음표(?) 연산자로 에러 전파를 간결하게!
let result = divide(10.0, 2.0)?; // 에러 발생 시 즉시 return Err(...)
```

---

### 💡 실무 팁: `and_then`으로 Null 체이닝 정복하기
C#에서 `user?.Address?.City?.ToUpper()`와 같이 중첩된 Null 체크를 하듯, Rust에서는 `and_then`을 사용합니다. `None`이 발생하는 순간 이후 과정이 무시되므로 안전하고 가독성이 좋습니다.

```rust
let city = user
    .and_then(|u| u.address)
    .and_then(|a| a.city)
    .map(|c| c.to_uppercase())
    .unwrap_or("UNKNOWN".to_string());
```

