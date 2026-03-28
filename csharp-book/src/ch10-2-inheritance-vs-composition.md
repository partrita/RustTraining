# 상속보다는 구성: 객체 지향의 새로운 패러다임

> **학습 목표:** Rust에 왜 클래스 상속이 없는지 그 철학적 이유를 이해하고, **트레이트(Trait)**와 **구조체(Struct)**의 조합(Composition)이 어떻게 더 유연하고 안전한 설계를 가능하게 하는지 배웁니다.

---

### 1. 상속(Inheritance)의 한계와 Rust의 선택
C#과 같은 고전적인 OOP 언어는 클래스 상속을 통해 코드를 재사용합니다. 하지만 이는 계층 구조가 깊어질수록 **강한 결합도(Tight Coupling)**와 **다이아몬드 문제** 같은 복잡성을 유발합니다.

Rust는 **"구성이 상속보다 낫다(Composition over Inheritance)"**는 원칙에 따라, 데이터(구조체)와 동작(트레이트)을 명확히 분리합니다.

---

### 2. 구성(Composition)을 통한 다형성
Rust에서는 상속 계층도를 그리는 대신, 필요한 기능을 트레이트로 정의하고 각 타입을 그 트레이트들로 '조립'합니다.

```rust
// [동작 정의]
trait Walkable { fn walk(&self); }
trait Swimmable { fn swim(&self); }

// [데이터 정의]
struct Duck { name: String }

// [기능 조립]
impl Walkable for Duck { fn walk(&self) { println!("뒤뚱뒤뚱"); } }
impl Swimmable for Duck { fn swim(&self) { println!("첨벙첨벙"); } }

// [다중 제약 조건 사용]
fn travel<T: Walkable + Swimmable>(animal: &T) {
    animal.walk();
    animal.swim();
}
```

---

### 3. 상속 vs 구성 비교

| **비교 항목** | **C# 클래스 상속** | **Rust 트레이트 구성** |
| :--- | :--- | :--- |
| **코드 재사용** | 부모 클래스의 멤버 상속 | 트레이트의 기본 구현(Default impl) |
| **결합도** | 매우 높음 (부모 변경 시 자식 영향) | 낮음 (독립적인 트레이트 구현) |
| **추상화 방식** | 'is-a' 관계 (고정적) | 'can-do' 관계 (유연함) |
| **실행 성능** | 가상 메서드 테이블(vtable) 비용 | 정적 디스패치 (제로 비용) |

---

### 💡 실무 팁: 수평적 확장성
C#에서 `Bird`와 `Plane`이 공통적으로 `Fly()` 기능을 가져야 한다면 공통 기반 클래스를 찾기 어렵거나 인터페이스를 써야 합니다. Rust에서는 단순히 `Flyable` 트레이트를 양쪽 구조체에 구현하기만 하면 됩니다. 타입 간의 혈연관계(상속)가 없어도 기능적 공통점만 있다면 얼마든지 함께 묶어 처리할 수 있습니다.

