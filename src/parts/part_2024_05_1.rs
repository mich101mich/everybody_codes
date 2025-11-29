use super::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/2024-05-1.txt");
    //     let input = "2 3 4 5
    // 3 4 5 2
    // 4 5 2 3
    // 5 2 3 4";

    let mut columns: [Vec<usize>; 4] = std::array::from_fn(|_| vec![]);

    for row in input.lines() {
        for (col, x) in row.split_ascii_whitespace().map(parse_u).enumerate() {
            columns[col].push(x);
        }
    }

    for i in 0..10 {
        let x = columns[i % 4].remove(0);
        let out_col = &mut columns[(i + 1) % 4];
        if x - 1 < out_col.len() {
            out_col.insert(x - 1, x);
        } else {
            let remaining = x - 1 - out_col.len();
            out_col.insert(out_col.len() - remaining, x);
        }
    }

    let result = columns[0][0] * 1000 + columns[1][0] * 100 + columns[2][0] * 10 + columns[3][0];

    result!(result);
}
