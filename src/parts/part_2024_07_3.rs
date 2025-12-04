use super::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/2024-07-3.txt");
    // let input = "";
    let raw_track = "S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
- + +   + =   =     =      =   == = - -     - =  =         =-=        -
= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=       
= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =          
+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
-               = = = =   +  +  ==+ = = +   =        ++    =          -
-               = + + =   +  -  = + = = +   =        +     =          -
--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-";
    let raw_track = char_grid(raw_track);
    let valid = |pos| raw_track.in_bounds(pos) && raw_track[pos] != ' ';

    let mut track = vec![];
    let mut track_pos = p2(0, 0);
    let mut dir = Dir::Right;
    loop {
        if !valid(track_pos + dir) {
            if valid(track_pos + dir.clockwise()) {
                dir = dir.clockwise();
            } else {
                assert!(
                    valid(track_pos + dir.counter_clockwise()),
                    "track_pos={track_pos:?}, dir={dir}"
                );
                dir = dir.counter_clockwise();
            }
        }
        track_pos += dir;
        track.push(raw_track[track_pos]);
        if raw_track[track_pos] == 'S' {
            break;
        }
    }

    const ROUNDS: usize = 2024;
    let total_steps = track.len() * ROUNDS;

    // Notes:
    // - The track is designed in such a way that the power can never drop below 0, so the saturate-at-0 thing doesn't matter
    // - This means that any '+' will actually be a +1 for all remaining steps, and vice-versa for '-'
    // - This means we can calculate the influence of each character in our sequence just with the track

    let mut influence = [0i64; 11];
    for (i, c) in track.iter().cycle().take(total_steps).copied().enumerate() {
        if matches!(c, '+' | '-') {
            continue; // this is just a global modifier, independent of the chosen instructions
        }
        let remaining_steps = total_steps - 1 - i;
        influence[i % 11] += remaining_steps as i64;
    }

    let (_, other_instructions) = input.split_once(':').unwrap();
    let other_knight = other_instructions
        .chars()
        .filter(|b| *b != ',')
        .zip(influence)
        .map(|(c, power)| match c {
            '+' => power,
            '-' => -power,
            _ => 0,
        })
        .sum::<i64>();

    let mut result = 0;
    for i in 0..(3u32.pow(11)) {
        let (mut plus, mut minus) = (0, 0);
        let mut rest = i;
        let sum = influence
            .into_iter()
            .map(|power| {
                let current = rest % 3;
                rest /= 3;
                match current {
                    0 => {
                        plus += 1;
                        power
                    }
                    1 => {
                        minus += 1;
                        -power
                    }
                    _ => 0,
                }
            })
            .sum::<i64>();
        if plus == 5 && minus == 3 && sum > other_knight {
            result += 1;
        }
    }

    result!(result);
}
