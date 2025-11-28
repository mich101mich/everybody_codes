use super::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/2024-04-1.txt");
    // let input = "";

    let parsed = input.lines().map(parse_u).to_vec();
    let target = parsed.iter().min().unwrap();
    let total = parsed.iter().sum::<usize>();
    let result = total - parsed.len() * target;

    result!(result);
}
