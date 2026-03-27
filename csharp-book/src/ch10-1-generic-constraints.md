## 제네릭 제약 조건: where vs 트레이트 바운드(Trait Bounds)

> **학습 내용:** Rust의 트레이트 바운드와 C#의 `where` 제약 조건 비교, `where` 절 문법, 조건부 트레이트 구현, 연관 타입(Associated types), 그리고 고차 트레이트 바운드(HRTBs).
>
> **난이도:** 🔴 고급

### C#의 제네릭 제약 조건
```csharp
// where 절을 사용한 C# 제네릭 제약 조건
public class Repository<T> where T : class, IEntity, new()
{
    public T Create()
    {
        return new T();  // new() 제약 조건이 있어야 매개변수 없는 생성자 호출 가능
    }
    
    public void Save(T entity)
    {
        if (entity.Id == 0)  // IEntity 제약 조건이 Id 속성을 제공함
        {
            entity.Id = GenerateId();
        }
        // 데이터베이스에 저장
    }
}

// 여러 타입 매개변수와 제약 조건
public class Converter<TInput, TOutput> 
    where TInput : IConvertible
    where TOutput : class, new()
{
    public TOutput Convert(TInput input)
    {
        var output = new TOutput();
        // IConvertible을 사용한 변환 로직
        return output;
    }
}

// 제네릭의 가변성(Variance)
public interface IRepository<out T> where T : IEntity
{
    IEnumerable<T> GetAll();  // 공변성(Covariant) - 더 파생된 타입을 반환 가능
}

public interface IWriter<in T> where T : IEntity
{
    void Write(T entity);  // 반공변성(Contravariant) - 더 기본 타입을 수용 가능
}
```

### 트레이트 바운드를 사용한 Rust의 제네릭 제약 조건
```rust
use std::fmt::{Debug, Display};
use std::clone::Clone;

// 기본적인 트레이트 바운드
pub struct Repository<T> 
where 
    T: Clone + Debug + Default,
{
    items: Vec<T>,
}

impl<T> Repository<T> 
where 
    T: Clone + Debug + Default,
{
    pub fn new() -> Self {
        Repository { items: Vec::new() }
    }
    
    pub fn create(&self) -> T {
        T::default()  // Default 트레이트가 기본값을 제공함 (C#의 new()와 유사)
    }
    
    pub fn add(&mut self, item: T) {
        println!("항목 추가 중: {:?}", item);  // 출력을 위한 Debug 트레이트
        self.items.push(item);
    }
    
    pub fn get_all(&self) -> Vec<T> {
        self.items.clone()  // 복제를 위한 Clone 트레이트
    }
}

// 다양한 문법을 사용한 다중 트레이트 바운드
pub fn process_data<T, U>(input: T) -> U 
where 
    T: Display + Clone,
    U: From<T> + Debug,
{
    println!("처리 중: {}", input);  // Display 트레이트
    let cloned = input.clone();         // Clone 트레이트
    let output = U::from(cloned);       // 변환을 위한 From 트레이트
    println!("결과: {:?}", output);   // Debug 트레이트
    output
}

// 연관 타입 (C#의 제네릭 제약 조건과 유사한 역할)
pub trait Iterator {
    type Item;  // 제네릭 매개변수 대신 연관 타입을 사용
    
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Collect<T> {
    fn collect<I: Iterator<Item = T>>(iter: I) -> Self;
}

// 고차 트레이트 바운드 (HRTBs, 고급 기능)
fn apply_to_all<F>(items: &[String], f: F) -> Vec<String>
where 
    F: for<'a> Fn(&'a str) -> String,  // 모든 수명(lifetime)에 대해 작동하는 함수
{
    items.iter().map(|s| f(s)).collect()
}

// 조건부 트레이트 구현
impl<T> PartialEq for Repository<T> 
where 
    T: PartialEq + Clone + Debug + Default,
{
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}
```

