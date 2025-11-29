use super::*;

pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../../input/2024-06-2.txt");
    // let input = "";

    let child_map = input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(key, value)| (key, value.split(',').to_vec()))
        .to_map();

    let mut nodes = vec![("RR", "R".to_string())];
    let mut next_nodes = vec![];
    loop {
        let mut num_fruits = 0;
        let mut fruit_parent = None;
        for (node, path) in nodes.drain(..) {
            let Some(children) = child_map.get(node) else {
                continue;
            };
            for &child in children {
                let child_path = format!("{}{}", path, child.chars().next().unwrap());
                if child == "@" {
                    num_fruits += 1;
                    if num_fruits == 1 {
                        fruit_parent = Some(child_path.clone());
                    } else {
                        fruit_parent = None;
                    }
                } else {
                    next_nodes.push((child, child_path));
                }
            }
        }

        if let Some(best) = fruit_parent {
            result!(best);
            break;
        }

        std::mem::swap(&mut nodes, &mut next_nodes);
        next_nodes.clear();
    }
}
