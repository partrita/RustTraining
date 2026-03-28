# 12. 흔히 발생하는 함정들: 9가지 실수와 해결책 🔴

> **학습 목표:**
> - 비동기 Rust 개발 시 가장 자주 마주치는 **9가지 버그**의 유형과 해결 방법을 익힙니다.
> - **실행기를 블록(Blocking)**하는 것이 왜 치명적인지 이해하고 `spawn_blocking` 활용법을 배웁니다.
> - **취소 위험(Cancellation Hazards)**: `.await` 도중에 퓨처가 드롭될 때 발생하는 상태 불일치 문제를 파악합니다.
> - `tokio-console`, `tracing` 등 비동기 전용 **디버깅 도구** 사용법을 익힙니다.
> - `time::pause()`를 활용해 실제 시간을 기다리지 않고 **비동기 로직을 테스트**하는 기법을 배웁니다.

---

### 1. 실행기 블록하기 (가장 흔한 실수)
비동기 워커 스레드에서 `std::fs`나 `std::thread::sleep` 같은 동기식 블로킹 코드를 실행하면, 해당 스레드에 할당된 수천 개의 다른 태스크들이 모두 멈춰버립니다.

```rust
// ❌ 나쁜 예: 실행기 스레드 전체를 500ms 동안 마비시킴
async fn bad_handler() {
    std::thread::sleep(Duration::from_millis(500));
}

// ✅ 좋은 예: 블로킹 작업 전용 스레드 풀로 작업을 넘김
async fn good_handler() {
    tokio::task::spawn_blocking(|| {
        std::thread::sleep(Duration::from_millis(500));
    }).await.unwrap();
}

// ✅ 최선의 예: 비동기 전용 함수를 사용함
async fn best_handler() {
    tokio::time::sleep(Duration::from_millis(500)).await;
}
```

---

### 2. `.await` 지점을 넘어서는 MutexGuard 보유
`.await`를 하는 동안 락(`MutexGuard`)을 들고 있으면 다른 스레드가 락을 얻지 못해 교착 상태(Deadlock)에 빠질 수 있습니다. 또한 `std::sync::MutexGuard`는 `!Send`이므로 멀티스레드 런타임에서 컴파일 에러가 납니다.

- **해결책**: 락의 범위를 `{ }` 블록으로 제한하여 `.await` 전에 해제되거나, **`tokio::sync::Mutex`**를 사용하세요.

---

### 3. 취소 위험과 상태 불일치
비동기 작업은 언제든지 드롭(취소)될 수 있습니다. 만약 "돈 인출"과 "돈 입금" 사이에 `.await`가 있고 거기서 작업이 취소된다면 데이터 무결성이 깨집니다.
- **해결책**: 중요한 작업은 취소되어도 안전하도록 **원자적(Atomic)**으로 구성하거나, 데이터베이스 **트랜잭션**을 활용하세요.

---

### 4. 비동기 드롭(Drop)의 부재
Rust의 `Drop` 트레이트는 동기식입니다. 따라서 `drop()` 메서드 안에서 `.await`를 쓸 수 없습니다.
- **해결책**: `tokio::spawn`을 이용해 정리 작업을 백그라운드로 넘기거나, 명시적인 `async fn shutdown(self)` 메서드를 제공하세요.

---

### 5. 의도치 않은 순차 실행
`.await`를 한 줄씩 쓰면 앞의 작업이 완전히 끝나야 다음 작업이 시작됩니다.

```rust
// ❌ 순차적: 총 2초 소요
let a = fetch_a().await; // 1초 대기
let b = fetch_b().await; // 1초 대기

// ✅ 동시 실행: 총 1초 소요
let (a, b) = tokio::join!(fetch_a(), fetch_b());
```

---

### 💡 실무 팁: 디버깅은 `tokio-console`로
프로그램이 이유 없이 멈춘 것 같다면 `tokio-console`을 연결해 보세요. 어떤 태스크가 어디서 `Pending` 상태로 오래 머물고 있는지, 어떤 락을 기다리고 있는지 실시간으로 시각화해 줍니다.

---

### 🏋️ 연습 문제: 버그 찾기
**도전 과제:** 다음 코드에서 비동기 성능과 안정성을 해치는 요소 3가지를 찾아보세요.

```rust
async fn process(urls: Vec<String>) {
    let results = std::sync::Mutex::new(vec![]);
    for url in urls {
        let res = fetch(url).await;
        let mut guard = results.lock().unwrap();
        save(res).await; // 결과를 저장하는 비동기 함수
        guard.push(res);
    }
}
```

<details>
<summary>🔑 정답 및 해설 보기</summary>
1. **순차 실행**: `for` 루프 안에서 `await`를 하므로 URL을 하나씩 처리합니다. (`join!`이나 스트림 권장)
2. **락 유지**: `save(res).await`를 호출하는 동안 `MutexGuard`를 계속 들고 있습니다. (심각한 성능 저하 및 교착 상태 위험)
3. **효율성**: 모든 결과를 수집한 뒤 한꺼번에 처리하면 뮤텍스 자체가 필요 없을 수도 있습니다.
</details>

---

### 📌 요약
- 비동기 스레드에서 **절대** 블로킹 코드를 실행하지 마세요.
- **`.await` 전후**의 락 보유 기간을 최소화하세요.
- 비동기 작업은 **언제든 취소**될 수 있음을 가정하고 코드를 짜세요.
- **`tokio::test`**와 **`time::pause()`**를 활용해 시간 관련 로직을 완벽히 검증하세요.

