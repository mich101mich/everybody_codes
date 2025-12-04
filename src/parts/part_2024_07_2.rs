use super::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/2024-07-2.txt");
    // let input = "";
    let track = "S-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--
-                                                                     -
=                                                                     =
+                                                                     +
=                                                                     +
+                                                                     =
=                                                                     =
-                                                                     -
--==++++==+=+++-=+=-=+=-+-=+-=+-=+=-=+=--=+++=++=+++==++==--=+=++==+++-";
    let track = char_grid(track);

    const ROUNDS: usize = 10;

    let mut parsed = input
        .lines()
        .map(|l| l.split_once(':').unwrap())
        .map(|(letter, instructions)| {
            let steps = instructions.chars().filter(|b| *b != ',').to_vec();
            let mut iter = steps.iter().cycle().copied();
            let mut power = 10u32;
            let mut total = 0;
            let mut track_pos = p2(0, 0);
            let mut dir = Dir::Right;
            let mut rounds = 0;
            loop {
                if !track.in_bounds(track_pos + dir) {
                    dir = dir.clockwise();
                }
                track_pos += dir;
                if track[track_pos] == 'S' {
                    rounds += 1;
                    if rounds == ROUNDS {
                        break;
                    }
                }

                match (track[track_pos], iter.next().unwrap()) {
                    ('+', _) | ('=' | 'S', '+') => power += 1,
                    ('-', _) | ('=' | 'S', '-') => power = power.saturating_sub(1),
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
