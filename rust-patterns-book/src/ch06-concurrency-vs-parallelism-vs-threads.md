# 6. 동시성 vs 병렬성 vs 스레드 🟡

> **학습 목표:**
> - **동시성(Concurrency)**과 **병렬성(Parallelism)**의 정확한 차이를 이해합니다.
> - OS 스레드, 스코프 스레드, 데이터 병렬 처리를 위한 **Rayon**을 익힙니다.
> - 공유 상태를 위한 프리미티브: **Arc, Mutex, RwLock, Atomics, Condvar**를 배웁니다.
> - **OnceLock/LazyLock**을 이용한 지연 초기화와 무잠금(Lock-free) 패턴을 학습합니다.

---

### 용어 정리: 동시성 ≠ 병렬성

이 두 용어는 자주 혼용되지만, 기술적으로는 명확히 구분됩니다.

| 구분 | 동시성 (Concurrency) | 병렬성 (Parallelism) |
| :--- | :--- | :--- |
| **정의** | 여러 작업이 진행 중임을 관리함 | 여러 작업이 **동시에** 실행됨 |
| **하드웨어 요구사항** | 싱글 코어로도 가능 (시분할) | 반드시 멀티 코어가 필요함 |
| **비유** | 요리사 한 명이 여러 요리를 번갈아 만듦 | 여러 요리사가 각자 요리를 하나씩 만듦 |
| **Rust 도구** | `async/await`, 채널, `select!` | `rayon`, `thread::spawn`, `par_iter()` |

---

### 스코프 스레드 (std::thread::scope)

Rust 1.63부터 도입된 스코프 스레드는 부모 스레드의 스택 데이터를 `Arc` 없이도 안전하게 빌려올 수 있게 해줍니다.

```rust
let mut data = vec![1, 2, 3];

thread::scope(|s| {
    // 부모의 data를 직접 빌려올 수 있음!
    s.spawn(|| println!("합계: {}", data.iter().sum::<i32>()));
    s.spawn(|| println!("최대값: {:?}", data.iter().max()));
});

// 스택을 벗어나기 전에 모든 스레드가 종료됨을 컴파일러가 보장함
data.push(4); 
```

---

### Rayon: 데이터 병렬 처리

`rayon`은 표준 반복자를 병렬 반복자로 바꾸는 아주 간단한 방법을 제공합니다.

```rust
use rayon::prelude::*;

let data: Vec<u64> = (0..1_000_000).collect();

// 순차 처리:
let sum = data.iter().map(|x| x * x).sum();

// 병렬 처리: .iter()를 .par_iter()로 바꾸면 끝!
let sum = data.par_iter().map(|x| x * x).sum();
```

---

### 공유 상태: Arc, Mutex, RwLock, Atomics

스레드 간에 가변 상태를 공유해야 할 때 Rust는 안전한 추상화를 제공합니다.

| 프리미티브 | 용도 | 특징 |
| :--- | :--- | :--- |
| **`Mutex<T>`** | 짧은 임계 구역 보호 | 한 번에 한 스레드만 접근 가능 |
| **`RwLock<T>`** | 읽기 위주, 쓰기 드문 경우 | 여러 명 읽기 가능, 쓰기는 독점적 |
| **`Atomics`** | 단순 카운터, 플래그 | 하드웨어 수준의 원자적 연산 (잠금 없음) |
| **`Condvar`** | 조건 대기 | 특정 조건이 참이 될 때까지 스레드를 재움 |

---

### 지연 초기화: OnceLock과 LazyLock

글로벌 설정이나 정규표현식처럼 런타임에 단 한 번만 초기화가 필요한 경우, 이제 표준 라이브러리의 기능을 직접 사용하세요.

```rust
use std::sync::LazyLock;

// 1.80부터 지원: 전역 정규표현식을 매크로 없이 선언
static RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^[a-z]+$").unwrap()
});

fn main() {
    // 처음 접근할 때 초기화되고 이후엔 재사용됨
    if RE.is_match("rust") { /* ... */ }
}
```
> **팁**: 기존 코드의 `lazy_static!`을 `LazyLock`으로 교체하면 외부 의존성을 줄일 수 있습니다.

---

### 📝 연습 문제: 스코프 스레드를 활용한 병렬 맵(Map) ★★ (~25분)

`rayon`을 사용하지 않고 `std::thread::scope`만을 사용하여, 데이터를 N개의 청크로 나누어 각 스레드에서 처리하는 `parallel_map` 함수를 작성해 보세요.

---

### 📌 요약
- **스코프 스레드**를 쓰면 `Arc` 없이도 로컬 데이터를 스레드에 넘길 수 있습니다.
- 컬렉션 처리는 **`rayon::par_iter()`**가 가장 간편하고 강력한 도구입니다.
- 운영 환경에서는 `OnceLock`/`LazyLock`을 적극 활용하고, 복잡한 무잠금(Lock-free) 로직은 검증된 크레이트(`crossbeam`, `dashmap` 등)를 사용하세요.

