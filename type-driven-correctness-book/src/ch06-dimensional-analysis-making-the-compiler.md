# 6. 차원 분석 — 컴파일러가 단위를 검사하게 만들기 🟢

> **학습 목표:** 뉴타입(Newtype) 래퍼와 `uom` 크레이트가 어떻게 컴파일러를 단위 검사 엔진으로 변모시키는지 배웁니다. 이를 통해 수억 달러 규모의 우주선을 파괴했던 것과 같은 범주의 버그를 사전에 차단하는 법을 익힙니다.
>
> **관련 장:** [02장](ch02-typed-command-interfaces-request-determi.md) (이 장의 타입을 사용하는 명령 인터페이스), [07장](ch07-validated-boundaries-parse-dont-validate.md) (유효성 검증 경계), [10장](ch10-putting-it-all-together-a-complete-diagn.md) (통합)

---

### 화성 기후 궤도선(Mars Climate Orbiter)의 교훈

1999년, NASA의 화성 기후 궤도선은 한 팀이 추력 데이터를 **파운드-힘 초(lb·s)** 단위로 보냈는데, 제어 팀은 이를 **뉴턴-초(N·s)** 단위로 받아들이는 바람에 손실되었습니다. 우주선은 예정된 226km가 아닌 57km 고도에서 화성 대기권에 진입하여 공중 분해되었습니다.

이 버그의 근본 원인은 **두 값 모두 `double` 타입이었다**는 데 있습니다. 컴파일러는 두 단위를 구분할 수 없었습니다. 물리량을 다루는 모든 하드웨어 진단 도구에도 동일한 범주의 버그가 숨어 있습니다.

```c
// C — 모두 double 타입이며 단위 검사가 없음
double read_temperature(int sensor_id);   // 섭씨? 화씨? 켈빈?
double read_voltage(int channel);         // 볼트? 밀리볼트?
```

---

### 물리량을 위한 뉴타입(Newtypes)

가장 간단하고 확실한 '설계에 의한 올바름' 접근법은 **각 단위를 고유한 타입으로 감싸는 것**입니다.

```rust,ignore
/// 섭씨 온도
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(pub f64);

/// 화씨 온도
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Fahrenheit(pub f64);

/// 전압 (볼트)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Volts(pub f64);

// 명시적인 변환 구현:
impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self { Fahrenheit(c.0 * 9.0 / 5.0 + 32.0) }
}
```

이제 컴파일러가 단위 불일치를 잡아냅니다.

```rust,ignore
fn check_limit(temp: Celsius, limit: Celsius) -> bool {
    temp > limit  // ✅ 동일한 단위 — 컴파일 성공
}

// fn bad_check(temp: Celsius, voltage: Volts) -> bool {
//     temp > voltage  // ❌ 에러: 타입 불일치 (Celsius vs Volts)
// }
```

**런타임 비용은 제로**입니다. 뉴타입은 컴파일된 바이너리에서 원시 `f64` 값으로 존재합니다. 래퍼는 오직 타입 시스템상의 개념일 뿐입니다.

---

### 센서 파이프라인 적용 예시

원시 ADC 값을 읽어 물리 단위로 변환하고 임계값과 비교하는 전 과정을 타입 수준에서 검사할 수 있습니다.

```rust,ignore
/// ADC 읽기 — 아직 물리량이 아님
pub struct AdcReading { pub channel: u8, pub raw: u16 }

pub struct TempCal { pub scale: f64, pub offset: f64 }

impl TempCal {
    /// 원시 ADC 값을 Celsius로 변환. 반환 타입이 Celsius임을 보장함.
    pub fn convert(&self, adc: AdcReading) -> Celsius {
        Celsius(adc.raw as f64 * self.scale + self.offset)
    }
}

/// 임계값 검사 — 단위가 일치할 때만 컴파일됨
pub struct Threshold<T: PartialOrd> { pub warning: T, pub critical: T }

impl<T: PartialOrd> Threshold<T> {
    pub fn check(&self, value: &T) -> bool { *value >= self.critical }
}
```

이제 `Threshold<Celsius>`에 `Volts` 값을 전달하려고 하면 컴파일러가 즉시 오류를 발생시킵니다. 섭씨와 볼트를 비교하는 논리적 실수는 더 이상 발생할 수 없습니다.

---

### uom 크레이트 활용

실제 운영 환경에서는 [`uom`](https://crates.io/crates/uom) 크레이트를 사용하는 것이 좋습니다. 수백 개의 단위를 지원하며, 자동 단위 변환 및 유도 단위(예: Watt = Volt × Ampere) 계산을 **런타임 오버헤드 없이** 제공합니다.

```rust,ignore
// uom 사용 예시 (의사 코드)
let temp = Celsius::new(85.0);
let volt = Volts::new(12.0);
// temp + volt; // ❌ 컴파일 에러 — 온드와 전압은 더할 수 없음
```

---

### 핵심 요약

1. **뉴타입은 비용 없이 혼동을 방지함** — `Celsius`와 `Rpm`은 내부적으로 같은 `f64`지만, 컴파일러는 이를 완전히 다른 타입으로 취급합니다.
2. **우주선 손실 버그의 원천 차단** — `Pounds`를 기대하는 곳에 `Newtons`를 전달하는 실수는 이제 컴파일 에러입니다.
3. **제네릭 임계값 검사** — `Threshold<T>` 패턴을 통해 모든 물리량에 대해 재사용 가능하면서도 타입 안전한 검사 로직을 구현할 수 있습니다.
4. **uom 크레이트** — 복잡한 유도 단위와 산술 연산이 필요할 때 매우 강력한 도구입니다.

