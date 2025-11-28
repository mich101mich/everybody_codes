use super::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/2024-03-3.txt");
    //     let input = "..........
    // ..###.##..
    // ...####...
    // ..######..
    // ..######..
    // ...####...
    // ..........";

    let mut grid = hashtag_grid(input).map(|c| if *c { -1 } else { 0 });

    let neigh = grid.moore();

    let mut queue = grid
        .grid_iter_index()
        .filter(|(_, x)| **x == -1)
        .filter(|(pos, _)| {
            grid.is_on_any_border(*pos) || neigh.get_all_neighbors(*pos).any(|n| grid[n] == 0)
        })
        .map(|(pos, _)| pos)
        .to_vec();
    for pos in &queue {
        grid[*pos] = 1;
    }

    let mut next_queue = Vec::new();
    for depth in 2.. {
        next_queue.clear();
        for pos in &queue {
            for n in neigh.get_all_neighbors(*pos) {
                if grid[n] == -1 {
                    grid[n] = depth;
                    next_queue.push(n);
                }
            }
        }
        if next_queue.is_empty() {
            break;
        }
        std::mem::swap(&mut queue, &mut next_queue);
    }

    let result = grid.sum();

    result!(result);
}
