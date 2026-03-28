# 동시성: GIL의 굴레를 벗어나 진정한 병렬로

> **학습 목표:** 파이썬의 가장 큰 제약 중 하나인 **GIL(Global Interpreter Lock)**의 한계를 이해하고, Rust가 이를 어떻게 극복하여 진정한 멀티코어 병렬 처리를 구현하는지 배웁니다. `Send`, `Sync` 트레이트를 통한 안전한 데이터 공유와 `async/await`의 차이점을 익힙니다.

---

### 1. No GIL: 진정한 병렬 처리가 가능한 이유
파이썬의 스레드는 GIL 때문에 한 번에 하나의 스레드만 실행될 수 있지만, Rust는 그런 제약이 없습니다. 4코어 CPU라면 4개의 스레드가 동시에 작업을 처리하여 약 4배의 성능 향상을 얻을 수 있습니다.

```python
# [Python] GIL 때문에 CPU 작업은 여러 스레드를 써도 속도가 같음
import threading
def cpu_bound():
    # 무거운 연산...
threads = [threading.Thread(target=cpu_bound) for _ in range(4)]
# 실행 시간은 싱글 스레드와 거의 동일
```

```rust
// [Rust] 진정한 병렬 처리
use std::thread;
let handles: Vec<_> = (0..4).map(|_| {
    thread::spawn(|| {
        // 무거운 연산... (각 스레드가 각기 다른 코어에서 병렬 실행)
    })
}).collect();
```

---

### 2. 데이터 경합 방지: Send와 Sync
파이썬은 런타임에 데이터 경합(Data Race)이 발생하여 값이 꼬일 수 있지만, Rust는 컴파일 타임에 이를 원천 차단합니다.

- **`Send`**: 이 타입은 다른 스레드로 소유권을 넘길 수 있음.
- **`Sync`**: 이 타입은 여러 스레드에서 동시에 참조할 수 있음.

컴파일러가 이 속성을 자동으로 검사하므로, 안전하지 않은 방식으로 데이터를 공유하려고 하면 빌드조차 되지 않습니다.

---

### 3. 동시성 도구 비교 (Python vs Rust)

| **기능** | **Python** | **Rust** | **비고** |
| :--- | :--- | :--- | :--- |
| **잠금(Lock)** | `threading.Lock()` | **`Mutex<T>`** / **`RwLock<T>`** | Rust는 `Arc`와 함께 사용 |
| **채널(Channel)** | `queue.Queue()` | **`mpsc::channel()`** | 메시지 패싱 방식 권장 |
| **비동기 실행** | `asyncio` | **`Tokio`** / **`async-std`** | Rust는 런타임을 선택 가능 |
| **병렬 반복문** | `multiprocessing.Pool` | **`Rayon` (`par_iter`)** | Rust는 별도 프로세스 없이 스레드로 처리 |

---

### 4. 비동기 프로그래밍 (async/await)
파이썬의 `asyncio`는 싱글 스레드 기반의 이벤트 루프를 사용하지만, Rust의 비동기 런타임(예: `Tokio`)은 기본적으로 멀티스레드 워커 풀을 사용합니다. 즉, 비동기 작업 중에도 CPU 연산을 병렬로 처리할 수 있습니다.

---

### 💡 실무 팁: `Rayon`의 마법
CPU 집약적인 작업을 처리할 때, Rust에서는 `Rayon` 크레이트를 써보세요. 기존의 `.iter()`를 `.par_iter()`로 바꾸기만 해도 자동으로 모든 CPU 코어를 사용하는 병렬 반복문으로 변신합니다. 별도의 멀티프로세싱 설정이나 직렬화 오버헤드 없이 압도적인 성능을 낼 수 있습니다.

