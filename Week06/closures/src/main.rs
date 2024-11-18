fn main() {
    example();
    println!();
    example02();
    println!();
    move_value_and_catch_by_ref();
    println!();
    move_value_and_catch_by_ref02();
    println!();
    fn_mut_example();
    println!();
    change_captured_variable();
    println!();
    fn_once_example();
    println!();
}

// CLOSURES

// Синтаксис
// |x: u32| -> u32 { x + 1 }
// |x| { x + 1}
// |x| x + 1

fn closure_capture() {
    let other = String::from("foo");                    // <-+
    //   |
    Some("bar").map(|s| s.len() + other.len());             // <-+
}

// Зад кулисите компилатора създава една структура и една функция
// Структура с която запомянваме променливите, които сме прихванали
struct State {
    other: String,
}

impl State {
    // Функция, която съдържа логиката
    fn call(&self, s: &str) -> usize {
        s.len() + self.other.len()
    }
}

// Как бихме използвали нашата имплементация
fn map_option(opt: Option<&str>, f: State) -> Option<usize> {
    match opt {
        Some(s) => Some(f.call(s)),
        None => None
    }
}

fn consume_map_option() {
    let other = String::from("foo");

    map_option(Some("bar"), State { other });
}

// Closure-ите за разлика от нашата имплементация, не консумират прихванатите променливи по подразбиране
fn consume_closure_variables() {
    let other = String::from("foo");

    println!("{:?}", Some("bar").map(|s| s.len() + other.len()));
    println!("{:?}", other); // OK

    println!("{:?}", map_option(Some("bar"), State { other }));
    // println!("{:?}", other) // грешка use of moved value `other`
}

// Всъщност имплементацията изглежда така
struct State2<'a> {
    other: &'a String,
}

impl<'a> State2<'a> {
    fn call(&self, s: &str) -> usize {
        s.len() + self.other.len()
    }
}

// Всъщност и това не е вярно
// Ако променливата се използва зад референция, ще я прихване по референция
fn example() {
    let nums = vec![1, 2, 3];

    let f = || {
        for n in nums.iter() { // Vec::iter(&self)
            println!("{}", n)       // ==> прихваща nums като &Vec<i32>
        }
    };

    f();
    println!("{:?}", nums);
}

fn example02() {
    let nums = vec![1, 2, 3];

    let f = || {
        for n in nums.into_iter() { // Vec::into_iter(self)
            println!("{}", n)            // ==> прихваща nums като Vec<i32>
        }
    };

    f();
    // println!("{:?}", nums); // ERROR!
}

// Можем да променим семантиката с ключовата дума move
fn move_closure() {
    let other = String::from("foo");

    println!("{:?}", Some("bar").map(|s| s.len() + other.len()));
    println!("{:?}", other); // OK

    println!("{:?}", Some("bar").map(move |s| s.len() + other.len()));
    // println!("{:?}", other); // грешка: use of moved value: `other`
}

/*
let closure = |s| s.len() + other.len();

// генерира
struct State<'a> {
    other: &'a String
}

let closure = move |s| s.len() + other.len();

// генерира
struct State {
    other: String
}
 */

// Ако искаме да преместим някоя стойност, но да прихванем друга по референция:
fn move_value_and_catch_by_ref() {
    let nums = vec![0, 1, 2, 3];
    let cookies = String::from("cookies");

    let f = || {
        // move `nums`
        let nums = nums;

        println!("{:?}", nums);
        println!("{:?}", cookies);
    };

    //println!("{:?}", nums); error -> moved in closure
    println!("{:?}", cookies);
}

// Друг вариант използвайки move closure
fn move_value_and_catch_by_ref02() {
    let nums = vec![0, 1, 2, 3];
    let cookies = String::from("cookies");

    let f = {
        let cookies_ref = &cookies; // ще премести cookies_ref във функцията,
        // но типа на cookies_ref e &String

        move || {
            println!("{:?}", nums);
            println!("{:?}", cookies_ref);
        }
    };
    //println!("{:?}", nums);
    println!("{:?}", cookies);
    f();
}

// CLOSURES КАТО АРГУМЕНТИ

// Как да подадем анонимна функция като аргумент?
// fn map_option(opt: Option<&str>, f:???) -> Option<u32> {
//      match opt {
//          Some(s) => Some(f(s)),
//          None => None
// }

// fn map_option<F>(opt: Option<&str>, f: F) -> Option<u32>
// where F: FnOnce(&str) -> u32
// {
//     match opt {
//         Some(s) => Some(f(s)),
//         None => None
//     }
// }

// FN TRAITS

