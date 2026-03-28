# 7. 유효성 검증 경계 — 검증하지 말고 파싱하라 🟡

> **학습 목표:** 시스템 경계에서 데이터를 단 한 번만 검증하고, 그 유효성 증명을 전용 타입에 담아 두 번 다시 재검증하지 않는 기법을 배웁니다. IPMI FRU 데이터(원시 바이트), Redfish JSON(구조화된 문서), 그리고 IPMI SEL 레코드(중첩된 다형성 데이터)를 예로 들어 실습합니다.
>
> **관련 장:** [02장](ch02-typed-command-interfaces-request-determi.md) (타입 지정 명령), [06장](ch06-dimensional-analysis-making-the-compiler.md) (차원 타입), [11장](ch11-fourteen-tricks-from-the-trenches.md) (봉인된 트레이트, FromStr), [14장](ch14-testing-type-level-guarantees.md) (속성 기반 테스트)

---

### 문제 요망: 사방에 흩어진 검증 (Shotgun Validation)

일반적인 코드에서는 검증 로직이 도처에 흩어져 있습니다. 데이터를 받는 모든 함수가 "혹시 모르니" 재검증을 수행합니다.

```c
// C — 코드 전체에 흩어진 검증 로직
int process_fru_data(uint8_t *data, int len) {
    if (data == NULL) return -1;          // 널 체크
    if (len < 8) return -1;              // 최소 길이 체크
    if (data[0] != 0x01) return -1;      // 포맷 버전 체크
    if (checksum(data, len) != 0) return -1; // 체크섬 확인
    // ... 이후 호출되는 함수들에서 똑같은 체크를 반복 ...
}
```

이 방식의 문제점:
1. **중복** — 동일한 검사 코드가 수십 군데 나타납니다.
2. **불완전성** — 실수로 단 한 군데라도 검증을 빼먹으면 보안 취약점이나 버그가 됩니다.

---

### 검증하지 말고 파싱하라 (Parse, Don't Validate)

올바른 접근법은 **경계에서 단 한 번만 검증하고, 그 성공 증명을 타입에 담는 것**입니다.

```rust,ignore
/// 아직 검증되지 않은 원시 바이트 데이터
pub struct RawFruData(Vec<u8>);

/// 유효성이 검증된 IPMI FRU 데이터. 
/// TryFrom을 통해서만 생성될 수 있으며, 일단 생성되면 모든 데이터가 올바름을 보장함.
pub struct ValidFru {
    format_version: u8,
    board_area_offset: u8,
    data: Vec<u8>,
}

impl TryFrom<RawFruData> for ValidFru {
    type Error = String;
    fn try_from(raw: RawFruData) -> Result<Self, String> {
        let data = raw.0;
        if data.len() < 8 { return Err("너무 짧음".into()); }
        if data[0] != 0x01 { return Err("버전 불일치".into()); }
        // ... 모든 유효성 검사 수행 ...
        Ok(ValidFru { format_version: data[0], board_area_offset: data[3], data })
    }
}
```

이제 `&ValidFru`를 인자로 받는 모든 함수는 데이터가 올바르다는 것을 **알고 있습니다**. 더 이상의 널 체크나 인덱스 범위 체크는 불필요합니다.

---

### 다형성 데이터 검증: IPMI SEL 레코드

IPMI 시스템 이벤트 로그(SEL)는 16바이트의 고정 길이를 갖지만, 그 내용의 해석은 이전 바이트에 따라 달라지는 중첩된 구조를 가지고 있습니다.

**단계 1: 외부 프레임 파싱**  
첫 번째 `TryFrom`은 레코드 타입(시스템 이벤트, OEM 타임스탬프 등)에 따라 적절한 열거형 변형으로 분기합니다.

```rust,ignore
pub enum ValidSelRecord {
    SystemEvent(SystemEventRecord),
    OemTimestamped(OemTimestampedRecord),
    OemNonTimestamped(OemNonTimestampedRecord),
}
```

**단계 2: 시스템 이벤트 파싱 (센서 타입별 분기)**  
이후 내부 파싱 로직은 센서 타입(온도, 전압, 메모리 등)에 따라 데이터를 타이핑된 구조체로 변환합니다.

```rust,ignore
pub enum TypedEvent {
    Threshold(ThresholdEvent),
    SensorSpecific(SensorSpecificEvent), // 메모리 에러, 팬 고장 등 센서별 특화 데이터
}
```

**단계 3: 타이핑된 데이터 소비**  
파싱이 완료되면 하위 로직에서는 중첩된 열거형을 패턴 매칭하여 처리합니다. 컴파일러는 모든 케이스(OEM 레코드 포함)가 누락 없이 처리되도록 강제합니다.

---

### 핵심 요약

1. **검증은 경계에서 한 번만** — 데이터를 신뢰할 수 없는 외부(네트워크, 파일, 하드웨어)에서 앱 내부로 들여오기 직전에 수행합니다.
2. **타입이 증거다** — 어떤 함수가 `ValidFru` 타입을 인자로 받는다면, 컴파일러가 그 함수의 호출 전에 검증이 이미 완료되었음을 보증하는 것입니다.
3. **불변성(Invariants) 유지** — 한 번 파싱된 데이터는 수정 불가능하게(Immutable) 처리하여 유효성 증명이 훼손되지 않도록 합니다.
4. **다형성 처리** — 복잡한 유니온(Union) 구조는 중첩된 열거형으로 파싱하여, 런타임의 불안정한 포인터 캐스팅을 컴파일 타임의 타입 안전한 패턴 매칭으로 바꿉니다.

