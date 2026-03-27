# 비동기 Rust: 퓨처(Futures)에서 운영 환경까지

## 저자 소개

- Microsoft SCHIE(Silicon and Cloud Hardware Infrastructure Engineering) 팀의 수석 펌웨어 아키텍트
- 보안, 시스템 프로그래밍(펌웨어, 운영체제, 하이퍼바이저), CPU 및 플랫폼 아키텍처, C++ 시스템 분야의 업계 전문가
- 2017년(@AWS EC2)부터 Rust로 프로그래밍을 시작했으며, 그 이후로 이 언어와 사랑에 빠짐

---

Rust의 비동기 프로그래밍에 대한 심층 가이드입니다. `tokio::main`으로 시작해서 내부 구조를 대충 넘어가는 대부분의 비동기 튜토리얼과 달리, 이 가이드는 `Future` 트레이트, 폴링(polling), 상태 머신(state machines)과 같은 기본 원칙부터 이해를 쌓아 올립니다. 그 다음 실세계의 패턴, 런타임 선택, 그리고 운영 환경에서의 함정들로 나아갑니다.

## 대상 독자
- 동기식 Rust는 작성할 수 있지만 비동기 방식이 혼란스러운 Rust 개발자
- C#, Go, Python 또는 JavaScript 배경이 있어 `async/await`는 알지만 Rust의 모델은 모르는 개발자
- `Future is not Send`, `Pin<Box<dyn Future>>` 오류를 겪었거나 "왜 내 프로그램이 멈추지?"라는 의문을 가진 모든 분

## 사전 요구 사항

다음 개념들에 익숙해야 합니다:
- 소유권(Ownership), 빌려오기(borrowing) 및 수명(lifetimes)
- 트레이트(Traits) 및 제네릭(generics) (`impl Trait` 포함)
- `Result<T, E>` 및 `?` 연산자 사용
- 기초적인 멀티스레딩 (`std::thread::spawn`, `Arc`, `Mutex`)

사전 비동기 Rust 경험은 필요하지 않습니다.

## 이 책을 사용하는 법

**처음에는 순차적으로 읽으세요.** 파트 I~III는 서로를 기반으로 구축됩니다. 각 장에는 다음 기호가 표시되어 있습니다:

| 기호 | 의미 |
|--------|---------|
| 🟢 | 초급 — 기초 개념 |
| 🟡 | 중급 — 이전 장의 지식 필요 |
| 🔴 | 고급 — 심층 내부 구조 또는 운영 패턴 |

각 장에는 다음 내용이 포함됩니다:
- 상단의 **"학습 내용"** 블록
- 시각적 학습자를 위한 **Mermaid 다이어그램**
- 해설이 숨겨진 **인라인 연습 문제**
- 핵심 아이디어를 요약한 **핵심 요약(Key Takeaways)**
- 관련 장에 대한 **상호 참조(Cross-references)**

## 학습 진도 가이드

| 장 | 주제 | 권장 시간 | 체크포인트 |
|----------|-------|----------------|------------|
| 1–5 | 비동기 작동 원리 | 6–8 시간 | `Future`, `Poll`, `Pin`을 설명할 수 있고, 왜 Rust에 내장 런타임이 없는지 이해함 |
| 6–10 | 생태계 | 6–8 시간 | 퓨처를 수동으로 구현하고, 런타임을 선택하며, tokio의 API를 사용할 수 있음 |
| 11–13 | 운영 환경의 비동기 | 6–8 시간 | 스트림, 적절한 에러 처리, 우아한 종료(graceful shutdown)를 포함한 운영 수준의 비동기 코드를 작성할 수 있음 |
| 캡스톤 | 채팅 서버 | 4–6 시간 | 모든 개념을 통합하여 실제 비동기 애플리케이션을 구축함 |

**총 예상 시간: 22–30 시간**

## 연습 문제 풀이

모든 본문 장에는 인라인 연습 문제가 있습니다. 캡스톤(16장)은 모든 내용을 하나의 프로젝트로 통합합니다. 학습 효과를 극대화하려면:

1. **해설을 보기 전에 직접 풀어보세요.** 고민하는 과정에서 학습이 일어납니다.
2. **코드를 직접 타이핑하세요. 복사해서 붙여넣지 마세요.** Rust의 문법은 근육 기억이 중요합니다.
3. **모든 예제를 실행해 보세요.** `cargo new async-exercises`를 만들고 진행하면서 테스트하세요.

## 목차

### 파트 I: 비동기 작동 원리

- [1. 왜 Rust의 비동기는 다른가요?](ch01-why-async-is-different-in-rust.md) 🟢 — 근본적인 차이점: Rust에는 내장 런타임이 없습니다.
- [2. Future 트레이트](ch02-the-future-trait.md) 🟡 — `poll()`, `Waker`, 그리고 모든 것을 작동하게 만드는 계약
- [3. Poll의 작동 원리](ch03-how-poll-works.md) 🟡 — 폴링 상태 머신과 최소한의 실행기(executor)
- [4. Pin과 Unpin](ch04-pin-and-unpin.md) 🔴 — 왜 자기 참조 구조체(self-referential structs)에 피닝(pinning)이 필요한가
- [5. 상태 머신의 실체](ch05-the-state-machine-reveal.md) 🟢 — 컴파일러가 `async fn`으로부터 실제로 생성하는 것

### 파트 II: 생태계

- [6. 수동으로 Future 구현하기](ch06-building-futures-by-hand.md) 🟡 — 처음부터 구현해보는 TimerFuture, Join, Select
- [7. 실행기와 런타임](ch07-executors-and-runtimes.md) 🟡 — tokio, smol, async-std, embassy — 선택 방법
- [8. Tokio 심층 분석](ch08-tokio-deep-dive.md) 🟡 — 런타임 종류, spawn, 채널, 동기화 기본 요소
- [9. Tokio가 적합하지 않은 경우](ch09-when-tokio-isnt-the-right-fit.md) 🟡 — LocalSet, FuturesUnordered, 런타임 중립적 설계
- [10. 비동기 트레이트 (Async Traits)](ch10-async-traits.md) 🟡 — RPITIT, dyn dispatch, trait_variant, 비동기 클로저

### 파트 III: 운영 환경의 비동기

- [11. 스트림과 AsyncIterator](ch11-streams-and-asynciterator.md) 🟡 — 비동기 반복(iteration), AsyncRead/Write, 스트림 결합기(combinators)
- [12. 흔히 발생하는 함정들](ch12-common-pitfalls.md) 🔴 — 9가지 운영 버그와 이를 방지하는 방법
- [13. 운영 패턴](ch13-production-patterns.md) 🔴 — 우아한 종료(Graceful shutdown), 백프레셔(backpressure), Tower 미들웨어

### 부록

- [요약 및 참조 카드](ch15-summary-and-reference-card.md) — 빠른 조회를 위한 표 및 의사 결정 트리
- [캡스톤 프로젝트: 비동기 채팅 서버](ch16-capstone-project.md) — 완전한 비동기 애플리케이션 구축

***
