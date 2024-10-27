use solution::*;

#[test]
fn test_basic() {
    let mut fib_iter = FibIter::new();
    fib_iter.next();

    let mut rev_fib_iter: RevFibIter = fib_iter.rev();
    rev_fib_iter.next();

    let _words: Vec<String> = fib_split("Fibonacci words!");

    let (_words, _rest): (Vec<String>, &str) =
    fib_split_n("Lorem ipsum dolor sit amet, consectetur adipiscing elit.", 1);

    let (_words, _rest): (Vec<String>, &str) =
    fib_split_n_symmetric("Lorem ipsum dolor sit amet, consectetur adipiscing elit.", 1);
}

#[test]
fn test01() {
    let mut fib_iter = FibIter::new();

    assert!(1 == fib_iter.next());
    assert!(1 == fib_iter.next());
    assert!(2 == fib_iter.next());
    assert!(3 == fib_iter.next());
    assert!(5 == fib_iter.next());
    assert!(8 == fib_iter.next());
    assert!(13 == fib_iter.next());
    assert!(21 == fib_iter.next());
}