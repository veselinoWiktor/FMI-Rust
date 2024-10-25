fn main() {
    create_instance();
    println!();
    short_syntax_instance();
    println!();
    field_access();
    println!();
    field_access_via_ref();
    println!();
    field_mutation();
    println!();
    structs_clone();
    println!();
    print_struct();
    println!();
    struct_update_syntax();
    println!();
    struct_update_syntax_default();
    println!();
}

#[derive(Clone, Debug, Default)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64
}

fn create_instance() {
    let _user = User {
        username: String::from("Иванчо"),
        email: String::from("ivan40@abv.bg"),
        sign_in_count: 10    
    };
}

fn short_syntax_instance() {
    let username = String::from("Иванчо");
    let email = String::from("ivan40@abv.bg");
    let sign_in_count = 10;

    // дълъг синтаксис
    let _user = User {
        username: username.clone(),
        email: email.clone(),
        sign_in_count: sign_in_count,
    };

    // кратък синтаксис
    let _user = User { username, email, sign_in_count };
}

fn field_access() {
    let user = User {
        username: String::from("Иванчо"),
        email: String::from("ivan40@abv.bg"),
        sign_in_count: 10  
    };

    println!("{}, {}", user.username, user.email);
}

fn field_access_via_ref() {
    let user = User {
        username: String::from("Иванчо"),
        email: String::from("ivan40@abv.bg"),
        sign_in_count: 10  
    };

    let user_ref = &user;

    // Полетата се достъпват по същия начин и през референция.
    // Автоматично се правят необходимия брой дереференцирания
    println!("{}, {}, {}", user_ref.username, user_ref.email, user_ref.sign_in_count);
}

fn field_mutation() {
    // Можем да променяме стойността на полета, ако инстанцията е дефинирана като mut!!!
    let mut user = User {
        username: String::from("Иванчо"),
        email: String::from("ivan40@abv.bg"),
        sign_in_count: 10  
    };

    user.email = "ivan40.ivanov@abv.bg".to_string();
    println!("{}", user.email);
}

fn _structs_move() {
    let user1 = User {
        username: String::from("Иванчо"),
        email: String::from("ivan40@abv.bg"),
        sign_in_count: 10  
    };

    let user2 = user1;

    // println!("user1 = {}, {}", user1.username, user1.email);
    println!("user2 = {}, {}", user2.username, user2.email);

    /*
    error[E0382]: borrow of moved value: `user1`
    --> src/bin/main_b3c03fba678cada8014f8bd9534eee85648a700d.rs:16:44
    |
    8  | let user1 = User {
    |     ----- move occurs because `user1` has type `User`, which does not implement the `Copy` trait
    ...
    14 | let user2 = user1;
    |             ----- value moved here
    15 |
    16 | println!("user1 = {}, {}", user1.username, user1.email);
    |                                            ^^^^^^^^^^^ value borrowed here after move
    |
    note: if `User` implemented `Clone`, you could clone the value
    --> src/bin/main_b3c03fba678cada8014f8bd9534eee85648a700d.rs:2:1
    |
    2  | struct User {
    | ^^^^^^^^^^^ consider implementing `Clone` for this type
    ...
    14 | let user2 = user1;
    |             ----- you could clone this value
    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

    For more information about this error, try `rustc --explain E0382`.
    error: could not compile `rust` (bin "main_b3c03fba678cada8014f8bd9534eee85648a700d") due to 1 previous error
    */
}

fn structs_clone() {
    let user1 = User {
        username: String::from("Иванчо"),
        email: String::from("ivan40@abv.bg"),
        sign_in_count: 10,
    };
    
    // За да можем да създаваме копия на нашата структура, тя трябва да имплементира trait-a Clone.
    // Чрез атрибута #[derive(Clone)] компилатора автоматично ще ни създаде имплементация на Clone.
    // Аналогично #[derive(Copy)] ще имплементира Copy - но трябва всички полета да са Copy.
    let user2 = user1.clone();
    
    println!("user1 = {}, {}", user1.username, user1.email);
    println!("user2 = {}, {}", user2.username, user2.email);
}

fn print_struct() {
    let user1 = User {
        username: String::from("Иванчо"),
        email: String::from("ivan40@abv.bg"),
        sign_in_count: 10,
    };

    let user2 = user1.clone();

    // Аналогично може да използваме атрибута #[derive(Debug)] за получим имплементация на trait-а Debug.
    // Това ни позволява да принтираме нашата структура с println! използвайки placeholder {:?}.
    println!("user1 = {:?}", user1);
    println!("user2 = {:?}", user2);
}

fn struct_update_syntax() {
    let user = User {
        username: String::from("Иванчо"),
        email: String::from("ivan40@abv.bg"),
        sign_in_count: 10
    };

    // Можем да дадем стойност само на част от полетата и останалите да попълним
    // от друга инстанция.
    let hacker = User {
        email: String::from("hackerman@133t.hax"),
        ..user
    };

    println!("{:?}", hacker);
}

fn _struct_update_syntax_move() {
    let user = User {
        username: String::from("Иванчо"),
        email: String::from("ivan40@abv.bg"),
        sign_in_count: 10
    };

    let hacker = User {
        email: String::from("hackerman@133t.hax"),
        // Това ще премести полетата от оригиналната инстация
        ..user
    };

    println!("{:?}", hacker);
    // println!("{:?}", user);

    /* 
        error[E0382]: borrow of partially moved value: `user`
        --> src/bin/main_99773612f004f2399763eb71a56006b2f8c4c270.rs:16:18
        |
        10 |   let hacker = User {
        |  ______________-
        11 | |     email: String::from("hackerman@l33t.hax"),
        12 | |     ..user
        13 | | };
        | |_- value partially moved here
        ...
        16 |   println!("{:?}", user);
        |                    ^^^^ value borrowed here after partial move
        |
        = note: partial move occurs because `user.username` has type `String`, which does not implement the `Copy` trait
        = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

        For more information about this error, try `rustc --explain E0382`.
        error: could not compile `rust` (bin "main_99773612f004f2399763eb71a56006b2f8c4c270") due to 1 previous error
    */
}

fn struct_update_syntax_default() {
    // Синтаксиса е удобен за попълване на стойности по подразбиране.
    // Напр. ако структурата имплементира trait-a Default можем да използваме функцията defualt
    let user = User {
        username: String::from("Иванчо"),
        ..User::default()
    };

    println!("{:?}", user);
}