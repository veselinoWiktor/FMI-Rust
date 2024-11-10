fn main() {
    let input = "Ramen";
    let output = reverse(input);
    let expected = "nemaR";
    assert_eq!(output, expected);
}

pub fn reverse(input: &str) -> String {    
    let mut res: String = String::new();

    for ch in input.chars().rev() {
        res.push(ch);
    }

    res
}
