pub fn run() {
    let input = include_str!("../../input/2024-01-1.txt");

    let result = input
        .bytes()
        .map(|b| match b {
            b'A' => 0,
            b'B' => 1,
            _ => 3,
        })
        .sum::<usize>();
    result!(result);
}
