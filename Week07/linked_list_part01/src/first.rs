use std::mem;

// pub says we want people outside this module to be able to use List
pub struct List {
    head: Link
}

enum Link {
    Empty,
    More(Box<Node>)
}

struct Node {
    elem: i32,
    next: Link
}

impl List {
    /// # Notes:
    /// - Self is an alias for "that type I wrote at the top next to ```impl```".
    /// Great for not repeating yourself!
    /// - We create an instance of a struct much the same way we declare it, except instead of providing
    /// the types of its fields, we initialize them with values.
    /// - We refer to variants of the enum using ```::```, which is the ***namespacing operator***
    /// - The last expression of a function is implicitly returned. This makes a simple functions
    /// a little neater. You can still use ```return``` to return early like other C-like languages.
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty)
        });

        self.head = Link::More(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            },
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut curr_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this think until this pattern doesn't match"
        while let Link::More(mut boxed_node) = curr_link {
            curr_link = mem::replace(&mut boxed_node.next, Link::Empty);

            // `boxed_node` goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }

    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