```mermaid
graph TD
    subgraph "C# 제네릭 제약 조건"
        CS_WHERE["where T : class, IInterface, new()"]
        CS_RUNTIME["[단점] 일부 런타임 타입 검사 발생<br/>가상 메서드 디스패치"]
        CS_VARIANCE["[장점] 공변성/반공변성 지원<br/>in/out 키워드"]
        CS_REFLECTION["[단점] 런타임 리플렉션 가능<br/>typeof(T), is, as 연산자"]
        CS_BOXING["[단점] 인터페이스 제약 조건을 위한<br/>값 타입 박싱 발생 가능"]
        
        CS_WHERE --> CS_RUNTIME
        CS_WHERE --> CS_VARIANCE
        CS_WHERE --> CS_REFLECTION
        CS_WHERE --> CS_BOXING
    end
    
    subgraph "Rust 트레이트 바운드"
        RUST_WHERE["where T: Trait + Clone + Debug"]
        RUST_COMPILE["[장점] 컴파일 타임 결정<br/>단형성화(Monomorphization)"]
        RUST_ZERO["[장점] 제로 비용 추상화<br/>런타임 오버헤드 없음"]
        RUST_ASSOCIATED["[장점] 연관 타입 지원<br/>제네릭보다 유연함"]
        RUST_HKT["[장점] 고차 트레이트 바운드<br/>고급 타입 관계 정의"]
        
        RUST_WHERE --> RUST_COMPILE
        RUST_WHERE --> RUST_ZERO
        RUST_WHERE --> RUST_ASSOCIATED
        RUST_WHERE --> RUST_HKT
    end
    
    subgraph "유연성 비교"
        CS_FLEX["C#의 유연성<br/>[장점] 가변성(Variance)<br/>[장점] 런타임 타입 정보<br/>[단점] 성능 오버헤드"]
        RUST_FLEX["Rust의 유연성<br/>[장점] 제로 비용<br/>[장점] 컴파일 타임 안전성<br/>[단점] 가변성 미지원(현재)"]
    end
    
    style CS_RUNTIME fill:#fff3e0,color:#000
    style CS_BOXING fill:#ffcdd2,color:#000
    style RUST_COMPILE fill:#c8e6c9,color:#000
    style RUST_ZERO fill:#c8e6c9,color:#000
    style CS_FLEX fill:#e3f2fd,color:#000
    style RUST_FLEX fill:#c8e6c9,color:#000
```

---

## 연습 문제

<details>
<summary><strong>🏋️ 연습 문제: 제네릭 리포지토리(Repository)</strong> (클릭하여 확장)</summary>

다음 C# 제네릭 리포지토리 인터페이스를 Rust 트레이트로 변환하십시오.

```csharp
public interface IRepository<T> where T : IEntity, new()
{
    T GetById(int id);
    IEnumerable<T> Find(Func<T, bool> predicate);
    void Save(T entity);
}
```

요구사항:
1. `fn id(&self) -> u64`를 가진 `Entity` 트레이트를 정의하십시오.
2. `T: Entity + Clone`인 `Repository<T>` 트레이트를 정의하십시오.
3. 항목을 `Vec<T>`에 저장하는 `InMemoryRepository<T>`를 구현하십시오.
4. `find` 메서드는 `impl Fn(&T) -> bool`을 인자로 받아야 합니다.

<details>
<summary>🔑 정답</summary>

```rust
trait Entity: Clone {
    fn id(&self) -> u64;
}

trait Repository<T: Entity> {
    fn get_by_id(&self, id: u64) -> Option<&T>;
    fn find(&self, predicate: impl Fn(&T) -> bool) -> Vec<&T>;
    fn save(&mut self, entity: T);
}

struct InMemoryRepository<T> {
    items: Vec<T>,
}

impl<T: Entity> InMemoryRepository<T> {
    fn new() -> Self { Self { items: Vec::new() } }
}

impl<T: Entity> Repository<T> for InMemoryRepository<T> {
    fn get_by_id(&self, id: u64) -> Option<&T> {
        self.items.iter().find(|item| item.id() == id)
    }
    fn find(&self, predicate: impl Fn(&T) -> bool) -> Vec<&T> {
        self.items.iter().filter(|item| predicate(item)).collect()
    }
    fn save(&mut self, entity: T) {
        if let Some(pos) = self.items.iter().position(|e| e.id() == entity.id()) {
            self.items[pos] = entity;
        } else {
            self.items.push(entity);
        }
    }
}

#[derive(Clone, Debug)]
struct User { user_id: u64, name: String }

impl Entity for User {
    fn id(&self) -> u64 { self.user_id }
}

fn main() {
    let mut repo = InMemoryRepository::new();
    repo.save(User { user_id: 1, name: "Alice".into() });
    repo.save(User { user_id: 2, name: "Bob".into() });

    let found = repo.find(|u| u.name.starts_with('A'));
    assert_eq!(found.len(), 1);
}
```

**C#과의 주요 차이점**: `new()` 제약 조건 대신 `Default` 트레이트를 사용합니다. `Func<T, bool>` 대신 `Fn(&T) -> bool`을 사용합니다. 예외를 던지는 대신 `Option`을 반환합니다.

</details>
</details>

***
