### Unsafe Rust

> **학습 내용:** `unsafe`를 언제 어떻게 사용하는지 배웁니다 — 원시 포인터(raw pointer) 역참조, Rust에서 C를 호출하거나 그 반대의 경우를 위한 FFI(Foreign Function Interface), 문자열 상호 운용을 위한 `CString`/`CStr`, 그리고 unsafe 코드 주위에 안전한 래퍼(wrapper)를 작성하는 방법을 다룹니다.

- ```unsafe```는 Rust 컴파일러가 일반적으로 허용하지 않는 기능들에 대한 접근을 허용합니다.
    - 원시 포인터 역참조
    - *가변(mutable)* 정적 변수 접근
    - https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html 참조
- 큰 힘에는 큰 책임이 따릅니다.
    - ```unsafe```는 컴파일러에게 "나 프로그래머가 컴파일러가 보통 보장하는 불변성(invariants)을 유지할 책임을 지겠다"고 말하는 것과 같습니다.
    - 에일리어싱(aliased)된 가변 및 불변 참조자가 없음을 보장해야 하며, 댕글링 포인터, 유효하지 않은 참조 등이 없음을 보장해야 합니다.
    - ```unsafe```의 사용은 가능한 가장 작은 범위로 제한되어야 합니다.
    - ```unsafe```를 사용하는 모든 코드에는 가정을 설명하는 "safety" 주석이 있어야 합니다.

### Unsafe Rust 예제
```rust
unsafe fn harmless() {}
fn main() {
    // Safety: 해롭지 않은 unsafe 함수를 호출합니다.
    unsafe {
        harmless();
    }
    let a = 42u32;
    let p = &a as *const u32;
    // Safety: p는 스코프 내에 남아있는 변수에 대한 유효한 포인터입니다.
    unsafe {
        println!("{}", *p);
    }
    // Safety: 안전하지 않음; 예시 목적으로만 사용됨
    let dangerous_buffer = 0xb8000 as *mut u32;
    unsafe {
        println!("이제 충돌이 발생할 것입니다!!!");
        *dangerous_buffer = 0; // 대부분의 현대적인 기기에서 SEGV가 발생합니다.
    }
}
```

### 간단한 FFI 예제 (C에서 사용되는 Rust 라이브러리 함수)

## FFI 문자열: CString 및 CStr

FFI는 *Foreign Function Interface*의 약자로, Rust가 C와 같은 다른 언어로 작성된 함수를 호출하거나 그 반대의 경우를 위해 사용하는 메커니즘입니다.

C 코드와 인터페이스할 때, Rust의 `String` 및 `&str` 타입(널 종결자가 없는 UTF-8)은 C 문자열(널 종결자가 있는 바이트 배열)과 직접 호환되지 않습니다. Rust는 이 목적으로 `std::ffi`에서 `CString`(소유형) 및 `CStr`(빌림형)을 제공합니다.

| 타입 | 대응 개념 | 사용 시기 |
|------|-------------|----------|
| `CString` | `String` (소유형) | Rust 데이터로부터 C 문자열을 생성할 때 |
| `&CStr` | `&str` (빌림형) | 외래 코드로부터 C 문자열을 받을 때 |

```rust
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

fn demo_ffi_strings() {
    // C와 호환되는 문자열 생성 (널 종결자 추가)
    let c_string = CString::new("Hello from Rust").expect("CString::new 실패");
    let ptr: *const c_char = c_string.as_ptr();

    // C 문자열을 다시 Rust로 변환 (포인터를 신뢰하므로 unsafe)
    // Safety: ptr은 유효하며 널로 종결됨 (위에서 방금 생성함)
    let back_to_rust: &CStr = unsafe { CStr::from_ptr(ptr) };
    let rust_str: &str = back_to_rust.to_str().expect("유효하지 않은 UTF-8");
    println!("{}", rust_str);
}
```

