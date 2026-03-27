## Unsafe Rust

> **학습 내용:** `unsafe`가 허용하는 작업(원시 포인터, FFI, 검사되지 않은 형변환), 안전한 래퍼(safe wrapper) 패턴, 네이티브 코드 호출을 위한 C# P/Invoke와 Rust FFI 비교, 그리고 `unsafe` 블록 작성을 위한 안전 체크리스트.
>
> **난이도:** 🔴 상급

Unsafe Rust를 사용하면 빌림 검사기(borrow checker)가 확인하지 못하는 작업을 수행할 수 있습니다. 꼭 필요한 경우에만 신중하게 사용해야 하며, 명확한 문서화가 동반되어야 합니다.

> **심화 내용**: Unsafe 코드를 사용한 안전한 추상화 패턴(아레나 할당자, 락 프리 구조, 커스텀 vtable 등)에 대해서는 [Rust 패턴](../../rust-patterns-book/src/summary.md)을 참조하세요.

### Unsafe가 필요한 경우
```rust
// 1. 원시 포인터(raw pointer) 역참조
let mut value = 42;
let ptr = &mut value as *mut i32;
// 안전성(SAFETY): ptr은 유효하고 살아있는 지역 변수를 가리킵니다.
unsafe {
    *ptr = 100; // unsafe 블록 내에서만 가능
}

// 2. unsafe 함수 호출
unsafe fn dangerous() {
    // 호출자가 불변성(invariants)을 유지해야 하는 내부 구현
}

// 안전성(SAFETY): 이 예제 함수에는 유지해야 할 불변성이 없습니다.
unsafe {
    dangerous(); // 호출자가 책임을 집니다.
}

// 3. 가변 정적 변수(mutable static variables) 접근
static mut COUNTER: u32 = 0;
// 안전성(SAFETY): 단일 스레드 컨텍스트이므로 COUNTER에 대한 동시 접근이 없습니다.
unsafe {
    COUNTER += 1; // 스레드 안전하지 않음 — 호출자가 동기화를 보장해야 합니다.
}

// 4. unsafe 트레이트 구현
unsafe trait UnsafeTrait {
    fn do_something(&self);
}
```

### C#과의 비교: unsafe 키워드
```csharp
// C# unsafe - 유사한 개념이지만 범위가 다릅니다.
unsafe void UnsafeExample()
{
    int value = 42;
    int* ptr = &value;
    *ptr = 100;
    
    // C#의 unsafe는 포인터 연산에 중점을 둡니다.
    // Rust의 unsafe는 소유권/빌림 규칙의 완화에 중점을 둡니다.
}

// C# fixed - 관리되는 객체 핀 고정
unsafe void PinnedExample()
{
    byte[] buffer = new byte[100];
    fixed (byte* ptr = buffer)
    {
        // ptr은 이 블록 내에서만 유효합니다.
    }
}
```

### 안전한 래퍼 (Safe Wrappers)
```rust
/// 핵심 패턴: unsafe 코드를 안전한 API로 감싸기
pub struct SafeBuffer {
    data: Vec<u8>,
}

impl SafeBuffer {
    pub fn new(size: usize) -> Self {
        SafeBuffer { data: vec![0; size] }
    }
    
    /// 안전한 API — 경계 검사(bounds check)가 포함된 접근
    pub fn get(&self, index: usize) -> Option<u8> {
        self.data.get(index).copied()
    }
    
    /// 빠른 비검사 접근 — unsafe하지만 경계 검사를 통해 안전하게 래핑됨
    pub fn get_unchecked_safe(&self, index: usize) -> Option<u8> {
        if index < self.data.len() {
            // 안전성(SAFETY): index가 범위 내에 있음을 방금 확인했습니다.
            Some(unsafe { *self.data.get_unchecked(index) })
        } else {
            None
        }
    }
}
```

***

## FFI를 통한 C#과의 상호 운용성 (Interop)

Rust는 C#에서 P/Invoke를 통해 호출할 수 있는 C 호환 함수를 노출할 수 있습니다.

```mermaid
graph LR
    subgraph "C# 프로세스"
        CS["C# 코드"] -->|"P/Invoke"| MI["마샬링 레이어\nUTF-16 → UTF-8\n구조체 레이아웃"]
    end
    MI -->|"C ABI 호출"| FFI["FFI 경계"]
    subgraph "Rust cdylib (.so / .dll)"
        FFI --> RF["extern \"C\" fn\n#[no_mangle]"]
        RF --> Safe["안전한 Rust\n내부 로직"]
    end

    style FFI fill:#fff9c4,color:#000
    style MI fill:#bbdefb,color:#000
    style Safe fill:#c8e6c9,color:#000
```