// Fn
// FnMut
// FnOnce

// Имат специален синтаксис, например:
// Fn()
// FnMut(u32, u32) -> bool
// FnOnce() -> String

trait FnOnce<Args> {
    type Output;
    fn call_once(self, args: Args) -> Self::Output;
}

trait FnMut<Args>: FnOnce<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

trait Fn<Args>: FnMut<Args> {
    fn call(&self, args: Args) -> Self::Output;
}

// FN

// &self
// Функцията се нуждае само от споделена референция към прихванатото състояние
// Може да чете състоянието, но не може да го модифицира

// FNMUT

// &mut self
// Функцията се нуждае от ексклузивна референция към прихванатото състояние
// Може да променя състоянието
// Или да променя външни променливи, прихванати по &mut T

// Пример функция, която си променя състояние
fn fn_mut_example() {
    let mut counter = 0;

    let mut f = move || {
        counter += 1;
        counter
    };

    println!("{:?}", f());
    println!("{:?}", f());
    println!("{:?}", f());
}

// Пример - функция, която променя външна прихваната променлива
fn change_captured_variable() {
    let mut buffer = String::new();

    let mut f = |text| {
        buffer.push_str(text);
        buffer.len()
    };

    println!("{}", f("foo"));
    println!("{}", f("bar"));
    println!("{}", buffer);
}

// FNONCE

// self
// функцията консумира прихванатото състояние
// може да се извика само веднъж

fn fn_once_example() {
    let nums = vec![1, 2, 3];

    let f = || {
        for n in nums.into_iter() {
            println!("{}", n);
        }
    };

    f();
    // f(); this will throw error!
}

// Когато създадем closure, компилатора имплементира всички trait-ове, които може

// -> изисква ownership -- FnOnce
// -> изисква &mut -- FnOnce + FnMut
// -> изисква & -- FnOnce + FnMut + Fn

// Защо?

// -> имаме state -- можем да извикаме call(&state), call_mut(&mut state) и call_once(state)
// -> имаме &mut state -- можем да извикаме call(&state), call_mut(&mut state)
// -> имаме &state -- можем да извикаме call(&state)

// Кои трейтове да използваме?

// Ако приемаме closure като аргумент:
// -> FnOnce -> FnMut -> Fn
// Ако връщаме closure:
// -> Fn -> FnMut -> FnOnce

// fn и Fn

// fn е указател към функция (function pointer)
// функция без състояние

// fn типовете имплементират всички Fn traits
// -> Можем да подаваме нормални функции на места, където се очаква тип, имплементиращ някой Fn trait
// Closures, които не прихващат нищо, могат да се конвертират до fn

// let add_one: fn(u32) -> u32 = |x| x + 1


// Closures като резултат
// Как да върнем анонимна функция като резултат?
// fn make_auto_incrementer() -> ? ? ? {
//      let mut counter = 0;
//      move | | {
//      counter += 1;
//      counter
//      }
// }

// Чрез trait object
fn make_auto_incrementer() -> Box<dyn std::ops::FnMut() -> u32> {
    let mut counter = 0;
    Box::new(move || {
        counter += 1;
        counter
    })
}

fn test_closure_return_function() {
    let mut inc = make_auto_incrementer();
    println!("{}", inc());
    println!("{}", inc());
    println!("{}", inc());
}

// Чрез impl Trait

fn make_auto_incrementer02() -> impl std::ops::FnMut() -> u32 {
    let mut counter = 0;
    move || {
        counter += 1;
        counter
    }
}
// Фунцкията връща конкретен тип
// Компилатора генерира анонимен тип спрямо тялото на функцията
// За типа се знае единствено, че имплементира зададените trait-ове
// (в случая FnMut() -> u32)

// Удобно за използване когато:
// -> не можем да назовем типа на резултата (напр. closure, impl Trait върнат от другат функция)
// -> не искаме да назовем типа, защото е много сложен (напр. вложени итератори и адаптори)
// -> искаме да скрием имплементационен детайл, но не можем да си позволим алокацията за Box<dyn Trait>

fn test_closure_return_function02() {
    let mut inc = make_auto_incrementer02();
    println!("{}", inc());
    println!("{}", inc());
    println!("{}", inc());
}

// impl Trait като аргумент

// impl Trait може да се напише и в позицията на аргумент.
// Тогава е (почти) еквивалентен на generic parameter

fn print1(t: impl std::fmt::Display) {
    println!("{}", t)
}

fn print2<T: std::fmt::Display>(t: T) {
    println!("{}", t);
}

// Разликата -- за print2 можем да използваме turbofish - print2::<String>(s);
// за print1 не можем