> **주의**: `CString::new()`는 입력에 중간 널 바이트(`\0`)가 포함되어 있으면 에러를 반환합니다. 항상 `Result`를 처리하십시오. 아래 FFI 예제에서 `CStr`이 광범위하게 사용되는 것을 볼 수 있습니다.

- ```FFI``` 메서드는 컴파일러가 이름을 바꾸지(mangle) 않도록 ```#[no_mangle]```로 표시해야 합니다.
- 크레이트를 정적 라이브러리로 컴파일할 것입니다.
    ```rust
    #[no_mangle] 
    pub extern "C" fn add(left: u64, right: u64) -> u64 {
        left + right
    }
    ```
- 다음 C 코드를 컴파일하고 정적 라이브러리에 링크할 것입니다.
    ```c
    #include <stdio.h>
    #include <stdint.h>
    extern uint64_t add(uint64_t, uint64_t);
    int main() {
        printf("Add returned %llu\n", add(21, 21));
    }
    ``` 

### 복잡한 FFI 예제
- 다음 예제에서는 Rust 로깅 인터페이스를 생성하고 이를 [PYTHON] 및 ```C```에 노출할 것입니다.
    - 동일한 인터페이스가 Rust와 C에서 네이티브하게 어떻게 사용될 수 있는지 볼 것입니다.
    - ```C```용 헤더 파일을 생성하기 위해 ```cbindgen```과 같은 도구를 사용하는 방법을 알아볼 것입니다.
    - ```unsafe``` 래퍼가 안전한 Rust 코드로 가는 다리 역할을 어떻게 하는지 볼 것입니다.

## 로거 헬퍼 함수
```rust
fn create_or_open_log_file(log_file: &str, overwrite: bool) -> Result<File, String> {
    if overwrite {
        File::create(log_file).map_err(|e| e.to_string())
    } else {
        OpenOptions::new()
            .write(true)
            .append(true)
            .open(log_file)
            .map_err(|e| e.to_string())
    }
}

fn log_to_file(file_handle: &mut File, message: &str) -> Result<(), String> {
    file_handle
        .write_all(message.as_bytes())
        .map_err(|e| e.to_string())
}
```

## 로거 구조체
```rust
struct SimpleLogger {
    log_level: LogLevel,
    file_handle: File,
}

impl SimpleLogger {
    fn new(log_file: &str, overwrite: bool, log_level: LogLevel) -> Result<Self, String> {
        let file_handle = create_or_open_log_file(log_file, overwrite)?;
        Ok(Self {
            file_handle,
            log_level,
        })
    }

    fn log_message(&mut self, log_level: LogLevel, message: &str) -> Result<(), String> {
        if log_level as u32 <= self.log_level as u32 {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let message = format!("Simple: {timestamp} {log_level} {message}\n");
            log_to_file(&mut self.file_handle, &message)
        } else {
            Ok(())
        }
    }
}
```

## 테스트
- Rust에서 기능을 테스트하는 것은 매우 쉽습니다.
    - 테스트 메서드는 ```#[test]```로 장식되며, 컴파일된 바이너리의 일부가 아닙니다.
    - 테스트 목적으로 목(mock) 메서드를 만드는 것이 쉽습니다.
```rust
#[test]
fn testfunc() -> Result<(), String> {
    let mut logger = SimpleLogger::new("test.log", false, LogLevel::INFO)?;
    logger.log_message(LogLevel::TRACELEVEL1, "Hello world")?;
    logger.log_message(LogLevel::CRITICAL, "Critical message")?;
    Ok(()) // 여기서 컴파일러가 자동으로 logger를 드롭합니다.
}
```
```bash
cargo test
```

## (C)-Rust FFI
- cbindgen은 내보낸 Rust 함수에 대한 헤더 파일을 생성하는 데 유용한 도구입니다.
    - cargo를 사용하여 설치할 수 있습니다.
