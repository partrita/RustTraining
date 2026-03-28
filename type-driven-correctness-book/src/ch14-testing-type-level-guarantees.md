# 18. 타입 수준 보장 테스트하기 🟡

> **학습 목표:** 유효하지 않은 코드가 *컴파일되지 않음*을 테스트하는 법(`trybuild`), 유효성 경계에 대한 퍼징(`proptest`), RAII 불변성 검증, 그리고 `cargo-show-asm`을 통한 제로 비용 추상화 증명법을 배웁니다.
>
> **관련 장:** [03장](ch03-single-use-types-cryptographic-guarantee.md) (논스에 대한 컴파일 실패 테스트), [07장](ch07-validated-boundaries-parse-dont-validate.md) (경계에 대한 속성 기반 테스트), [05장](ch05-protocol-state-machines-type-state-for-r.md) (세션에 대한 RAII)

---

### 타입 수준 보장 테스트의 필요성

'올바른 구성(Correct-by-construction)' 패턴은 버그를 런타임에서 컴파일 타임으로 옮깁니다. 하지만 **유효하지 않은 코드가 실제로 컴파일에 실패하는지** 어떻게 자동화된 테스트로 보장할 수 있을까요? 또한, 유효성 검증 경계가 수만 개의 무작위 입력에도 견딜 수 있는지 어떻게 확인할까요?

---

### 1. `trybuild`를 이용한 컴파일 실패 테스트

[`trybuild`](https://crates.io/crates/trybuild) 크레이트를 사용하면 특정 코드가 **컴파일되지 않아야 함**을 단언(assert)할 수 있습니다. 이는 리팩토링 과정에서 타입 수준의 불변성이 깨지는 것을 방지하는 데 필수적입니다.

```rust,ignore
#[test]
fn type_safety_tests() {
    let t = trybuild::TestCases::new();
    // 이 경로의 파일들은 컴파일에 실패해야 테스트가 통과함
    t.compile_fail("tests/ui/*.rs");
}
```

예를 들어, 단회용 타입인 `Nonce`를 두 번 사용하려 하면 컴파일 에러가 발생해야 합니다.

---

### 2. 유효성 경계에 대한 속성 기반 테스트 (Proptest)

07장에서 배운 유효성 검증 경계가 모든 잘못된 입력을 걸러내는지 확인하기 위해 [`proptest`](https://crates.io/crates/proptest)를 사용합니다. 이는 수천 가지의 무작위 입력을 생성하여 경계를 압박합니다.

```rust,ignore
proptest! {
    #[test]
    fn valid_fru_never_panics(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
        // 어떤 데이터가 들어와도 유효성 검사를 통과한 이후에는 패닉이 발생하지 않아야 함
        if let Ok(fru) = ValidFru::try_from(data) {
            let _ = fru.board_area(); // 절대 패닉이 나면 안 됨
        }
    }
}
```

---

### 3. 제로 비용 추상화: 어셈블리로 증명하기

"뉴타입이나 팬텀 타입을 쓰면 런타임 성능이 떨어지지 않을까?" 하는 걱정은 버려도 좋습니다. `cargo-show-asm`을 통해 확인해보면, 원시 데이터 타입을 쓸 때와 **동일한 기계어**가 생성됨을 알 수 있습니다.

```bash
cargo install cargo-show-asm
cargo asm my_crate::add_rpm # 생성된 어셈블리 확인
```

결과를 보면 구조체 래퍼는 완전히 제거되고 순수한 레지스터 연산만 남는 것을 확인할 수 있습니다.

---

### 핵심 요약

1. **실패에 대한 테스트** — `trybuild`는 "허용되지 않는 행위"가 여전히 불가능함을 보장합니다.
2. **무작위 입력을 통한 검증** — `proptest`는 수작업으로 만들기 힘든 엣지 케이스들을 찾아내어 경계를 더욱 견고하게 만듭니다.
3. **증거 기반의 제로 비용** — 어셈블리 분석을 통해 타입 안전성이 성능 저하를 일으키지 않음을 정적으로 확인할 수 있습니다.
4. **테스트 피라미드** — 타입 시스템이 가장 넓은 기반을 지탱하고, 그 위에 속성 기반 테스트와 단위 테스트, 그리고 컴파일 실패 테스트가 층층이 쌓여 완벽한 안전망을 형성합니다.

