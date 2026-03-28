# 10. 비동기 트레이트: 추상화의 완성 🟡

> **학습 목표:**
> - 트레이트 내부에서 `async fn`을 안정적으로 사용하기까지의 역사와 **RPITIT** 기술을 이해합니다.
> - **정적 디스패치**와 **동적 디스패치(`dyn`)** 환경에서 비동기 트레이트를 다루는 법을 배웁니다.
> - 멀티스레드 환경을 위한 **`Send` 제약 조건** 해결사, **`trait_variant`**를 익힙니다.
> - Rust 1.85에서 안정화된 **비동기 클로저(`async Fn`)**의 활용법을 파악합니다.

---

### 역사적 배경: 왜 트레이트 비동기는 어려웠나?
오랜 기간 Rust에서는 트레이트 안에 `async fn`을 직접 쓸 수 없었습니다. 비동기 함수는 내부적으로 이름 없는 복잡한 `Future` 타입을 반환하는데, 트레이트 시스템이 이를 일반화해서 다루기에 기술적 한계가 있었기 때문입니다.

**드디어 해결되었습니다!** (Rust 1.75+)
이제 특별한 크레이트 없이도 트레이트 내에 `async fn`을 선언할 수 있습니다.

---

### 1. 정적 디스패치 (RPITIT)
가장 권장되는 방식입니다. 컴파일 타임에 타입을 확정하므로 오버헤드가 전혀 없는 '제로 비용 추상화'를 실현합니다.

```rust
trait DataStore {
    async fn get(&self, key: &str) -> Option<String>;
}

// ✅ 제네릭과 함께 쓰면 성능 저하 없이 작동합니다.
async fn lookup<S: DataStore>(store: &S, key: &str) {
    if let Some(val) = store.get(key).await {
        println!("검색 결과: {val}");
    }
}
```

---

### 2. 동적 디스패치와 `Send` 문제
만약 `Vec<Box<dyn DataStore>>`처럼 동적으로 타입을 갈아 끼워야 한다면(`dyn`), 컴파일러는 퓨처의 크기를 알 수 없어 에러를 냅니다. 또한, 멀티스레드 실행기(Tokio)에서 쓰려면 퓨처가 `Send`여야 한다는 제약도 따라붙습니다.

#### 해결사: `trait_variant`
Rust 팀에서 만든 이 도구는 `Send` 버전의 트레이트를 자동으로 생성해 줍니다.

```rust
// Cargo.toml: trait-variant = "0.1"
#[trait_variant::make(SendDataStore: Send)]
trait DataStore {
    async fn get(&self, key: &str) -> Option<String>;
}

// 이제 'SendDataStore'를 사용하면 dyn 디스패치와 tokio::spawn이 모두 가능해집니다.
```

---

### 3. 비동기 클로저 (Rust 1.85+)
콜백 함수나 미들웨어를 짤 때 고대하던 기능입니다. 비동기 블록을 반환하는 일반 클로저의 구질구질한 문법을 한 줄로 정리해 줍니다.

```rust
// 1.85 이전: 어설픈 우회책
let fetcher = move || async move { reqwest::get(url).await };

// 1.85 이후: 네이티브 비동기 클로저
let fetcher = async move || { reqwest::get(url).await };
```

---

### 💡 실무 팁: `async-trait` 크레이트는 이제 졸업하세요
과거에는 `#[async_trait]` 매크로가 필수였지만, 이는 모든 퓨처를 강제로 힙(Heap)에 할당(`Box::pin`)하는 오버헤드가 있었습니다. 최신 Rust 프로젝트라면 성능을 위해 **네이티브 `async fn`**과 **정적 디스패치**를 우선적으로 고려하세요.

---

### 🏋️ 연습 문제: 캐시 서비스 설계하기
**도전 과제:** `get`과 `set` 메서드를 가진 비동기 `Cache` 트레이트를 설계하고, 다음 두 가지 방식으로 구현해 보세요.
1. `HashMap`을 사용하는 메모리 캐시
2. 네트워크 지연(20ms)을 시뮬레이션하는 가짜 외부 캐시

<details>
<summary>🔑 정답 및 힌트 보기</summary>
트레이트에 `async fn get(...)`과 `async fn set(...)`을 선언합니다. 메모리 캐시는 `tokio::sync::Mutex<HashMap>`을 써서 구현하고, 외부 캐시 구현체는 메서드 내부에서 `tokio::time::sleep`을 호출하여 지연을 발생시키면 됩니다. 두 구현체 모두 `Cache` 트레이트를 만족하므로 하나의 제네릭 함수에서 동일하게 다룰 수 있습니다.
</details>

---

### 📌 요약
- Rust 1.75부터 트레이트 내 **`async fn`**이 정식 지원됩니다.
- **`trait_variant`**를 쓰면 `dyn` 디스패치와 `Send` 문제를 쉽게 풀 수 있습니다.
- **비동기 클로저**는 1.85부터 더 깔끔한 콜백 설계를 도와줍니다.
- 성능이 민감한 구간에선 `dyn`보다 제네릭을 통한 **정적 디스패치**를 쓰세요.

