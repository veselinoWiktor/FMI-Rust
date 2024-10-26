fn main() {
    patter_matching_example();
    println!();
    match_returns_value();
    println!();
    match_return_from_fn();
    println!();
    match_supports_code_blocks();
    println!();
    match_should_cover_all_values();
    println!();
    match_everything_else();
    println!();
    match_problem();
    println!();
    if_let();
    println!();
    while_let();
    println!();
    if_let_and_while_let();
    println!();
    pattern_matching_guards();
    println!();
    pattern_matching_ranges();
    println!();
    pattern_matching_bindings();
    println!();
    pattern_matching_mutiple_patterns();
    println!();
    pattern_matching_structs();
    println!();
    pattern_matching_slices();
    println!();
}

fn patter_matching_example() {
    let x = Some(42_u32);

    match x {
        Some(val) => println!("Value: {}", val),
        None      => println!("No value found!"),
    }
}

fn match_returns_value() {
    let x = Some(4);

    // match може да върне стойност
    let y = match x {
        Some(val) => Some(val * val),
        None => None,
    };

    println!("{:?}", y);
}

fn match_return_from_fn() -> Option<i32> {
    let x = Some(4);

    // match може да върне стойност
    let y = match x {
        Some(val) => Some(val * val),
        None => return None,
    };

    println!("{:?}", y);
    y
}

fn match_supports_code_blocks() {
    let x= Some(20);
    let _y = match x {
        Some(val) => {
            println!("Will return {}", val * val);
            Some(val * val)
        },
        None => {
            println!("Will do nothing!!");
            None
        }
    };
}

fn match_should_cover_all_values() {
    // let x = Some(3);
    // let y = match x {
    //     Some(i) => Some(i + 1)
    // };

    /*
    error[E0004]: non-exhaustive patterns: `None` not covered
        --> src/main.rs:62:19
        |
    62  |     let y = match x {
        |                   ^ pattern `None` not covered
        |
    note: `Option<i32>` defined here
    --> C:\Users\vvese\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\option.rs:571:1
        |
    571 | pub enum Option<T> {
        | ^^^^^^^^^^^^^^^^^^
    ...
    575 |     None,
        |     ---- not covered
        = note: the matched value is of type `Option<i32>`
    help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
        |
    63  ~         Some(i) => Some(i + 1),
    64  +         None => todo!()
        |
    */
}

fn match_everything_else() {
    let x = 32;
    match x {
        69 => println!("Nice."),
        666 => println!(r"\m/"),
        _ => println!(r"¯\_(ツ)_/¯"),
    }

    // НО! Ръкавите трябва да са от един и същи тип
}

fn match_problem() {
    let some_value = Some(6);

    // Понякога да използваме match за един случай и да покрием
    // всички останали с _ е прекалено много код
    match some_value {
        Some(6) => println!(),
        _ => ()
    }
}

fn if_let() {
    let some_value = Some(8);

    if let Some(8) = some_value {
        println!("::::)");
    }
}

fn while_let() {
    let so_eighty = [8,8,88,8];
    let mut iter8or = so_eighty.iter();

    // `next()` метода на итератора връща `Option`
    while let Some(8) = iter8or.next(){
        println!("∞");
    }
}

fn if_let_and_while_let() {
    let counts = [1, 2, 3, 4];
    let mut counter = counts.iter();

    if let Some(n) = counter.next() {
        print!("{}", n);
        while let Some(n) = counter.next() {
            print!(" and {}", n);
        }
        println!();
    }
}

fn pattern_matching_guards() {
    let pair = (2, -2);

    match pair {
        (x, y) if x == y =>                     println!("Едно и също"),
        (x, y) if x + y == 0 =>                 println!("Противоположни"),
        (x, y) if x % 2 == 1 && y % 2 == 0 =>   println!("X е нечетно, Y е четно"),
        (x, _) if x % 2 == 1 =>                 println!("X e нечетно"),
        _ =>                                    println!("Нищо интерсно!")
    }
}

fn pattern_matching_ranges() {
    let age: i32 = -5;

    match age {
        n if n < 0 => println!("Ще се родя след {} години.", n.abs()),
        0 => println!("Новореодено съм!"),
        1 ..= 12 => println!("Аз съм лапе!"),
        13 ..= 19 => println!("Аз съм тийн"),
        _ => println!("Аз съм дърт!")
    }
}

fn pattern_matching_bindings() {
    let age: i32 = -5;

    match age {
        n if n < 0 => println!("Ще се родя след {} години.", n.abs()),
        0 => println!("Новореодено съм."),
        n @ 1 ..= 12 => println!("Аз съм лапе на {}.", n),
        n @ 13 ..= 19 => println!("Аз съм тийн на {}.", n),
        n => println!("Аз съм дърт, на {} съм вече", n)
    }
}

fn pattern_matching_mutiple_patterns() {
    let score: u32 = 1;

    match score {
        0 | 1 => println!("слабичко :("),
        _     => println!("стаа")
    }
}

struct User {
    name: &'static str,
    age: u8
}

fn pattern_matching_structs() {
    let user = User {
        name: "Гошо",
        age: 6
    };

    match user {
        User { name: "Пешо", age: _ } => println!("Ко стаа, Пешо!"),
        User { name: _, age: 12 } => println!("Ко стаа, лапе!"),
        User { name: x, age: _} => println!("Ко стаа, {}", x),
    }
}

fn pattern_matching_slices() {
    let cake: &[&str] = &["vanilla", "strawberry", "chocolate"];

    match cake {
        [] => println!("Turns out it's a lie :/"),
        [_one_item] => println!("One slice is better than nothing"),
        _ => println!("Wow, that's a lotta slices!")
    }
}