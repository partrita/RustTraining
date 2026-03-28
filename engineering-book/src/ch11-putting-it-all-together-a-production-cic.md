# 11. 종합 정리 — 운영 환경용 CI/CD 파이프라인 🟡

> **학습 내용:**
> - 다단계 GitHub Actions CI 워크플로 구성 (검사 → 테스트 → 커버리지 → 보안 → 교차 빌드 → 릴리스)
> - `rust-cache`와 `save-if` 설정을 이용한 캐싱 전략
> - Nightly 스케줄에 따른 Miri 및 새니타이저 실행
> - `Makefile.toml`과 pre-commit 훅을 이용한 작업 자동화
> - `cargo-dist`를 이용한 자동 릴리스
>
> **참조:** [빌드 스크립트](ch01-build-scripts-buildrs-in-depth.md) · [교차 컴파일](ch02-cross-compilation-one-source-many-target.md) · [벤치마킹](ch03-benchmarking-measuring-what-matters.md) · [코드 커버리지](ch04-code-coverage-seeing-what-tests-miss.md) · [Miri/새니타이저](ch05-miri-valgrind-and-sanitizers-verifying-u.md) · [의존성 관리](ch06-dependency-management-and-supply-chain-s.md) · [릴리스 프로필](ch07-release-profiles-and-binary-size.md) · [컴파일 타임 도구](ch08-compile-time-and-developer-tools.md) · [`no_std`](ch09-no-std-and-feature-verification.md) · [Windows](ch10-windows-and-conditional-compilation.md)

개별 도구들도 유용하지만, 모든 푸시마다 이들을 자동으로 조율하는 파이프라인은 개발 경험을 혁신적으로 바꿔놓습니다. 이 장에서는 1~10장에서 다룬 도구들을 하나의 응집력 있는 CI/CD 워크플로로 통합합니다.

---

### 1. 전체 GitHub Actions 워크플로

모든 검증 단계를 병렬로 실행하는 단일 워크플로 파일 예시입니다.

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  CARGO_ENCODED_RUSTFLAGS: "-Dwarnings"  # 경고를 에러로 처리 (최상위 크레이트 전용)

jobs:
  # ─── 1단계: 빠른 피드백 (< 2분) ───
  check:
    name: Check + Clippy + Format
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - uses: Swatinem/rust-cache@v2
      - name: Check compilation
        run: cargo check --workspace --all-targets --all-features
      - name: Clippy lints
        run: cargo clippy --workspace --all-targets --all-features -- -D warnings
      - name: Formatting
        run: cargo fmt --all -- --check

  # ─── 2단계: 테스트 (< 5분) ───
  test:
    name: Test (${{ matrix.os }})
    needs: check
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Run tests
        run: cargo test --workspace

  # ─── 3단계: 교차 컴파일 (< 10분) ───
  cross:
    name: Cross (${{ matrix.target }})
    needs: check
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-musl
            os: ubuntu-latest
          - target: aarch64-unknown-linux-gnu
            os: ubuntu-latest
            use_cross: true
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: taiki-e/install-action@cross
      - name: Build
        run: cross build --release --target ${{ matrix.target }}

  # ─── 4단계: 커버리지 (< 10분) ───
  coverage:
    name: Code Coverage
    needs: check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: taiki-e/install-action@cargo-llvm-cov
      - name: Generate coverage
        run: cargo llvm-cov --workspace --fail-under-lines 75
```

---

### 2. CI 캐싱 전략

`Swatinem/rust-cache@v2`는 Rust CI의 표준입니다. 대규모 워크스페이스에서는 다음과 같은 튜닝이 필요합니다.

- **`save-if` 활용**: `main` 브랜치에서만 캐시를 저장하고, PR에서는 읽기만 하도록 설정하여 캐시가 오염되는 것을 방지합니다.
- **`prefix-key`**: 캐시 크기가 너무 커지면(>5GB) 접두사를 변경하여 캐시를 초기화하세요.
- **타겟별 분리**: 교차 컴파일 타겟마다 별도의 캐시 키를 사용하세요.

---

### 3. `cargo-make`를 이용한 작업 자동화

[`cargo-make`](https://sagiegurari.github.io/cargo-make/)는 플랫폼에 독립적인 작업 실행 도구입니다.

```toml
# Makefile.toml 예시
[tasks.dev]
description = "CI와 동일한 로컬 검증 실행"
dependencies = ["check", "test", "clippy", "fmt-check"]

[tasks.coverage]
description = "HTML 커버리지 보고서 생성 및 열기"
install_crate = "cargo-llvm-cov"
command = "cargo"
args = ["llvm-cov", "--workspace", "--html", "--open"]
```

이제 `cargo make dev` 한 번으로 모든 로컬 검증을 마칠 수 있습니다.

---

### 4. 자동 릴리스: `cargo-release` 및 `cargo-dist`

- **`cargo-release`**: 버전 번호 올리기, 태그 생성, `Cargo.lock` 업데이트를 자동화합니다.
- **`cargo-dist`**: GitHub Releases에 업로드할 바이너리 아카이브와 설치 스크립트를 생성합니다. `cargo dist init` 명령어로 손쉽게 시작할 수 있습니다.

---

### 핵심 요약

1. **병렬 실행** — 빠른 검증(check)을 통과한 후 무거운 작업들(test, cross, coverage)을 병렬로 돌려 시간을 단축하세요.
2. **캐시 관리** — `main` 브랜치 위주의 캐시 저장 정책으로 PR 빌드 속도를 높이세요.
3. **로컬 자동화** — CI와 동일한 환경을 로컬에서 `cargo-make`로 재현하여 커밋 전 실수를 방지하세요.
4. **배포 자동화** — 버전 관리와 바이너리 배포를 도구화하여 실수를 줄이고 고품질의 아티팩트를 제공하세요.

