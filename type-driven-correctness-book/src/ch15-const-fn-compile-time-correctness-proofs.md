# 10. Const Fn — 컴파일 타임 올바름 증명 🟠

> **학습 목표:** `const fn`과 `assert!`가 어떻게 컴파일러를 강력한 증명 엔진으로 변모시키는지 배웁니다. SRAM 메모리 맵, 레지스터 레이아웃, 프로토콜 프레임, 비트 필드 마스크, 클록 트리 등을 **런타임 비용 없이** 컴파일 타임에 검증하는 법을 익힙니다.
>
> **관련 장:** [04장](ch04-capability-tokens-zero-cost-proof-of-aut.md) (역량 토큰), [06장](ch06-dimensional-analysis-making-the-compiler.md) (차원 분석), [09장](ch09-phantom-types-for-resource-tracking.md) (팬텀 타입)

---

### 문제 요망: 거짓말하는 메모리 맵

임베디드 및 시스템 프로그래밍에서 메모리 맵은 부트로더, 펌웨어, 데이터 섹션, 스택의 위치를 정의하는 가장 기초적인 설계도입니다. C 언어에서는 보통 `#define` 상수로 이를 관리하는데, 영역 간의 관계(중첩 여부 등)를 구조적으로 파악하기 어렵습니다.

```c
/* C — 영역이 겹치거나 SRAM 크기를 초과해도 잡아낼 수 없음 */
#define SRAM_BASE   0x20000000
#define SRAM_SIZE   (256 * 1024)
#define FW_SIZE     (200 * 1024)
#define DATA_SIZE   (80 * 1024) // 200 + 80 = 280KB? SRAM(256KB)을 초과함!
```

이런 실수는 배포 후에야 신비로운 크래시로 나타납니다.

---

### Const Fn: 컴파일러를 증명 엔진으로

Rust의 `const fn`은 컴파일 타임에 실행될 수 있습니다. `const fn` 내부의 `assert!`가 실패하면 **컴파일 에러**가 발생합니다. 즉, 컴파일러 자체가 여러분의 설계 의도를 검증하는 감사관이 됩니다.

#### 검증된 SRAM 메모리 맵 예시

```rust,ignore
pub struct Region { pub base: u32, pub size: u32 }

impl Region {
    pub const fn new(base: u32, size: u32) -> Self {
        assert!(size > 0, "지역 크기는 0보다 커야 합니다");
        Self { base, size }
    }
    pub const fn end(&self) -> u32 { self.base + self.size }
    pub const fn overlaps(&self, other: &Region) -> bool {
        self.base < other.end() && other.base < self.end()
    }
}

pub struct SramMap {
    pub total: Region,
    pub firmware: Region,
    pub data: Region,
}

impl SramMap {
    pub const fn verified(total: Region, fw: Region, data: Region) -> Self {
        // 중첩 여부 및 전체 크기 초과 여부를 컴파일 타임에 증명
        assert!(total.contains(&fw), "펌웨어가 SRAM을 초과함");
        assert!(total.contains(&data), "데이터 섹션이 SRAM을 초과함");
        assert!(!fw.overlaps(&data), "펌웨어와 데이터 영역이 겹침");
        Self { total, firmware: fw, data }
    }
}
```

이제 상수를 정의하기만 하면 컴파일러가 모든 제약 조건을 체크합니다.

```rust,ignore
// ✅ 모든 조건 만족 — 컴파일 성공
const SRAM: SramMap = SramMap::verified(
    Region::new(0x2000_0000, 256 * 1024),
    Region::new(0x2000_0000, 128 * 1024),
    Region::new(0x2002_0000, 64 * 1024),
);

// ❌ 조건 위반시 컴파일 에러 발생!
```

---

### 레지스터 및 프로토콜 프레임 레이아웃

동일한 기법을 하드웨어 레지스터 맵이나 네트워크 프로토콜 프레임에도 적용할 수 있습니다. 레지스터 오프셋이 정렬(Alignment)되어 있는지, 필드들이 서로 겹치지는 않는지 등을 무조건적으로 보장할 수 있습니다.

```rust,ignore
// 비트 필드 마스크 중첩 방지 예시
const SPI_EN:     BitField = BitField::new(0, 1);   // 0번 비트
const SPI_MODE:   BitField = BitField::new(1, 2);   // 1-2번 비트
const SPI_CLKDIV: BitField = BitField::new(4, 4);   // 4-7번 비트

// 컴파일 타임 증명: 어떤 필드도 비트를 공유하지 않음
const _: () = {
    assert!(fields_disjoint(&SPI_EN, &SPI_MODE));
    assert!(fields_disjoint(&SPI_EN, &SPI_CLKDIV));
    // ... 모든 조합 확인 ...
};
```

---

### 핵심 요약

1. **상수화된 올바름** — `const fn`과 `assert!`를 사용하면 런타임에 발생할 '신비로운 버그'를 컴파일 타임의 '명확한 에러 메시지'로 바꿀 수 있습니다.
2. **제로 비용 보장** — 모든 검증은 컴파일 시 완료되며, 실제 바이너리에는 검사 로직이 포함되지 않습니다. 결과물은 검사 없이 작성된 C 코드만큼이나 가볍습니다.
3. **증거 기반 설계** — `VerifiedAddr`와 같은 타입을 통해, 어떤 주소값이 특정 영역 내에 있음이 정적으로 증명된 상태로 안전하게 데이터를 다룰 수 있습니다.
4. **점진적 정밀도 향상** — 메모리 맵 수준의 거친 검증부터 레지스터 비트 단위의 정밀한 검증까지 동일한 패턴으로 확장 가능합니다.