```bash
cargo install cbindgen
cbindgen 
```
- 함수와 구조체는 ```#[no_mangle]``` 및 ```#[repr(C)]```를 사용하여 내보낼 수 있습니다.
    - 실제 구현에 대한 `**`를 전달하고 성공 시 0, 에러 시 0이 아닌 값을 반환하는 일반적인 인터페이스 패턴을 가정하겠습니다.
    - **불투명(Opaque) 대 투명(Transparent) 구조체**: 우리의 `SimpleLogger`는 *불투명 포인터*(`*mut SimpleLogger`)로 전달됩니다 — C 쪽에서는 그 필드에 절대 접근하지 않으므로 `#[repr(C)]`가 **필요하지 않습니다**. C 코드가 구조체 필드를 직접 읽거나 써야 할 때만 `#[repr(C)]`를 사용하십시오.

```rust
// 불투명(Opaque) — C는 포인터만 보유하며 필드를 검사하지 않음. #[repr(C)] 불필요.
struct SimpleLogger { /* Rust 전용 필드들 */ }

// 투명(Transparent) — C가 필드를 직접 읽고 씀. 반드시 #[repr(C)]를 사용해야 함.
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
```
```c
typedef struct SimpleLogger SimpleLogger;
uint32_t create_simple_logger(const char *file_name, struct SimpleLogger **out_logger);
uint32_t log_entry(struct SimpleLogger *logger, const char *message);
uint32_t drop_logger(struct SimpleLogger *logger);
```

- 많은 상태 체크가 필요함에 유의하십시오.
- Rust가 자동으로 메모리를 해제하지 않도록 명시적으로 메모리를 누출(leak)시켜야 합니다.
```rust
#[no_mangle] 
pub extern "C" fn create_simple_logger(file_name: *const std::os::raw::c_char, out_logger: *mut *mut SimpleLogger) -> u32 {
    use std::ffi::CStr;
    // 포인터가 NULL이 아닌지 확인
    if file_name.is_null() || out_logger.is_null() {
        return 1;
    }
    // Safety: 전달된 포인터는 계약에 의해 NULL이거나 0으로 종결됩니다.
    let file_name = unsafe {
        CStr::from_ptr(file_name)
    };
    let file_name = file_name.to_str();
    // file_name에 잘못된 문자가 없는지 확인
    if file_name.is_err() {
        return 1;
    }
    let file_name = file_name.unwrap();
    // 몇 가지 기본값을 가정합니다; 실제 상황에서는 이를 전달받을 것입니다.
    let new_logger = SimpleLogger::new(file_name, false, LogLevel::CRITICAL);
    // 로거를 생성할 수 있었는지 확인
    if new_logger.is_err() {
        return 1;
    }
    let new_logger = Box::new(new_logger.unwrap());
    // 이것은 Box가 범위를 벗어날 때 드롭되는 것을 방지합니다.
    let logger_ptr: *mut SimpleLogger = Box::leak(new_logger);
    // Safety: logger는 null이 아니며 logger_ptr은 유효합니다.
    unsafe {
        *out_logger = logger_ptr;
    }
    return 0;
}
```

- ```log_entry()```에서도 비슷한 에러 체크를 수행합니다.
```rust
#[no_mangle]
pub extern "C" fn log_entry(logger: *mut SimpleLogger, message: *const std::os::raw::c_char) -> u32 {
    use std::ffi::CStr;
    if message.is_null() || logger.is_null() {
        return 1;
    }
    // Safety: message는 null이 아닙니다.
    let message = unsafe {
        CStr::from_ptr(message)
    };
    let message = message.to_str();
    // file_name에 잘못된 문자가 없는지 확인
    if message.is_err() {
        return 1;
    }
    // Safety: logger는 이전에 create_simple_logger()에 의해 생성된 유효한 포인터입니다.
    unsafe {
        (*logger).log_message(LogLevel::CRITICAL, message.unwrap()).is_err() as u32
    }
}

#[no_mangle]
pub extern "C" fn drop_logger(logger: *mut SimpleLogger) -> u32 {
    if logger.is_null() {
        return 1;
    }
    // Safety: logger는 이전에 create_simple_logger()에 의해 생성된 유효한 포인터입니다.
    unsafe {
        // 이것은 Box<SimpleLogger>를 재구성하며, 범위를 벗어날 때 드롭됩니다.
        let _ = Box::from_raw(logger);
    }
    0
}
```

