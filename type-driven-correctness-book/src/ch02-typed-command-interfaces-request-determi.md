# 2. 타입이 지정된 명령 인터페이스 — 요청이 응답을 결정함 🟡

> **학습 목표:** 명령 트레이트의 연관 타입이 어떻게 요청과 응답 사이에 컴파일 타임 결합(binding)을 생성하는지 배웁니다. 이를 통해 IPMI, Redfish, NVMe 프로토콜 전반에서 발생하는 파싱 불일치, 단위 혼동, 암시적 타입 변환 등의 문제를 제거하는 방법을 익힙니다.
>
> **관련 장:** [01장](ch01-the-philosophy-why-types-beat-tests.md) (철학), [06장](ch06-dimensional-analysis-making-the-compiler.md) (차원 타입), [07장](ch07-validated-boundaries-parse-dont-validate.md) (유효성 검증 경계), [10장](ch10-putting-it-all-together-a-complete-diagn.md) (통합)

---

### 타입이 없는 늪 (The Untyped Swamp)

IPMI, Redfish, NVMe Admin, PLDM과 같은 대부분의 하드웨어 관리 스택은 `원시 바이트 입력 → 원시 바이트 출력` 구조로 시작됩니다. 이는 테스트로도 일부만 잡아낼 수 있는 종류의 버그를 만들어냅니다.

```rust,ignore
use std::io;

struct BmcRaw { /* ipmitool 핸들 */ }

impl BmcRaw {
    fn raw_command(&self, net_fn: u8, cmd: u8, data: &[u8]) -> io::Result<Vec<u8>> {
        // ... 실제로는 ipmitool 등을 호출 ...
        Ok(vec![0x00, 0x19, 0x00]) // 임시 데이터
    }
}

fn diagnose_thermal(bmc: &BmcRaw) -> io::Result<()> {
    let raw = bmc.raw_command(0x04, 0x2D, &[0x20])?;
    let cpu_temp = raw[0] as f64;        // 🤞 0번 바이트가 온도 값이 맞겠지?

    let raw = bmc.raw_command(0x04, 0x2D, &[0x30])?;
    let fan_rpm = raw[0] as u32;         // 🐛 버그: 팬 속도는 2바이트 리틀 엔디언임

    let raw = bmc.raw_command(0x04, 0x2D, &[0x40])?;
    let voltage = raw[0] as f64;         // 🐛 버그: 1000으로 나눠야 함

    if cpu_temp > fan_rpm as f64 {       // 🐛 버그: 섭씨(°C)와 RPM을 비교하고 있음
        println!("문제 발생");
    }

    log_temp(voltage);                   // 🐛 버그: 전압(V)을 온도 로그 함수에 전달
    Ok(())
}

fn log_temp(t: f64) { println!("온도: {t}°C"); }
```

| # | 버그 내용 | 발견 시점 |
|---|-----|------------|
| 1 | 팬 RPM을 2바이트가 아닌 1바이트로 파싱 | 운영 환경에서 새벽 3시 |
| 2 | 전압 수치 보정(Scaling) 누락 | 모든 PSU가 과전압으로 오진됨 |
| 3 | 섭씨(°C)와 RPM을 직접 비교 | 어쩌면 영원히 발견 못 함 |
| 4 | 전압 값이 온도 로거로 전달됨 | 6개월 뒤 과거 데이터를 분석할 때 |

**근본 원인:** 모든 데이터가 `Vec<u8>` → `f64`로 취급되며, 개발자의 "기도"에 의존하고 있습니다.

---

### 타이핑된 명령(Typed Command) 패턴

#### 1단계 — 도메인 뉴타입(Newtype) 정의

```rust,ignore
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rpm(pub u32);  // u32: 원시 IPMI 센서 값 (정수 RPM)

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Volts(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Watts(pub f64);
```

> **`Rpm(u32)` vs `Rpm(f64)`에 대하여:** 이 장에서는 IPMI 센서 읽기 값이 정수이므로 `u32`를 사용합니다. 06장(차원 분석)에서는 산술 연산(평균, 스케일링)을 지원하기 위해 `f64`를 사용합니다. 두 방식 모두 유효하며, 뉴타입 패턴은 내부 타입이 무엇이든 단위 간의 혼동을 방지합니다.

#### 2단계 — 명령 트레이트 정의 (타입 인덱싱된 디스패치)

연관 타입인 `Response`가 핵심입니다. 이 타입은 각 명령 구조체와 그 반환 타입을 컴파일 타임에 묶어줍니다. 각 구현체는 `Response`를 특정 도메인 타입으로 고정하므로, `execute()`는 항상 정확한 타입을 반환하게 됩니다.

