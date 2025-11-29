use super::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/2024-05-2.txt");
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

    let mut seen = HashMap::<String, usize>::new();
    for i in 0.. {
        let x = columns[i % 4].remove(0);
        let out_col = &mut columns[(i + 1) % 4];
        if x - 1 < out_col.len() {
            out_col.insert(x - 1, x);
        } else {
            let remaining = x - 1 - out_col.len();
            out_col.insert(out_col.len() - remaining, x);
        }
        let id = format!(
            "{}{}{}{}",
            columns[0][0], columns[1][0], columns[2][0], columns[3][0]
        );
        let count = seen.entry(id).or_default();
        *count += 1;
        if *count == 2024 {
            let number: usize = format!(
                "{}{}{}{}",
                columns[0][0], columns[1][0], columns[2][0], columns[3][0]
            )
            .parse()
            .unwrap();
            let result = (i + 1) * number;
            result!(result);
            break;
        }
    }
}
