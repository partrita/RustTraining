# 에러 처리: 예외(Exception)를 넘어 Result로

> **학습 목표:** 파이썬의 `try-except` 방식과 Rust의 `Result<T, E>` 기반 에러 처리의 근본적인 차이를 배웁니다. 에러가 발생할 수 있음을 함수 시그니처에 명시하고, `?` 연산자를 사용해 예외 전파를 우아하게 처리하는 법을 익힙니다.

---

### 1. 예외(Exception) vs Result
파이썬은 어디서든 예외를 던지고(raise) 어디서든 잡을(except) 수 있지만, Rust는 에러를 **반환값(Value)**으로 취급합니다.

| **구분** | **Python (예외)** | **Rust (Result)** | **비고** |
| :--- | :--- | :--- | :--- |
| **흐름 제어** | 예외 발생 시 스택을 거슬러 올라감 | 일반적인 함수 반환 흐름을 따름 | Rust가 훨씬 예측 가능함 |
| **명시성** | 어떤 에러가 날지 코드를 봐야 함 | 함수 타입에 에러 종류가 명시됨 | Rust는 컴파일 타임에 확인 가능 |
| **강제성** | 처리를 잊어도 실행은 됨 (런타임 에러) | 처리를 잊으면 컴파일 경고/에러 발생 | Rust는 에러 처리를 강제함 |

---

### 2. Result<T, E>의 구조
Result는 성공(`Ok`) 혹은 실패(`Err`) 중 하나의 상태를 가지는 열거형입니다.

```rust
enum Result<T, E> {
    Ok(T),  // 성공 시 결과값
    Err(E), // 실패 시 에러 내용
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("0으로 나눌 수 없습니다".to_string())
    } else {
        Ok(a / b)
    }
}
```

---

### 3. `?` 연산자: 우아한 에러 전파
파이썬에서 `try-except` 블록 없이 예외가 위로 전달되는 것처럼, Rust에서는 `?` 하나로 에러를 상위 함수로 보낼 수 있습니다.

```rust
fn read_config() -> Result<String, io::Error> {
    // 파일 읽기에 실패하면 즉시 함수를 종료하고 Err을 반환합니다.
    let content = fs::read_to_string("config.txt")?;
    Ok(content)
}
```

---

### 4. 커스텀 에러 정의 (`thiserror`)
실무에서는 `thiserror` 크레이트를 사용해 여러 종류의 에러를 하나의 열거형으로 묶어서 관리합니다.

```rust
#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("입출력 오류: {0}")]
    Io(#[from] std::io::Error), // io::Error를 자동으로 MyError::Io로 변환
    
    #[error("데이터가 존재하지 않음")]
    NotFound,
}
```

---

### 💡 실무 팁: `unwrap()`은 테스트에서만
값이 반드시 있다고 확신할 때 쓰는 `unwrap()`은 운영 코드에서 가급적 피해야 합니다. 대신 `expect("에러 메시지")`를 써서 실패 이유를 명시하거나, `?`를 사용하는 것이 Rust다운 방식입니다.

