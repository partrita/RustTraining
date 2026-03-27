# Rust 동시성(Concurrency)

> **학습 내용:** Rust의 동시성 모델 — 스레드(Threads), `Send`/`Sync` 마커 트레이트, `Mutex<T>`, `Arc<T>`, 채널(Channels), 그리고 컴파일러가 어떻게 컴파일 타임에 데이터 경합(data races)을 방지하는지 배웁니다. 사용하지 않는 스레드 안전성 기능에 대해서는 런타임 오버헤드가 발생하지 않습니다.

- Rust는 C++의 `std::thread`와 유사하게 동시성에 대한 내장 지원을 제공합니다.
    - 주요 차이점: Rust는 `Send` 및 `Sync` 마커 트레이트를 통해 **컴파일 타임에 데이터 경합을 방지**합니다.
    - C++에서 뮤텍스 없이 여러 스레드 간에 `std::vector`를 공유하는 것은 정의되지 않은 동작(UB)이지만 컴파일은 잘 됩니다. Rust에서는 아예 컴파일되지 않습니다.
    - Rust의 `Mutex<T>`는 단순히 접근을 제어하는 것이 아니라 **데이터를 감쌉니다**. 잠금(locking) 없이는 말 그대로 데이터에 접근할 수 없습니다.
- `thread::spawn()`을 사용하여 클로저 `||`를 병렬로 실행하는 별도의 스레드를 생성할 수 있습니다.
```rust
use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("스레드 카운트: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 0..5 {
        println!("메인 스레드: {i}");
        thread::sleep(Duration::from_millis(5));
    }

    handle.join().unwrap(); // handle.join()은 생성된 스레드가 종료될 때까지 기다립니다.
}
```

# Rust 동시성
- ```thread::scope()```는 주변 환경(scope)에서 변수를 빌려와야 하는 경우에 사용할 수 있습니다. 이는 ```thread::scope```가 내부 스레드가 반환될 때까지 대기하기 때문에 안전하게 작동합니다.
- 이 연습 문제를 ```thread::scope``` 없이 실행하여 어떤 문제가 발생하는지 확인해 보세요.
```rust
use std::thread;
fn main() {
  let a = [0, 1, 2];
  thread::scope(|scope| {
      scope.spawn(|| {
          for x in &a {
            println!("{x}");
          }
      });
  });
}
```
----
# Rust 동시성
- ```move```를 사용하여 소유권을 스레드로 이전할 수도 있습니다. `[i32; 3]`과 같은 `Copy` 타입의 경우, `move` 키워드는 데이터를 클로저로 복사하며, 원본은 계속 사용할 수 있습니다.
```rust
use std::thread;
fn main() {
  let mut a = [0, 1, 2];
  let handle = thread::spawn(move || {
      for x in a {
        println!("{x}");
      }
  });
  a[0] = 42;    // 스레드로 전송된 복사본에는 영향을 주지 않습니다.
  handle.join().unwrap();
}
```

# Rust 동시성
- ```Arc<T>```는 여러 스레드 간에 *읽기 전용* 참조를 공유하는 데 사용될 수 있습니다.
    - ```Arc```는 Atomic Reference Counted(원자적 참조 카운팅)의 약자입니다. 참조 카운트가 0에 도달할 때까지 참조가 해제되지 않습니다.
    - ```Arc::clone()```은 데이터를 복제하지 않고 단순히 참조 카운트만 증가시킵니다.
```rust
use std::sync::Arc;
use std::thread;
fn main() {
    let a = Arc::new([0, 1, 2]);
    let mut handles = Vec::new();
    for i in 0..2 {
        let arc = Arc::clone(&a);
        handles.push(thread::spawn(move || {
            println!("스레드 {i}: {arc:?}");
        }));
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
}
```

