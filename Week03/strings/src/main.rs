fn main() {
    string_length();
    println!();
    string_indexing();
    println!();
    string_iteration();
    println!();
}

fn string_length() {
    // str::len() Връща дължината на низ в брой байтове
    let hi = "Здравей! 😊";

    println!("{}", hi.len());
    println!("{}", hi.chars().count());
}

fn string_indexing() {
    // При взимане на резен от низ се оказват брой байтове!!!
    let sub_hi = &"Здравей! 😊"[0..6];
    println!("{:?}", sub_hi); // Здр

    // let sub_hi = &"Здравей! 😊"[0..3]; // Error!!!
    // println!("{:?}", sub_hi)
    /* 
    byte index 3 is not a char boundary; it is inside 'д' (bytes 2..4) of `Здравей! 😊`
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    error: process didn't exit successfully: `target\debug\strings.exe` (exit code: 101)
    */
}

fn string_iteration() {
    // for c in "Здравей! 😊" {
    //     //...
    // }
    /*
    error[E0277]: `&str` is not an iterator
        --> src/main.rs:30:14
        |
        30 |     for c in "Здравей! 😊" {
        |              ^^^^^^^^^^^^^ `&str` is not an iterator; try calling `.chars()` or `.bytes()`
        |
        = help: the trait `Iterator` is not implemented for `&str`, which is required by `&str: IntoIterator`
        = note: required for `&str` to implement `IntoIterator`

        For more information about this error, try `rustc --explain E0277`.
        error: could not compile `strings` (bin "strings") due to 1 previous error
    */

    for b in "Здравей! 😊".bytes() {
        print!("{:02x} ", b)
    }
    println!();
    println!();

    for c in "Здравей! 😊".chars() {
        let char_string: String = c.to_string();
        let char_utf8 = char_string.as_bytes();

        println!("{}: code_point={:#x}, utf8={:x?}", c, c as u32, char_utf8);
    }
    println!();

    // as_bytes() преобразува &str в &[u8]
    println!("{:x?}", "Здравей! 😊".as_bytes());

    // bytes() връща итератор по байтовете на низа
    let bytes: Vec<u8> = "Здравей! 😊".bytes().collect();
    println!("{:x?}", bytes);

    // chars() връща итератор по символите в низа
    let chars: Vec<char> = "Здравей! 😊".chars().collect();
    println!("{:x?}", chars);

    let numbers = [1, 2, 3].iter(); // std::slice::Iter
    let chars = "abc".chars(); // std::slice::Chars
    let words: Vec<&str> = "one two three".split_whitespace().collect(); // std::str::SplitWhiteSpace

    println!("{:?}", numbers);
    println!("{:?}", chars);
    println!("{:?}", words);
}

