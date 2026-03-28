# Unsafe Rust와 FFI: 시스템의 심연으로

> **학습 목표:** Rust의 안전 보호장치를 잠시 해제하는 **`unsafe`**의 용도와 위험성을 이해합니다. C#의 P/Invoke와 대응하는 **FFI(Foreign Function Interface)**를 통해 Rust 코드를 C#에서 호출하거나, 반대로 네이티브 라이브러리를 사용하는 실전 패턴을 익힙니다.

---

### 1. `unsafe`는 '위험'이 아니라 '약속'입니다
`unsafe` 블록은 컴파일러에게 **"이 부분의 안전성은 내가 직접 책임질 테니 간섭하지 마"**라고 약속하는 것입니다. 주로 다음과 같은 경우에 사용합니다.

- **원시 포인터(Raw Pointer) 역참조**: `*const T`, `*mut T`를 사용해 메모리 주소에 직접 접근할 때
- **FFI 호출**: 다른 언어(C, C++)로 작성된 함수를 호출할 때
- **가변 정적 변수 접근**: 전역 상태를 수정할 때
- **안전하지 않은 트레이트 구현**: 컴파일러가 검증할 수 없는 속성을 보장할 때

---

### 2. 안전한 래퍼(Safe Wrapper) 패턴
Rust 표준 라이브러리의 철학입니다. 내부적으로는 성능이나 시스템 접근을 위해 `unsafe`를 쓰더라도, 사용자에게는 절대 사고가 날 수 없는 **안전한 인터페이스**만 노출합니다.

```rust
pub struct MyBuffer {
    ptr: *mut u8,
    len: usize,
}

impl MyBuffer {
    pub fn get(&self, index: usize) -> Option<u8> {
        if index < self.len {
            // 내부에서만 신중하게 unsafe 사용
            Some(unsafe { *self.ptr.add(index) })
        } else {
            None
        }
    }
}
```

---

### 3. FFI: C#과 Rust의 만남
Rust로 고성능 코드를 짜고, 이를 C#에서 P/Invoke로 불러 쓰는 것은 매우 흔한 패턴입니다.

- **`#[no_mangle]`**: 컴파일러가 함수 이름을 바꾸지 않도록 고정합니다.
- **`extern "C"`**: 표준 C 호출 규약을 따르도록 합니다.
- **`#[repr(C)]`**: 구조체의 메모리 레이아웃을 C와 호환되게 맞춥니다.

```rust
// [Rust 코드]
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 { a + b }
```

```csharp
// [C# 코드]
[DllImport("my_rust_lib")]
public static extern int add(int a, int b);
```

---

### 💡 실무 팁: 패닉(Panic) 주의보!
Rust 코드에서 발생한 **패닉이 FFI 경계를 넘어가 C#으로 전달되면 정의되지 않은 동작(UB)**이 발생해 프로그램이 즉시 뻗을 수 있습니다. FFI 함수 내부에서는 반드시 `catch_unwind`를 사용하거나, 에러 코드를 반환하는 식으로 설계하여 패닉이 밖으로 새 나가지 않게 막아야 합니다.

