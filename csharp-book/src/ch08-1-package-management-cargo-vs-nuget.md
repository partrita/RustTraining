## 패키지 관리: Cargo vs NuGet

> **학습 내용:** `Cargo.toml` vs `.csproj`, 버전 명시법, `Cargo.lock`,
> 조건부 컴파일을 위한 기능 플래그(feature flags), 그리고 NuGet/dotnet에 대응하는 주요 Cargo 명령어.
>
> **난이도:** 🟢 기초

### 의존성 선언(Dependency Declaration)

#### C# NuGet 의존성
```xml
<!-- MyApp.csproj -->
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
  </PropertyGroup>
  
  <PackageReference Include="Newtonsoft.Json" Version="13.0.3" />
  <PackageReference Include="Serilog" Version="3.0.1" />
  <PackageReference Include="Microsoft.AspNetCore.App" />
  
  <ProjectReference Include="../MyLibrary/MyLibrary.csproj" />
</Project>
```

#### Rust Cargo 의존성
```toml
# Cargo.toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"               # crates.io에서 가져옴 (NuGet과 유사)
serde = { version = "1.0", features = ["derive"] }  # 기능 플래그 포함
log = "0.4"
tokio = { version = "1.0", features = ["full"] }

# 로컬 의존성 (ProjectReference와 유사)
my_library = { path = "../my_library" }

# Git 의존성
my_git_crate = { git = "https://github.com/user/repo" }

# 개발용 의존성 (테스트용 패키지와 유사)
[dev-dependencies]
criterion = "0.5"               # 벤치마킹
proptest = "1.0"               # 속성 기반 테스트(Property testing)
```

### 버전 관리(Version Management)

#### C# 패키지 버전 관리
```xml
<!-- 중앙 집중식 패키지 관리 (Directory.Packages.props) -->
<Project>
  <PropertyGroup>
    <ManagePackageVersionsCentrally>true</ManagePackageVersionsCentrally>
  </PropertyGroup>
  
  <PackageVersion Include="Newtonsoft.Json" Version="13.0.3" />
  <PackageVersion Include="Serilog" Version="3.0.1" />
</Project>

<!-- 재현 가능한 빌드를 위한 packages.lock.json -->
```

#### Rust 버전 관리
```toml
# Cargo.toml - 시맨틱 버저닝(Semantic versioning)
[dependencies]
serde = "1.0"        # 1.x.x 버전과 호환 (>=1.0.0, <2.0.0)
log = "0.4.17"       # 0.4.x 버전과 호환 (>=0.4.17, <0.5.0)
regex = "=1.5.4"     # 정확히 일치하는 버전
chrono = "^0.4"      # 캐럿(Caret) 요구사항 (기본값)
uuid = "~1.3.0"      # 틸드(Tilde) 요구사항 (>=1.3.0, <1.4.0)

# Cargo.lock - 재현 가능한 빌드를 위한 정확한 버전 (자동 생성됨)
[[package]]
name = "serde"
version = "1.0.163"
# ... 정확한 의존성 트리 기록
```

### 패키지 소스(Package Sources)

#### C# 패키지 소스
```xml
<!-- nuget.config -->
<configuration>
  <packageSources>
    <add key="nuget.org" value="https://api.nuget.org/v3/index.json" />
    <add key="MyCompanyFeed" value="https://pkgs.dev.azure.com/company/_packaging/feed/nuget/v3/index.json" />
  </packageSources>
</configuration>
```

#### Rust 패키지 소스
```toml
# .cargo/config.toml
[source.crates-io]
replace-with = "my-awesome-registry"

[source.my-awesome-registry]
registry = "https://my-intranet:8080/index"

# 대체 레지스트리
[registries]
my-registry = { index = "https://my-intranet:8080/index" }

# Cargo.toml에서의 사용
[dependencies]
my_crate = { version = "1.0", registry = "my-registry" }
```

### 주요 명령어 비교

| 작업 | C# 명령어 | Rust 명령어 |
|------|------------|-------------|
| 패키지 복구 | `dotnet restore` | `cargo fetch` |
| 패키지 추가 | `dotnet add package Newtonsoft.Json` | `cargo add serde_json` |
| 패키지 제거 | `dotnet remove package Newtonsoft.Json` | `cargo remove serde_json` |
| 패키지 업데이트 | `dotnet update` | `cargo update` |
| 패키지 목록 확인 | `dotnet list package` | `cargo tree` |
| 보안 감사 | `dotnet list package --vulnerable` | `cargo audit` |
| 빌드 결과물 정리 | `dotnet clean` | `cargo clean` |

### 기능(Features): 조건부 컴파일

#### C# 조건부 컴파일
```csharp
#if DEBUG
    Console.WriteLine("디버그 모드");
#elif RELEASE
    Console.WriteLine("릴리스 모드");
#endif

// 프로젝트 파일의 기능 설정
<PropertyGroup Condition="'$(Configuration)'=='Debug'">
    <DefineConstants>DEBUG;TRACE</DefineConstants>
</PropertyGroup>
```

#### Rust 기능 게이트(Feature Gates)
```toml
# Cargo.toml
[features]
default = ["json"]              # 기본 기능
json = ["serde_json"]          # serde_json을 활성화하는 기능
xml = ["serde_xml"]            # 대체 직렬화 기능
advanced = ["json", "xml"]     # 복합 기능

[dependencies]
serde_json = { version = "1.0", optional = true }
serde_xml = { version = "0.4", optional = true }
```

```rust
// 기능을 기반으로 한 조건부 컴파일
#[cfg(feature = "json")]
use serde_json;

#[cfg(feature = "xml")]
use serde_xml;

pub fn serialize_data(data: &MyStruct) -> String {
    #[cfg(feature = "json")]
    return serde_json::to_string(data).unwrap();
    
    #[cfg(feature = "xml")]
    return serde_xml::to_string(data).unwrap();
    
    #[cfg(not(any(feature = "json", feature = "xml")))]
    return "직렬화 기능이 활성화되지 않았습니다".to_string();
}
```

### 외부 크레이트 사용하기

#### C# 개발자를 위한 인기 크레이트

| C# 라이브러리 | Rust 크레이트 | 용도 |
|------------|------------|---------|
| Newtonsoft.Json | `serde_json` | JSON 직렬화/역직렬화 |
| HttpClient | `reqwest` | HTTP 클라이언트 |
| Entity Framework | `diesel` / `sqlx` | ORM / SQL 툴킷 |
| NLog/Serilog | `log` + `env_logger` | 로깅 |
| xUnit/NUnit | 내장 `#[test]` | 유닛 테스트 |
| Moq | `mockall` | 모킹(Mocking) |
| Flurl | `url` | URL 조작 |
| Polly | `tower` | 탄력성 패턴(Resilience patterns) |

#### 예시: HTTP 클라이언트 마이그레이션
```csharp
// C# HttpClient 사용 예시
public class ApiClient
{
    private readonly HttpClient _httpClient;
    
    public async Task<User> GetUserAsync(int id)
    {
        var response = await _httpClient.GetAsync($"/users/{id}");
        var json = await response.Content.ReadAsStringAsync();
        return JsonConvert.DeserializeObject<User>(json);
    }
}
```

```rust
// Rust reqwest 사용 예시
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    id: u32,
    name: String,
}

struct ApiClient {
    client: reqwest::Client,
}

impl ApiClient {
    async fn get_user(&self, id: u32) -> Result<User, reqwest::Error> {
        let user = self.client
            .get(&format!("https://api.example.com/users/{}", id))
            .send()
            .await?
            .json::<User>()
            .await?;
        
        Ok(user)
    }
}
```

***
