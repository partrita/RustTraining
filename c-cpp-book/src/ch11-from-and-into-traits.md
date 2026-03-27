# Rust From 및 Into 트레이트

> **학습 내용:** Rust의 타입 변환 트레이트 — 실패하지 않는 변환을 위한 `From<T>` 및 `Into<T>`, 실패할 수 있는 변환을 위한 `TryFrom` 및 `TryInto`를 배웁니다. `From`을 구현하면 `Into`를 공짜로 얻게 됩니다. C++의 변환 연산자와 생성자를 대체합니다.

- ```From```과 ```Into```는 타입 변환을 용이하게 하는 상호 보완적인 트레이트입니다.
- 타입들은 일반적으로 ```From``` 트레이트를 구현합니다. ```String::from()```은 "&str"을 ```String```으로 변환하며, 컴파일러는 자동으로 ```&str.into```를 유도해낼 수 있습니다.
```rust
struct Point {x: u32, y: u32}
// 튜플로부터 Point를 생성
impl From<(u32, u32)> for Point {
    fn from(xy : (u32, u32)) -> Self {
        Point {x : xy.0, y: xy.1}       // 튜플 요소를 사용하여 Point 생성
    }
}
fn main() {
    let s = String::from("Rust");
    let x = u32::from(true);
    let p = Point::from((40, 42));
    // let p : Point = (40, 42).into(); // 위의 코드와 다른 형태의 동일한 표현
    println!("s: {s} x:{x} p.x:{} p.y {}", p.x, p.y);   
}
```

# 연습 문제: From 및 Into
- ```Point```를 ```TransposePoint```라는 타입으로 변환하는 ```From``` 트레이트를 구현하세요. ```TransposePoint```는 ```Point```의 ```x```와 ```y``` 요소를 서로 바꿉니다.

<details><summary>풀이 (클릭하여 확장)</summary>

```rust
struct Point { x: u32, y: u32 }
struct TransposePoint { x: u32, y: u32 }

impl From<Point> for TransposePoint {
    fn from(p: Point) -> Self {
        TransposePoint { x: p.y, y: p.x }
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };
    let tp = TransposePoint::from(p);
    println!("전치됨: x={}, y={}", tp.x, tp.y);  // x=20, y=10

    // .into() 사용 — From이 구현되면 자동으로 작동합니다.
    let p2 = Point { x: 3, y: 7 };
    let tp2: TransposePoint = p2.into();
    println!("전치됨: x={}, y={}", tp2.x, tp2.y);  // x=7, y=3
}
// 출력:
// 전치됨: x=20, y=10
// 전치됨: x=7, y=3
```

</details>

# Rust Default 트레이트
- ```Default```는 타입의 기본값을 구현하는 데 사용될 수 있습니다.
    - 타입들은 ```Derive``` 매크로를 사용하여 ```Default```를 사용하거나 직접 구현을 제공할 수 있습니다.
```rust
#[derive(Default, Debug)]
struct Point {x: u32, y: u32}
#[derive(Debug)]
struct CustomPoint {x: u32, y: u32}
impl Default for CustomPoint {
    fn default() -> Self {
        CustomPoint {x: 42, y: 42}
    }
}
fn main() {
    let x = Point::default();   // Point{0, 0} 생성
    println!("{x:?}");
    let y = CustomPoint::default();
    println!("{y:?}");
}
```

### Rust Default 트레이트
- ```Default``` 트레이트는 다음과 같은 여러 사용 사례가 있습니다.
    - 일부만 복사하고 나머지는 기본 초기화를 사용하는 경우
    - ```unwrap_or_default()```와 같은 메서드에서 ```Option``` 타입에 대한 기본 대안으로 사용
```rust
#[derive(Debug)]
struct CustomPoint {x: u32, y: u32}
impl Default for CustomPoint {
    fn default() -> Self {
        CustomPoint {x: 42, y: 42}
    }
}
fn main() {
    let x = CustomPoint::default();
    // y는 덮어쓰고, 나머지 요소들은 기본값을 사용함
    let y = CustomPoint {y: 43, ..CustomPoint::default()};
    println!("{x:?} {y:?}");
    let z : Option<CustomPoint> = None;
    // unwrap_or_default()를 unwrap()으로 바꿔보세요.
    println!("{:?}", z.unwrap_or_default());
}
```

### 기타 Rust 타입 변환
- Rust는 암시적 타입 변환을 지원하지 않으며, ```명시적``` 변환을 위해 ```as```를 사용할 수 있습니다.
- ```as```는 축소 변환(narrowing) 등에 의한 데이터 손실 가능성이 있으므로 신중하게 사용해야 합니다. 일반적으로 가능하면 ```into()```나 ```from()```을 사용하는 것이 바람직합니다.
```rust
fn main() {
    let f = 42u8;
    // let g : u32 = f;    // 컴파일되지 않음
    let g = f as u32;      // OK, 하지만 권장되지 않음. 축소 변환 규칙이 적용됨
    let g : u32 = f.into(); // 가장 권장되는 형태; 실패하지 않으며 컴파일러에 의해 체크됨
    //let k : u8 = f.into();  // 컴파일 실패; 축소 변환은 데이터 손실을 초래할 수 있음
    
    // 축소 변환 작업을 시도하려면 try_into를 사용해야 합니다.
    if let Ok(k) = TryInto::<u8>::try_into(g) {
        println!("{k}");
    }
}
```
