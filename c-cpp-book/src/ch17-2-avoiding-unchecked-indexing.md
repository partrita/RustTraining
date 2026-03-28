# 17-2. 코드 안전: 검사되지 않은 인덱싱(`[]`) 피하기 🟡

> **학습 목표:**
> - Rust에서 `vec[i]`가 왜 잠재적 위험 요소인지 이해하고, **`.get()`**, **반복자**, **`entry()`** API 등 더 안전한 대안들을 배웁니다.
> - C++의 정의되지 않은 동작(Undefined Behavior)을 명시적인 에러 처리 방식으로 대체하여 견고한 프로그램을 만드는 방법을 익힙니다.
> - JSON 데이터를 안전하게 파싱하는 **`serde`**의 강력한 기능을 살펴봅니다.

---

### 1. 왜 `[]` 연산자가 위험한가?
C++에서 `vec[i]`는 범위를 벗어나면 메모리 오염이나 예측 불가능한 크래시를 일으키지만, Rust의 `[]`는 즉시 **패닉(Panic)**을 발생시켜 프로그램을 종료시킵니다. 인덱스가 반드시 유효하다는 것을 100% 확신할 수 있는 상황이 아니라면, 안전한 접근 메서드를 사용해야 합니다.

```rust
let v = vec![1, 2, 3];

// [위험] 인덱스 10이 없으므로 프로그램이 패닉으로 종료됨
// let x = v[10]; 

// [안전] Option<&i32>를 반환하여 프로그램 흐름을 제어함
match v.get(10) {
    Some(val) => println!("값: {}", val),
    None => println!("해당 인덱스에 값이 없습니다."),
}

// 한 줄로 안전하게 처리하기
let val = v.get(10).unwrap_or(&0); // 값이 없으면 기본값 0 참조
```

---

### 2. `Option`과 `Result`를 다루는 우아한 방법
값을 꺼내기 위해 매번 `match`를 쓰는 대신, 함수형 어댑터를 사용하면 코드가 훨씬 간결해집니다.

| **메서드** | **용도** | **C++ 대응 패턴** |
| :--- | :--- | :--- |
| **`.map()`** | 값이 있을 때만 특정 변환 수행 | `if (opt) { transform(*opt); }` |
| **`.and_then()`** | 중첩된 Option을 평탄하게 연결 | `if (a) { if (a->get_b()) { ... } }` |
| **`.unwrap_or()`** | 기본값 제공 | `opt.value_or(default)` |
| **`.ok()?`** | 에러를 무시하고 Option으로 변환 | 에러 체크 후 `nullopt` 반환 |

```rust
// [실전 예시] 중첩된 구성 데이터 안전하게 가져오기
let vendor = device_info
    .get("Hardware")            // Option<&Value>
    .and_then(|v| v.get("Vendor")) // Option<&Value>
    .and_then(|v| v.as_str())      // Option<&str>
    .unwrap_or("Unknown Vendor");  // 최종 결과 또는 기본값
```

---

### 3. JSON 처리: `nlohmann`에서 `serde`로
C++에서 `nlohmann::json`을 써서 일일이 키를 검사하던 번거로움은 이제 끝났습니다. Rust의 **`serde`**는 타입 시스템을 통해 JSON을 구조체에 자동으로 안전하게 매핑합니다.

```rust
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct FanConfig {
    pub name: String,
    #[serde(default)] // 키가 없으면 해당 타입의 기본값(0) 사용
    pub max_rpm: u32,
    #[serde(rename = "sensor_id")] // JSON의 sensor_id 필드를 id 변수에 매핑
    pub id: u8,
}

// 단 한 줄로 안전하게 파싱 (실패 시 Result 반환)
let config: FanConfig = serde_json::from_str(json_data)?;
```
- **`#[serde(default)]`**: 필드가 누락되어도 크래시 대신 `Default` 값을 채워줍니다.
- **`#[serde(tag = "type")]`**: JSON의 특정 필드 값에 따라 서로 다른 열거형(`enum`) 변형으로 자동 역직렬화합니다.

---

### 📌 결론: 방어적 프로그래밍의 정석
"값이 반드시 존재한다"는 가설은 프로그래밍에서 가장 위험한 믿음 중 하나입니다. Rust의 **`.get()`**과 **`serde`**를 적극 활용하여, 예외 상황에서도 죽지 않고 유연하게 동작하는 견고한 프로그램을 설계하세요.

---

### 💡 요약
- 인덱스 접근 시에는 **`.get()`**을 기본으로 사용하세요.
- **`.and_then()`** 체이닝으로 중첩된 데이터를 깔끔하게 파헤치세요.
- JSON 파싱은 **`serde`**를 통해 타입 안전하게 처리하세요.
- 패닉은 발생시키기보다 **`Option/Result`**로 전환하여 제어하는 것이 좋습니다.

