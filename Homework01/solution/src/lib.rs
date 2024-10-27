pub struct FibIter {
    fib_seq: Vec<u32>,
    curr_idx: usize
}

impl FibIter {
    pub fn new() -> FibIter {
        FibIter { fib_seq: vec![1, 1], curr_idx: 0 }
    }

    pub fn next(&mut self) -> u32 {
        self.curr_idx = self.curr_idx + 1;

        if self.curr_idx == self.fib_seq.len() {
            self.fib_seq.push(self.fib_seq[self.curr_idx - 2] + self.fib_seq[self.curr_idx - 1]);
        }

        self.fib_seq[self.curr_idx - 1]
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
        let res: Option<u32>;
        if self.curr_idx >= 0 {
            res = Some(self.rev_fib_seq[self.curr_idx]);
        }
        else {
            res = None
        }

        self.curr_idx = self.curr_idx - 1;
        res
    }
}

pub fn fib_split(text: &str) -> Vec<String> {
    todo!()
}

pub fn fib_split_n(text: &str, n: u32) -> (Vec<String>, &str) {
    todo!()
}

pub fn fib_split_n_symmetric(text: &str, n: u32) -> (Vec<String>, &str) {
    todo!()
}
