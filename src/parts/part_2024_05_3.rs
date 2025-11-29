use super::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/2024-05-3.txt");
    //     let input = "2 3 4 5
    // 6 7 8 9";

    let mut columns: [Vec<usize>; 4] = std::array::from_fn(|_| vec![]);

    for row in input.lines() {
        for (col, x) in row.split_ascii_whitespace().map(parse_u).enumerate() {
            columns[col].push(x);
        }
    }

    let mut seen = HashSet::<u64>::new();
    let mut best = [0; 4];
    let state = std::hash::RandomState::new();
    for i in 0.. {
        let x = columns[i % 4].remove(0);
        let out_col = &mut columns[(i + 1) % 4];
        let mut remaining = x;
        loop {
            if remaining - 1 < out_col.len() {
                out_col.insert(remaining - 1, x);
                break;
            }
            remaining -= out_col.len();

            if remaining - 1 < out_col.len() {
                out_col.insert(out_col.len() - (remaining - 1), x);
                break;
            }
            remaining -= out_col.len();
        }
        let id = std::array::from_fn(|c| columns[c][0]);
        best = best.max(id);

        if i % 4 == 0 {
            use std::hash::BuildHasher;
            let h = state.hash_one(&columns);
            if !seen.insert(h) {
                break;
            }
        }
    }

    let result = format!("{}{}{}{}", best[0], best[1], best[2], best[3])
        .parse::<usize>()
        .unwrap();
    result!(result);
}
