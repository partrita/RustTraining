# From과 Into 트레이트: 타입 변환의 정석

> **학습 목표:** 파이썬의 생성자 기반 타입 변환(`int()`, `str()`)을 대체하는 Rust의 **`From`**과 **`Into`** 트레이트를 배웁니다. 실패할 수 있는 변환을 처리하는 **`TryFrom`**의 활용법과, 관용적인 타입 변환 설계 방식을 익힙니다.

---

### 1. From과 Into: 안전하고 효율적인 변환
파이썬에서는 `Celsius(fahrenheit_value)` 처럼 생성자 내에서 변환 로직을 처리하는 경우가 많지만, Rust는 트레이트를 통해 이를 표준화합니다.

- **`From<T>`**: T 타입으로부터 현재 타입을 만드는 방법을 정의합니다.
- **`Into<T>`**: 현재 타입을 T 타입으로 변환하는 방법을 정의합니다. (보통 `From`을 구현하면 자동으로 생성됩니다.)

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

// 사용 예시
let c = Celsius::from(Fahrenheit(212.0)); // From 방식
let c: Celsius = Fahrenheit(212.0).into(); // Into 방식
```

---

### 2. TryFrom: 실패할 수 있는 변환
파이썬에서 `int("abc")`를 실행하면 런타임에 에러가 나지만, Rust는 `TryFrom`을 통해 변환 실패 가능성을 컴파일 타임에 강제합니다.

```rust
// 문자열을 정수로 파싱 (실패 가능하므로 Result 반환)
let n: Result<i32, _> = "42".parse(); 

// 커스텀 타입 검증
impl TryFrom<u32> for Port {
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > 65535 {
            Err("포트 번호 범위를 초과함".to_string())
        } else {
            Ok(Port(value as u16))
        }
    }
}
```

---

### 3. 문자열 변환 패턴 (String Conversions)
파이썬 개발자가 가장 자주 사용하게 될 변환들입니다.

| **용도** | **Python** | **Rust** | **비고** |
| :--- | :--- | :--- | :--- |
| **객체 → 문자열** | `str(x)` | **`x.to_string()`** | `Display` 구현 필요 |
| **문자열 → 숫자** | `int("42")` | **`"42".parse::<i32>()`** | `Result` 반환 |
| **&str → String** | - | **`String::from(s)`** / **`.to_owned()`** | 힙 메모리 할당 발생 |
| **String → &str** | - | **`&s`** | 메모리 비용 없음 (참조) |

---

### 4. 핵심 규칙: From을 구현하라
항상 `From`을 구현하세요. 그러면 `Into`는 컴파일러가 알아서 구현해 줍니다. 반대로 `Into`를 직접 구현하는 것은 권장되지 않습니다.

---

### 💡 실무 팁: `impl Into<T>`를 활용한 유연한 함수 설계
함수의 매개변수 타입을 `impl Into<String>`으로 설정하면, 호출하는 쪽에서 `&str`을 넣든 `String`을 넣든 컴파일러가 알아서 알맞은 형태로 변환해 줍니다. 라이브러리 설계 시 매우 강력한 도구가 됩니다.

