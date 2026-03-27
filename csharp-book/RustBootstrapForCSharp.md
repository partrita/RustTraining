# C# 개발자를 위한 Rust 부트스트랩 (Rust Bootstrap for C# Developers)

C# 경험이 있는 개발자를 위한 구조화된 Rust 입문 가이드입니다. 이 가이드는 단순히 Rust의 사용법(*how*)을 넘어, 왜(*why*) 그렇게 설계되었는지에 대한 개념적 이해를 돕기 위해 단계별로 내용을 구성했습니다.

## 과정 개요 (Course Overview)
- **Rust의 가치** - C# 개발자에게 Rust가 중요한 이유
- **시작하기** - 설치, 도구 활용, 그리고 첫 번째 프로그램
- **기본 구성 요소** - 타입, 변수, 제어 흐름
- **데이터 구조** - 배열, 튜플, 구조체
- **패턴 매칭 및 열거형** - Rust의 핵심 개념
- **모듈 및 크레이트** - 코드 조직화 및 의존성 관리 (.NET 어셈블리와의 비교)
- **트레이트 및 제네릭** - 고급 타입 시스템
- **에러 처리** - 안전을 향한 Rust의 접근 방식
- **메모리 관리** - 소유권, 빌림, 그리고 수명(Lifetimes)
- **실전 마이그레이션** - 실제 사례 연구

## 목차 (Table of Contents)

