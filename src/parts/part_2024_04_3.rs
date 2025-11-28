use super::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/2024-04-3.txt");
    //     let input = "2
    // 4
    // 5
    // 6
    // 8";

    let mut parsed = input.lines().map(parse_u).to_vec();
    parsed.sort_unstable();

    let mut prefix_sum = parsed.clone();
    let mut total = 0;
    for (i, o) in parsed.iter().zip(&mut prefix_sum) {
        total += *i;
        *o = total;
    }

    let mut cache = HashMap::<usize, usize>::new();
    let mut cost = |x: usize| {
        *cache.entry(x).or_insert_with(|| {
            let i = match parsed.binary_search(&x) {
                Ok(i) => i,
                Err(i) => i,
            };
            // all elements in 0..i need to be increased, elements in i.. need to be decreased
            let left_sum = prefix_sum[i];
            let left_target = x * (i + 1);
            let right_sum = total - left_sum;
            let right_target = x * (parsed.len() - (i + 1));
            left_target - left_sum + right_sum - right_target
        })
    };

    let mut lower = *parsed.iter().min().unwrap();
    let mut upper = *parsed.iter().max().unwrap();
    let mut result = total;
    while upper - lower > 1 {
        let mid = lower.midpoint(upper);
        let at = cost(mid);
        let prev = cost(mid - 1);
        result = result.min(at).min(prev);

        if prev > at {
            // downwards slop => increase
            lower = mid;
        } else {
            upper = mid;
        }
    }

    result!(result);
}
