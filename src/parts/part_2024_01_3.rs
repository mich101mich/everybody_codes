pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/2024-01-3.txt");
    // let input = "xBxAAABCDxCC";

    let result = input
        .as_bytes()
        .chunks_exact(3)
        .map(|chunk| {
            let mut sum = 0;
            let mut count = 3;
            for c in chunk {
                sum += match c {
                    b'A' => 0,
                    b'B' => 1,
                    b'C' => 3,
                    b'D' => 5,
                    _ => {
                        count -= 1;
                        0
                    }
                };
            }
            match count {
                1 => sum,
                2 => sum + 2,
                3 => sum + 6,
                _ => 0,
            }
        })
        .sum::<i32>();

    result!(result);
}