# Rust 동시성
- ```Arc<T>```를 ```Mutex<T>```와 결합하여 가변(mutable) 참조를 제공할 수 있습니다.
    - ```Mutex```는 보호된 데이터를 보호하며, 잠금을 보유한 스레드만 접근할 수 있도록 보장합니다.
    - `MutexGuard`는 범위를 벗어날 때 자동으로 해제됩니다(RAII). 참고: `std::mem::forget`을 사용하면 가드를 여전히 누출시킬 수 있으므로, "잠금 해제를 잊는 것이 불가능하다"는 표현보다는 "잠금 누출이 불가능하다"는 표현이 더 정확합니다.
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            // 여기서 MutexGuard가 드롭되어 잠금이 자동으로 해제됩니다.
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("최종 카운트: {}", *counter.lock().unwrap());
    // 출력: 최종 카운트: 5
}
```

# Rust 동시성: RwLock
- `RwLock<T>`는 **여러 명의 동시 읽기 작업자** 또는 **한 명의 독점 쓰기 작업자**를 허용합니다. 이는 C++의 읽기/쓰기 잠금 패턴(`std::shared_mutex`)과 동일합니다.
    - 읽기 작업이 쓰기 작업보다 훨씬 많을 때(예: 설정, 캐시) `RwLock`을 사용하세요.
    - 읽기/쓰기 빈도가 비슷하거나 임계 영역(critical sections)이 짧을 때는 `Mutex`를 사용하세요.
```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let config = Arc::new(RwLock::new(String::from("v1.0")));
    let mut handles = Vec::new();

    // 5개의 읽기 스레드 생성 — 모두 동시에 실행될 수 있습니다.
    for i in 0..5 {
        let config = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            let val = config.read().unwrap();  // 여러 읽기 작업자 OK
            println!("읽기 스레드 {i}: {val}");
        }));
    }

    // 한 명의 쓰기 스레드 — 모든 읽기 작업자가 끝날 때까지 대기(block)합니다.
    {
        let config = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            let mut val = config.write().unwrap();  // 독점적 접근
            *val = String::from("v2.0");
            println!("쓰기 스레드: {val}로 업데이트됨");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

# Rust 동시성: Mutex poisoning
- 스레드가 `Mutex` 또는 `RwLock`을 보유한 상태에서 **패닉(panic)**이 발생하면, 잠금은 **중독(poisoned)** 상태가 됩니다.
    - 이후의 `.lock()` 호출은 `Err(PoisonError)`를 반환합니다. 이는 데이터가 일관되지 않은 상태일 수 있음을 나타냅니다.
    - 데이터가 여전히 유효하다고 확신한다면 `.into_inner()`를 통해 복구할 수 있습니다.
    - 이는 C++에 대응하는 개념이 없습니다. `std::mutex`에는 중독 개념이 없으며, 패닉이 발생한 스레드는 그냥 잠긴 상태로 남게 됩니다.
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let data2 = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut guard = data2.lock().unwrap();
        guard.push(4);
        panic!("으악!");  // 잠금이 이제 중독되었습니다.
    });

    let _ = handle.join();  // 스레드가 패닉 상태로 종료됨

    // 이후의 잠금 시도는 Err(PoisonError)를 반환합니다.
    match data.lock() {
        Ok(guard) => println!("데이터: {guard:?}"),
        Err(poisoned) => {
            println!("잠금이 중독되었습니다! 복구 중...");
            let guard = poisoned.into_inner();  // 그래도 데이터에 접근
            println!("복구된 데이터: {guard:?}");  // [1, 2, 3, 4] — 패닉 전에 push가 성공함
        }
    }
}
```

# Rust 동시성: 원자적 타입(Atomics)
- 간단한 카운터나 플래그의 경우, `std::sync::atomic` 타입들을 사용하면 `Mutex`의 오버헤드를 피할 수 있습니다.
    - `AtomicBool`, `AtomicI32`, `AtomicU64`, `AtomicUsize` 등
    - C++의 `std::atomic<T>`와 동일하며, 같은 메모리 순서(memory ordering) 모델(`Relaxed`, `Acquire`, `Release`, `SeqCst`)을 사용합니다.
```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("카운터: {}", counter.load(Ordering::SeqCst));
    // 출력: 카운터: 10000
}
```

| 기본 요소 | 사용 시기 | C++ 대응 개념 |
|-----------|-------------|----------------|
| `Mutex<T>` | 일반적인 가변 공유 상태 | `std::mutex` + 수동 데이터 연결 |
| `RwLock<T>` | 읽기 위주의 작업 부하 | `std::shared_mutex` |
| `Atomic*` | 간단한 카운터, 플래그, 락프리 패턴 | `std::atomic<T>` |
| `Condvar` | 조건이 참이 될 때까지 대기 | `std::condition_variable` |

# Rust 동시성: Condvar
- `Condvar`(조건 변수)는 스레드가 **다른 스레드가 조건이 변경되었음을 알릴 때까지 잠들게** 합니다.
    - 항상 `Mutex`와 함께 사용됩니다. 패턴은 다음과 같습니다: 잠금 획득, 조건 확인, 준비되지 않았다면 대기, 준비되면 작업 수행.
    - C++의 `std::condition_variable` / `std::condition_variable::wait`와 동일합니다.
    - **가짜 깨어남(spurious wakeups)**을 처리합니다 — 항상 루프 내에서 조건을 다시 확인하십시오 (또는 `wait_while`/`wait_until` 사용).
```rust
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    // 신호를 기다리는 작업 스레드 생성
    let pair2 = Arc::clone(&pair);
    let worker = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut ready = lock.lock().unwrap();
        // wait: 신호가 올 때까지 잠듭니다 (가짜 깨어남에 대비해 항상 루프에서 다시 확인)
        while !*ready {
            ready = cvar.wait(ready).unwrap();
        }
        println!("작업자: 조건 충족, 진행합니다!");
    });

    // 메인 스레드에서 작업을 수행한 후 작업자에게 신호를 보냄
    thread::sleep(std::time::Duration::from_millis(100));
    {
        let (lock, cvar) = &*pair;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cvar.notify_one();  // 대기 중인 스레드 하나를 깨움 (notify_all()은 모두 깨움)
    }

    worker.join().unwrap();
}
```

> **Condvar 대 채널 사용 시기:** 스레드가 가변 상태를 공유하고 해당 상태에 대한 조건(예: "버퍼가 비어 있지 않음")을 기다려야 하는 경우 `Condvar`를 사용하십시오. 스레드가 *메시지*를 전달해야 하는 경우에는 채널(`mpsc`)을 사용하십시오. 채널이 일반적으로 추론하기 더 쉽습니다.

# Rust 동시성
- Rust 채널은 ```Sender```와 ```Receiver``` 사이에서 메시지를 교환하는 데 사용될 수 있습니다.
    - 이는 ```mpsc``` 또는 ```Multi-producer, Single-Consumer```(다중 생산자, 단일 소비자)라고 불리는 패러다임을 사용합니다.
    - ```send()```와 ```recv()``` 모두 스레드를 대기(block)시킬 수 있습니다.
```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    tx.send(10).unwrap();
    tx.send(20).unwrap();
    
    println!("수신됨: {:?}", rx.recv());
    println!("수신됨: {:?}", rx.recv());

    let tx2 = tx.clone();
    tx2.send(30).unwrap();
    println!("수신됨: {:?}", rx.recv());
}
```

# Rust 동시성
- 채널은 스레드와 결합될 수 있습니다.
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    for _ in 0..2 {
        let tx2 = tx.clone();
        thread::spawn(move || {
            let thread_id = thread::current().id();
            for i in 0..10 {
                tx2.send(format!("메시지 {i}")).unwrap();
                println!("{thread_id:?}: 메시지 {i} 전송함");
            }
            println!("{thread_id:?}: 완료");
        });
    }

    // 복제된 모든 송신자가 드롭될 때 rx.iter()가 종료되도록 원래의 송신자를 드롭합니다.
    drop(tx);

    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("메인: {msg} 받음");
    }
}
```



