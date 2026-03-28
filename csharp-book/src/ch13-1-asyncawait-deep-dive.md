# 비동기 심층 분석: C# Task vs Rust Future

> **학습 목표:** C#의 `Task`와 Rust의 `Future`가 어떻게 다른지 근본적인 설계 철학을 파헤칩니다. Rust의 **지연 실행(Lazy)** 모델을 이해하고, 외부 런타임인 **Tokio**를 활용해 고성능 비동기 애플리케이션을 구축하는 법을 마스터합니다.

---

### 1. 지연 실행(Lazy) vs 즉시 실행(Eager)
이것이 C# 개발자가 Rust 비동기를 접할 때 가장 먼저 겪는 당혹감의 원인입니다.

| **비교 항목** | **C# `Task<T>`** | **Rust `Future<Output = T>`** |
| :--- | :--- | :--- |
| **시작 시점** | 생성 즉시 실행 시작 | **`.await`를 호출해야 실행 시작** |
| **런타임** | .NET CLR 내장 | 외부 라이브러리 (Tokio 등) |
| **할당** | 기본적으로 힙(Heap) 할당 | 기본적으로 스택(Stack) 할당 |
| **취소 방식** | `CancellationToken` (협력적) | **Drop** (강제적/즉각적) |

```rust
// [진짜 아무 일도 일어나지 않습니다!]
let future = do_something_async(); 

// [이때서야 비로소 실행됩니다]
let result = future.await; 
```

---

### 2. 비동기 런타임: Tokio
Rust 언어 자체에는 비동기 코드를 실행할 '엔진'이 없습니다. 따라서 **Tokio**와 같은 외부 런타임을 표준처럼 사용합니다.

- **`#[tokio::main]`**: 프로그램의 진입점을 비동기 런타임으로 감싸줍니다.
- **`tokio::spawn`**: C#의 `Task.Run`처럼 백그라운드에서 독립적인 작업을 시작합니다.

---

### 3. 취소의 미학: `select!`와 Drop
C#에서는 취소를 위해 토큰을 일일이 전달해야 하지만, Rust는 `tokio::select!`를 통해 여러 작업 중 하나가 끝나면 나머지를 **즉시 드롭(Drop)**하여 취소합니다.

```rust
tokio::select! {
    val = some_async_work() => println!("완료: {}", val),
    _ = tokio::time::sleep(Duration::from_secs(5)) => println!("타임아웃!"),
}
// 타임아웃이 먼저 발생하면 some_async_work는 즉시 중단됩니다.
```

---

### 4. `Pin`: 메모리 고정의 필요성
C#은 GC가 객체를 옮겨도 참조를 자동으로 업데이트해주지만, Rust는 GC가 없습니다. 비동기 상태 머신이 자기 자신을 가리키는 포인터를 가질 때, 데이터가 메모리에서 이동하면 대형 사고가 납니다. **`Pin`**은 이를 방지하기 위해 데이터를 특정 메모리 주소에 **못 박아두는** 역할을 합니다.

---

### 💡 실무 팁: `join_all`로 병렬 처리하기
여러 비동기 작업을 동시에 실행하고 모두 끝날 때까지 기다리고 싶다면 `futures::future::join_all`을 사용하세요. C#의 `Task.WhenAll`과 동일하게 동작하며, 모든 작업이 병렬로 실행됩니다.