- Rust를 사용하거나 (C) 프로그램을 작성하여 (C)-FFI를 테스트할 수 있습니다.
```rust
#[test]
fn test_c_logger() {
    // c".."는 NULL로 종결된 문자열을 생성합니다.
    let file_name = c"test.log".as_ptr() as *const std::os::raw::c_char;
    let mut c_logger: *mut SimpleLogger = std::ptr::null_mut();
    assert_eq!(create_simple_logger(file_name, &mut c_logger), 0);
    // 이것은 c"..." 문자열을 생성하는 수동적인 방법입니다.
    let message = b"message from C\0".as_ptr() as *const std::os::raw::c_char;
    assert_eq!(log_entry(c_logger, message), 0);
    drop_logger(c_logger);
}
```
```c
#include "logger.h"
...
int main() {
    SimpleLogger *logger = NULL;
    if (create_simple_logger("test.log", &logger) == 0) {
        log_entry(logger, "Hello from C");
        drop_logger(logger); /* 핸들을 닫는 등의 작업에 필요함 */
    } 
    ...
}
```

## unsafe 코드의 정확성 보장
- 요약하자면 ```unsafe```를 사용하는 데는 신중한 생각이 필요합니다.
    - 항상 코드에 의해 만들어진 안전 가정을 문서화하고 전문가와 함께 검토하십시오.
    - 정확성을 확인하는 데 도움이 되는 cbindgen, Miri, Valgrind와 같은 도구를 사용하십시오.
    - **FFI 경계를 가로질러 패닉이 전파되도록 두지 마십시오** — 이것은 정의되지 않은 동작(UB)입니다. FFI 진입점에서 `std::panic::catch_unwind`를 사용하거나, 프로필에서 `panic = "abort"`를 설정하십시오.
    - 구조체가 FFI를 통해 공유된다면, C와 호환되는 메모리 레이아웃을 보장하기 위해 `#[repr(C)]`를 표시하십시오.
    - https://doc.rust-lang.org/nomicon/intro.html (unsafe Rust의 어두운 예술을 다루는 "Rustonomicon")을 참고하십시오.
    - 내부 전문가의 도움을 받으십시오.

### 검증 도구: Miri 대 Valgrind

C++ 개발자들은 Valgrind와 새니타이저(sanitizers)에 익숙할 것입니다. Rust에는 이들에 더해 Rust 전용 UB를 훨씬 더 정확하게 잡아내는 **Miri**가 있습니다.

| | **Miri** | **Valgrind** | **C++ 새니타이저 (ASan/MSan/UBSan)** |
|---|---------|-------------|--------------------------------------|
| **탐지 항목** | Rust 전용 UB: 스택 빌림(stacked borrows), 유효하지 않은 `enum` 판별자, 초기화되지 않은 읽기, 에일리어싱 위반 | 메모리 누수, 해제 후 사용, 유효하지 않은 읽기/쓰기, 초기화되지 않은 메모리 | 버퍼 오버플로, 해제 후 사용, 데이터 경합, UB |
| **작동 방식** | MIR(Rust의 중간 표현) 해석 — 네이티브 실행 아님 | 실행 시 컴파일된 바이너리 계측 | 컴파일 시 계측 |
| **FFI 지원** | ❌ FFI 경계를 넘을 수 없음 (C 호출 건너뜀) | ✅ FFI를 포함한 모든 컴파일된 바이너리에서 작동 | ✅ C 코드도 새니타이저로 컴파일된 경우 작동 |
| **속도** | 네이티브보다 ~100배 느림 | ~10-50배 느림 | ~2-5배 느림 |
| **사용 시기** | 순수 Rust `unsafe` 코드, 데이터 구조 불변성 확인 | FFI 코드, 전체 바이너리 통합 테스트 | FFI의 C/C++ 쪽, 성능에 민감한 테스트 |
| **에일리어싱 버그 탐지** | ✅ Stacked Borrows 모델 | ❌ | 부분적 (데이터 경합에 대해 TSan 사용) |