## Rust가 데이터 경합을 방지하는 이유: Send 및 Sync

- Rust는 컴파일 타임에 스레드 안전성을 강제하기 위해 두 가지 마커 트레이트를 사용합니다:
    - `Send`: 타입이 다른 스레드로 안전하게 **이전(transfer)**될 수 있는 경우 `Send`입니다.
    - `Sync`: 타입이 여러 스레드 간에 참조(`&T`)를 통해 안전하게 **공유**될 수 있는 경우 `Sync`입니다.
- 대부분의 타입은 자동으로 `Send + Sync`입니다. 주목할 만한 예외:
    - `Rc<T>`는 `Send`도 `Sync`도 아닙니다 (스레드에는 `Arc<T>`를 사용하세요).
    - `Cell<T>`와 `RefCell<T>`은 `Sync`가 아닙니다 (`Mutex<T>` 또는 `RwLock<T>`를 사용하세요).
    - 원시 포인터(`*const T`, `*mut T`)는 `Send`도 `Sync`도 아닙니다.
- 이것이 컴파일러가 스레드 간에 `Rc<T>`를 사용하는 것을 막는 이유입니다 — `Rc<T>`는 말 그대로 `Send`를 구현하지 않기 때문입니다.
- `Arc<Mutex<T>>`는 `Rc<RefCell<T>>`의 스레드 안전한 대응물입니다.

