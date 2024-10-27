fn main() {
    println!("identity<T>: {}; T=32", identity::<i32>(32));
    generic_method();
    println!();
    println!("dist = {:?}", Point::<f32> { x: 5.0, y: 10.0 }.dist_from_origin());
    generic_methods_example02();
}

fn _identity_i32(value: i32) -> i32 {
    value
}

fn _identity_i8(value: u8) -> u8 {
    value
}

fn identity<T>(value: T) -> T {
    value
}

// Можем да пишем по-сложни функции.. ама всъщност не
fn _sum<T>(a: T, _b: T) -> T {
    // a + b
    a
}
/*
error[E0369]: cannot add `T` to `T`
  --> src/main.rs:19:7
   |
19 |     a + b
   |     - ^ - T
   |     |
   |     T
   |
help: consider restricting type parameter `T`
   |
18 | fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
   |         +++++++++++++++++++++++++++
*/

// generic функцията трябва да е валидна за всички типове, с които може да се инстанцира
// sum<T> трябва да е валидна за всички възможни T
// но не всеки тип имплементира операцията +
// за целта трябва да ограничим T само до типове, които имплементират субиране

struct Point<T> {
    x: T,
    y: T,
}

fn _generic_structs() {
    // Може да я създадем с цели числа
    let _integer = Point::<i8> { x: 12, y: 12 };

    // или с числа с плаваща запетая.
    let _float = Point { x: 1.0, y: 4.0 };
}

// Ако искаме да позволим двете координати да са различни типове
struct PointTwo<T, U> {
    x: T,
    y: U,
}

fn _generic_struct_with_two_types() {
    let _both_integer = PointTwo { x: 5, y: 10 };
    let _both_float = PointTwo { x: 1.0, y: 4.0 };
    let _integer_and_string = PointTwo { x: 5, y: "4.0" };
}

// Generics enums
enum Message<T, A> {
    Text(T),
    Action(A)
}

// Generics methods
impl<T> Point<T> {
    fn coords(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}
// Може да го преведем както:
// за всяко T: impl Point<T> { ... }

fn generic_method() {
    let p = Point::<i32> { x: 5, y: 10 };
    println!("coords = {:?}", p.coords());
}

// Generics специализирани имплементации
// не е задължително да имплементираме методите за всички типове T

impl Point<f32> {
    fn dist_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T> PointTwo<T, T> {
    fn coords_1(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

impl<T, U> PointTwo<T, U> {
    fn coords_2(&self) -> (&T, &U) {
        (&self.x, &self.y)
    }

    fn mixup<V, W>(self, other: PointTwo<V, W>) -> PointTwo<T, W> {
        PointTwo { x: self.x, y: other.y }
    }
}

fn generic_methods_example02() {
    let p1 = PointTwo { x: 5, y: 10.4 };
    let p2 = PointTwo { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {:?}", p3.x);
    println!("p3.y = {:?}", p3.y);
}


// Константни Шаблони
fn f<T, const N: usize>(_a: [T; N]) {}

