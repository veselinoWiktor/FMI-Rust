fn main() {
    no_dicriminant();
    println!();
    C_like_enum_size();
    println!();
    non_zero_optimization();
    println!();
    option_enum();
    println!();
}

// Скучно име, идващо от C, където са доста ограничени
// По-готини имена са "algebric datatype" и "sum type"
// Също известно като "tagged union"
enum IpAddrKind {
    V4,
    V6,
}

fn enum_instance() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

fn route(ip_type: IpAddrKind) {}

fn enum_as_parameter() {
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn enum_in_data() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

fn enum_in_data_readable() {
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));
}

// Можем да спестим памет като знаем, че IPv4 използва стойности от 0-255
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn enum_data_v3() {
    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));
}

// Повечето типове са "контейнери" - struct, list, tuple
struct User2 {
    username: String,
    email: String,
    sign_in_count: u64,
}
// множеството от възможни стойности за 'User' е декартово произведение
// String x String x u64

// Могат да се използват взаимно изключващи се данни
struct IpAddr4 {
    v4: (u8, u8, u8, u8),
    v6: String,
}

// Warning: фалшив Rust код, няма NULL
// let home = IpAddr4 { v4: (127,0,0,1), v6: NULL}
// let loopback = IpAddr4 {v4: NULL, v6: String::from("::1")}
// Не е особено ясно - човек трябва "да се усети" като чете кода

// Enum позволява да се изрази ексклузивност - или един пакет данни, или друг
enum IpAddr5 {
    V4(u8, u8, u8, u8),
    V6(String),
}
// Sum type - множество от възможни стойности е обединение на множества
// V4(u8, u8, u8, u8) обединено с V6(String)

// При struct е възможно да имаме стойности, които са:
//      Валидни побитово
//      Невалидни логически
// Чрез enum можем да изразим всички валидни стойности чрез типовата система
// make invalid states unrepresentable

enum Message {
    Quit,
    Move { x: i64, y: i64 },
    Write(String),
    ChangeColor(i64, i64, i64),
}

fn enum_variants() {
    Message::Quit;
    Message::Move { x: 3, y: 4 };
    Message::Write(String::from("baba"));
    Message::ChangeColor(255, 0, 0);
}

// Enum варианти като структури
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i64,
    y: i64,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i64, i64, i64); // tuple struct

fn enum_variant_as_structs() {
    QuitMessage;
    MoveMessage { x: 3, y: 4 };
    WriteMessage(String::from("baba"));
    ChangeColorMessage(255, 0, 0);
}

// enum Методи
impl Message {
    fn call(&self) {
        // ...
    }
}

fn enum_method_call() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

// Enum вариантите на Message имат:
// 8 байта за дискриминанта
// 24 байта за данните

// без дискриминанта
use std::mem;

enum Basic {
    A,
    B,
}

fn no_dicriminant() {
    println!("{:?}", mem::size_of::<Basic>());
}

// C-like enum

#[derive(Debug)]
#[repr(i32)]
enum Basic2{
    A = 0,
    B = 12,
}

fn C_like_enum_size() {
    println!("{:?}", Basic2::B);
    println!("{:?}", Basic2::B as i32);
    // println!("{:?}", 12 as Basic2);

    /* 
    error[E0605]: non-primitive cast: `i32` as `Basic2`
        --> src/main.rs:170:22
        |
    170 |     println!("{:?}", 12 as Basic2);
        |                      ^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
    */
}

#[derive(Debug)]
#[repr(i32)]
enum Basic3 {
    A = 0,
    B = 12,
}

impl Basic {
    fn from_i32(i: i32) -> Basic3 {
        match i {
            0 => Basic3::A,
            12 => Basic3::B,
            _ => panic!("грешка!")
        }
    }
}

// Non-zer оптимизация
enum LeanAndMean {
    A,
    B(String),
    C
}

fn non_zero_optimization() {
    println!("Without enum: {:?}", mem::size_of::<String>());
    println!("With enum:    {:?}", mem::size_of::<LeanAndMean>());
}
// String има поле (ptr), което не може да е нула/null
// Компилатора го използва като слот, където да сложи другия вариант (LeanAndMean::A)

fn option_enum() {
    let some_number = Some(5);
    let some_string = Some("string");
    let absent_number: Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", absent_number);
}

