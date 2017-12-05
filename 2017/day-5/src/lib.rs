pub fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .lines()
        .map(|n| n.parse().expect("could not parse input num"))
        .collect()
}

#[test]
fn test_parse() {
    assert_eq!(
        parse("0\n3\n0\n1\n-3"),
        [0, 3, 0, 1, -3],
    )
}
