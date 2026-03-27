# Rust 엔지니어링 관행 — `cargo build` 그 이상을 향해

## 저자 소개

- Microsoft SCHIE(Silicon and Cloud Hardware Infrastructure Engineering) 팀의 수석 펌웨어 아키텍트
- 보안, 시스템 프로그래밍(펌웨어, 운영체제, 하이퍼바이저), CPU 및 플랫폼 아키텍처, C++ 시스템 분야의 업계 전문가
- 2017년(@AWS EC2)부터 Rust로 프로그래밍을 시작했으며, 이후 이 언어의 매력에 빠져 활동 중

---

> 많은 팀이 너무 늦게 발견하곤 하는 Rust 툴체인 기능들에 대한 실무 가이드입니다.
> 빌드 스크립트, 교차 컴파일(cross-compilation), 벤치마킹, 코드 커버리지,
> 그리고 Miri와 Valgrind를 이용한 안전성 검증을 다룹니다. 각 장에서는
> 실제 하드웨어 진단 코드베이스(대규모 멀티 크레이트 워크스페이스)에서 추출한
> 구체적인 예제를 사용하여, 모든 기술을 실제 운영 코드에 직접 적용할 수 있도록 구성했습니다.

## 이 책의 활용 방법

이 책은 **자기 주도 학습 또는 팀 워크숍**을 위해 설계되었습니다. 각 장은 대부분 독립적으로 구성되어 있으므로 순서대로 읽거나 필요한 주제를 골라서 학습하실 수 있습니다.

### 난이도 범례

| 기호 | 레벨 | 의미 |
|:------:|-------|---------|
| 🟢 | 입문 (Starter) | 명확한 패턴을 가진 직관적인 도구 — 첫날부터 바로 활용 가능 |
| 🟡 | 중급 (Intermediate) | 툴체인 내부 구조나 플랫폼 개념에 대한 이해가 필요함 |
| 🔴 | 고급 (Advanced) | 깊이 있는 툴체인 지식, 나이틀리(nightly) 기능 또는 다중 도구 오케스트레이션 필요 |

### 학습 권장 시간

| 파트 | 장 | 예상 시간 | 핵심 성과 |
|------|----------|:---------:|-------------|
| **I — 빌드 및 배포** | 01–02장 | 3–4시간 | 빌드 메타데이터, 교차 컴파일, 정적 바이너리 |
| **II — 측정 및 검증** | 03–05장 | 4–5시간 | 통계적 벤치마킹, 커버리지 게이트, Miri/새니타이저 |
| **III — 강화 및 최적화** | 06–10장 | 6–8시간 | 공급망 보안, 릴리스 프로필, 컴파일 타임 도구, `no_std`, Windows |
| **IV — 통합** | 11–13장 | 3–4시간 | 운영 환경용 CI/CD 파이프라인, 실전 팁, 종합 실습 |
| | | **16–21시간** | **전체 운영 엔지니어링 파이프라인 완성** |

### 실습 진행 방법

각 장에는 난이도 표시가 있는 **🏋️ 실습**이 포함되어 있습니다. 솔루션은 확장 가능한 `<details>` 블록에 제공되니, 먼저 직접 실습해 본 후 결과를 확인하시기 바랍니다.

- 🟢 실습은 대개 10~15분 내에 완료할 수 있습니다.
- 🟡 실습은 20~40분 정도 소요되며, 로컬에서 도구를 실행해야 할 수도 있습니다.
- 🔴 실습은 상당한 설정과 실험이 필요합니다 (1시간 이상).

## 선수 지식

