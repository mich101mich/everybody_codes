use super::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/2024-02-3.txt");
    //     let input = "WORDS:THE,OWE,MES,ROD,RODEO

    // HELWORLT
    // ENIGWDXL
    // TRODEOAL";

    let mut lines = input.lines();
    let words = lines
        .next()
        .unwrap()
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .to_vec();

    lines.next().unwrap(); // empty line

    let grid = Grid::from(lines.map(byte_line).to_vec());
    let (w, h) = grid.size();

    let mut is_runic = grid.map(|_| false);

    for word_str in &words {
        let word = word_str.as_bytes();
        let right_iter = |start: Point| {
            word.iter()
                .enumerate()
                .map(move |(i, c)| (p2((start.x + i) % w, start.y), *c))
        };
        let left_iter = |start: Point| {
            word.iter()
                .enumerate()
                .map(move |(i, c)| (p2((start.x + w - i) % w, start.y), *c))
        };
        let down_iter = |start: Point| {
            word.iter()
                .enumerate()
                .map(move |(i, c)| (p2(start.x, start.y + i), *c))
        };
        let up_iter = |start: Point| {
            word.iter()
                .enumerate()
                .map(move |(i, c)| (p2(start.x, start.y - i), *c))
        };

        for start in grid.grid_index_iter() {
            if grid[start] != word[0] {
                continue;
            }
            if right_iter(start).all(|(pos, c)| grid[pos] == c) {
                // pv!(word_str, start, "right");
                right_iter(start).for_each(|(pos, _)| is_runic[pos] = true);
            }
            if word.len() == 1 {
                continue; // no need to check other directions for single-letter words
            }
            if left_iter(start).all(|(pos, c)| grid[pos] == c) {
                // pv!(word_str, start, "left");
                left_iter(start).for_each(|(pos, _)| is_runic[pos] = true);
            }
            if start.y + word.len() <= h && down_iter(start).all(|(pos, c)| grid[pos] == c) {
                // pv!(word_str, start, "down");
                down_iter(start).for_each(|(pos, _)| is_runic[pos] = true);
            }
            if start.y >= word.len() - 1 && up_iter(start).all(|(pos, c)| grid[pos] == c) {
                // pv!(word_str, start, "up");
                up_iter(start).for_each(|(pos, _)| is_runic[pos] = true);
            }
        }

        // pv!(word_str);
        // let mut chars = grid.clone();
        // for (pos, c) in chars.grid_iter_mut_index() {
        //     if !is_runic[pos] {
        //         *c = b' ';
        //     }
        // }
        // chars.print();
        // is_runic.fill(false);
    }

    let result = is_runic.count();
    result!(result);
}