### 1. 서론 및 동기 (Introduction and Motivation)
- [빠른 참조: Rust vs C#](#빠른-참조-rust-vs-c)
- [C# 개발자에게 Rust가 필요한 이유](#c-개발자에게-rust가-필요한-이유)
- [Rust가 해결해 주는 C#의 주요 문제점](#rust가-해결해-주는-c-의-주요-문제점)
- [C# 대신 Rust를 선택해야 하는 경우](#c-대신-rust를-선택해야-하는-경우)

### 2. 시작하기 (Getting Started)
- [설치 및 설정](#설치-및-설정)
- [첫 번째 Rust 프로그램](#첫-번째-rust-프로그램)
- [Cargo vs NuGet/MSBuild](#cargo-vs-nugetmsbuild)
- [C# 개발자를 위한 IDE 설정](#ide-설정)

### 3. 기본 타입 및 변수 (Basic Types and Variables)
- [내장 타입 비교](#내장-타입-비교)
- [변수와 가변성](#변수와-가변성)
- [문자열 타입: String vs &str](#문자열-타입-string-vs-str)
- [주석 및 문서화](#주석-및-문서화)

### 4. 제어 흐름 (Control Flow)
- [조건문](#조건문)
- [루프 및 반복](#루프-및-반복)
- [표현식 블록](#표현식-블록)
- [함수 vs 메서드](#함수-vs-메서드)

### 5. 데이터 구조 (Data Structures)
- [배열 및 슬라이스](#배열-및-슬라이스)
- [튜플](#튜플)
- [구조체 vs 클래스](#구조체-vs-클래스)
- [참조 및 빌림 기초](#참조-및-빌림-기초)

### 6. 패턴 매칭 및 열거형 (Pattern Matching and Enums)
- [열거형 vs C# 열거형](#열거형-vs-c-열거형)
- [Match 표현식](#match-표현식)
- [널 안전성을 위한 Option<T>](#널-안전성을-위한-optiont)
- [에러 처리를 위한 Result<T, E>](#에러-처리를-위한-resultt-e)

### 7. 모듈 및 크레이트 (Modules and Crates)
- [Rust 모듈 vs C# 네임스페이스](#rust-모듈-vs-c-네임스페이스)
- [크레이트 vs .NET 어셈블리](#크레이트-vs-net-어셈블리)
- [패키지 관리: Cargo vs NuGet](#패키지-관리-cargo-vs-nuget)
- [가시성 및 접근 제어](#가시성-및-접근-제어)

### 8. 트레이트 및 제네릭 (Traits and Generics)
- [트레이트 vs 인터페이스](#트레이트-vs-인터페이스)
- [제네릭 타입 및 함수](#제네릭-타입-및-함수)
- [트레이트 바운드 및 제약 조건](#트레이트-바운드-및-제약-조건)
- [주요 표준 라이브러리 트레이트](#주요-표준-라이브러리-트레이트)

### 9. 컬렉션 및 에러 처리 (Collections and Error Handling)
- [Vec<T> vs List<T>](#vect-vs-listt)
- [HashMap vs Dictionary](#hashmap-vs-dictionary)
- [반복자 패턴](#반복자-패턴)
- [포괄적인 에러 처리](#포괄적인-에러-처리)

### 10. 메모리 관리 (Memory Management)
- [소유권의 이해](#소유권의-이해)
- [이동 의미론 vs 참조 의미론](#이동-의미론-vs-참조-의미론)
- [빌림과 수명](#빌림과-수명)
- [스마트 포인터](#스마트-포인터)

### 11. 실전 마이그레이션 예제 (Practical Migration Examples)
- [설정 관리](#설정-관리)
- [데이터 처리 파이프라인](#데이터-처리-파이프라인)
- [HTTP 클라이언트 및 API](#http-클라이언트-및-api)
- [파일 I/O 및 직렬화](#파일-io-및-직렬화)

### 12. 다음 단계 및 권장 사례 (Next Steps and Best Practices)
- [Rust vs C# 테스트 방식](#rust-vs-c-테스트-방식)
- [C# 개발자가 자주 겪는 함정](#c-개발자가-자주-겪는-함정)
- [학습 경로 및 리소스](#학습-경로-및-리소스)
- [고급 주제로 넘어가기](#고급-주제로-넘어가기)

***

## 빠른 참조: Rust vs C# (Quick Reference)

| **개념** | **C#** | **Rust** | **주요 차이점** |
|-------------|--------|----------|-------------------|
| 메모리 관리 | 가비지 컬렉터 (GC) | 소유권 시스템 | 제로 비용, 결정적 정리 |
| 널 참조 | 어디에나 존재 (`null`) | `Option<T>` | 컴파일 타임 널 안전성 |
| 에러 처리 | 예외 (Exceptions) | `Result<T, E>` | 명시적 처리, 숨겨진 제어 흐름 없음 |
| 가변성 | 기본적으로 가변 (Mutable) | 기본적으로 불변 (Immutable) | 수정을 위해서는 명시적 선언 필요 |
| 타입 시스템 | 참조/값 타입 | 소유권 타입 | 이동 의미론, 빌림(Borrowing) |
| 어셈블리 | GAC, 앱 도메인 | 크레이트 (Crate) | 정적 링크, 런타임 불필요 |
| 네임스페이스 | `using System.IO` | `use std::fs` | 모듈 시스템 |
| 인터페이스 | `interface IFoo` | `trait Foo` | 기본 구현 가능 |
| 제네릭 | `List<T> where T : class` | `Vec<T> where T: Clone` | 제로 비용 추상화 |
| 스레딩 | locks, async/await | 소유권 + Send/Sync | 데이터 경합 예방 |
| 성능 | JIT 컴파일 | AOT 컴파일 | 예측 가능성, GC 중단 없음 |

***

## C# 개발자에게 Rust가 필요한 이유 (The Case for Rust)

### 런타임 세금 없는 성능 (Performance Without the Runtime Tax)
```csharp
// C# - 뛰어난 생산성, 하지만 런타임 오버헤드 존재
public class DataProcessor
{
    private List<int> data = new List<int>();
    
    public void ProcessLargeDataset()
    {
        // 할당이 발생할 때마다 GC 유발 가능성
        for (int i = 0; i < 10_000_000; i++)
        {
            data.Add(i * 2); // GC 압박
        }
        // 처리 도중 예측 불가능한 GC 중단 발생 가능
    }
}
// 실행 시간: 가변적 (GC로 인해 50-200ms)
// 메모리: ~80MB (GC 오버헤드 포함)
// 예측 가능성: 낮음 (GC 중단 때문)
```

```rust
// Rust - C#과 유사한 표현력, 하지만 런타임 오버헤드 제로
struct DataProcessor {
    data: Vec<i32>,
}

impl DataProcessor {
    fn process_large_dataset(&mut self) {
        // 제로 비용 추상화
        for i in 0..10_000_000 {
            self.data.push(i * 2); // GC 압박 없음
        }
        // 결정적인 성능 보장
    }
}
// 실행 시간: 일정 (~30ms)
// 메모리: ~40MB (정확한 할당량)
// 예측 가능성: 높음 (GC 없음)
```

### 런타임 체크 없는 메모리 안전성 (Memory Safety Without Runtime Checks)
```csharp
// C# - 오버헤드를 동반한 런타임 안전성
public class UnsafeOperations
{
    public string ProcessArray(int[] array)
    {
        // 런타임 범위 체크 (Bounds checking)
        if (array.Length > 0)
        {
            return array[0].ToString(); // NullReferenceException 가능성
        }
        return null; // 널 전파
    }
    
    public void ProcessConcurrently()
    {
        var list = new List<int>();
        
        // 데이터 경합(Data race) 가능성, 세심한 락(lock) 관리 필요
        Parallel.For(0, 1000, i =>
        {
            lock (list) // 런타임 오버헤드
            {
                list.Add(i);
            }
        });
    }
}
```

```rust
// Rust - 런타임 비용 없는 컴파일 타임 안전성
struct SafeOperations;

impl SafeOperations {
    // 컴파일 타임 널 안전성, 런타임 체크 불필요
    fn process_array(array: &[i32]) -> Option<String> {
        array.first().map(|x| x.to_string())
        // 널 참조 발생 불가능
        // 안전함이 증명되면 범위 체크 로직이 컴파일 타임에 최적화되어 제거됨
    }
    
    fn process_concurrently() {
        use std::sync::Mutex;
        use std::thread;
        
        let data = Mutex::new(Vec::new());
        
        // 데이터 경합이 컴파일 타임에 차단됨
        let handles: Vec<_> = (0..1000).map(|i| {
            let data = &data;
            thread::spawn(move || {
                data.lock().unwrap().push(i);
                // 단일 스레드 상황에서는 락 오버헤드 없음
            })
        }).collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
```

***

## Rust가 해결해 주는 C#의 주요 문제점 (Common C# Pain Points)

### 1. 10억 달러짜리 실수: 널 참조 (Null References)
```csharp
// C# - 널 참조 예외는 런타임의 시한폭탄입니다.
public class UserService
{
    public string GetUserDisplayName(User user)
    {
        // 이 중 어느 것이라도 NullReferenceException을 발생시킬 수 있음
        return user.Profile.DisplayName.ToUpper();
        //     ^^^^^ ^^^^^^^ ^^^^^^^^^^^ ^^^^^^^
        //     런타임에 null일 수 있음
    }
    
    // Nullable 참조 타입(C# 8+)을 사용하더라도:
    public string GetDisplayName(User? user)
    {
        return user?.Profile?.DisplayName?.ToUpper() ?? "Unknown";
        // 여전히 런타임에 null 관련 이슈가 발생할 수 있음
    }
}
```

```rust
// Rust - 컴파일 타임에 널 안전성 보장
struct UserService;

impl UserService {
    fn get_user_display_name(user: &User) -> Option<String> {
        user.profile.as_ref()?
            .display_name.as_ref()
            .map(|name| name.to_uppercase())
        // 컴파일러가 None 케이스 처리를 강제함
        // 널 포인터 예외 발생이 구조적으로 불가능함
    }
    
    fn get_display_name_safe(user: Option<&User>) -> String {
        user.and_then(|u| u.profile.as_ref())
            .and_then(|p| p.display_name.as_ref())
            .map(|name| name.to_uppercase())
            .unwrap_or_else(|| "Unknown".to_string())
        // 명시적인 처리, 예상치 못한 상황 없음
    }
}
```

### 2. 숨겨진 예외와 제어 흐름 (Hidden Exceptions)
```csharp
// C# - 예외는 어디서든 발생할 수 있습니다.
public async Task<UserData> GetUserDataAsync(int userId)
{
    // 각 호출마다 서로 다른 예외가 발생할 수 있음
    var user = await userRepository.GetAsync(userId);        // SqlException
    var permissions = await permissionService.GetAsync(user); // HttpRequestException  
    var preferences = await preferenceService.GetAsync(user); // TimeoutException
    
    return new UserData(user, permissions, preferences);
    // 호출자는 어떤 예외가 발생할지 알기 어려움
}
```

```rust
// Rust - 모든 에러는 함수 시그니처에 명시됩니다.
#[derive(Debug)]
enum UserDataError {
    DatabaseError(String),
    NetworkError(String),
    Timeout,
    UserNotFound(i32),
}

async fn get_user_data(user_id: i32) -> Result<UserData, UserDataError> {
    // 모든 에러가 명시적으로 처리됨
    let user = user_repository.get(user_id).await
        .map_err(UserDataError::DatabaseError)?;
    
    let permissions = permission_service.get(&user).await
        .map_err(UserDataError::NetworkError)?;
    
    let preferences = preference_service.get(&user).await
        .map_err(|_| UserDataError::Timeout)?;
    
    Ok(UserData::new(user, permissions, preferences))
    // 호출자는 발생 가능한 에러를 정확히 인지할 수 있음
}
```

***

## C# 대신 Rust를 선택해야 하는 경우 (When to Choose Rust)

### ✅ 다음의 경우 Rust를 선택하세요:
- **성능이 핵심인 경우**: 실시간 시스템, 고주파 거래(HFT), 게임 엔진
- **메모리 사용량이 중요한 경우**: 임베디드 시스템, 클라우드 비용 절감, 모바일 앱
- **예측 가능성이 필요한 경우**: 의료 기기, 자동차, 금융 시스템
- **보안이 최우선인 경우**: 암호학, 네트워크 보안, 시스템 레벨 코드
- **장시간 실행되는 서비스**: GC 중단이 문제가 되는 환경
- **자원이 제한된 환경**: IoT, 에지 컴퓨팅
- **시스템 프로그래밍**: CLI 도구, 데이터베이스, 웹 서버, 운영체제

### ✅ 다음의 경우 C#을 유지하세요:
- **빠른 애플리케이션 개발**: 일반적인 비즈니스 앱, CRUD 앱
- **방대한 기존 코드베이스**: 마이그레이션 비용이 너무 큰 경우
- **팀의 숙련도**: Rust의 학습 곡선이 주는 이점보다 큰 경우
- **엔터프라이즈 통합**: .NET 프레임워크나 Windows 의존성이 높은 경우
- **GUI 애플리케이션**: WPF, WinUI, Blazor 생태계 활용 시
- **출시 속도(Time to Market)**: 성능보다 개발 속도가 압도적으로 중요한 경우

***

## 설치 및 설정 (Installation and Setup)

### Rust 설치하기
```bash
# Rust 설치 (Windows, macOS, Linux 공통)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows 사용자는 https://rustup.rs/ 에서 설치 파일을 직접 다운로드할 수도 있습니다.
```

### Rust 도구 vs C# 도구 대응표
| C# 도구 | Rust 대응 도구 | 용도 |
|---------|----------------|---------|
| `dotnet new` | `cargo new` | 새 프로젝트 생성 |
| `dotnet build` | `cargo build` | 프로젝트 컴파일 |
| `dotnet run` | `cargo run` | 프로젝트 실행 |
| `dotnet test` | `cargo test` | 테스트 실행 |
| NuGet | Crates.io | 패키지 저장소 |
| MSBuild | Cargo | 빌드 시스템 |
| Visual Studio | VS Code + rust-analyzer | IDE |

***

## 첫 번째 Rust 프로그램 (Your First Rust Program)

### C# Hello World
```csharp
// Program.cs
using System;

namespace HelloWorld
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello, World!");
        }
    }
}
```

### Rust Hello World
```rust
// main.rs
fn main() {
    println!("Hello, World!");
}
```

### C# 개발자를 위한 주요 차이점
1. **클래스가 필요 없음** - 함수는 최상위 수준(top-level)에서 존재할 수 있습니다.
2. **네임스페이스가 없음** - 대신 모듈(module) 시스템을 사용합니다.
3. **`println!`은 매크로임** - 끝에 붙은 `!`에 주목하세요.
4. **`println!` 뒤에 세미콜론 유무** - 표현식(expression)과 문(statement)의 차이입니다.
5. **명시적인 반환 타입 생략** - `main`은 유닛 타입인 `()`를 반환합니다.

***

## Cargo vs NuGet/MSBuild

### 프로젝트 설정 (Project Configuration)

**C# (.csproj)**
```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
  </PropertyGroup>
  
  <PackageReference Include="Newtonsoft.Json" Version="13.0.3" />
  <PackageReference Include="Serilog" Version="3.0.1" />
</Project>
```

**Rust (Cargo.toml)**
```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"    # Newtonsoft.Json에 대응
log = "0.4"           # Serilog에 대응
```

### 워크스페이스 vs 솔루션

**C# 솔루션 (.sln)**
```
MySolution/
├── MySolution.sln
├── WebApi/
│   └── WebApi.csproj
├── Business/
│   └── Business.csproj
└── Tests/
    └── Tests.csproj
```

**Rust 워크스페이스 (Cargo.toml)**
```toml
[workspace]
members = [
    "web_api",
    "business", 
    "tests"
]
```

***

## 변수와 가변성 (Variables and Mutability)

### C# 변수 선언
```csharp
// C# - 변수는 기본적으로 가변(mutable)입니다.
int count = 0;           // 가변
count = 5;               // ✅ 가능

readonly int maxSize = 100;  // 초기화 후 불변
// maxSize = 200;        // ❌ 컴파일 에러

const int BUFFER_SIZE = 1024; // 컴파일 타임 상수
```

### Rust 변수 선언
```rust
// Rust - 변수는 기본적으로 불변(immutable)입니다.
let count = 0;           // 기본적으로 불변
// count = 5;            // ❌ 컴파일 에러: 불변 변수에 두 번 할당할 수 없음

let mut count = 0;       // 명시적으로 가변 선언
count = 5;               // ✅ 가능

const BUFFER_SIZE: usize = 1024; // 컴파일 타임 상수
```

### C# 개발자를 위한 사고방식의 전환
```rust
// 'let'을 기본적으로 'readonly'라고 생각하세요.
let name = "John";       // C#의 readonly string name = "John"; 과 유사
let mut age = 30;        // C#의 int age = 30; 과 유사

// 변수 섀도잉 (Variable shadowing - Rust만의 특징)
let spaces = "   ";      // 문자열 타입
let spaces = spaces.len(); // 이제 숫자 타입 (usize)이 됨
// 이는 값을 수정하는 '변이(mutation)'와는 다릅니다 - 새로운 변수를 만드는 것입니다.
```

***

## 데이터 타입 비교 (Data Types Comparison)

### 기본 타입 (Primitive Types)

| C# 타입 | Rust 타입 | 크기 | 범위 |
|---------|-----------|------|-------|
| `byte` | `u8` | 8 bits | 0 ~ 255 |
| `sbyte` | `i8` | 8 bits | -128 ~ 127 |
| `short` | `i16` | 16 bits | -32,768 ~ 32,767 |
| `ushort` | `u16` | 16 bits | 0 ~ 65,535 |
| `int` | `i32` | 32 bits | -2³¹ ~ 2³¹-1 |
| `uint` | `u32` | 32 bits | 0 ~ 2³²-1 |
| `long` | `i64` | 64 bits | -2⁶³ ~ 2⁶³-1 |
| `ulong` | `u64` | 64 bits | 0 ~ 2⁶⁴-1 |
| `float` | `f32` | 32 bits | IEEE 754 |
| `double` | `f64` | 64 bits | IEEE 754 |
| `bool` | `bool` | 1 bit | true/false |
| `char` | `char` | 32 bits | 유니코드 스칼라값 |

### 크기별 타입 (Size Types)
```csharp
// C# - int는 언제나 32비트입니다.
int arrayIndex = 0;
long fileSize = file.Length;
```

```rust
// Rust - 크기 타입은 포인터 크기(32비트 또는 64비트)에 따라 달라집니다.
let array_index: usize = 0;    // C의 size_t와 유사
let file_size: u64 = file.len(); // 명시적 64비트
```

### 타입 추론 (Type Inference)
```csharp
// C# - var 키워드
var name = "John";        // string으로 추론
var count = 42;           // int로 추론
```

```rust
// Rust - 자동 타입 추론
let name = "John";        // &str (문자열 슬라이스)로 추론
let count = 42;           // i32 (기본 정수형)로 추론
```

***

## 문자열 타입: String vs &str (String Types)

C# 개발자가 가장 혼란스러워하는 부분 중 하나입니다. 자세히 살펴보겠습니다.

### C# 문자열 처리
```csharp
// C# - 단순한 문자열 모델
string name = "John";           // 문자열 리터럴
string greeting = "Hello, " + name;  // 문자열 연결
string upper = name.ToUpper();  // 메서드 호출
```

### Rust 문자열 타입
```rust
// Rust - 두 가지 주요 문자열 타입

// 1. &str (문자열 슬라이스) - C#의 ReadOnlySpan<char>와 유사
let name: &str = "John";        // 문자열 리터럴 (불변, 빌려온 값)

// 2. String - C#의 StringBuilder나 가변 문자열과 유사
let mut greeting = String::new();       // 빈 문자열 생성
greeting.push_str("Hello, ");          // 추가
greeting.push_str(name);               // 추가

// 또는 직접 생성
let greeting = String::from("Hello, John");
let greeting = "Hello, John".to_string();  // &str을 String으로 변환
```

### 언제 어떤 타입을 써야 하나요?

| 시나리오 | 사용 타입 | C# 대응 개념 |
|----------|-----|---------------|
| 문자열 리터럴 | `&str` | `string` 리터럴 |
| 함수 매개변수 (읽기 전용) | `&str` | `string` 또는 `ReadOnlySpan<char>` |
| 소유권이 있고 수정 가능한 문자열 | `String` | `StringBuilder` |
| 소유권이 있는 문자열 반환 | `String` | `string` |

***

## C# 개발자를 위한 필수 Rust 키워드 매핑 (Keywords Reference)

### 가시성 및 접근 제어 (Visibility)

| C# | Rust | 참고 사항 |
|----|----|-----------|
| `public` | `pub` | 어디서나 접근 가능 |
| `private` | (없음) | Rust는 기본값이 private임 |
| `internal` | `pub(crate)` | 현재 크레이트 내에서만 공개 |
| `protected` | (없음) | Rust는 상속이 없으므로 구성(composition) 사용 |

### 메모리 및 소유권 (Memory & Ownership)

| C# | Rust | 참고 사항 |
|----|----|-----------|
| `ref` | `&mut` | 가변 참조 (Mutable reference) |
| `in` | `&` | 불변 참조 (Immutable reference) |
| `out` | `Result` / `(T, U)` | 반환값 또는 튜플로 대체 |
| `new` | `Box::new()` | 힙 할당 시 사용 |

### 타입 정의 (Type Definitions)

| C# | Rust | 참고 사항 |
|----|----|-----------|
| `class` / `struct` | `struct` | 데이터 구조 정의 |
| `interface` | `trait` | 동작(behavior) 정의 |
| `enum` | `enum` | Rust의 열거형은 훨씬 강력함 (데이터 포함 가능) |
| `using` alias | `type` | 타입 별칭 정의 |

***

## 소유권의 이해 (Understanding Ownership)

소유권은 Rust의 가장 독특한 특징이자 C# 개발자에게 가장 큰 개념적 변화를 요구하는 부분입니다.

### C# 메모리 모델 (복습)
```csharp
// C# - 자동 메모리 관리
public void ProcessData()
{
    var data = new List<int> { 1, 2, 3 };
    ProcessList(data);
    // data는 여전히 여기서 접근 가능합니다.
    Console.WriteLine(data.Count);
    
    // 더 이상 참조가 없으면 GC가 정리합니다.
}

public void ProcessList(List<int> list)
{
    list.Add(4);  // 원본 리스트를 수정함
}
```

### Rust 소유권 규칙
1. **각 값은 단 하나의 소유자(owner)를 가집니다.**
2. **소유자가 범위를 벗어나면 값은 버려집니다(dropped).**
3. **소유권은 이전(move)될 수 있습니다.**

```rust
// Rust - 명시적 소유권 관리
fn process_data() {
    let data = vec![1, 2, 3];  // data가 벡터를 소유함
    process_list(data);        // 소유권이 함수 내부로 이동(move)됨
    // println!("{:?}", data);  // ❌ 에러: data는 더 이상 소유권이 없음
}

fn process_list(mut list: Vec<i32>) {  // list가 이제 벡터를 소유함
    list.push(4);
    // 함수가 끝나면 list가 범위를 벗어나며 데이터가 정리됨
}
```

### C# 개발자를 위한 "이동(Move)"의 이해
```csharp
// C# - 참조가 복사되며, 객체는 그대로 유지됨
var original = new List<int> { 1, 2, 3 };
var reference = original;  // 두 변수가 같은 객체를 가리킴
original.Add(4);
Console.WriteLine(reference.Count);  // 4 - 같은 객체이므로
```

```rust
// Rust - 소유권 자체가 이전됨
let original = vec![1, 2, 3];
let moved = original;       // 소유권이 이전됨
// println!("{:?}", original);  // ❌ 에러: 원본 변수는 이제 무효화됨
println!("{:?}", moved);    // ✅ 정상: moved가 새로운 소유자임
```

***

## 빌림 기초 (Borrowing Basics)

빌림은 C#에서 참조를 사용하는 것과 비슷하지만, 컴파일 타임에 안전성이 보장됩니다.

### Rust 빌림 (Borrowing)
```rust
// &와 &mut를 이용한 빌림
fn modify_value(value: &mut i32) {  // 가변 빌림
    *value += 10;
}

fn read_value(value: &i32) {        // 불변 빌림
    println!("{}", value);
}

fn main() {
    let mut x = 5;
    
    read_value(&x);      // 불변으로 빌려줌
    modify_value(&mut x); // 가변으로 빌려줌
    
    println!("{}", x);   // 소유권은 여전히 여기에 있음
}
```

### 빌림 규칙 (컴파일 타임에 강제됨!)
1. **여러 개의 불변 참조는 가질 수 있습니다.**
2. **가변 참조는 단 하나만 가질 수 있습니다.**
3. **가변 참조와 불변 참조를 동시에 가질 수 없습니다.**

이 규칙들은 **데이터 경합(Data Race)**을 원천적으로 차단합니다.

***

## 에러 처리 기초 (Error Handling)

C#의 예외 모델에서 Rust의 명시적 에러 처리로의 전환입니다.

### C# 예외 처리
```csharp
// C# - 예외 기반 에러 처리
public string ReadConfig(string path)
{
    try {
        return File.ReadAllText(path);
    } catch (FileNotFoundException) {
        throw new InvalidOperationException("파일을 찾을 수 없음");
    }
}
```

### Rust Result 기반 에러 처리
```rust
use std::fs;

// 커스텀 에러 정의
#[derive(Debug)]
enum ConfigError {
    FileNotFound,
    AccessDenied,
}

// Result를 반환하는 함수
fn read_config(path: &str) -> Result<String, ConfigError> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(_) => Err(ConfigError::FileNotFound),
    }
}

fn main() {
    // 에러를 명시적으로 처리
    match read_config("config.txt") {
        Ok(content) => println!("설정 내용: {}", content),
        Err(ConfigError::FileNotFound) => println!("파일이 없습니다."),
        Err(_) => println!("알 수 없는 에러 발생"),
    }
}
```

### ? 연산자 (C#의 await와 유사한 흐름)
```rust
// ? 연산자를 이용한 에러 전파
fn process_file(path: &str) -> Result<String, ConfigError> {
    let content = read_config(path)?;  // 에러 발생 시 즉시 반환
    Ok(content.to_uppercase())
}
```

***

## 컬렉션: Vec<T>와 HashMap

### Vec<T> (C#의 List<T>에 대응)
```rust
let mut numbers = vec![1, 2, 3];
numbers.push(4);

// 안전한 접근
if let Some(first) = numbers.get(0) {
    println!("첫 번째 요소: {}", first);
}
```

### HashMap (C#의 Dictionary<K,V>에 대응)
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Alice".to_string(), 100);

// 값 가져오기
let score = scores.get("Alice").copied().unwrap_or(0);
```

***

## 트레이트 - Rust의 인터페이스 (Traits)

트레이트는 공유된 동작을 정의하는 Rust의 방식입니다.

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle { radius: f64 }

impl Drawable for Circle {
    fn draw(&self) {
        println!("반지름 {}인 원을 그립니다.", self.radius);
    }
}
```

***

## 학습 경로 및 다음 단계 (Learning Path)

### 1~2주 차: 기초 다지기
- 환경 설정 및 Hello World
- 소유권과 빌림 개념 익히기
- 기본적인 구조체와 메서드 구현

### 1~2개월 차: 중급 목표
- 반복자(Iterator)와 클로저(Closure) 마스터
- 트레이트와 제네릭 활용
- 프로젝트 모듈 구조화

### C# 개발자를 위한 마지막 조언
1. **컴파일러를 믿으세요**: Rust의 컴파일 에러는 여러분을 괴롭히는 것이 아니라 도와주는 것입니다.
2. **작게 시작하세요**: 처음부터 복잡한 시스템을 만들려 하지 말고, 작은 CLI 도구부터 시작해 보세요.
3. **소유권 시스템을 즐기세요**: 처음에는 제약처럼 느껴지지만, 익숙해지면 이보다 강력한 안전 장치가 없음을 깨닫게 될 것입니다.

---

**축하합니다!** 이제 C#에서 Rust로 나아가기 위한 첫걸음을 떼셨습니다. 더 깊이 있는 내용은 [고급 과정 가이드](./RustTrainingForCSharp.md)에서 확인해 보세요.
