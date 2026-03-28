# 에러 처리: 예외(Exception)를 넘어선 명시적 설계

> **학습 목표:** Rust가 왜 `try-catch` 예외 시스템 대신 `Result<T, E>`와 `Option<T>`를 사용하는지 이해합니다. 에러 전파를 간결하게 만드는 `?` 연산자의 마법과, 모든 에러 경로를 명시적으로 다룸으로써 어떻게 더 견고한 소프트웨어를 만드는지 배웁니다.

---

### 1. 예외(Exception) vs Result 타입
C#에서는 언제 어디서 예외가 터질지 모르는 '숨겨진 제어 흐름'이 존재합니다. 반면 Rust는 에러 발생 가능성을 **함수의 반환 타입**에 명시합니다.

| **특징** | **C# (Exceptions)** | **Rust (Result & Option)** |
| :--- | :--- | :--- |
| **에러 표현** | `throw new Exception()` | `Err(Error)` 또는 `None` 반환 |
| **제어 흐름** | 스택 되감기 (런타임 비용 높음) | 일반적인 값 반환 (비용 없음) |
| **강제성** | `try-catch`를 잊어도 컴파일됨 | **반드시 처리해야 컴파일됨** |
| **가독성** | 함수 시그니처만으로 에러 예측 불가 | 반환 타입에 에러 종류가 명시됨 |

---

### 2. `?` 연산자: 우아한 에러 전파
C#에서 에러를 상위로 던지기 위해 아무것도 안 하거나 `throw;`를 하듯, Rust에서는 `?` 하나로 해결합니다.

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // 파일 열기 성공하면 f에 담고, 실패하면 즉시 함수 탈출(return Err)
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

---

### 3. `Option<T>`: Null 안전성의 완성
C#의 `null`은 값이 있을 수도 없을 수도 있음을 암시하지만, 체크를 강제하지는 않습니다. Rust의 `Option<T>`는 이를 타입 시스템으로 끌어올렸습니다.

```rust
fn find_user(id: u32) -> Option<User> {
    // 찾으면 Some(user), 못 찾으면 None 반환
}

// [사용 예시]
if let Some(user) = find_user(1) {
    println!("찾은 사용자: {}", user.name);
} else {
    println!("사용자를 찾을 수 없습니다.");
}
```

---

### 4. 패닉(Panic): 복구 불가능한 에러
모든 에러를 `Result`로 처리할 필요는 없습니다. 배열 인덱스 초과나 시스템 자원 고갈처럼 **프로그램이 더 이상 진행될 수 없는 상황**에는 `panic!()` 매크로를 사용하여 안전하게 프로그램을 종료합니다.

---

### 💡 실무 팁: `anyhow`와 `thiserror`
- **`thiserror`**: 라이브러리를 만들 때, 명확하고 구조화된 에러 타입을 정의하기 위해 사용합니다.
- **`anyhow`**: 애플리케이션(main 등)을 만들 때, 여러 종류의 에러를 하나로 묶어(`Result<T, anyhow::Error>`) 쉽고 빠르게 처리하기 위해 사용합니다.

