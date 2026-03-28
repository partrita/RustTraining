# 9. Tokio가 만능은 아닙니다: 대안 탐색 🟡

> **학습 목표:**
> - `tokio::spawn`의 **`'static` 제약** 때문에 발생하는 불편함과 `Arc` 남발 문제를 인식합니다.
> - `!Send` 퓨처를 안전하게 실행하기 위한 **`LocalSet`**의 활용법을 배웁니다.
> - 데이터를 빌려올 수 있는 동시성 도구인 **`FuturesUnordered`**를 익힙니다.
> - 여러 태스크를 묶어서 관리하는 **`JoinSet`**의 장점을 파악합니다.
> - 특정 런타임에 종속되지 않는 라이브러리 설계 원칙을 정리합니다.

---

### `'static` 제약의 늪
Tokio의 `spawn`은 태스크가 언제 끝날지 모르기 때문에 모든 데이터의 소유권을 태스크가 가져가길 원합니다. 하지만 이는 실무에서 매우 번거로울 때가 많습니다.

```rust
async fn process_items(items: &[String]) {
    // ❌ Error: items는 빌려온 데이터라 'static이 아닙니다.
    // for item in items {
    //     tokio::spawn(async move { process(item).await; });
    // }

    // 😐 해결책: 매번 클론하거나 Arc로 감싸야 함 (귀찮음!)
    for item in items {
        let item = item.clone();
        tokio::spawn(async move { process(item).await; });
    }
}
```

---

### 대안 1: `FuturesUnordered` (빌림 친화적 동시성)
`FuturesUnordered`는 퓨처들을 현재 태스크 안에서 동시에 실행합니다. 스레드를 옮겨 다니지 않으므로 **데이터를 빌려올 수 있고, `'static` 제약도 없습니다.**

```rust
use futures::stream::{FuturesUnordered, StreamExt};

async fn process_items(items: &[String]) {
    let mut futures = FuturesUnordered::new();
    
    for item in items {
        // ✅ item을 빌려올 수 있음! 클론할 필요가 없습니다.
        futures.push(async move { process(item).await });
    }

    // 모든 작업이 끝날 때까지 대기
    while let Some(res) = futures.next().await {
        println!("결과: {res:?}");
    }
}
```

---

### 대안 2: `LocalSet` (`!Send` 지원)
`Rc`나 `RefCell`처럼 스레드 간 이동이 불가능한(`!Send`) 타입을 비동기 코드에서 써야 한다면 `LocalSet`이 정답입니다. 모든 작업을 현재 스레드에 고정시켜 실행합니다.

---

### 대안 3: `JoinSet` (태스크 군단 관리)
많은 수의 태스크를 스폰하고, 이들이 완료되는 대로 결과를 수집하거나 한꺼번에 취소해야 한다면 `JoinSet`이 가장 깔끔합니다. (Tokio 1.21 이상 권장)

---

### 💡 실무 팁: 라이브러리 제작자는 "중립"을 지키세요
여러분이 만드는 라이브러리가 내부에 `tokio::spawn`이나 `tokio::time`을 직접 포함하고 있다면, 그 라이브러리를 쓰는 사용자도 강제로 Tokio를 써야만 합니다.
- **좋은 예**: `std::future::Future`와 `futures` 크레이트의 공통 트레이트만 사용하세요.
- **최선의 예**: 시간이 필요하다면 타이머를 인자로 받거나, 사용자가 실행기를 주입할 수 있게 설계하세요.

---

### 🏋️ 연습 문제: 어떤 도구를 쓸까요?
**상황:** 10개의 DB 쿼리를 동시에 날려야 합니다. 쿼리에 쓰일 데이터는 함수 인자로 넘어온 슬라이스(`&[Query]`)에 들어있습니다. 데이터를 복사(Clone)하고 싶지는 않습니다. 어떤 도구가 적절할까요?

<details>
<summary>🔑 정답 및 해설 보기</summary>
**정답:** `FuturesUnordered`가 가장 적절합니다. 
`tokio::spawn`을 쓰려면 슬라이스의 데이터를 일일이 클론해야 하지만, `FuturesUnordered`는 현재 컨텍스트를 유지하므로 안전하게 참조를 사용할 수 있습니다.
</details>

---

### 📌 요약
- `tokio::spawn`의 `'static` 제약이 버겁다면 **`FuturesUnordered`**를 고려하세요.
- 스레드 이동이 안 되는 데이터는 **`LocalSet`**에서 처리하세요.
- 대규모 태스크 관리는 **`JoinSet`**이 효율적입니다.
- 라이브러리는 런타임 중립적으로 설계하여 호환성을 높이세요.

