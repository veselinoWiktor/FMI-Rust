fn main() {
    println!("{}", fibunacci(10));
}

fn fibunacci(n: i32) -> i32 {
    if n == 1 {
        return 1;
    } else if n == 0 {
        return 0;
    }

    return fibunacci(n - 1) + fibunacci(n - 2);
}