**권장 사항**: 순수 Rust unsafe에는 Miri를, FFI 통합에는 Valgrind를 사용하는 **두 가지 모두**를 활용하십시오.

- **Miri** — Valgrind가 볼 수 없는 Rust 전용 UB(에일리어싱 위반, 유효하지 않은 enum 값, 스택 빌림)를 잡아냅니다:
    ```bash
    rustup +nightly component add miri
    cargo +nightly miri test                    # Miri에서 모든 테스트 실행
    cargo +nightly miri test -- test_name       # 특정 테스트 실행
    ```
    > ⚠️ Miri는 nightly 버전이 필요하며 FFI 호출을 실행할 수 없습니다. unsafe Rust 로직을 테스트 가능한 단위로 격리하십시오.

- **Valgrind** — 이미 익숙한 도구로, FFI를 포함한 컴파일된 바이너리에서 작동합니다:
    ```bash
    sudo apt install valgrind
    cargo install cargo-valgrind
    cargo valgrind test                         # Valgrind에서 모든 테스트 실행
    ```
    > FFI 코드에서 흔히 발생하는 `Box::leak` / `Box::from_raw` 패턴의 누수를 잡아냅니다.

- **cargo-careful** — 일반 테스트와 Miri 사이의 수준에서 추가적인 런타임 체크를 활성화하여 테스트를 실행합니다:
    ```bash
    cargo install cargo-careful
    cargo +nightly careful test
    ```

## Unsafe Rust 요약
- ```cbindgen```은 (C) FFI를 Rust로 연결하는 데 유용한 도구입니다.
    - 반대 방향의 FFI 인터페이스에는 ```bindgen```을 사용하십시오 (광범위한 문서를 참고하십시오).
- **작성한 unsafe 코드가 정확하다거나, 안전한 Rust에서 사용하기에 괜찮다고 가정하지 마십시오. 실수하기 매우 쉽고, 겉보기에 올바르게 작동하는 코드라도 미묘한 이유로 틀릴 수 있습니다.**
    - 도구를 사용하여 정확성을 검증하십시오.
    - 여전히 의심스럽다면 전문가의 조언을 구하십시오.
- ```unsafe``` 코드에는 가정과 그것이 왜 정확한지에 대한 명시적인 문서 주석이 있는지 확인하십시오.
    - ```unsafe``` 코드의 호출자 역시 안전에 대한 해당 주석이 있어야 하며 제한 사항을 준수해야 합니다.

# 연습 문제: 안전한 FFI 래퍼 작성

🔴 **도전** — unsafe 블록, 원시 포인터, 안전한 API 설계에 대한 이해가 필요함

- `unsafe` FFI 스타일 함수 주위에 안전한 Rust 래퍼를 작성하십시오. 이 연습은 호출자가 제공한 버퍼에 포맷된 문자열을 쓰는 C 함수를 호출하는 상황을 시뮬레이션합니다.
- **1단계**: 원시 `*mut u8` 버퍼에 인사말을 쓰는 unsafe 함수 `unsafe_greet`를 구현하십시오.
- **2단계**: `Vec<u8>` 버퍼를 할당하고, unsafe 함수를 호출하고, `String`을 반환하는 안전한 래퍼 `safe_greet`를 작성하십시오.
- **3단계**: 모든 unsafe 블록에 적절한 `// Safety:` 주석을 추가하십시오.

