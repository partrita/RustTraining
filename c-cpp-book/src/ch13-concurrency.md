# 13. 두려움 없는 동시성 (Fearless Concurrency) 🔴

> **학습 목표:**
> - Rust의 동시성 모델인 **스레드(Threads)**, **`Send`와 `Sync`** 마커 트레이트의 역할을 이해합니다.
> - **`Arc<T>`**와 **`Mutex<T>`**를 조합해 안전하게 데이터를 공유하는 법을 익힙니다.
> - **채널(Channels)**을 통한 메시지 패싱(Message Passing) 방식을 배웁니다.
> - 컴파일러가 어떻게 **데이터 경합(Data Race)**을 원천 봉쇄하는지 파악하고, 성능 저하 없이 안전하게 멀티스레딩 프로그램을 작성합니다.

---

### Rust 동시성 철학: "컴파일 타임에 잡는 버그"
C++에서는 여러 스레드가 하나의 `std::vector`를 뮤텍스 없이 동시에 수정하는 실수를 해도 컴파일러가 잡아주지 않으며, 이는 런타임에 심각한 '정의되지 않은 동작(UB)'으로 이어집니다. 반면 Rust는 소유권 규칙을 스레드 경계까지 확장하여, **안전하지 않은 공유는 아예 빌드 자체가 되지 않도록** 설계되었습니다.

### 1. 기본 스레드 생성 (`thread::spawn`)
`thread::spawn`은 새로운 OS 스레드를 만들고 클로저를 병렬로 실행합니다.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("하위 스레드: 작업 중 {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("메인 스레드: 진행 중 {i}");
        thread::sleep(Duration::from_millis(1));
    }

    // 하위 스레드가 끝날 때까지 대기 (C++의 join()과 동일)
    handle.join().expect("스레드 실행 중 에러 발생");
}
```

---

### 2. 데이터 공유와 접근 제어 (`Arc`와 `Mutex`)
멀티스레드 환경에서 하나의 데이터를 여럿이서 쓰고 싶을 때 사용합니다.

- **`Arc<T>` (Atomic Reference Counted)**: 여러 스레드가 데이터를 '공동 소유'할 수 있게 해주는 원자적 참조 카운터입니다. (C++의 `std::shared_ptr`와 유사하지만 스레드 간 안전성이 보장됨)
- **`Mutex<T>`**: 데이터를 보호하여 한 번에 하나의 스레드만 접근하도록 강제합니다. Rust의 뮤텍스는 데이터를 **감싸고(Wrap)** 있으므로, **잠금(Lock)을 획득하지 않고서는 데이터에 접근이 기술적으로 불가능**합니다.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // 잠금 획득 (반드시 거쳐야 함)
            *num += 1; 
        }); // 가드(Guard)가 드롭되면서 잠금이 자동으로 해제됨 (RAII)
        handles.push(handle);
    }

    for handle in handles { handle.join().unwrap(); }
    println!("최종 결과: {}", *counter.lock().unwrap());
}
```

---

### 3. 메시지 패싱: 채널 (Channels)
"메모리를 공유해서 소통하지 말고, 소통해서 메모리를 공유하라"는 철학입니다.

- **`mpsc` (Multi-producer, Single-consumer)**: 여러 송신자(`Sender`)가 하나의 수신자(`Receiver`)에게 메시지를 보냅니다.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let msg = String::from("작업 완료 알림");
        tx.send(msg).unwrap(); 
        // 소유권이 채널로 넘어갔으므로 여기서 msg를 다시 쓸 수 없음 (안전!)
    });

    let received = rx.recv().unwrap(); // 데이터가 올 때까지 블록
    println!("받은 메시지: {received}");
}
```

---

### 💡 스레드 안전의 핵심: `Send`와 `Sync`
Rust 컴파일러는 두 가지 '마커 트레이트'를 사용해 스레드 안전성을 판단합니다.
1.  **`Send`**: 데이터의 소유권을 다른 스레드로 **넘길 수 있음**을 의미합니다.
2.  **`Sync`**: 여러 스레드에서 참조(`&T`)를 통해 **동시 접근해도 안전함**을 의미합니다.

> **참고**: `Rc<T>`는 스레드 안전하지 않은 참조 카운터를 쓰므로 `Send`와 `Sync`가 없습니다. 이를 스레드 간에 넘기려 하면 즉시 **컴파일 에러**가 발생하여 버그를 사전에 차단합니다.

---

### 📊 C++ 대비 주요 차이점

| **기능** | **C++** | **Rust** | **이점** |
| :--- | :--- | :--- | :--- |
| **데이터 경합** | 개발자의 주의 필요 (런타임 UB) | **컴파일러가 원천 차단** | 100% 안전성 보장 |
| **뮤텍스 설계** | 데이터와 뮤텍스가 분리됨 | **데이터가 뮤텍스 안에 캡슐화됨** | 실수로 잠금 없이 접근할 수 없음 |
| **참조 카운팅** | `std::shared_ptr` (복잡함) | `Rc`(단일) / **`Arc`(멀티스레드)** | 용도에 따른 명확한 구분 |
| **메시지 패싱** | 표준에 없음 (직접 구현/라이브러리) | **`mpsc` 모듈 기본 제공** | 현대적인 동시성 패턴 장려 |

---

### 📌 요약
- **`thread::spawn`**으로 스레드를 생성하고 **`join()`**으로 기다립니다.
- 공유 데이터는 **`Arc<Mutex<T>>`** 조합이 정석입니다.
- **채널**을 통한 메시지 패싱은 소유권 이동을 활용해 안전하게 데이터를 전달합니다.
- **`Send`와 `Sync`** 트레이트 덕분에 데이터 경합 걱정 없이 코딩할 수 있습니다.

