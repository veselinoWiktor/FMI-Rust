fn main() {
    println!("Hello, world!");
    rust_variables();
    println!();
    variables_shadowing();
    println!();
    variables_mutability();
    println!();
    fundamental_types();
    println!();
    integers_in_other_numerical_systems();
    println!();
    floating_point_numbers();
    println!();
    boolean_type();
    println!();
    unit_type();
    println!();
    arrays();
    println!();
    tuple_type();
    println!();
    cast_types();
    println!();
    over_under_flow_in_debug();
    println!();
}

fn rust_variables() {
    // let NAME = VALUE;
    // let NAME: TYPE = VALUE;

    let _x = 5;
    let _y: i32 = 3;

    // Rust е статично типизиран език, но можем да не пишем типа, ако е ясен от контекста
    let _z: i32 = 5; // указваме типа на _z
    let _w = _z; // типа на '_w' е 'i32', защото '_z' е 'i32'

    let _v = 5; // типа на '_v' e 'i32', защото '_c' е 'i32'
    let _c: i32 = _v;
}

fn variables_shadowing() {
    let x = 10;
    println!("{x}");
    let x = x + 10;
    println!("{x}");
    let x = x * 3;
    println!("{x}");
}

fn variables_mutability() {
    // Променливите са mutable по подразбиране
    let _x = 5;
    // x += 1; Този ред ще хвърли компилационна грешка

    let mut z = 5;
    z += 1; // Това вече се компилира
    println!("{z}");
}

fn fundamental_types() {
    // Цяло число
    let x: i32 = 42;
    println!("{x}");

    // Специфичен тип
    let x = 42u32;
    println!("{x}");

    // Големи числа
    let x = 135_245;
    println!("{x}");
}

fn integers_in_other_numerical_systems() {
    // Hex
    let x = 0xFFFFFF;
    println!("{x}");

    // Octal
    let x = 0o77;
    println!("{x}");

    // Binary
    let x = 0b1010101010000;
    println!("{x}");
}

fn floating_point_numbers() {
    let x = 3.14;
    println!("{x}");
}

fn boolean_type() {
    let x = true;
    println!("{x}");
}

fn unit_type() {
    // тип ()
    // стойност ()
    // тип с една единствена стойност
    // големина 0 байта, не носи информация
    // използва се за функции, които не връщат стойност и на други места

    let x: () = ();
    println!("{x:?}");
}

fn arrays() {
    let arr: [i32; 3] = [1, 2, 3];

    let nested: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

    println!("{}", arr[2]);
    println!("{}", nested[0][0]);
}

fn tuple_type() {
    let tuple: (i32, u32, bool) = (1, 2, false);

    println!("{}", tuple.0);
    println!("{}", tuple.1);
    println!("{}", tuple.2);
}

fn cast_types()
{
    // Няма неявно конвертиране между различни числови типове
    // let x: i32 = 1; \ не се компилира
    // let y: u64 = x; /

    // Аритметичните операции не могат да се прилагат върху различни типове
    // let x = 4_u32 - 1_u8; Не се компилира

    // За конвертиране между типове се използва ключовата дума as
    let one = true as u8;
    let two_hundred = -56_i8 as u8;
    let three = 3.14 as u32;

    println!("one: {one}\ntwo_hundred: {two_hundred}\nthree: {three}");
}

fn over_under_flow_in_debug()
{
    let x = 255_u8;
    let y = x + 1;
    println!("{y}");
}

// НЯМА ОПЕРАТОРИ -- И ++

/* Rust поддържа и блокови коментари */