## C# 개발자를 위한 필수 Rust 도구 (Essential Rust Tooling for C# Developers)

> **학습 내용:** C#의 개발 도구에 대응하는 Rust의 도구들을 알아봅니다 — Clippy (Roslyn 분석기),
> rustfmt (dotnet format), cargo doc (XML 문서), cargo watch (dotnet watch), 그리고 VS Code 확장 프로그램.
>
> **난이도:** 🟢 초급

### 도구 비교 (Tool Comparison)

| C# 도구 | Rust 대응 도구 | 설치 방법 | 용도 |
|---------|----------------|---------|---------|
| Roslyn 분석기 | **Clippy** | `rustup component add clippy` | 린트(Lint) + 스타일 제안 |
| `dotnet format` | **rustfmt** | `rustup component add rustfmt` | 자동 포맷팅 |
| XML 문서 주석 | **`cargo doc`** | 기본 내장 | HTML 문서 생성 |
| OmniSharp / Roslyn | **rust-analyzer** | VS Code 확장 | IDE 지원 |
| `dotnet watch` | **cargo-watch** | `cargo install cargo-watch` | 저장 시 자동 재빌드 |
| — | **cargo-expand** | `cargo install cargo-expand` | 매크로 확장 결과 확인 |
| `dotnet audit` | **cargo-audit** | `cargo install cargo-audit` | 보안 취약점 스캔 |

### Clippy: 자동화된 코드 검토 도구 (Automated Code Reviewer)
```bash
# 프로젝트에서 Clippy 실행
cargo clippy

# 경고를 에러로 처리 (CI/CD용)
cargo clippy -- -D warnings

# 제안사항 자동 수정
cargo clippy --fix
```

```rust
// Clippy는 수백 가지의 안티 패턴을 찾아냅니다:

// Clippy 적용 전:
if x == true { }           // 경고: bool 값과의 비교 체크
let _ = vec.len() == 0;    // 경고: 대신 .is_empty() 사용 권장
for i in 0..vec.len() { }  // 경고: 대신 .iter().enumerate() 사용 권장

// Clippy 제안 적용 후:
if x { }
let _ = vec.is_empty();
for (i, item) in vec.iter().enumerate() { }
```

### rustfmt: 일관된 포맷팅 (Consistent Formatting)
```bash
# 모든 파일 포맷팅
cargo fmt

# 변경 없이 포맷팅 상태만 확인 (CI/CD용)
cargo fmt -- --check
```

```toml
# rustfmt.toml — 포맷팅 설정 커스터마이징 (.editorconfig와 유사)
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
```

### cargo doc: 문서 생성 (Documentation Generation)
```bash
# 문서 생성 및 열기 (의존성 라이브러리 포함)
cargo doc --open

# 문서 내 테스트 코드 실행
cargo test --doc
```

```rust
/// 원의 넓이를 계산합니다.
///
/// # 인자 (Arguments)
/// * `radius` - 원의 반지름 (음수여서는 안 됩니다)
///
/// # 예제 (Examples)
/// ```
/// let area = my_crate::circle_area(5.0);
/// assert!((area - 78.54).abs() < 0.01);
/// ```
///
/// # 패닉 (Panics)
/// `radius`가 음수인 경우 패닉이 발생합니다.
pub fn circle_area(radius: f64) -> f64 {
    assert!(radius >= 0.0, "반지름은 음수일 수 없습니다");
    std::f64::consts::PI * radius * radius
}
// /// ``` 블록 안의 코드는 `cargo test` 실행 시 함께 컴파일되고 실행됩니다!
```

### cargo watch: 자동 재빌드 (Auto-Rebuild)
```bash
# 파일 변경 시 자동 재빌드 (dotnet watch와 유사)
cargo watch -x check          # 타입 체크만 수행 (가장 빠름)
cargo watch -x test           # 저장 시 테스트 실행
cargo watch -x 'run -- args'  # 저장 시 프로그램 실행
cargo watch -x clippy         # 저장 시 린트 실행
```

### cargo expand: 매크로 생성 결과 확인 (See What Macros Generate)
```bash
# derive 매크로 등으로 확장된 결과물 확인
cargo expand --lib            # lib.rs 확장
cargo expand module_name      # 특정 모듈 확장
```

### 권장 VS Code 확장 프로그램 (Recommended VS Code Extensions)

| 확장 프로그램 | 용도 |
|-----------|---------|
| **rust-analyzer** | 코드 완성, 인라인 에러 표시, 리팩토링 |
| **CodeLLDB** | 디버거 (Visual Studio 디버거와 유사) |
| **Even Better TOML** | Cargo.toml 구문 강조 |
| **crates** | Cargo.toml에서 최신 크레이트 버전 표시 |
| **Error Lens** | 에러/경고를 인라인으로 즉시 표시 |

***

이 가이드에서 언급된 고급 주제들에 대해 더 자세히 알아보려면 다음 교육 문서를 참조하십시오:

- **[Rust 패턴 (Rust Patterns)](../../rust-patterns-book/src/SUMMARY.md)** — 핀 프로젝션(Pin projections), 커스텀 할당자(custom allocators), 아레나 패턴(arena patterns), 락-프리(lock-free) 데이터 구조 및 고급 unsafe 패턴
- **[Async Rust 교육 (Async Rust Training)](../../async-book/src/SUMMARY.md)** — tokio 심층 분석, 비동기 취소 안전성(async cancellation safety), 스트림 처리 및 프로덕션용 비동기 아키텍처
- **[C++ 개발자를 위한 Rust 교육 (Rust Training for C++ Developers)](../../c-cpp-book/src/SUMMARY.md)** — 팀에 C++ 경험자가 있는 경우 유용합니다. 이동 의미론(move semantics) 매핑, RAII 차이점, 템플릿 vs 제네릭 등을 다룹니다.
- **[C 개발자를 위한 Rust 교육 (Rust Training for C Developers)](../../c-cpp-book/src/SUMMARY.md)** — 상호 운용성(interop) 시나리오에 적합합니다. FFI 패턴, 임베디드 Rust 디버깅 및 `no_std` 프로그래밍을 다룹니다.
