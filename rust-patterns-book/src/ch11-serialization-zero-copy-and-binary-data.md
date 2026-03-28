# 11. 직렬화, 제로 카피, 바이너리 데이터 🟡

> **학습 목표:**
> - `serde`의 핵심: 파생 매크로, 속성(Attribute), 열거형 표현 방식을 익힙니다.
> - 고성능 읽기 작업을 위한 **제로 카피(Zero-copy)** 역직렬화 기술을 배웁니다.
> - 다양한 직렬화 포맷(JSON, TOML, bincode, MessagePack)의 생태계를 이해합니다.
> - `repr(C)`, `zerocopy`, `bytes::Bytes`를 이용한 바이너리 데이터 처리 기법을 마스터합니다.

---

### Serde 기초: 직렬화와 역직렬화의 표준

`serde`는 Rust에서 데이터를 다양한 포맷으로 변환하는 유니버설 프레임워크입니다. **데이터 모델**(구조체)과 **포맷**(JSON, binary 등)을 완벽히 분리합니다.

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    port: u16,
    #[serde(default)] // 필드가 없으면 기본값(0) 사용
    max_conn: usize,
}
```
> **핵심 통찰**: 구조체에 `Serialize`와 `Deserialize`를 한 번만 구현해 두면, 코드 수정 없이 JSON, TOML, YAML, bincode 등 수십 가지 포맷과 호환됩니다.

---

### 일반적인 Serde 속성 (Attributes)

| 속성 | 레벨 | 효과 |
| :--- | :--- | :--- |
| `rename_all = "camelCase"` | 컨테이너 | 모든 필드를 camelCase 등으로 일괄 변환 |
| `deny_unknown_fields` | 컨테이너 | 알 수 없는 키가 있으면 에러 (엄격한 파싱) |
| `default` | 필드 | 값이 누락되었을 때 `Default` 트레이트 값 사용 |
| `rename = "..."` | 필드 | 특정 필드 이름만 변경하여 직렬화 |
| `skip` | 필드 | 직렬화/역직렬화 대상에서 완전히 제외 |
| `flatten` | 필드 | 중첩된 구조체의 필드를 평탄화하여 포함 |

---

### 제로 카피(Zero-copy) 역직렬화

새로운 메모리를 할당하지 않고, 입력 버퍼의 데이터를 직접 참조하는 고성능 기법입니다.

```rust
#[derive(Deserialize)]
struct BorrowedRecord<'a> {
    name: &'a str,  // 입력 문자열 버퍼를 직접 가리킴 (복사 제로)
    value: &'a str,
}
```
> **사용 시점**: 대용량 로그 파일 파싱처럼 속도가 중요하고 입력 버퍼가 메모리에 계속 유지될 때 사용하세요. 입력 버퍼가 일시적이라면 `String`을 사용하는 일반적인 역직렬화를 써야 합니다.

---

### 바이너리 데이터 처리와 repr(C)

하드웨어 레지스터나 네트워크 프로토콜 헤더를 정밀하게 다뤄야 할 때 사용합니다.

- **`#[repr(C)]`**: 필드 배치 순서를 C 언어 규칙과 일치시켜 예측 가능한 메모리 레이아웃을 보장합니다.
- **`zerocopy` / `bytemuck`**: `unsafe`한 `transmute` 대신, 컴파일 타임에 안전성이 검증된 제로 카피 변환을 제공합니다.
- **`bytes::Bytes`**: 참조 카운팅이 적용된 바이트 버퍼입니다. 대규모 이진 데이터를 여러 스레드나 컴포넌트 간에 복사 없이 공유할 때 필수적입니다.

---

### 📝 연습 문제: 커스텀 Serde 역직렬화 ★★★ (~45분)

`"30s"`, `"5m"`, `"2h"`와 같은 사람이 읽기 쉬운 문자열을 `std::time::Duration`으로 변환하는 커스텀 역직렬화기(Deserializer)를 구현해 보세요. 반대로 직렬화할 때도 원래의 형식으로 돌아가야 합니다.

---

### 📌 요약
- **`serde`** 속성을 잘 활용하면 외부 API의 복잡한 명세도 깔끔하게 매핑할 수 있습니다.
- 성능이 극한으로 중요하다면 **제로 카피**와 **`Cow`**를 조합하세요.
- 바이너리 통신에는 **`repr(C)`**와 **`zerocopy`** 크레이트를 사용하는 것이 안전합니다.
- 참조 카운팅 바이트 버퍼가 필요하다면 **`bytes`** 크레이트를 고려하세요.

