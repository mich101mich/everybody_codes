pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/2024-02-1.txt");
    // let input = "";

    let mut lines = input.lines();
    let words = lines
        .next()
        .unwrap()
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',');

    let text = lines.nth(1).unwrap();

    let mut result = 0;
    for word in words {
        let mut remaining = text;
        while let Some(idx) = remaining.find(word) {
            result += 1;
            remaining = &remaining[idx + 1..];
        }
    }

    result!(result);
}
