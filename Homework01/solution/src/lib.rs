pub struct FibIter {
    fib_seq: Vec<u32>,
    curr_idx: usize
}

impl FibIter {
    pub fn new() -> FibIter {
        FibIter { fib_seq: vec![1, 1], curr_idx: 0 }
    }

    pub fn next(&mut self) -> u32 {
        let result;

        if self.curr_idx == self.fib_seq.len() {
            self.fib_seq.push(self.fib_seq[self.curr_idx - 1] + self.fib_seq[self.curr_idx - 2]); // Add one more element
        }

        result = self.fib_seq[self.curr_idx]; 
        self.curr_idx = self.curr_idx + 1;

        result      
    }

    pub fn rev(self) -> RevFibIter {
        let mut rev_fib = self.fib_seq;
        rev_fib.reverse();
        RevFibIter { rev_fib_seq: rev_fib, curr_idx: 0 }
    }
}

pub struct RevFibIter {
    rev_fib_seq: Vec<u32>,
    curr_idx: usize
}

impl RevFibIter {
    pub fn next(&mut self) -> Option<u32> {
        match self.curr_idx < self.rev_fib_seq.len() {
            true => {
                let current = self.rev_fib_seq[self.curr_idx];
                self.curr_idx = self.curr_idx + 1;
                Some(current)
            }
            false => None
        }
    }
}

pub fn fib_split(text: &str) -> Vec<String> {
    let mut fib_iter = FibIter::new();
    let mut text_chars = text.chars();
    let mut result: Vec<String> = Vec::new();

    loop {
        let fib_num = fib_iter.next();
        let mut curr_str = String::new();
        for _n in 0..fib_num {
            match text_chars.next() {
                Some(char) => curr_str.push(char),
                None => 
                {
                    if curr_str.chars().count() != 0 {
                        result.push(curr_str.clone());
                    }
                    return result;
                }
            }
        }
        result.push(curr_str.clone());
    }
}

pub fn fib_split_n(text: &str, n: u32) -> (Vec<String>, &str) {
    let mut fib_iter = FibIter::new();
    let mut text_chars = text.chars();
    let mut result: Vec<String> = Vec::new();

    for _i in 1..=n {
        let fib_num = fib_iter.next();
        let mut curr_str = String::new();

        for _j in 0..fib_num {
            match text_chars.next() {
                Some(char) => curr_str.push(char),
                None => 
                {
                    panic!("Няма достатъчно букви!");
                }
            }
        }

        result.push(curr_str.clone());
    }

    (result, text_chars.as_str())
}

pub fn fib_split_n_symmetric(text: &str, n: u32) -> (Vec<String>, &str) {
    let mut fib_iter = FibIter::new();
    let mut text_chars = text.chars();
    let mut result: Vec<String> = Vec::new();

    for _i in 1..=n {
        let fib_num = fib_iter.next();
        let mut curr_str = String::new();

        for _j in 0..fib_num {
            match text_chars.next() {
                Some(char) => curr_str.push(char),
                None => 
                {
                    panic!("Няма достатъчно букви!");
                }
            }
        }

        result.push(curr_str.clone());
    }

    let mut rev_fib_iter = fib_iter.rev();

    while let Some(fib_num) = rev_fib_iter.next() {
        let mut curr_str = String::new();

        for _j in 0..fib_num {
            match text_chars.next() {
                Some(char) => curr_str.push(char),
                None => 
                {
                    panic!("Няма достатъчно букви!");
                }
            }
        }
        result.push(curr_str.clone());

    }

    (result, text_chars.as_str())
}


// TESTS
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
fn ensure_iterators_are_working_correctly() {
    let mut fib_iter = FibIter::new();

    assert!(1 == fib_iter.next());
    assert!(1 == fib_iter.next());
    assert!(2 == fib_iter.next());
    assert!(3 == fib_iter.next());
    assert!(5 == fib_iter.next());

    let mut rev_fib_iter = fib_iter.rev();

    assert!(Some(5) == rev_fib_iter.next());
    assert!(Some(3) == rev_fib_iter.next());
    assert!(Some(2) == rev_fib_iter.next());
    assert!(Some(1) == rev_fib_iter.next());
    assert!(Some(1) == rev_fib_iter.next());
    assert!(None == rev_fib_iter.next());
    assert!(None == rev_fib_iter.next());
    assert!(None == rev_fib_iter.next());
    assert!(None == rev_fib_iter.next());
    assert!(None == rev_fib_iter.next());
}

// took it from internet because funcs return Vec<String> and vec![...] is Vec<&str>
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[test]
fn test_fib_split_works() {
    println!("{:?}",fib_split("Fibonacci words!📝!аа"));
    assert!(fib_split("Fibonacci words!📝!аа") == vec!["F", "i", "bo", "nac", "ci wo", "rds!📝!аа"]);
}

#[test]
fn test_fib_split_n_works() {
    println!("{:?}",fib_split_n("Lorem ipsum dolor sit amet.", 6)); // debugging purposes
    assert!(fib_split_n("Lоrem ipsum dolor s📝t amet.📝", 6) == (vec_of_strings!["L", "о", "re", "m i", "psum ", "dolor s📝"], "t amet.📝"));
}

#[test]
fn test_fib_split_n_symmetric_works() {
    println!("{:?}", fib_split_n_symmetric("Lorem ipsum dolor sit amet.", 5)); // debuging purposes
    assert!(fib_split_n_symmetric("Lorem ipsum dolor s📝t amet.", 5) == (vec_of_strings!["L", "o", "re", "m i", "psum ", "dolor", " s📝", "t ", "a", "m"], "et."));
}