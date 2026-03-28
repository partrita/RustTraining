# 11. 타입 변환의 정석: From과 Into 🟡

> **학습 목표:**
> - Rust에서 안전하고 관용적인(Idiomatic) 타입 변환 도구인 **`From`**, **`Into`**, **`Default`** 트레이트를 배웁니다.
> - 절대로 실패하지 않는 변환과 실패 가능성이 있는 변환(**`TryFrom`**, **`TryInto`**)을 구분합니다.
> - C++의 암시적 변환(Implicit Conversion)이나 생성자가 유발하던 잠재적 버그를 Rust가 어떻게 원천 차단하는지 알아봅니다.

---

### 안전한 변환의 기초: `From`과 `Into`
`From`과 `Into`는 서로 대칭을 이루는 트레이트입니다. 가장 중요한 점은 **`From`을 구현하면 컴파일러가 `Into`를 자동으로 구현해 준다**는 사실입니다. (거꾸로는 안 됩니다.)

- **`From`**: "A로부터 B를 만든다"는 관점 (`Point::from(data)`)
- **`Into`**: "A를 B로 바꾼다"는 관점 (`data.into()`)

```rust
struct Point { x: u32, y: u32 }

// 튜플 (u32, u32)로부터 Point를 만드는 방법 정의
impl From<(u32, u32)> for Point {
    fn from(tuple: (u32, u32)) -> Self {
        Point { x: tuple.0, y: tuple.1 }
    }
}

fn main() {
    // 1. From 사용: 명시적이고 읽기 쉬움
    let p1 = Point::from((10, 20));
    
    // 2. Into 사용: 제네릭 인자나 타입 추론이 가능할 때 유용함
    let p2: Point = (30, 40).into(); 
    
    println!("p1: ({}, {}), p2: ({}, {})", p1.x, p1.y, p2.x, p2.y);
}
```

---

### 기본값 정의: `Default` 트레이트
C++의 기본 생성자와 유사한 역할을 합니다. 구조체의 모든 필드에 합리적인 초기값을 부여하고 싶을 때 사용합니다.

- **자동 생성**: `#[derive(Default)]`를 붙이면 모든 필드가 해당 타입의 기본값(0, false, 빈 문자열 등)으로 초기화됩니다.
- **구조체 업데이트 문법**: 특정 필드만 바꾸고 나머지는 기본값을 쓸 때 매우 강력합니다.

```rust
#[derive(Debug, Default)]
struct Config {
    port: u16,
    debug_mode: bool,
    log_level: String,
}

fn main() {
    // 포트만 9000으로 바꾸고 나머지는 기본값 사용
    let custom_config = Config {
        port: 9000,
        ..Config::default() 
    };
    
    println!("{custom_config:?}");
}
```

---

### 실패할 수 있는 변환: `TryFrom`과 `TryInto`
큰 숫자를 작은 타입으로 바꾸거나, 유효하지 않은 데이터를 변환할 때는 에러 처리가 필수입니다. Rust는 이를 위해 `Result`를 반환하는 `TryFrom` / `TryInto`를 제공합니다.

```rust
use std::convert::TryInto;

fn main() {
    let big_num: i64 = 1000;
    
    // i64를 u8로 변환 시도 (255를 넘어가면 에러 발생)
    let result: Result<u8, _> = big_num.try_into();
    
    match result {
        Ok(n) => println!("변환 성공: {n}"),
        Err(_) => println!("변환 실패: 숫자가 u8 범위를 벗어납니다!"),
    }
}
```

---

### 💡 실무 팁: C++ 개발자를 위한 요약
- **암시적 변환 금지**: Rust는 "대충 알아서 바꿔주겠지"라는 기대를 허용하지 않습니다. 모든 변환은 `into()`, `from()`, `as` 등을 통해 코드에 명확히 드러나야 합니다.
- **예측 가능성**: 함수가 `Into<T>`를 받는다면, 어떤 타입들을 넘길 수 있는지 `From` 구현체 목록만 보고 확실히 알 수 있습니다.
- **소유권과 결합**: 변환 과정에서 원본 데이터의 소유권이 자연스럽게 이동(`Move`)되므로, 변환 후에 실수로 옛 데이터를 다시 사용하는 버그를 방지합니다.

---

### 📌 요약
- 새로운 타입을 정의한다면 다른 타입과의 변환을 위해 **`From`**을 구현하세요.
- 기본값이 필요한 설정값 등에는 **`Default`**를 적극 활용하세요.
- 데이터 손실이나 오류가 발생할 수 있는 변환은 반드시 **`TryInto`**로 처리하세요.
- 숫자 캐스팅(`as`)은 데이터가 잘려 나갈 수 있으므로 가급적 `TryInto` 사용을 권장합니다.

