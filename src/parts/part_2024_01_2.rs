fn map_letter(c: u8) -> i32 {
    match c {
        b'A' => 0,
        b'B' => 1,
        b'C' => 3,
        b'D' => 5,
        _ => -2,
    }
}
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/2024-01-2.txt");
    // let input = "AxBCDDCAxD";

    let result = input
        .as_bytes()
        .chunks_exact(2)
        .map(|c| map_letter(c[0]) + map_letter(c[1]) + 2)
        .map(|v| v.max(0))
        .sum::<i32>();
    result!(result);
}
