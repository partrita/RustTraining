# 18. 요약 및 참조 카드 🟢

### 패턴 결정 가이드

- **원시 타입의 타입 안전성이 필요한가?** → **뉴타입(Newtype)** 패턴 (3장)
- **컴파일 타임에 상태 전이를 강제해야 하는가?** → **타입 상태(Type-state)** 패턴 (3장)
- **런타임 데이터는 없지만 타입 정보가 필요한가?** → **PhantomData** (4장)
- **Rc/Arc의 순환 참조를 끊어야 하는가?** → **Weak<T>** (9장)
- **여러 타입 중 하나를 다뤄야 하는가?**
    - 닫힌 집합(종류가 정해짐) → **열거형(Enum)**
    - 열린 집합(추가 가능), 성능 중요 → **제네릭(Generics)**
    - 열린 집합, 유연함 중요 → **dyn Trait** (2장)
- **스레드 간 상태 공유가 필요한가?**
    - 단순 카운터/플래그 → **원자적(Atomics)** 타입
    - 짧은 임계 구역 → **Mutex**
    - 읽기 위주 작업 → **RwLock**
    - 지연 초기화 → **OnceLock / LazyLock** (6장)
    - 복잡한 상태 관리 → **액터(Actor) + 채널** (5장)
- **계산을 병렬화해야 하는가?**
    - 컬렉션 처리 → **rayon::par_iter**
    - 백그라운드 태스크 → **thread::spawn**
    - 지역 데이터 대여 → **thread::scope**
- **비동기 I/O나 네트워킹이 필요한가?** → **tokio + async/await** (16장)
- **에러 처리가 필요한가?**
    - 라이브러리 → **thiserror** (`#[derive(Error)]`)
    - 애플리케이션 → **anyhow** (`Result<T>`) (10장)
- **값의 메모리 이동을 막아야 하는가?** → **Pin<T>** (9장)

---

### 트레이트 경계 치트 시트

| 경계 | 의미 |
| :--- | :--- |
| `T: Clone` | 복제 가능함 |
| `T: Send` | 다른 스레드로 소유권 이전 가능 |
| `T: Sync` | 여러 스레드에서 참조(&T) 공유 가능 |
| `T: 'static` | 비-정적 참조를 포함하지 않음 (수명 제한 없음) |
| `T: Sized` | 컴파일 타임에 크기를 알 수 있음 (기본값) |
| `T: ?Sized` | 크기를 모를 수 있음 (slice, trait object 등) |
| `T: Into<U>` | `U` 타입으로 변환 가능 |
| `T: AsRef<U>` | `&U`로 빌려올 수 있음 |
| `F: Fn()` | 불변으로 빌려 호출 가능한 클로저 |
| `F: FnMut()` | 가변으로 빌려 호출 가능한 클로저 |
| `F: FnOnce()` | 한 번만 호출 가능하며 소유권을 소비할 수 있음 |

---

### 수명 생략(Lifetime Elision) 규칙

컴파일러는 다음 세 가지 경우에 수명을 자동으로 삽입합니다.
1. 각 참조 인자는 자신만의 수명을 가집니다. (`fn foo<'a, 'b>(x: &'a str, y: &'b str)`)
2. 입력 인자가 정확히 하나라면, 그 수명이 모든 출력에 적용됩니다.
3. `&self`나 `&mut self`가 있다면, 그 수명이 모든 출력에 적용됩니다.

---

### 추가 학습 리소스

| 리소스 | 추천 이유 |
| :--- | :--- |
| [Rust Design Patterns](https://rust-unofficial.github.io/patterns/) | 이디오마틱한 패턴과 안티 패턴의 집대성 |
| [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) | 세련된 공개 API 제작을 위한 공식 체크리스트 |
| [Rust Atomics and Locks](https://marabos.nl/atomics/) | 동시성 프리미티브를 깊게 파고드는 필독서 |
| [The Rustonomicon](https://doc.rust-lang.org/nomicon/) | Unsafe Rust와 내부 깊숙한 곳을 다루는 공식 가이드 |
| [Effective Rust](https://www.lurklurk.org/effective-rust/) | Rust 코드를 개선하는 35가지 구체적인 방법 |

---

*Rust 디자인 패턴 및 엔지니어링 가이드 종료*

