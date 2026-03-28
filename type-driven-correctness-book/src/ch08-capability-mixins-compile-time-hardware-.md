# 8. 의무 믹스인 — 컴파일 타임 하드웨어 계약 🟡

> **학습 목표:** 재료 트레이트(버스 역량)와 믹스인(Mixin) 트레이트, 담요 구현(Blanket impls)을 결합하여 진단 코드의 중복을 제거하고, 모든 하드웨어 의존성이 컴파일 타임에 충족됨을 보장하는 방법을 배웁니다.
>
> **관련 장:** [04장](ch04-capability-tokens-zero-cost-proof-of-aut.md) (역량 토큰), [09장](ch09-phantom-types-for-resource-tracking.md) (팬텀 타입), [10장](ch10-putting-it-all-together-a-complete-diagn.md) (통합)

---

### 문제 요망: 진단 코드의 중복

서버 플랫폼의 여러 하위 시스템(팬, 온도 센서, 전원 등)은 서로 다른 하드웨어 버스(SPI, I2C, GPIO)를 사용하지만 진단 로직은 매우 유사합니다. 추상화가 없으면 로직의 대부분을 복사-붙여넣기하게 됩니다.

```c
// C — 하 하위 시스템 간에 중복된 로직
int run_fan_diag(spi_bus_t *spi, i2c_bus_t *i2c) {
    // ... SPI 센서 읽기 ...
    // ... I2C 레지스터 확인 ...
    // ... 임계값 비교 ...
}

int run_cpu_diag(i2c_bus_t *i2c, gpio_t *gpio) {
    // ... I2C 레지스터 확인 (팬 진단과 동일) ...
    // ... GPIO 경고 확인 ...
    // ... 임계값 비교 (팬 진단과 동일) ...
}
```

---

### 재료 트레이트 (하드웨어 역량)

각 버스나 주변 장치를 트레이트로 정의합니다. 진단 컨트롤러는 자신이 어떤 버스를 가지고 있는지 선언합니다.

```rust,ignore
pub trait HasSpi {
    type Spi: SpiBus;
    fn spi(&self) -> &Self::Spi;
}

pub trait HasI2c {
    type I2c: I2cBus;
    fn i2c(&self) -> &Self::I2c;
}

// SPI, I2C 버스의 실제 동작 정의
pub trait SpiBus { fn transfer(&self, data: &[u8]) -> Vec<u8>; }
pub trait I2cBus { fn read_reg(&self, addr: u8, reg: u8) -> u8; }
```

---

### 믹스인 트레이트 (진단 동작 제공)

믹스인은 필요한 역량(재료)을 가진 모든 타입에 **자동으로** 기능을 제공합니다.

```rust,ignore
/// 팬 진단 믹스인 — SPI와 I2C를 가진 모든 타입에 대해 자동 구현됨
pub trait FanDiagMixin: HasSpi + HasI2c {
    fn run_fan_diagnostic(&self) -> bool {
        let speed = self.spi().transfer(&[0x80]); // SPI로 속도 읽기
        self.i2c().read_reg(0x2E, 0x01);          // I2C로 설정 읽기
        true
    }
}

// 담요 구현(Blanket Implementation) — 조건만 맞으면 공짜로 기능을 얻음
impl<T: HasSpi + HasI2c> FanDiagMixin for T {}
```

---

### 구체적인 컨트롤러: 필요한 기능만 골라 담기

실제 진단 컨트롤러는 자신이 가진 버스 역량만 선언하면, 해당하는 모든 믹스인을 **상속**받게 됩니다.

```rust,ignore
/// 메인보드 컨트롤러는 모든 버스를 가지고 있으므로 모든 믹스인을 자동으로 얻음
pub struct BaseBoardController {
    spi: LinuxSpi,
    i2c: LinuxI2c,
}

impl HasSpi for BaseBoardController {
    type Spi = LinuxSpi;
    fn spi(&self) -> &LinuxSpi { &self.spi }
}

impl HasI2c for BaseBoardController {
    type I2c = LinuxI2c;
    fn i2c(&self) -> &LinuxI2c { &self.i2c }
}

// 이제 BaseBoardController는 FanDiagMixin을 자동으로 구현하게 됨
```

---

### 설계에 의한 올바름 (Correct-by-Construction)

이 패턴이 왜 안전한가요?

1. **의존성 강제** — SPI 버스 없이 `run_fan_diagnostic()`을 호출하는 것은 불가능합니다.
2. **실수 방지** — 컨트롤러에서 `HasSpi` 구현을 제거하면, 이를 사용하는 모든 믹스인 메서드가 컴파일 타임에 사라집니다.
3. **쉬운 모의 테스트(Mocking)** — 실제 버스 대신 `MockSpi`를 사용하는 컨트롤러를 만들면, 진단 로직은 그대로 유지하면서 테스트할 수 있습니다.
4. **유연한 확장** — 새로운 하드웨어 플랫폼이 추가되어도 역량만 나열하면 기존 진단 로직을 즉시 사용할 수 있습니다.

---

### 핵심 요약

1. **재료 트레이트로 하드웨어 역량 선언** — `HasSpi`, `HasI2c` 등을 통해 컨트롤러가 무엇을 할 수 있는지 정의합니다.
2. **믹스인과 담요 구현으로 공통 로직 공유** — 코드 복사 없이 필요한 역량이 있는 곳에만 기능을 주입합니다.
3. **플랫폼 독립성** — 진단 로직은 실제 하드웨어 구현이 아닌 역량 트레이트에만 의존합니다.
4. **컴파일 타임 계약** — 모든 하드웨어 의존성이 컴파일 시점에 체크되므로, 배포된 코드에서 버스 누락으로 인한 런타임 에러가 발생하지 않습니다.

