fn main() {
    strings();
    println!();
    constant_string();
    println!();
    string_concatanations();
    println!();
    dynamic_strings();
    println!();
    initializing_dynamic_strings();
    println!();
    assignment_of_static_strings();
    println!();
    borrow_of_values();
    println!();
    arrays_and_slices();
    println!();
}

fn strings() {
    let a = "Hello";
    let b = a;

    println!("{a}");
    println!("{b} again!");
}

fn constant_string() {
    let a: &'static str = "Hello"; // "статична" референция към str
    println!("{a}"); // константа данна, не може да се променя
}

fn string_concatanations() {
    let _a = "Hello";
    let _b = " again!";
    // let message = a + b;
    /*
    error[E0369]: cannot add `&str` to `&str`
        --> src/main.rs:27:21
        |
        27 |     let message = a + b;
        |                   - ^ - &str
        |                   | |
        |                   | `+` cannot be used to concatenate two `&str` strings
        |                   &str
        |
        = note: string concatenation requires an owned `String` on the left
        help: create an owned `String` from a string reference
        |
        27 |     let message = a.to_owned() + b;
        |                    +++++++++++
     */
}

fn dynamic_strings() {
    let mut a = String::from("Hello"); // Динамично заделя памет на низа, стойноста може да се променя
    println!("{a}!");

    a.push_str(" again!");
    println!("{a}");
}

fn initializing_dynamic_strings() {
    // Следните начини за създване на динамичен низ са напълно еквивалентни
    let _s1 = String::from("Hello!");
    let _s2 = "Hello!".to_string();
    let _s3 = "Hello!".to_owned();
}

fn assignment_of_static_strings() {
    let a = "Hello";
    let b = a;

    println!("{a}!");
    println!("{b} again!")
}

fn assignment_of_dynamic_strings() {
    let s1 = String::from("Cookies!");
    let s2 = s1;

    // println!("{s1}");
    // println!("Mmm, {s2}");
    /*
    error[E0382]: borrow of moved value: `s1`
        --> src/bin/main_9bd3356d7798f6992589bc25a22e3d30f92002bb.rs:5:16
        |
        2 | let s1 = String::from("Cookies!");
        |     -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
        3 | let s2 = s1;
        |          -- value moved here
        4 |
        5 | println!("{}", s1);
        |                ^^ value borrowed here after move
        |
        = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        help: consider cloning the value if the performance cost is acceptable
        |
        3 | let s2 = s1.clone();
        |            ++++++++

        For more information about this error, try `rustc --explain E0382`.
        error: could not compile `rust` (bin "main_9bd3356d7798f6992589bc25a22e3d30f92002bb") due to 1 previous error
        */
}

fn owenership() {
    /*
    В някои други езици (с менaжирана памет)
    Foo myFoo = new Foo();

    myObj1.foo = myFoo;
    myObj2.foo = myFoo;
    няма концепция за собственост над стойност или обект
    всички места, от където се реферира обект, държат указател към динамично заделен обект
    всички места, от където се реферира обект, са равноправни
    */

    // В Rust (и други езици)
    let my_foo = String::new();
    {
        let _foo_ref = &my_foo;
    }

    // променливата my_foo има собственост над стойността
    // променливата my_foo оказва къде в паметта ще бъде записана стойността
    // стойността може да има само един собственик
    // могат да се вземат временни референции към стойността
    // (споделена собственост се имплементира с библиотечен тип)
}

fn ownership_lifetime() {
    let _str1 = String::new();
    let _str2 = String::new();
    // когато собственикът излезе от scope, се извиква деструктура на стойността
} // извиква се деструктура на 'str2', после и на 'str2'

fn move_semantics() {
    let s1 = String::from("Cookies!");
    // 's1' е собственик на низа "Cookies!"

    let s2 = s1;
    // 's2' е собственик на низа "Cookies!"

    println!("Mmm, {s2}");

    // Променливата 's2' става новия собственик, променливата s1 спира да е собственик
    // стойността не се копира
    // стойността не се променя

    // При преместване:
    // 1) Паметта на старта променлива се копира побитово в новата променлива
    // 2) Старата променлива се маркира като невалидна за компилатора
}

fn assignment_of_dynamic_strings_fix() {
    // Ако искаме да ползваме и двете променливи, трябва да направим копие на стойността

    let s1 = String::from("Cookies!");
    let s2 = s1.clone();

    println!("{s1}");
    println!("Mmm, {s2}");
}