```rust,ignore
pub trait IpmiCmd {
    /// 타입 인덱스 — execute()가 무엇을 반환할지 결정합니다.
    type Response;

    fn net_fn(&self) -> u8;
    fn cmd_byte(&self) -> u8;
    fn payload(&self) -> Vec<u8>;

    /// 파싱 로직을 캡슐화 — 각 명령은 자신의 바이트 레이아웃을 알고 있습니다.
    fn parse_response(&self, raw: &[u8]) -> io::Result<Self::Response>;
}
```

#### 3단계 — 명령별 구조체 구현

```rust,ignore
pub struct ReadTemp { pub sensor_id: u8 }
impl IpmiCmd for ReadTemp {
    type Response = Celsius;
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.sensor_id] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Celsius> {
        if raw.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "응답이 비어 있음"));
        }
        // 참고: 01장의 예제는 SDR 메타데이터 없이 동작을 보여주기 위해 
        // `raw[0] as i8 as f64`를 사용했습니다. 여기서는 IPMI 사양 §35.5의 
        // 공식에 따라 처리합니다. 실무에서는 전체 SDR 공식을 적용하세요: 
        // 결과 = (M × raw + B) × 10^(R_exp).
        Ok(Celsius(raw[0] as f64)) 
    }
}

pub struct ReadFanSpeed { pub fan_id: u8 }
impl IpmiCmd for ReadFanSpeed {
    type Response = Rpm;
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.fan_id] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Rpm> {
        if raw.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData,
                format!("팬 속도는 2바이트가 필요하지만 {}바이트 수신", raw.len())));
        }
        Ok(Rpm(u16::from_le_bytes([raw[0], raw[1]]) as u32))
    }
}

pub struct ReadVoltage { pub rail: u8 }
impl IpmiCmd for ReadVoltage {
    type Response = Volts;
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.rail] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Volts> {
        if raw.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData,
                format!("전압은 2바이트가 필요하지만 {}바이트 수신", raw.len())));
        }
        Ok(Volts(u16::from_le_bytes([raw[0], raw[1]]) as f64 / 1000.0))
    }
}
```

#### 4단계 — 실행기 (제로 비용 단형성화)

```rust,ignore
pub struct BmcConnection { pub timeout_secs: u32 }

impl BmcConnection {
    pub fn execute<C: IpmiCmd>(&self, cmd: &C) -> io::Result<C::Response> {
        let raw = self.raw_send(cmd.net_fn(), cmd.cmd_byte(), &cmd.payload())?;
        cmd.parse_response(&raw)
    }

    fn raw_send(&self, _nf: u8, _cmd: u8, _data: &[u8]) -> io::Result<Vec<u8>> {
        Ok(vec![0x19, 0x00]) // 임시 구현
    }
}
```

#### 5단계 — 네 가지 버그가 모두 컴파일 에러가 됨

```rust,ignore
fn diagnose_thermal_typed(bmc: &BmcConnection) -> io::Result<()> {
    let cpu_temp: Celsius = bmc.execute(&ReadTemp { sensor_id: 0x20 })?;
    let fan_rpm:  Rpm     = bmc.execute(&ReadFanSpeed { fan_id: 0x30 })?;
    let voltage:  Volts   = bmc.execute(&ReadVoltage { rail: 0x40 })?;

    // 버그 #1 — 발생 불가능: 파싱 로직이 ReadFanSpeed::parse_response 안에 캡슐화됨
    // 버그 #2 — 발생 불가능: 단위 보정이 ReadVoltage::parse_response 안에 캡슐화됨

    // 버그 #3 — 컴파일 에러:
    // if cpu_temp > fan_rpm { }
    //    ^^^^^^^^   ^^^^^^^ Celsius vs Rpm → "타입 불일치(mismatched types)" ❌

    // 버그 #4 — 컴파일 에러:
    // log_temperature(voltage);
    //                 ^^^^^^^ Volts 타입은 Celsius 타입을 기대하는 곳에 전달 불가 ❌

    if cpu_temp > Celsius(85.0) { println!("CPU 과열: {:?}", cpu_temp); }
    if fan_rpm < Rpm(4000)      { println!("팬 속도 낮음: {:?}", fan_rpm); }

    Ok(())
}

fn log_temperature(t: Celsius) { println!("온도: {:?}", t); }
fn log_voltage(v: Volts)       { println!("전압: {:?}", v); }
```

---

### IPMI: 혼동할 수 없는 센서 데이터 읽기

새로운 센서를 추가하는 작업은 구조체 하나와 impl 하나로 끝납니다. 파싱 코드가 여기저기 흩어지지 않습니다.