| 개념 | 학습 위치 |
|---------|-------------------|
| Cargo 워크스페이스 레이아웃 | [Rust Book 14.3장](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) |
| 기능 플래그 (Feature flags) | [Cargo Reference — Features](https://doc.rust-lang.org/cargo/reference/features.html) |
| `#[cfg(test)]` 및 기본 테스트 | Rust Patterns 12장 |
| `unsafe` 블록 및 FFI 기초 | Rust Patterns 10장 |

## 장별 의존성 맵

```text
                 ┌──────────┐
                 │ ch00     │
                 │   소개   │
                 └────┬─────┘
        ┌─────┬───┬──┴──┬──────┬──────┐
        ▼     ▼   ▼     ▼      ▼      ▼
      ch01  ch03 ch04  ch05   ch06   ch09
      Build Bench Cov  Miri   Deps   no_std
        │     │    │    │      │      │
        │     └────┴────┘      │      ▼
        │          │           │    ch10
        ▼          ▼           ▼   Windows
       ch02      ch07        ch07    │
       Cross    RelProf     RelProf  │
        │          │           │     │
        │          ▼           │     │
        │        ch08          │     │
        │      CompTime        │     │
        └──────────┴───────────┴─────┘
                   │
                   ▼
                 ch11
               CI/CD 파이프라인
                   │
                   ▼
                ch12 ─── ch13
              실전 팁   빠른 참조
```

**순서 상관없이 읽기 가능**: 01, 03, 04, 05, 06, 09장은 서로 독립적입니다.
**선행 장 학습 후 읽기 권장**: 02장(01장 필요), 07~08장(03~06장 선학습 시 유리), 10장(09장 선학습 시 유리).
**마지막에 읽기 권장**: 11장(모든 내용을 통합), 12장(실전 팁), 13장(참조).

## 주석이 달린 목차

### 파트 I — 빌드 및 배포 (Build & Ship)

| # | 장 | 난이도 | 설명 |
|---|---------|:----------:|-------------|
| 1 | [빌드 스크립트 — `build.rs` 심층 분석](ch01-build-scripts-buildrs-in-depth.md) | 🟢 | 컴파일 타임 상수, C 코드 컴파일, Protobuf 생성, 시스템 라이브러리 링크, 안티 패턴 |
| 2 | [교차 컴파일 — 하나의 소스, 다양한 타겟](ch02-cross-compilation-one-source-many-target.md) | 🟡 | 타겟 트리플(Target triples), musl 정적 바이너리, ARM 교차 컴파일, `cross` 도구, `cargo-zigbuild`, GitHub Actions |

### 파트 II — 측정 및 검증 (Measure & Verify)

| # | 장 | 난이도 | 설명 |
|---|---------|:----------:|-------------|
| 3 | [벤치마킹 — 중요한 지표 측정하기](ch03-benchmarking-measuring-what-matters.md) | 🟡 | Criterion.rs, Divan, `perf` 플레임그래프, PGO, CI에서의 지속적인 벤치마킹 |
| 4 | [코드 커버리지 — 테스트가 놓치는 부분 확인하기](ch04-code-coverage-seeing-what-tests-miss.md) | 🟢 | `cargo-llvm-cov`, `cargo-tarpaulin`, `grcov`, Codecov/Coveralls CI 통합 |
| 5 | [Miri, Valgrind 및 새니타이저](ch05-miri-valgrind-and-sanitizers-verifying-u.md) | 🔴 | MIR 인터프리터, Valgrind memcheck/Helgrind, ASan/MSan/TSan, cargo-fuzz, loom |

### 파트 III — 강화 및 최적화 (Harden & Optimize)

| # | 장 | 난이도 | 설명 |
|---|---------|:----------:|-------------|
| 6 | [의존성 관리 및 공급망 보안](ch06-dependency-management-and-supply-chain-s.md) | 🟢 | `cargo-audit`, `cargo-deny`, `cargo-vet`, `cargo-outdated`, `cargo-semver-checks` |
| 7 | [릴리스 프로필 및 바이너리 크기](ch07-release-profiles-and-binary-size.md) | 🟡 | 릴리스 프로필 구조, LTO 트레이드오프, `cargo-bloat`, `cargo-udeps` |
| 8 | [컴파일 시간 및 개발자 도구](ch08-compile-time-and-developer-tools.md) | 🟡 | `sccache`, `mold`, `cargo-nextest`, `cargo-expand`, `cargo-geiger`, 워크스페이스 린트, MSRV |
| 9 | [`no_std` 및 기능 검증](ch09-no-std-and-feature-verification.md) | 🔴 | `cargo-hack`, `core`/`alloc`/`std` 계층, 커스텀 패닉 핸들러, `no_std` 코드 테스트 |
| 10 | [Windows 및 조건부 컴파일](ch10-windows-and-conditional-compilation.md) | 🟡 | `#[cfg]` 패턴, `windows-sys`/`windows` 크레이트, `cargo-xwin`, 플랫폼 추상화 |

### 파트 IV — 통합 (Integrate)

| # | 장 | 난이도 | 설명 |
|---|---------|:----------:|-------------|
| 11 | [종합 정리 — 운영 환경용 CI/CD 파이프라인](ch11-putting-it-all-together-a-production-cic.md) | 🟡 | GitHub Actions 워크플로, `cargo-make`, pre-commit 훅, `cargo-dist`, 캡스톤 프로젝트 |
| 12 | [실전 팁과 요령](ch12-tricks-from-the-trenches.md) | 🟡 | 검증된 10가지 패턴: `deny(warnings)` 함정, 캐시 튜닝, 의존성 중복 제거, RUSTFLAGS 등 |
| 13 | [빠른 참조 카드](ch13-quick-reference-card.md) | — | 주요 명령어 요약, 60개 이상의 의사결정 테이블 항목, 추가 학습 링크 |
