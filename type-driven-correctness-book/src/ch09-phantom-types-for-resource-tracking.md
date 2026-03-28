# 9. 리소스 추적을 위한 팬텀 타입 🟡

> **학습 목표:** `PhantomData` 마커가 어떻게 레지스터 너비, DMA 방향, 파일 서술자 상태 등을 타입 수준에서 인코딩하는지 배웁니다. 이를 통해 리소스 불일치 버그 전반을 **런타임 비용 없이** 방지하는 법을 익힙니다.
>
> **관련 장:** [05장](ch05-protocol-state-machines-type-state-for-r.md) (타입 상태), [06장](ch06-dimensional-analysis-making-the-compiler.md) (차원 타입), [08장](ch08-capability-mixins-compile-time-hardware-.md) (믹스인), [10장](ch10-putting-it-all-together-a-complete-diagn.md) (통합)

---

### 문제 요망: 리소스의 혼동

하드웨어 리소스는 코드상에서 비슷해 보이지만 서로 교체해서 사용할 수 없습니다.

- 32비트 레지스터와 16비트 레지스터는 둘 다 "레지스터"입니다.
- 읽기용 DMA 버퍼와 쓰기용 DMA 버퍼는 둘 다 `*mut u8`처럼 보입니다.
- 열린 파일 서술자와 닫힌 파일 서술자는 둘 다 `i32`입니다.

C 언어에서는 이러한 구분이 모호합니다.

```c
// C — 모든 레지스터가 같아 보임
uint32_t read_reg32(volatile void *base, uint32_t offset);
uint16_t read_reg16(volatile void *base, uint32_t offset);

// 버그: 16비트 레지스터를 32비트 함수로 읽음
uint32_t status = read_reg32(pcie_bar, LINK_STATUS_REG); // 사실은 16비트임!
```

---

### 팬텀 타입 매개변수 (Phantom Type Parameters)

**팬텀 타입**은 구조체 정의에는 나타나지만 어떤 필드에서도 사용되지 않는 타입 매개변수입니다. 이는 오직 타입 수준의 정보를 담기 위해 존재합니다.

```rust,ignore
use std::marker::PhantomData;

// 레지스터 너비 마커 — 제로 크기 타입
pub struct Width16;
pub struct Width32;

/// 너비(W)에 의해 매개변수화된 레지스터 핸들.
/// PhantomData<W>는 런타임에 0바이트를 차지하며 오직 컴파일 타임 마커로만 작동함.
pub struct Register<W> {
    base: usize,
    offset: usize,
    _width: PhantomData<W>,
}

impl Register<Width16> {
    pub fn read(&self) -> u16 { /* 2바이트 읽기 */ 0 }
}

impl Register<Width32> {
    pub fn read(&self) -> u32 { /* 4바이트 읽기 */ 0 }
}

impl PcieConfig {
    pub fn vendor_id(&self) -> Register<Width16> {
        Register { base: self.base, offset: 0x00, _width: PhantomData }
    }
    pub fn bar0(&self) -> Register<Width32> {
        Register { base: self.base, offset: 0x10, _width: PhantomData }
    }
}
```

이제 컴파일러가 너비 불일치를 잡아냅니다.

```rust,ignore
let vid: u16 = cfg.vendor_id().read(); // ✅ u16 반환
let bar: u32 = cfg.bar0().read();      // ✅ u32 반환

// let bad: u32 = cfg.vendor_id().read(); // ❌ 에러: u16을 예상함
```

---

### DMA 버퍼 접근 제어

DMA 버퍼는 방향성을 갖습니다. **호스트에서 디바이스로**(쓰기) 또는 **디바이스에서 호스트로**(읽기)의 방향이 정해져 있습니다. 잘못된 방향으로 접근하면 데이터 오염이나 버스 에러가 발생합니다.

```rust,ignore
pub struct ToDevice;     // 호스트가 쓰고 디바이스가 읽음
pub struct FromDevice;   // 디바이스가 쓰고 호스트가 읽음

pub struct DmaBuffer<Dir> {
    ptr: *mut u8,
    len: usize,
    _dir: PhantomData<Dir>,
}

impl DmaBuffer<ToDevice> {
    pub fn write_data(&mut self, data: &[u8]) { /* 데이터 쓰기 가능 */ }
}

impl DmaBuffer<FromDevice> {
    pub fn read_data(&self) -> &[u8] { /* 데이터 읽기 가능 */ }
}
```

이제 `FromDevice` 버퍼에 데이터를 쓰려고 하거나 `ToDevice` 버퍼에서 데이터를 읽으려 하면 메서드 자체가 존재하지 않아 컴파일 에러가 발생합니다.

---

### 핵심 요약

1. **PhantomData는 비용 없이 타입 정보를 전달함** — 마커는 오직 컴파일러를 위해서만 존재하며 런타임 오버헤드는 없습니다.
2. **레지스터 너비 불일치 차단** — `Register<Width16>`은 `u32`가 아닌 `u16`만을 반환하도록 강제됩니다.
3. **구조적인 DMA 방향 강제** — `DmaBuffer<FromDevice>`에는 `write_data()` 메서드가 아예 정의되지 않습니다.
4. **차원 타입(06장)과의 결합** — 레지스터 읽기 결과에 `Celsius`와 같은 물리 단위를 즉시 부여하여 안전성을 극대화할 수 있습니다.
5. **컴파일 타임 전용** — 팬텀 타입은 컴파일 시점에 결정되는 속성을 인코딩하는 데 적합합니다. 런타임에 변하는 속성은 열거형(enum)을 사용하세요.

