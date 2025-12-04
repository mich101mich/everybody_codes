use super::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/2024-07-1.txt");
    // let input = "";

    const ROUNDS: usize = 10;

    let mut parsed = input
        .lines()
        .map(|l| l.split_once(':').unwrap())
        .map(|(letter, instructions)| {
            let steps = instructions
                .as_bytes()
                .iter()
                .filter(|b| **b != b',')
                .to_vec();
            let mut iter = steps.iter().cycle().copied();
            let mut power = 10u32;
            let mut total = 0;
            for _ in 0..ROUNDS {
                match iter.next().unwrap() {
                    b'+' => power += 1,
                    b'-' => power = power.saturating_sub(1),
                    _ => {}
                }
                total += power;
            }
            (letter, total)
        })
        .to_vec();

    parsed.sort_by_key(|(_, p)| std::cmp::Reverse(*p));

    let result = parsed.iter().map(|(l, _)| *l).collect::<String>();

    result!(result);
}