### Rust 라이브러리 (cdylib로 컴파일)
```rust
// src/lib.rs
#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn process_string(input: *const std::os::raw::c_char) -> i32 {
    // 안전성(SAFETY): input이 널이 아님을 확인하고(내부에서), 
    // 호출자가 널 종료 문자열(null-terminated string)을 전달했다고 가정합니다.
    let c_str = unsafe {
        if input.is_null() {
            return -1;
        }
        std::ffi::CStr::from_ptr(input)
    };
    
    match c_str.to_str() {
        Ok(s) => s.len() as i32,
        Err(_) => -1,
    }
}
```

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]
```

### C# 소비자 (P/Invoke)
```csharp
using System.Runtime.InteropServices;

public static class RustInterop
{
    [DllImport("my_rust_lib", CallingConvention = CallingConvention.Cdecl)]
    public static extern int add_numbers(int a, int b);
    
    [DllImport("my_rust_lib", CallingConvention = CallingConvention.Cdecl)]
    public static extern int process_string(
        [MarshalAs(UnmanagedType.LPUTF8Str)] string input);
}

// 사용 예시
int sum = RustInterop.add_numbers(5, 3);  // 8
int len = RustInterop.process_string("Hello from C#!");  // 15
```

### FFI 안전 체크리스트

Rust 함수를 C#에 노출할 때, 흔히 발생하는 버그를 방지하기 위한 규칙들입니다:

1. **항상 `extern "C"`를 사용하세요** — 이를 생략하면 Rust는 자신의 (불안정한) 호출 규약(calling convention)을 사용합니다. C# P/Invoke는 C ABI를 기대합니다.

2. **`#[no_mangle]`** — Rust 컴파일러가 함수 이름을 맹글링(mangling)하지 않도록 방지합니다. 이게 없으면 C#에서 심볼을 찾을 수 없습니다.

3. **패닉(panic)이 FFI 경계를 넘지 않게 하세요** — Rust 패닉이 C#으로 전파(unwinding)되는 것은 **정의되지 않은 동작(undefined behavior)**입니다. FFI 진입점에서 패닉을 포착하세요:
    ```rust
    #[no_mangle]
    pub extern "C" fn safe_ffi_function() -> i32 {
        match std::panic::catch_unwind(|| {
            // 실제 로직
            42
        }) {
            Ok(result) => result,
            Err(_) => -1,  // C#으로 패닉을 보내는 대신 에러 코드 반환
        }
    }
    ```

4. **불투명 구조체(Opaque) vs 투명 구조체(Transparent)** — C#이 포인터(불투명 핸들)만 들고 있다면 `#[repr(C)]`가 필요 없습니다. 하지만 C#이 `StructLayout`을 통해 구조체 필드를 직접 읽는다면 **반드시** `#[repr(C)]`를 사용해야 합니다:
    ```rust
    // 불투명(Opaque) — C#은 IntPtr만 보유함. #[repr(C)] 필요 없음.
    pub struct Connection { /* Rust 전용 필드들 */ }

    // 투명(Transparent) — C#이 필드를 직접 마샬링함. 반드시 #[repr(C)] 사용.
    #[repr(C)]
    pub struct Point { pub x: f64, pub y: f64 }
    ```

5. **널 포인터 검사** — 포인터를 역참조하기 전에 항상 유효성을 검사하세요. C#에서 `IntPtr.Zero`를 넘길 수 있습니다.

6. **문자열 인코딩** — C#은 내부적으로 UTF-16을 사용합니다. `MarshalAs(UnmanagedType.LPUTF8Str)`는 이를 Rust의 `CStr`이 사용하는 UTF-8로 변환해 줍니다. 이러한 규약을 명시적으로 문서화하세요.

### 전체 예제: 수명 주기 관리가 포함된 불투명 핸들 (Opaque Handle)

프로덕션에서 흔히 쓰이는 패턴입니다. Rust가 객체를 소유하고, C#은 불투명 핸들을 보유하며, 명시적인 생성/파괴 함수를 통해 수명 주기를 관리합니다.

**Rust 사이드** (`src/lib.rs`):
```rust
use std::ffi::{c_char, CStr};

pub struct ImageProcessor {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

/// 새로운 프로세서를 생성합니다. 유효하지 않은 크기인 경우 널을 반환합니다.
#[no_mangle]
pub extern "C" fn processor_new(width: u32, height: u32) -> *mut ImageProcessor {
    if width == 0 || height == 0 {
        return std::ptr::null_mut();
    }
    let proc = ImageProcessor {
        width,
        height,
        pixels: vec![0u8; (width * height * 4) as usize],
    };
    Box::into_raw(Box::new(proc)) // 힙에 할당하고 원시 포인터 반환
}

/// 그레이스케일 필터를 적용합니다. 성공 시 0, 널 포인터인 경우 -1을 반환합니다.
#[no_mangle]
pub extern "C" fn processor_grayscale(ptr: *mut ImageProcessor) -> i32 {
    // 안전성(SAFETY): ptr은 Box::into_raw에 의해 생성되었으며(널 아님), 여전히 유효합니다.
    let proc = match unsafe { ptr.as_mut() } {
        Some(p) => p,
        None => return -1,
    };
    for chunk in proc.pixels.chunks_exact_mut(4) {
        let gray = (0.299 * chunk[0] as f64
                  + 0.587 * chunk[1] as f64
                  + 0.114 * chunk[2] as f64) as u8;
        chunk[0] = gray;
        chunk[1] = gray;
        chunk[2] = gray;
    }
    0
}

/// 프로세서를 파괴합니다. 널 포인터를 인자로 넣어도 안전합니다.
#[no_mangle]
pub extern "C" fn processor_free(ptr: *mut ImageProcessor) {
    if !ptr.is_null() {
        // 안전성(SAFETY): ptr은 processor_new에서 Box::into_raw를 통해 생성되었습니다.
        unsafe { drop(Box::from_raw(ptr)); }
    }
}
```

