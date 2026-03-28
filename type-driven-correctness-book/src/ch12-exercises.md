# 16. 연습 문제 🟡

> **학습 목표:** 실무에서 접할 수 있는 하드웨어 시나리오 — NVMe 관리 명령, 펌웨어 업데이트 상태 머신, 센서 파イ프라인, PCIe 팬텀 타입, 다중 프로토콜 헬스 체크 등에 '올바른 구성(Correct-by-construction)' 패턴을 직접 적용해 봅니다.
>
> **관련 장:** [02장](ch02-typed-command-interfaces-request-determi.md) (연습 1), [05장](ch05-protocol-state-machines-type-state-for-r.md) (연습 2), [06장](ch06-dimensional-analysis-making-the-compiler.md) (연습 3), [09장](ch09-phantom-types-for-resource-tracking.md) (연습 4), [10장](ch10-putting-it-all-together-a-complete-diagn.md) (연습 5)

---

### 연습 1: NVMe 관리 명령 (타입 지정 명령)

NVMe 관리 명령을 위한 타입 안전한 인터페이스를 설계하세요.

- `Identify` 명령은 `IdentifyResponse`(모델명, 시리얼 등)를 반환해야 합니다.
- `GetLogPage` 명령은 `SmartLog`(온도, 여분 용량 등)를 반환해야 합니다.
- 명령 타입이 응답 타입을 결정하도록 설계하세요 (런타임 디스패치 없음).
- `NamespaceId` 뉴타입을 만들어 일반 `u32`와 혼동되지 않게 하세요.

---

### 연습 2: 펌웨어 업데이트 상태 머신 (타입 상태)

BMC 펌웨어 업데이트의 생명주기를 모델링하세요.

1. 상태: `Idle → Uploading → Verifying → Applying → Rebooting → Complete`
2. `Uploading`과 `Verifying` 단계에서만 `abort()`가 가능해야 합니다.
3. `Applying` 단계는 되돌릴 수 없으므로 `abort()` 메서드가 없어야 합니다.
4. `apply()`는 반드시 성공적인 검증(`Verifying`) 후에 얻은 `VerifiedImage` 토큰이 있어야만 호출 가능해야 합니다.

---

### 연습 3: 센서 데이터 파이프라인 (차원 분석)

다음 단계를 포함하는 센서 파이프라인을 구축하세요.

1. 뉴타입 정의: `RawAdc`, `Celsius`, `Fahrenheit`, `Volts`, `Watts`
2. `Celsius`와 `Fahrenheit` 간의 상호 변환(`From`) 구현
3. `Volts * Amperes = Watts`와 같은 물리 법칙을 트레이트로 정의
4. 제네릭 `Threshold<T>` 체커를 만들어 측정값이 임계치를 넘었는지 확인

---

### 연습 4: PCIe 케이퍼빌리티 탐색 (팬텀 타입 + 유효성 경계)

PCIe 설정 공간의 케이퍼빌리티 연결 리스트를 모델링하세요.

- `RawCapability`와 검증된 `ValidCapability<Kind>`를 분리하세요.
- `Kind`에는 `Msi`, `MsiX`, `PciExpress` 등 팬텀 타입을 사용하세요.
- 각 케이퍼빌리티마다 고유한 레지스터 접근 메서드를 제공하세요.

---

### 연습 5: 다중 프로토콜 헬스 체크 (역량 믹스인)

다양한 하위 시스템의 상태를 점검하는 라이브러리를 만드세요.

- `HasIpmi`, `HasRedfish`, `HasNvmeCli`와 같은 재료 트레이트를 정의하세요.
- 믹스인(Mixin)을 사용하여 특정 재료가 있을 때만 활성화되는 점검 기능을 만드세요 (예: `HasIpmi + HasRedfish`가 있으면 `BmcHealthMixin` 활성화).

---

### 핵심 요약

1. **실제 프로토콜 기반 실습** — NVMe, PCIe 등 시스템 엔지니어가 매일 다루는 기술들을 대상으로 연습합니다.
2. **패턴의 조화** — 각 연습 문제는 앞서 배운 핵심 패턴 중 하나 이상을 깊이 있게 다룹니다.
3. **직접 해보세요** — 해설을 보기 전에 각 시나리오를 코드로 구현해 보는 것이 가장 효과적인 학습 방법입니다.
4. **컴파일러와의 대화** — 의도적으로 틀린 코드를 작성해 보고, 컴파일러가 어떻게 그 실수를 잡아내는지 확인해 보세요.

