# 7. 클로저와 고계 함수 🟢

> **학습 목표:**
> - 세 가지 클로저 트레이트(`Fn`, `FnMut`, `FnOnce`)와 캡처 메커니즘을 이해합니다.
> - 클로저를 인자로 전달하거나 함수에서 반환하는 법을 익힙니다.
> - 함수형 프로그래밍 스타일의 핵심인 콤비네이터 체인과 반복자 어댑터를 배웁니다.
> - 적절한 트레이트 경계를 가진 나만의 고계 API(Higher-order API)를 설계합니다.

---

### 클로저 트레이트: Fn, FnMut, FnOnce

모든 클로저는 변수를 캡처하는 방식에 따라 다음 세 가지 트레이트 중 하나 이상을 구현합니다.

```rust
// FnOnce — 캡처한 값을 소비함 (딱 한 번만 호출 가능)
let name = String::from("Alice");
let greet = move || {
    drop(name); // name의 소유권을 가져와 소비함
};
greet(); // ✅ 성공
// greet(); // ❌ 에러: 이미 소비된 값을 또 쓸 수 없음

// FnMut — 캡처한 값을 가변으로 빌림 (여러 번 호출 가능)
let mut count = 0;
let mut increment = || {
    count += 1; // count를 가변으로 빌려서 수정함
};
increment(); // count == 1

// Fn — 캡처한 값을 불변으로 빌림 (여러 번, 동시에 호출 가능)
let display = |x: i32| println!("{x}");
display(1);
display(2);
```

#### 트레이트 계층 구조:
`Fn` : `FnMut` : `FnOnce` 순서로 상속 관계를 가집니다. 즉, `Fn`을 구현한 클로저는 자동으로 `FnMut`와 `FnOnce`도 만족합니다.

---

### 클로저를 인자로 전달하고 반환하기

```rust
// 정적 디스패치 (인라이닝 가능, 가장 빠름)
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }

// 동적 디스패치 (유연함, 약간의 오버헤드)
fn apply_dyn(f: &dyn Fn(i32) -> i32, x: i32) -> i32 { f(x) }

// 클로저 반환 (이름이 없는 타입이므로 impl Trait이나 Box 사용 필수)
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}
```

---

### 반복자 콤비네이터: Rust의 백미

명령형 루프를 함수형 스타일의 체인으로 바꾸면 가독성이 높아지고 버그가 줄어듭니다. LLVM 최적화 덕분에 성능 손실도 거의 없습니다.

```rust
let data = vec![1, 2, 3, 4, 5];

// 이디오마틱한 Rust 스타일:
let result: Vec<i32> = data.iter()
    .filter(|&&x| x % 2 == 0) // 필터링
    .map(|&x| x * x)          // 변환
    .collect();               // 수집
```

---

### With 패턴: 안전한 리소스 접근 가이드

일부 리소스(예: GPIO 핀, DB 트랜잭션)는 특정 작업 전후에 반드시 설정과 해제가 필요합니다. 사용자가 이를 깜빡하는 실수를 막기 위해 **클로저를 통해 리소스를 잠시 빌려주는** `with` 패턴을 사용합니다.

```rust
impl GpioController {
    /// 핀을 입력 모드로 설정하고 클로저 실행 후, 원래 상태로 복구함
    pub fn with_pin_input<R>(&self, pin: u8, mut f: impl FnMut(&GpioPin) -> R) -> R {
        self.set_direction(pin, Direction::In);
        let result = f(&GpioPin { pin });
        self.restore_direction(pin);
        result
    }
}

// 사용자는 설정/해제 로직을 고민할 필요가 없습니다.
gpio.with_pin_input(4, |pin| {
    pin.read()
});
```
> **With 패턴 vs RAII(Drop)**: 두 방식 모두 정리를 보장하지만, `with` 패턴은 특정 작업 블록 안에서만 리소스가 존재하도록 강제할 때 더 강력합니다.

---

### 📝 연습 문제: 고계 콤비네이터 파이프라인 ★★ (~25분)

여러 변환 과정을 체인으로 엮을 수 있는 `Pipeline` 구조체를 설계해 보세요. `.pipe(f)`로 변환을 추가하고 `.execute(input)`으로 전체 과정을 실행하는 구조입니다.

---

### 📌 요약
- **`FnMut`**를 기본 제약 조건으로 사용하세요. 가장 유연하게 호출될 수 있습니다.
- 콤비네이터 체인(`map`, `filter` 등)은 코드의 의도를 명확히 드러내며 성능 면에서도 우수합니다.
- **`with` 패턴**을 활용해 사용자가 리소스를 오용할 가능성을 컴파일 타임에 차단하세요.

