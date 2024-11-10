fn main() {
    ref_pointer();
    println!();
    box_example();
    println!();
    box_example02();
    println!();
    box_linked_list();
    println!();
    reference_linked_list();
    println!();
    for thing in vec_of_things() {
        println!("{}", thing);
    }
    println!();
    println!("{:?}", foo());
    println!();
    deref_example();
    println!();
    deref_example_box();
    println!();
}

fn _foo<T>(_t: T) {}

fn _bar<T: ?Sized>(_t: &T) {}

// fn foobar<T: ?Sized>(t: T) {} Невъзмжно, защото как ще алокира t? като не му знае големината

// &T smart pointer-а
// Най-простия начин да реферираме към пакет, където и да е в паметта (който не изисква unsafe)
// Няма ownership - нещо друго трябва да е owner на тази памет
// Нужда от описаване на lifetimes при уптореба във функции и структури
// Позволяват максимална ефективност при достъп - директно адресиране на памет

fn ref_pointer() {
    let potato = String::from(
        "
        Любов, любов, варен картоф,
        разрежеш го, а той суров.
    ",
    );

    let lines = potato.trim().lines().map(|l| l.trim());
    for line in lines {
        println!("{}", line)
    }
}
// В горния пример има само алокация на първия String, останалите методи -- trim, lines, map, само алокират малки стойности на стека.

// BOX
// Reference + ownership
fn box_example() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// Също прост - сочи към парче памет, само и единствено в heap-а
// Държи ownership над данните си, което значи, че няма нужда да се грижим за lifetimes
// std::unique_ptr от C++
// Но алокира нова стойност на heap-а, което може да е малко по-неефективно от reference към нещо на стека.
// Това обикновено не е практически проблем - Vec, String, и т,н си алокират неща на heap-а

// Условно казано, горе-долу, донякъде, моде да си представите, че:
// String<T> ~~ Box<str>
// Vec<T> ~~ Box<[T]>

fn box_example02() {
    let x = Box::new(5);
    let y = Box::new(3);

    //println!("{}", x + y); cannot add `Box<{integer}>` to `Box<{integer}>`
    println!("{}", *x + *y); // Трябва да се дереферира

    let x = &3;
    let y = &5;

    println!("{}", x + y);
    println!("{}", *x + *y);
    // Може и по двата начина
}

// А защо ни е Box всъщност?
// # Example
// #[derive(Debug)]
// enum List { // cycle detected when computing when `List` needs drop
//     Nil,
//     Cons(i32, List),
// }

// use List::{Cons, Nil};

// fn box_linked_list() {
//     let list = Cons(1, Cons(2, Cons(3, Nil)));

//     println!("{:?}", list);
// }

#[derive(Debug)]
enum List {
    Nil,
    Cons(i32, Box<List>),
}

use List::{Cons, Nil};

fn box_linked_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", list)
}

// Можем ли вместо Box да ползваме &?
#[derive(Debug)]
enum LinkedList<'a> {
    Nil2,
    Cons2(i32, &'a LinkedList<'a>),
}

use LinkedList::{Cons2, Nil2};

fn reference_linked_list() {
    let list = Cons2(1, &Cons2(2, &Nil2));

    println!("{:?}", list);
}

// Това работи, но не можем да местим тази стойност
// fn return_list<'a>(x: i32, y: i32) -> LinkedList<'a> {
//     let list = Nil2;
//     let list2 = Cons2(y, &list1);
//     Cons(x, &list2)
// }

// Никакъв проблерм ако позлваме Box, защото си държи ownership

use std::{fmt::Display, ops::Deref};
// Box на Trait Objects
// fn vec_of_things<'a>() -> Vec<&'a dyn Display> {
//     let x = 123;
//     vec![&x, &3.14, &"foobar"] // cannot return value referencing local variable `x`
// }

fn vec_of_things() -> Vec<Box<dyn Display>> {
    let x = 123;
    vec![Box::new(x), Box::new(3.14), Box::new("foobar")]
}

// Box<Error> ако ни мързи да правим error handling
fn get_x() -> Result<i32, std::io::Error> {
    Ok(3)
}
fn get_y() -> Result<i32, std::fmt::Error> {
    Ok(5)
}

fn foo() -> Result<i32, Box<dyn std::error::Error>> {
    let x = get_x()?;
    let y = get_y()?;

    Ok(x + y)
}

// Box и съпоставяне на образци
#[derive(Clone, Debug, PartialEq)]
// Типа Box<Term> не може да се pattern-match-не по компоненти -- вътрешността му е private.
pub enum Term {
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>),
    Value,
}

fn one_step_eval(t: Term) -> Result<Box<Term>, String> {
    match t {
        // не може Term::If(Term::True, t2, _) => Ok(t2),
        Term::If(t1, t2, _) if *t1 == Term::True => Ok(t2),
        // не може Term::If(Term::False, _, t3) => Ok(t3)
        Term::If(t1, _, t3) if *t1 == Term::False => Ok(t3),

        Term::If(t1, t2, t3) => Ok(Box::new(Term::If(one_step_eval(*t1)?, t2, t3))),

        any => Err(format!("Term can't be evaluated: {:?}", any)),
    }
}

// как работи * при нормалните references?
fn deref_example() {
    let mut x =5;
    {
        let y = &mut x;

        *y += 1;
        println!("y = {}", y);
    }
    println!("x = {}", x);
}

fn deref_example_box() {
    let mut x = 5;
    {
        let mut y = Box::new(x);

        *y += 1;
        println!("y = {}", y);
    }

    println!("x = {}", x);
}

// Deref
struct Mp3 {
    audio: Vec<u8>,
    artist: Option<String>,
    title: Option<String>
}

impl Deref for Mp3 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.audio
    }
}
