fn main() {
    println!("Hello, world!");
}

pub struct MapIter<I, F> {
    iter: I,
    func: F,
}

impl<I, F, T, U> MapIter<I, F>
where
    I: Iterator<Item = T>,
    F: Fn(T) -> U,
{
    pub fn new(iter: I, func: F) -> MapIter<I, F> {
        MapIter { iter, func }
    }
}

impl<I, F, T, U> Iterator for MapIter<I, F>
where
    I: Iterator<Item = T>,
    F: Fn(T) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&mut self.func)
    }
}

#[test]
fn test_map_iter() {
    let xs = &["x", "xx", "xxx"];
    let map_iter = MapIter::new(xs.iter(), |s| s.len());

    assert_eq!(
        map_iter.collect::<Vec<_>>(),
        vec![1, 2, 3],
    );
}