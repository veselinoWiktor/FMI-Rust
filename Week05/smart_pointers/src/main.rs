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
    rc();
    println!();
    rc_deref();
    println!();
    rc_mutate_value();
    println!();
    rc_mutable_value_fix();
    println!();
    rc_mutable_value_with_copy();
    println!();
    rc_mutable_with_Cow();
    println!();
    cell();
    println!();
    ref_cell();
    println!();
    double_ref_cell_mut_borrow();
    println!();
    print_tree();
    println!();
    weak_ref_example();
    println!();
    weak_ref_example2();
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
    let mut x = 5;
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
    title: Option<String>,
}

impl Deref for Mp3 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.audio
    }
}

// REFERENCE COUNTING

use std::rc::Rc;

fn rc() {
    let first = Rc::new(String::from("foobar"));
    let second = Rc::clone(&first);

    println!("{first}");
    println!("{second}");
}

fn rc_deref() {
    let a = Rc::new(3);
    let b = Rc::new(5);

    println!("{}", *a + *b);
}

fn rc_mutate_value() {
    let mut a = Rc::new(3);

    // *a = 5;

    println!("{:?}", a);

    /*
        error[E0594]: cannot assign to data in an `Rc`
       --> src/main.rs:255:5
        |
    255 |     *a = 5;
        |     ^^^^^^ cannot assign
        |
        = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<i32>`

    Rc не ни позволява да взимаме mutable reference към пазената стойност
    Това би нарушило ограничението за един &mut T/ много &T
    Но въпреки това има начини да модифицираме пазената стойниост
    */
}

fn rc_mutable_value_fix() {
    let mut a = Rc::new(3);
    *Rc::make_mut(&mut a) = 5;

    // Ако сме единствения собственик модифицираме директно пазената стойност
    // Ако не първо си правим копие на стойността и модифицираме копието

    println!("a: {}", a);
}

fn rc_mutable_value_with_copy() {
    let mut a = Rc::new(3);
    let b = Rc::clone(&a);
    // Дотук a и b сочат към една и съща стойност в паметта

    {
        let temp_ref = Rc::make_mut(&mut a);
        // Връщаме &mut към copy-on-write стойност
        *temp_ref = 5;
    }
    // Вече a и b сочат към различни стойности

    println!("a: {}", a);
    println!("b: {}", b);
}

use std::borrow::Cow;

fn rc_mutable_with_Cow() {
    let sparkle_hearth = String::from_utf8_lossy(&[240, 159, 146, 150]);

    match sparkle_hearth {
        // Cow::Borrowed(&str)
        Cow::Borrowed(s) => println!("Borrowed {}", s),
        Cow::Owned(s) => println!("Owned {}", s),
    }

    let hello = String::from_utf8_lossy(b"Hello \xF0\x90\x80World!");
    match hello {
        // Cow::Owned(String)
        Cow::Borrowed(s) => println!("Borrowed {}", s),
        Cow::Owned(s) => println!("Owned {}", s),
    }
}

// INTERNAL MUTABILITY

// Пазим състояние, невидимо за външния свят
// Искаме да модифицираме това състояние в методи, които са логически immutable
// Това се нарича internal mutability

use std::cell::Cell;

fn cell() {
    // Забележете, че няма `mut`
    let cell = Cell::new(10);

    println!("{}", cell.get());

    cell.set(42);
    println!("{}", cell.get());

    /*
    Cell

        Използва се предимно за Copy типове - някои функционалности работят само за Copy
        get прави копие на пазената стойност (иска Copy)
        set презаписва пазената стойност с новата
        replace записва нова стойност и връща старата
        into_inner ще кносумира Cell-а директно ще върне вътрешната стойност
        Не можем да вземем референция (&/&mut) към вътрешната стойност, само към обвиващия Cell
     */
}

use std::cell::RefCell;

fn ref_cell() {
    let cell = RefCell::new(String::from("foo")); // отново няма `mut`
    println!("{}", cell.borrow()); // -> Ref<String>

    cell.borrow_mut().push_str("bar"); // -> RefMut<String>
    println!("{}", cell.borrow());
}

