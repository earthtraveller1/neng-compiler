use super::*;

#[test]
fn test_tokenization() {
    let input = "int main() { printf(x); }";
    let expected = vec!["int", "main", "(", ")", "{", "printf", "(", "x", ")", ";", "}"];

    let actual = tokenize_string(input);
    assert_eq!(expected, actual);
}