> **직관적인 이해** *(Jon Gjengset)*: 값을 장난감이라고 생각해보세요.
> **`Send`** = 장난감을 다른 아이(스레드)에게 **줄 수 있음** — 소유권 이전이 안전함.
> **`Sync`** = 다른 아이들이 내 장난감을 **동시에 가지고 놀게 할 수 있음** — 참조 공유가 안전함.
> `Rc<T>`는 취약한(원자적이지 않은) 참조 카운터를 가지고 있습니다. 이를 넘겨주거나 공유하면 카운트가 손상될 수 있으므로, `Send`도 `Sync`도 아닙니다.


# 연습 문제: 멀티스레드 단어 계산기

🔴 **도전** — 스레드, Arc, Mutex, HashMap의 결합

- 텍스트 라인들이 담긴 `Vec<String>`이 주어지면, 각 라인당 하나의 스레드를 생성하여 해당 라인의 단어 수를 계산하세요.
- 결과를 수집하기 위해 `Arc<Mutex<HashMap<String, usize>>>`를 사용하세요.
- 모든 라인에 걸친 총 단어 수를 출력하세요.
- **보너스**: 공유 상태 대신 채널(`mpsc`)을 사용하여 이를 구현해 보세요.

<details><summary>풀이 (클릭하여 확장)</summary>

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let lines = vec![
        "the quick brown fox".to_string(),
        "jumps over the lazy dog".to_string(),
        "the fox is quick".to_string(),
    ];

    let word_counts: Arc<Mutex<HashMap<String, usize>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];
    for line in &lines {
        let line = line.clone();
        let counts = Arc::clone(&word_counts);
        handles.push(thread::spawn(move || {
            for word in line.split_whitespace() {
                let mut map = counts.lock().unwrap();
                *map.entry(word.to_lowercase()).or_insert(0) += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let counts = word_counts.lock().unwrap();
    let total: usize = counts.values().sum();
    println!("단어 빈도수: {counts:#?}");
    println!("총 단어 수: {total}");
}
// 출력 (순서는 다를 수 있음):
// 단어 빈도수: {
//     "the": 3,
//     "quick": 2,
//     "brown": 1,
//     "fox": 2,
//     "jumps": 1,
//     "over": 1,
//     "lazy": 1,
//     "dog": 1,
//     "is": 1,
// }
// 총 단어 수: 13
```

</details>