**시작 코드:**
```rust
use std::fmt::Write as _;

/// C 함수를 시뮬레이션함: 버퍼에 "Hello, <name>!"을 씀.
/// 쓴 바이트 수를 반환함 (널 종결자 제외).
/// # Safety
/// - `buf`는 적어도 `buf_len` 바이트의 쓰기 가능한 영역을 가리켜야 함
/// - `name`은 유효한 널 종결 C 문자열 포인터여야 함
unsafe fn unsafe_greet(buf: *mut u8, buf_len: usize, name: *const u8) -> isize {
    // TODO: 인사말 생성, buf에 바이트 복사, 길이 반환
    // 힌트: std::ffi::CStr::from_ptr를 사용하거나 바이트를 수동으로 반복함
    todo!()
}

/// 안전한 래퍼 — 공개 API에 unsafe가 없음
fn safe_greet(name: &str) -> Result<String, String> {
    // TODO: Vec<u8> 버퍼 할당, 널로 종결된 name 생성,
    // Safety 주석과 함께 unsafe 블록 내에서 unsafe_greet 호출,
    // 결과를 다시 String으로 변환
    todo!()
}

fn main() {
    match safe_greet("Rustacean") {
        Ok(msg) => println!("{msg}"),
        Err(e) => eprintln!("에러: {e}"),
    }
    // 예상 출력: Hello, Rustacean!
}
```

<details><summary>풀이 (클릭하여 확장)</summary>

```rust
use std::ffi::CStr;

/// C 함수를 시뮬레이션함: 버퍼에 "Hello, <name>!"을 씀.
/// 쓴 바이트 수를 반환하거나, 버퍼가 너무 작으면 -1을 반환함.
/// # Safety
/// - `buf`는 적어도 `buf_len` 바이트의 쓰기 가능한 영역을 가리켜야 함
/// - `name`은 유효한 널 종결 C 문자열 포인터여야 함
unsafe fn unsafe_greet(buf: *mut u8, buf_len: usize, name: *const u8) -> isize {
    // Safety: 호출자가 name이 유효한 널 종결 문자열임을 보장함
    let name_cstr = unsafe { CStr::from_ptr(name as *const std::os::raw::c_char) };
    let name_str = match name_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };
    let greeting = format!("Hello, {}!", name_str);
    if greeting.len() > buf_len {
        return -1;
    }
    // Safety: buf는 적어도 buf_len 바이트의 쓰기 가능한 영역을 가리킴 (호출자 보장)
    unsafe {
        std::ptr::copy_nonoverlapping(greeting.as_ptr(), buf, greeting.len());
    }
    greeting.len() as isize
}

/// 안전한 래퍼 — 공개 API에 unsafe가 없음
fn safe_greet(name: &str) -> Result<String, String> {
    let mut buffer = vec![0u8; 256];
    // C API를 위해 널로 종결된 name 버전 생성
    let name_with_null: Vec<u8> = name.bytes().chain(std::iter::once(0)).collect();

    // Safety: buffer는 256 바이트의 쓰기 가능한 영역을 가지며, name_with_null은 널로 종결됨
    let bytes_written = unsafe {
        unsafe_greet(buffer.as_mut_ptr(), buffer.len(), name_with_null.as_ptr())
    };

    if bytes_written < 0 {
        return Err("버퍼가 너무 작거나 이름이 유효하지 않습니다".to_string());
    }

    String::from_utf8(buffer[..bytes_written as usize].to_vec())
        .map_err(|e| format!("유효하지 않은 UTF-8: {e}"))
}

fn main() {
    match safe_greet("Rustacean") {
        Ok(msg) => println!("{msg}"),
        Err(e) => eprintln!("에러: {e}"),
    }
}
// 출력:
// Hello, Rustacean!
```

</details>