fn double_ref_cell_mut_borrow() {
    let cell = RefCell::new(String::from("foo"));

    let mut first = cell.borrow_mut();
    // let mut second = cell.borrow_mut(); // BOOM!

    /*
    thread 'main' panicked at src/main.rs:372:27:
    already borrowed: BorrowMutError
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    error: process didn't exit successfully: `target\debug\smart_pointers.exe` (exit code: 101)
    */

    /*
    RefCell

        Runtime borrow checker
        Помни колко immutable и mutable референции е раздал
        borrow() ще върне структура от тип Ref, която има deref до &T
        borrow_mut() ще върне стръктура от тип RefMut, която има deref до &mut T
        Ако не можем да вземем референция по стандартните правила на borrow checker-a ще получим panic! Вмесо компилационна грешка
     */
}

fn rc_with_ref_cell() {
    let first = Rc::new(RefCell::new(String::from("foo")));
    let second = Rc::clone(&first);

    first.borrow_mut().push_str("bar");
    println!("{}", first.borrow());

    /*
    Cell, RefCell

        Cell се използва предимно за Copy типове, RefCell - не само
        Cell връща стоности, RefCell връща Ref-ове и RefMut-овею
        Cell не хвърля panics, RefCell хвърля
        Заобикалят "нормалните" правила, съответно ако е възможно да се избягват
        Документацията го потвърждава: "… interior mutability is something of a last resort."
     */
}

// WEEK REFERENCE OF RC

// какво правим когато структурата ни може да има цикли?

// type alias
type TreeNodeRef = Rc<RefCell<TreeNode>>;
struct TreeNode {
    value: u32,
    parent: Option<TreeNodeRef>,
    children: Vec<TreeNodeRef>,
}

impl TreeNode{
    fn new(value: u32, parent: Option<TreeNodeRef>) -> TreeNodeRef {
        Rc::new(RefCell::new(TreeNode { value, parent, children: vec![] }))
    }
}

fn make_tree() -> Rc<RefCell<TreeNode>> {
    let root = TreeNode::new(0, None);
    let v1 = TreeNode::new(1, Some(Rc::clone(&root)));
    let v2 = TreeNode::new(2, Some(Rc::clone(&root)));

    {
        let mut r =root.borrow_mut();
        r.children.push(v1);
        r.children.push(v2);
    }

    root
}

fn print_tree()
{
    let tree = make_tree();

    println!("{:?}", tree.borrow().value);
    std::mem::drop(tree);

    /*
    Нищо не гърми, но родителя държи Rc към децата, а децата държат към Rc към родителя
    Получаваме цикъл от референции -- никога няма да се деалокират
    Това води до изтичане на памет
    Т.е може да имаме memory leak в safe code
    Затова нямаме гаранция, че деструктурите ще се извикат
    И затова същестува безопасната функция mem::forget()
     */
}

/*
Решение на проблема

Искаме родителя да е собственик на децата
Не искаме детето да е собственик на родителя
Затова използваме силни и слаби референции
 */

use std::mem;
use std::rc::{Weak};

fn weak_ref_example() {
    let rc = Rc::new(5);
    let weak = Rc::downgrade(&rc);

    println!("{:?}", Weak::upgrade(&weak)); // -> Option<Rc<T>>

    mem::drop(rc);
    println!("{:?}", Weak::upgrade(&weak)); // -> Option<Rc<T>>
}

fn weak_ref_example2() {
    let gosho_source = "Гошо, Гошо, скочи лошо";
    let shared_gosho = Rc::new(gosho_source); // shared_gosho { strong = 1, weak = 0 };

    let bratcheda = Rc::clone(&shared_gosho); // shared_gosho { strong = 2, weak = 0 };
    // или, shared_gosho.clone(), но така написано е по-яно

    let slabichko = Rc::downgrade(&shared_gosho); // shared_gosho { strong = 2, weak = 1 };
    println!("{:#?}", Weak::upgrade(&slabichko));// => Some("Гошо, Гошо, скочи лошо")
                                                // shared_gosho { strong = 3, weak = 1 };
                                                // shared_gosho { strong = 2, weak = 1 };

    std::mem::drop(bratcheda); // shared_gosho { strong = 1, weak = 1 };
    std::mem::drop(shared_gosho); // shared_gosho { strong = 0, weak = 1 }; => DROP!

    println!("{:#?}", Weak::upgrade(&slabichko)); // => None
}






