**C# 사이드**:
```csharp
using System.Runtime.InteropServices;

public sealed class ImageProcessor : IDisposable
{
    [DllImport("image_rust", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr processor_new(uint width, uint height);

    [DllImport("image_rust", CallingConvention = CallingConvention.Cdecl)]
    private static extern int processor_grayscale(IntPtr ptr);

    [DllImport("image_rust", CallingConvention = CallingConvention.Cdecl)]
    private static extern void processor_free(IntPtr ptr);

    private IntPtr _handle;

    public ImageProcessor(uint width, uint height)
    {
        _handle = processor_new(width, height);
        if (_handle == IntPtr.Zero)
            throw new ArgumentException("Invalid dimensions");
    }

    public void Grayscale()
    {
        if (processor_grayscale(_handle) != 0)
            throw new InvalidOperationException("Processor is null");
    }

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            processor_free(_handle);
            _handle = IntPtr.Zero;
        }
    }
}

// 사용 예시 — IDisposable이 Rust 메모리 해제를 보장합니다.
using var proc = new ImageProcessor(1920, 1080);
proc.Grayscale();
// proc.Dispose()가 자동으로 호출됨 → processor_free() 실행 → Rust가 Vec을 드롭함
```

> **핵심 통찰**: 이것은 C#의 `SafeHandle` 패턴에 해당하는 Rust식 방식입니다. Rust의 `Box::into_raw` / `Box::from_raw`를 통해 FFI 경계 너머로 소유권을 이전하고, C#의 `IDisposable` 래퍼를 통해 정리를 보장합니다.

---

## 연습 문제

<details>
<summary><strong>🏋️ 연습 문제: 원시 포인터를 위한 안전한 래퍼</strong> (클릭하여 펼치기)</summary>

C 라이브러리로부터 원시 포인터를 전달받았습니다. 이를 위한 안전한 Rust 래퍼를 작성해 보세요:

```rust
// 시뮬레이션된 C API
extern "C" {
    fn lib_create_buffer(size: usize) -> *mut u8;
    fn lib_free_buffer(ptr: *mut u8);
}
```

요구사항:
1. 원시 포인터를 감싸는 `SafeBuffer` 구조체를 만드세요.
2. `Drop`을 구현하여 `lib_free_buffer`를 호출하세요.
3. `as_slice()`를 통해 안전한 `&[u8]` 뷰를 제공하세요.
4. `SafeBuffer::new()`는 포인터가 널인 경우 `None`을 반환해야 합니다.

<details>
<summary>🔑 정답</summary>

```rust,ignore
struct SafeBuffer {
    ptr: *mut u8,
    len: usize,
}

impl SafeBuffer {
    fn new(size: usize) -> Option<Self> {
        // 안전성(SAFETY): lib_create_buffer는 유효한 포인터 또는 널을 반환합니다(아래에서 확인).
        let ptr = unsafe { lib_create_buffer(size) };
        if ptr.is_null() {
            None
        } else {
            Some(SafeBuffer { ptr, len: size })
        }
    }

    fn as_slice(&self) -> &[u8] {
        // 안전성(SAFETY): ptr은 널이 아니며(new에서 확인됨), 
        // len은 할당된 크기이고, 우리가 독점적인 소유권을 가집니다.
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl Drop for SafeBuffer {
    fn drop(&mut self) {
        // 안전성(SAFETY): ptr은 lib_create_buffer에 의해 할당되었습니다.
        unsafe { lib_free_buffer(self.ptr); }
    }
}

// 사용 예시: 모든 unsafe는 SafeBuffer 내부에 캡슐화됩니다.
fn process(buf: &SafeBuffer) {
    let data = buf.as_slice(); // 완전히 안전한 API
    println!("First byte: {}", data[0]);
}
```

**핵심 패턴**: `unsafe`를 작은 모듈 안에 캡슐화하고 `// 안전성(SAFETY):` 주석을 다세요. 그리고 100% 안전한 공개 API를 노출하세요. 이것이 Rust 표준 라이브러리의 작동 방식입니다 — `Vec`, `String`, `HashMap` 모두 내부적으로는 unsafe를 포함하지만 안전한 인터페이스를 제공합니다.

</details>
</details>

***