fn functions_with_parameter_String() {
    let s = String::from("hello"); // Дефинираме 's'

    takes_ownership(s); // Стойността на 's' се мести във функцията и
                        // затова нев е валидна след този ред
} // Тук 's' излиза от scope, но 's' е преместена съответно не се деалокира.

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // 'some_string' излиза от scope и се освобождава паметта

fn functions_that_return_String() {
    let s1 = gives_ownership();
    let s2 = takes_and_gives_back(s1);

    println!("{s2}");
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello!");
    some_string // Преместваме стойността на функцията, която ни е извикала
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn function_which_doesnt_change_ownership() {
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.")
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

/* Референции

    Позволяват заемане на стойност за определен период

    споделена референция:
      immutable borrow
      тип &T
      взимане на референция: let ref_x = &x;
      дереференциране: *ref_x;

    ексклузивна референция
      mutable borrow
      тип &mut T
      взимане на референция let ref_mut_x = &mut x;
      дреференциране: *ref_mut_x;

    Референцията е указател - адрес в паметта
    който не е NULL!!!!
    който винаги сочи към валидна стойност:
       алокирана и инициализирана
       жива - не е преместена или деалокирана след взимането на референцията
       това се проверява по време на компилация
*/
fn borrow_of_values() {
    let x = 123;

    {
        let ref_x = &x;
        println!("x = {ref_x}");
    }

    println!("x = {x}");
}

fn borrow_of_values_validity() {
    // let r;

    {
        let s = String::from("hello");
        //r = &s; ERROR!!!
    }

    // println!("{r}");

    /*
    error[E0597]: `s` does not live long enough
        --> src/main.rs:243:13
            |
        242 |         let s = String::from("hello");
            |             - binding `s` declared here
        243 |         r = &s;
            |             ^^ borrowed value does not live long enough
        244 |     }
            |     - `s` dropped here while still borrowed
        245 |
        246 |     println!("{r}");
            |               --- borrow later used here

        For more information about this error, try `rustc --explain E0597`.
     */
}

// fn returns_string() -> &String {
//     let s = String::from("Hello");
//     &s
// }

/*
    error[E0106]: missing lifetime specifier
        --> src/main.rs:265:24
            |
        265 | fn returns_string() -> &String {
            |                        ^ expected named lifetime parameter
            |
            = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
        help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
            |
        265 | fn returns_string() -> &'static String {
            |                         +++++++
        help: instead, you are more likely to want to return an owned value
            |
        265 - fn returns_string() -> &String {
        265 + fn returns_string() -> String {
            |

    error[E0515]: cannot return reference to local variable `s`
        --> src/main.rs:267:5
            |
        267 |     &s
            |     ^^ returns a reference to data owned by the current function
*/

fn passing_in_function() {
    let s1 = String::from("Hello");
    let len = calculate_length2(&s1);

    println!("The length of '{s1}' is {len}.")
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn mutating_in_function() {
    let mut s = String::from("hello!");
    change(&mut s);
    println!("{s}");
}

fn change(str: &mut String) {
    str.push_str(", world");
}

fn reference_and_move() {
    let s1 = String::from("hello");
    let r = &s1;

    // let s2 = s1; Error comes from here
    println!("{r}");

    /*
    error[E0505]: cannot move out of `s1` because it is borrowed
        --> src/main.rs:323:14
            |
        320 |     let s1 = String::from("hello");
            |         -- binding `s1` declared here
        321 |     let r = &s1;
            |             --- borrow of `s1` occurs here
        322 |
        323 |     let s2 = s1;
            |              ^^ move out of `s1` occurs here
        324 |     println!("{r}");
            |               --- borrow later used here
            |
        help: consider cloning the value if the performance cost is acceptable
            |
        321 -     let r = &s1;
        321 +     let r = s1.clone();
            |
    */
}

fn ref_to_shadowed_variable() {
    let s = String::from("first");
    let r = &s;

    let s = String::from("second");

    println!("{r}");
    println!("{s}");
}

fn ref_to_shadowed_variable2() {
    let s = String::from("hello");
    let s = &s;

    println!("{s}");
}

fn ref_to_temp_value() {
    let s = &String::from("hello");
    println!("{s}");
}

fn ref_to_temp_value2() {
    let s = &mut String::from("hello");
    s.push_str(", world");

    println!("{s}");
}

//  Borrow checker ПРАВИЛО:
/*  По всяко време към една стойност може да съществувар най-много едно от следните:
       1) Точно една екслузивна референция (&mut T)
       2) Произволен брой споделени референции (&T)

   ЗАЩО?

   Голяма категория от проблеми са породени от "shared mutable state"
       1) data races при многонишкови програми
       2) други видове бъгове при еднонишкови
       3) невъзможност за локален анализ

   Някой езици (основно чисто функционалните) решават този проблем, като забраняват мутацията,
   но това води до други неудобства

   Rust решава проблема, като забранвява едновременното споделяне и мутация
       1) Споделяне без мутация през &T
       2) Мутация без споделяне през &mut T
*/

fn two_mutable_borrows() {
    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{r1} {r2}");

    /*
    error[E0499]: cannot borrow `s` as mutable more than once at a time
        --> src/main.rs:398:14
            |
        397 |     let r1 = &mut s;
            |              ------ first mutable borrow occurs here
        398 |     let r2 = &mut s;
            |              ^^^^^^ second mutable borrow occurs here
        399 |
        400 |     println!("{r1} {r2}");
            |               ---- first borrow later used here

        For more information about this error, try `rustc --explain E0499`.
    */
}

fn two_mutable_borrows_fix() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    println!("{r1}");

    let r2 = &mut s;
    println!("{r2}");

    // Решение е да не ги използваме заедно
    // Референцията е жива от момента на създването й до момента на последното използване
    // Не е задължена да живее до края на scope-а (non-lexical lifetimes)
}

fn classical_example_from_cpp() {
    let mut vec = vec![1, 2, 3];

    for val in vec.iter() {
        // Do something...

        // vec.push(99); ERROR
    }

    /*
    error[E0502]: cannot borrow `vec` as mutable because it is also borrowed as immutable
        --> src/main.rs:438:9
            |
        435 |     for val in vec.iter() {
            |                ----------
            |                |
            |                immutable borrow occurs here
            |                immutable borrow later used here
        ...
        438 |         vec.push(99);
            |         ^^^^^^^^^^^^ mutable borrow occurs here

        For more information about this error, try `rustc --explain E0502`.
     */
}

fn strings_and_slices() {
    let s = String::from("hello, world");

    let r1 = &s[1..4];
    println!("{r1}");

    let r2 = &s[..s.len() - 2]; // &s[..-2] не е възможно
    println!("{r2}");

    let r3 = &s[..];
    println!("{r3}");

    /*
    Интервали
        началото и краят могат да се изпуснат
            1) start..end
            2) start..
            3) ..end
            4) ..
        има и затворени интервалию
            1) start..=end
            2) ..=end
    */
}

fn arrays_and_slices() {
    // Типа [T; N]

    // Хомогенен масив с фиксиран брой елементи
    // Размера трябва да се знае по време на компилация

    let point_coords = [1.0, 3.5, 0.0]; // тип  [f64; 3]
    println!("{point_coords:?}");
}

fn syntax_for_array_fill() {
    // Синтаксис за попълване с фиксирана стойност
    // [<елемент>; <брой>]

    let zero = [0.0; 3]; // тип [f64; 3];
    println!("{zero:?}")
}

fn vector_example() {
    // Динамичен масив с елементи от тип T
    // Автоматично си разширява капацитета при добавяне на елементи

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("{v:?}");
}

fn macro_vector() {
    let v = vec![1, 2, 3];

    println!("{v:?}");

    let v2 = vec![0; 8];
    println!("{v2:?}");
    // [0, 0, 0, 0, 0, 0, 0, 0]
}

fn array_slices() {
    let arr = [2, 4, 6, 8, 10];
    let arr_slice = &arr[1..4]; // тип &[i32]
    println!("{arr_slice:?}");

    let v = vec![2, 4, 6, 8, 10];
    let vec_slice = &v[1..4];
    println!("{vec_slice:?}");

    // Типа &[T]
    // Резен от масив (slice)
    // Репрезентиран като (ptr, len)
}

fn slice_literals() {
    let slice = &[2, 4, 6, 8, 10];

    println!("{slice:?}");
}

fn mutable_slices() {
    let mut v = vec![2, 4, 6, 8, 10, 12];
    let slice = &mut v[1..4];

    for elem in slice.iter_mut() {
        *elem += 1;
    }

    println!("{slice:?}");
    println!("{v:?}")
}
