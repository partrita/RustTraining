# 3. 단회용 타입 — 소유권을 통한 암호학적 보장 🟡

> **학습 목표:** Rust의 이동 의미론(Move semantics)이 어떻게 선형 타입 시스템(Linear type system)처럼 작동하여 논스(Nonce) 재사용, 키 합의 중복, 그리고 우발적인 퓨즈(Fuse) 재프로그래밍을 컴파일 타임에 방지하는지 배웁니다.
>
> **관련 장:** [01장](ch01-the-philosophy-why-types-beat-tests.md) (철학), [04장](ch13-reference-card.md) (역량 토큰), [05장](ch05-protocol-state-machines-type-state-for-r.md) (타입 상태), [14장](ch14-testing-type-level-guarantees.md) (컴파일 실패 테스트)

---

### 논스 재사용의 재앙 (The Nonce Reuse Catastrophe)

인증된 암호화(AES-GCM, ChaCha20-Poly1305 등)에서 동일한 키로 논스를 재사용하는 것은 **치명적**입니다. 이는 두 평문의 XOR 값을 노출시키고, 심지어 인증 키 자체를 유출할 수도 있습니다. 이는 이론적인 문제가 아닙니다.

- **2016년**: TLS의 AES-GCM에 대한 Forbidden Attack — 논스 재사용으로 평문 복구 가능함이 증명됨.
- **2020년**: 여러 IoT 펌웨어 업데이트 시스템에서 부실한 난수 생성기(RNG)로 인해 논스가 재사용되는 사례 발견.

C/C++에서 논스는 그저 `uint8_t[12]`일 뿐입니다. 이를 두 번 사용하는 것을 막을 장치가 없습니다.

```c
// C — 논스 재사용을 막을 수 없음
uint8_t nonce[12];
generate_nonce(nonce);
encrypt(key, nonce, msg1, out1);   // ✅ 첫 번째 사용
encrypt(key, nonce, msg2, out2);   // 🐛 치명적 버그: 동일한 논스 재사용
```

---

### 선형 타입으로서의 이동 의미론

Rust의 소유권 시스템은 사실상 **선형 타입 시스템**과 같습니다. `Copy`를 구현하지 않은 값은 정확히 한 번만 사용(이동)될 수 있기 때문입니다. 암호학 라이브러리인 `ring` 크레이트는 이 점을 활용합니다.

```rust,ignore
// ring::aead::Nonce 타입의 특징:
// - Clone 불가
// - Copy 불가
// - 사용 시 값으로 소비(Consume)됨
pub struct Nonce(/* 비공개 필드 */);
```

`Nonce`를 `seal_in_place()` 함수에 전달하면 **값이 이동**합니다.

```rust,ignore
fn seal_in_place(
    key: &SealingKey,
    nonce: Nonce,       // ← 참조가 아닌 값으로 "이동"함
    data: &mut Vec<u8>,
) -> Result<(), Error> {
    // ... 암호화 수행 ...
    // 함수가 끝나면 nonce는 소멸됨 — 다시 사용할 수 없음
    Ok(())
}
```

재사용을 시도하면 컴파일 에러가 발생합니다.

```rust,ignore
fn bad_encrypt(key: &SealingKey, data1: &mut Vec<u8>, data2: &mut Vec<u8>) {
    let nonce = Nonce::try_assume_unique_for_key(&[0u8; 12]).unwrap();
    seal_in_place(key, nonce, data1).unwrap();  // ✅ nonce가 여기서 이동함
    // seal_in_place(key, nonce, data2).unwrap();
    //                    ^^^^^ ERROR: 이동된 값 사용 (use of moved value) ❌
}
```

컴파일러가 각 논스가 정확히 한 번만 사용됨을 **증명**합니다. 별도의 테스트가 필요 없습니다.

---

### 하드웨어 응용: 일회성 퓨즈(OTP Fuse) 프로그래밍

서버 플랫폼에는 보안 키, 시리얼 번호 등을 저장하는 **일회성 프로그래밍 가능(OTP) 퓨즈**가 있습니다. 퓨즈 쓰기는 되돌릴 수 없으며, 서로 다른 데이터로 두 번 쓰려고 하면 하드웨어가 영구적으로 손상(Brick)될 수 있습니다.

```rust,ignore
/// 퓨즈 쓰기 페이로드. Clone/Copy 불가.
pub struct FusePayload {
    address: u32,
    data: Vec<u8>,
}

impl FuseController {
    /// 퓨즈를 프로그래밍함 — 페이로드를 소비하여 중복 쓰기를 방지함.
    pub fn program(
        &mut self,
        payload: FusePayload,  // ← 이동(Move) — 두 번 사용할 수 없음
    ) -> io::Result<()> {
        // ... 하드웨어에 쓰기 수행 ...
        // payload가 소비되었으므로, 같은 payload로 다시 시도하면 컴파일 에러
        Ok(())
    }
}
```

---

### 단회용 타입 사용 가이드

| 시나리오 | 단회용(이동) 의미론 권장 여부 |
|----------|:------:|
| 암호학적 논스(Nonce) | ✅ **무조건** — 재사용 시 보안 파괴 |
| 임시 키 합의 (DH, ECDH) | ✅ **무조건** — 재사용 시 순방향 비밀성 약화 |
| OTP 퓨즈 쓰기 | ✅ **무조건** — 중복 쓰기 시 하드웨어 손상 |
| 라이선스 활성화 코드 | ✅ **대체로** — 중복 활성화 방지 |
| 캘리브레이션 토큰 | ✅ **대체로** — 세션당 한 번의 조정 강제 |
| 데이터 버퍼 | ❌ 재사용이 필수적이므로 `&mut [u8]` 사용 |

---

### 핵심 요약

1. **이동 = 단회 사용** — `Clone`이나 `Copy`가 없는 타입은 정확히 한 번만 소비될 수 있으며, 컴파일러가 이를 강제합니다.
2. **패턴의 확장성** — 이 패턴은 암호학을 넘어 OTP 퓨즈, 캘리브레이션 토큰, 감사 로그 엔트리 등 "최대 한 번만 발생해야 하는" 모든 로직에 적용됩니다.
3. **순방향 비밀성(Forward Secrecy)** — 임시 키가 파생된 비밀값으로 이동된 후 메모리에서 즉시 사라지므로 보안이 강화됩니다.
4. **의심스러울 땐 Clone을 빼라** — 나중에 추가하는 것은 쉽지만, 공개된 API에서 `Clone`을 제거하는 것은 파괴적인 변경(Breaking change)입니다.

