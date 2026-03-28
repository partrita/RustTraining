# 진정한 불변성 vs Record의 환상

> **학습 목표:** C#의 `record` 타입이 왜 진정한 의미의 불변이 아닌지(얕은 불변성, 리플렉션 우회 등) 분석하고, Rust가 컴파일 타임에 어떻게 **깊은 불변성(Deep Immutability)**을 강제하는지 배웁니다. 또한 성능 최적화를 위한 구조적 공유(Structural Sharing) 패턴을 익힙니다.

---

### 1. C# Record: 얕은 불변성의 한계
C#의 `record`는 편리하지만, 참조 타입 필드가 포함되는 순간 '불변'의 약속은 깨지기 쉽습니다.

```csharp
// [C# 상황] record는 겉모습만 불변일 수 있습니다.
public record Config(string Host, List<string> Origins);

var config = new Config("localhost", new List<string> { "a.com" });

// 'with' 키워드로 새 객체를 만드는 것 같지만...
var newConfig = config with { Host = "127.0.0.1" };

// 내부 리스트는 여전히 가변적이며, 두 객체가 같은 리스트를 공유합니다!
config.Origins.Add("evil.com"); 

// 결과적으로 newConfig의 Origins도 소리 없이 변경됩니다. (버그의 온상)
Console.WriteLine(newConfig.Origins.Count); // 2!
```

---

### 2. Rust: 컴파일러가 보장하는 깊은 불변성
Rust에서 `let`으로 선언된 변수는 그 내부에 포함된 모든 데이터(트리 전체)를 불변으로 만듭니다.

```rust
// [Rust 상황] 진정한 불변성 강제
struct Config {
    host: String,
    origins: Vec<String>,
}

let config = Config {
    host: "localhost".to_string(),
    origins: vec!["a.com".to_string()],
};

// 다음 시도는 컴파일 에러를 발생시킵니다.
// config.origins.push("evil.com".to_string()); 
// ❌ 에러: 불변 데이터의 내부를 수정할 수 없습니다.
```

---

### 3. 구조적 공유와 효율적인 업데이트
데이터가 클 때 매번 전체를 복사하는 것은 비효율적입니다. Rust는 **`Rc<T>`**나 **`Arc<T>`**를 사용하여 읽기 전용 데이터를 안전하게 공유하면서, 필요한 부분만 새롭게 생성하는 패턴을 즐겨 사용합니다.

- **C#**: `ImmutableList` 등을 쓰려면 라이브러리 의존성과 성능 오버헤드가 큽니다.
- **Rust**: 소유권 모델 덕분에 공유 참조(`&T`)를 넘기는 것만으로도 추가 비용 없이 안전한 공유가 가능합니다.

---

### 💡 C# 개발자를 위한 사고 전환
C#에서 "이 객체가 변하지 않았을까?"를 걱정하며 방어적 복사(Defensive Copy)를 하던 습관을 버리세요. Rust에서는 **컴파일러가 당신의 뒷배가 되어줍니다.** `mut`이 붙지 않은 변수는 세대를 거쳐 전달되어도 그 내용이 절대 변하지 않음을 보장받을 수 있습니다.

---

### 📝 실습 연습: 불변성 체감하기

🟡 **중급 과정** — 아래 작업을 수행해 보세요.
1. `Config` 구조체를 정의하고 `host`, `port`, `tags(Vec<String>)` 필드를 넣으세요.
2. `let`으로 변수를 선언하고 `tags`에 새 항목을 추가해 보세요. 컴파일 에러 메시지를 확인합니다.
3. `mut`을 사용하여 명시적인 가변 복사본을 만드는 과정을 구현해 보세요.

