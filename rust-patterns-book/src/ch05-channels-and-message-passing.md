# 5. 채널과 메시지 패싱 🟢

> **학습 목표:**
> - `std::sync::mpsc`의 기초와 `crossbeam-channel`로 전환해야 하는 시점을 이해합니다.
> - 여러 소스의 메시지를 대기하는 `select!` 매크로 사용법을 익힙니다.
> - 유한(Bounded) vs 무한(Unbounded) 채널의 차이와 **백프레셔(Backpressure)** 전략을 배웁니다.
> - 가변 상태를 안전하게 캡슐화하는 **액터(Actor) 패턴**을 학습합니다.

---

### std::sync::mpsc — 표준 라이브러리 채널

Rust 표준 라이브러리는 여러 생산자(Multi-producer), 단일 소비자(Single-consumer) 구조의 채널을 제공합니다.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // 송신기(tx)와 수신기(rx) 쌍 생성
    let (tx, rx) = mpsc::channel();

    // 두 개의 생산자 스레드 스폰
    for i in 0..2 {
        let tx = tx.clone(); // 송신기를 복제로 여러 스레드에 전달
        thread::spawn(move || {
            tx.send(format!("생산자 {}의 메시지", i)).unwrap();
        });
    }

    // 소비자: 모든 송신기가 드롭될 때까지 메시지 수신
    for msg in rx {
        println!("수신: {msg}");
    }
}
```

**주요 특징**:
- **기본적으로 무한(Unbounded)**: 소비자가 느리면 메모리가 계속 차오를 위험이 있습니다.
- **백프레셔 지원**: `mpsc::sync_channel(N)`은 크기가 고정된 채널을 만들어, 채널이 가득 차면 생산자를 블록(Block)시킵니다.

---

### crossbeam-channel — 실무용 워크호스

실제 운영 환경에서는 표준 라이브러리보다 더 빠르고 기능이 많은 `crossbeam-channel`이 사실상의 표준으로 쓰입니다. 특히 여러 소비자(MPMC)를 지원하는 것이 큰 장점입니다.

```rust
// Bounded MPMC 채널 (용량 100)
let (tx, rx) = crossbeam_channel::bounded::<String>(100);

// 송신기와 수신기를 모두 .clone() 하여 여러 스레드에서 공유 가능
let tx2 = tx.clone();
let rx2 = rx.clone();
```

---

### 채널 선택 (select!)

Go 언어의 `select`와 유사하게, 여러 채널 중 하나라도 메시지가 준비되면 즉시 처리하도록 로직을 짤 수 있습니다.

```rust
loop {
    select! {
        recv(worker_rx) -> msg => println!("작업 처리 중: {:?}", msg),
        recv(ticker) -> _ => println!("1초 경과 (하트비트)"),
        recv(deadline) -> _ => {
            println!("타임아웃 — 종료");
            break;
        }
    }
}
```
> **Go 언어와의 비교**: `crossbeam`의 `select!` 매크로는 특정 채널의 기아 상태(Starvation)를 방지하기 위해 Go처럼 무작위 순서로 채널을 검사합니다.

---

### 유한(Bounded) vs 무한(Unbounded) 채널

| 유형 | 채널이 가득 찼을 때 | 메모리 사용 | 권장 용도 |
| :--- | :--- | :--- | :--- |
| **무한 (Unbounded)** | 절대 블록되지 않음 (힙 증가) | 무한 사용 가능 ⚠️ | 생산자가 확실히 소비자보다 느릴 때 |
| **유한 (Bounded)** | `send()`가 공간이 생길 때까지 대기 | 고정됨 (안전) | **실무 권장** — OOM 방지 및 백프레셔 제공 |
| **랑데뷰 (bounded(0))** | 소비자가 받을 준비가 되어야 전송 | 없음 | 스레드 간의 즉각적인 핸드오프 및 동기화 |

---

### 채널을 활용한 액터(Actor) 패턴

액터 패턴은 공유되는 가변 상태를 **뮤텍스 없이** 관리하는 훌륭한 방법입니다. 메시지 전송을 통해 순차적으로 상태를 변경하므로 경쟁 조건(Race condition)이 발생하지 않습니다.

```rust
// Counter에 대한 접근을 메시지로 직렬화(Serialize)
enum CounterMsg {
    Increment,
    Get(mpsc::Sender<i64>), // 결과를 돌려받을 채널 포함
}

// 스레드 여러 개가 하나의 Counter 핸들에 메시지를 보내면, 
// 액터 내부 루프가 하나씩 차례로 처리합니다.
```
> **뮤텍스 vs 액터**: 작업 시간이 길거나 잠금 순서(Lock ordering)를 고민하기 싫을 때 액터 패턴이 빛을 발합니다. 단순한 상태 변경은 뮤텍스가 더 효율적일 수 있습니다.

---

### 📝 연습 문제: 채널 기반 워크 풀(Worker Pool) ★★★ (~45분)

채널을 사용하여 다음 기능을 구현해 보세요:
- 디스패처가 작업을 채널로 보냅니다.
- N개의 워커(Worker) 스레드가 작업을 수신하여 처리하고 결과를 별도 채널로 보냅니다.
- `Arc<Mutex<Receiver>>`를 사용하여 워커 간의 작업 훔치기(Work-stealing)를 구현해 보세요.

---

### 📌 요약
- **`crossbeam-channel`**은 표준 라이브러리보다 강력하며 멀티 소비자(MPMC)를 지원합니다.
- **`select!`** 매크로를 통해 복잡한 폴링 로직을 선언적인 채널 선택으로 바꿀 수 있습니다.
- 운영 서버에서는 메모리 안전을 위해 항상 **유한(Bounded) 채널**을 먼저 고려하세요.