```rust,ignore
pub struct ReadPowerDraw { pub domain: u8 }
impl IpmiCmd for ReadPowerDraw {
    type Response = Watts;
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.domain] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Watts> {
        if raw.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData,
                format!("전력 소비량은 2바이트가 필요하지만 {}바이트 수신", raw.len())));
        }
        Ok(Watts(u16::from_le_bytes([raw[0], raw[1]]) as f64))
    }
}

// bmc.execute(&ReadPowerDraw { domain: 0 })를 호출하는 모든 곳에서 
// 자동으로 Watts 타입을 반환받습니다. 다른 곳에 파싱 코드를 둘 필요가 없습니다.
```

---

### Redfish: 스키마 기반 REST 엔드포인트

Redfish는 더 잘 어울립니다. 각 엔드포인트는 DMTF에서 정의한 특정 JSON 스키마를 반환하기 때문입니다.

```rust,ignore
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ThermalResponse {
    #[serde(rename = "Temperatures")]
    pub temperatures: Vec<RedfishTemp>,
    #[serde(rename = "Fans")]
    pub fans: Vec<RedfishFan>,
}

#[derive(Debug, Deserialize)]
pub struct RedfishTemp {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ReadingCelsius")]
    pub reading: f64,
    #[serde(rename = "UpperThresholdCritical")]
    pub critical_hi: Option<f64>,
    #[serde(rename = "Status")]
    pub status: RedfishHealth,
}

#[derive(Debug, Deserialize)]
pub struct RedfishFan {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Reading")]
    pub rpm: u32,
    #[serde(rename = "Status")]
    pub status: RedfishHealth,
}

// (중략: PowerResponse, ProcessorResponse 등도 유사한 방식으로 정의)

/// 타이핑된 Redfish 엔드포인트 — 각 엔드포인트는 자신의 응답 타입을 알고 있습니다.
pub trait RedfishEndpoint {
    type Response: serde::de::DeserializeOwned;
    fn method(&self) -> &'static str;
    fn path(&self) -> String;
}

pub struct GetThermal { pub chassis_id: String }
impl RedfishEndpoint for GetThermal {
    type Response = ThermalResponse;
    fn method(&self) -> &'static str { "GET" }
    fn path(&self) -> String {
        format!("/redfish/v1/Chassis/{}/Thermal", self.chassis_id)
    }
}

// ... execute() 구현 및 사용 예시 ...
```

---

### NVMe Admin: Identify와 Log Page의 구분

NVMe 관리 명령도 같은 형태를 따릅니다. 컨트롤러는 명령 코드(Opcode)로 이를 구분하지만, C 언어에서는 호출자가 4KB 완료 버퍼에 어떤 구조체를 씌워야 할지 직접 알고 있어야 합니다. 타입 지정 명령 패턴은 이 과정에서 실수가 발생하는 것을 원천 차단합니다.

```rust,ignore
pub trait NvmeAdminCmd {
    type Response;
    fn opcode(&self) -> u8;
    fn parse_completion(&self, data: &[u8]) -> io::Result<Self::Response>;
}

// Identify 명령 (Opcode 0x06)은 IdentifyResponse를 반환하도록 타입을 강제함
```

---

### 확장: 명령 스크립트를 위한 매크로 DSL

```rust,ignore
/// 일련의 타이핑된 IPMI 명령을 실행하고 결과 튜플을 반환합니다.
macro_rules! diag_script {
    ($bmc:expr; $($cmd:expr),+ $(,)?) => {{
        ( $( $bmc.execute(&$cmd)?, )+ )
    }};
}

fn full_pre_flight(bmc: &BmcConnection) -> io::Result<()> {
    let (temp, rpm, volts) = diag_script!(bmc;
        ReadTemp     { sensor_id: 0x20 },
        ReadFanSpeed { fan_id:    0x30 },
        ReadVoltage  { rail:      0x40 },
    );
    // 반환 타입: (Celsius, Rpm, Volts) — 모두 자동 추론됨
    Ok(())
}
```

---

### 요약

1. **연관 타입 = 컴파일 타임 계약** — 명령 트레이트의 `type Response`는 각 요청을 정확히 하나의 응답 타입과 연결합니다.
2. **파싱 캡슐화** — 바이트 레이아웃 정보는 호출자가 아닌 `parse_response` 내부에만 존재합니다.
3. **제로 비용 디바이스** — 제네릭 `execute<C: IpmiCmd>`는 vtable 없이 직접 호출로 단형성화되어 실행됩니다.
4. **하나의 패턴, 다양한 프로토콜** — IPMI, Redfish, NVMe, PLDM, MCTP 등 모든 하드웨어 프로토콜에 동일한 `trait Cmd { type Response; }` 형태를 적용할 수 있습니다.
5. **단계적 복잡도가 직관을 강화함** — 단순한 센서 읽기(IPMI)에서 정교한 JSON(Redfish), 로우 버퍼 구조체 매핑(NVMe)으로 나아가는 과정이 모두 하나의 일관된 패턴으로 설명됩니다.

