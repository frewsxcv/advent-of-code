pub fn tokenize_line<'a>(line: &'a str) -> Box<Iterator<Item = usize> + 'a> {
    Box::new(line.split_whitespace().map(|n| n.parse::<usize>().unwrap()))
}
