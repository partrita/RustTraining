# Unsafe Rust와 FFI: 파이썬과의 공생

> **학습 목표:** Rust의 안전 장치를 잠시 해제하는 **`unsafe`**의 개념을 이해하고, 이를 활용해 파이썬의 성능 병목 지점을 Rust로 대체하는 **PyO3** 라이브러리 사용법을 익힙니다. 또한 런타임 테스트를 위한 테스트 프레임워크 활용법을 배웁니다.

---

### 1. Unsafe Rust: 안전 장치 풀기
`unsafe`는 컴파일러가 검증할 수 없는 동작을 개발자가 책임지고 수행하겠다고 선언하는 도구입니다. 주로 하드웨어 제어나 파이썬과 같은 타 언어와의 통신(FFI) 시에 사용됩니다.

```rust
// C 언어의 abs 함수 호출 예시
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    // C 함수는 Rust 컴파일러가 안전성을 보장할 수 없으므로 unsafe 블록이 필요합니다.
    let result = unsafe { abs(-42) };
    println!("{result}"); // 42
}
```

---

### 2. PyO3: 파이썬 개발자를 위한 무기
PyO3를 사용하면 Rust 코드를 파이썬 모듈처럼 만들어서 쓸 수 있습니다. 이는 느린 파이썬 로직을 Rust로 재작성할 때 가장 강력한 방법입니다.

```rust
// Rust로 작성된 파이썬 확장 모듈 예시
use pyo3::prelude::*;

#[pyfunction]
fn rust_fibonacci(n: u32) -> u32 {
    // Rust의 빠른 연산 로직...
    n
}

#[pymodule]
fn my_extension(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_fibonacci, m)?)?;
    Ok(())
}
```

---

### 3. 테스트와 벤치마킹
파이썬의 `pytest`에 대응하는 Rust의 내장 테스트 시스템을 사용합니다.

- **인라인 테스트**: 코드 파일 안에 직접 테스트 코드를 작성합니다.
- **`cargo test`**: 모든 테스트를 한 번에 실행합니다.
- **`mockall`**: 인터페이스를 가짜로 만들어 테스트하는 모킹 기반 라이브러리.

| **기능** | **Python (pytest)** | **Rust (내장/도구)** | **비고** |
| :--- | :--- | :--- | :--- |
| **단언문** | `assert x == y` | **`assert_eq!(x, y)`** | |
| **에러 기대** | `with pytest.raises(E):` | **`#[should_panic]`** | 에러 발생을 테스트 |
| **모킹** | `unittest.mock` | **`mockall`** | 트레이트를 기반으로 모킹 |
| **벤치마크** | `timeit` | **`criterion`** | 정밀한 성능 측정 |

---

### 4. 핵심 전략: 외부로 노출되는 API는 항상 'Safe'하게
내부적으로 `unsafe`를 썼더라도, 외부 사용자에게는 불완전한 상태로 노출되지 않도록 안전한 인터페이스로 감싸(Wrapping)는 것이 Rust의 공식적인 권장 패턴입니다.

---

### 💡 실무 팁: `maturin`을 기억하세요
PyO3로 만든 프로젝트를 빌드하고 설치할 때는 `maturin`이라는 도구를 사용합니다. `maturin develop` 한 번이면 Rust 코드가 컴파일되어 현재 파이썬 가상 환경에 패키지로 즉시 설치됩니다. 개발 흐름이 매우 매끄러워집니다.

