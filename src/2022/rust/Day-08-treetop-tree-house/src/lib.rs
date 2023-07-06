type TreeCamp = Vec<Vec<u32>>;

// Tienes que checar si el arbolito que estas viendo es mas alto que los que estan a su alrededor
// Pero va desde su posicion hasta que sale
pub fn is_tile_visible(x: i32, y: i32, camp: &TreeCamp) -> bool {
    let current_height = &camp.get(x as usize).unwrap().get(y as usize).unwrap();

    // Check sides

    let row = camp.get(x as usize).unwrap();

    let splits = row.split_at(y as usize);
    // Check left
    let max_left = splits.0.iter().max().unwrap();

    if *current_height > max_left {
        return true;
    }

    let max_right = splits.1.iter().skip(1).max().unwrap();

    if *current_height > max_right {
        return true;
    }

    let mut max_up = 0;

    for i in 0..(x as usize) {
        let cmp_tree = camp.get(i).unwrap().get(y as usize).unwrap();

        if cmp_tree > &max_up {
            max_up = *cmp_tree;
        }
    }

    if **current_height > max_up {
        return true;
    }

    let mut max_down = 0;

    for i in (x as usize + 1)..(camp.len()) {
        let cmp_tree = camp.get(i).unwrap().get(y as usize).unwrap();

        if cmp_tree > &max_down {
            max_down = *cmp_tree;
        }
    }

    if **current_height > max_down {
        return true;
    }

    false
}

// checks if the tile is an edge
pub fn is_edge(x: i32, y: i32, num_rows: usize, num_cols: usize) -> bool {
    match (x, y) {
        (0, _) => true,
        (_, 0) => true,
        (x, _) if x == num_rows as i32 - 1 => true,
        (_, y) if y == num_cols as i32 - 1 => true,
        _ => false,
    }
}

// Do not check the edges, they wont be considered
pub fn scenic_score_tile(x: i32, y: i32, camp: &TreeCamp) -> u32 {
    println!("x: {}, y: {}", x, y);
    let reference = camp.get(x as usize).unwrap().get(y as usize).unwrap();

    let mut score = 1;

    // Check sides

    let row = camp.get(x as usize).unwrap();

    let splits = row.split_at(y as usize);

    // Check left
    let mut sum_score = 0;

    for i in splits.0.iter().rev() {
        sum_score += 1;
        if i >= reference {
            break;
        }
    }

    score *= sum_score;
    println!("left Score: {}", sum_score);
    sum_score = 0;

    for i in splits.1.iter().skip(1) {
        sum_score += 1;
        if i >= reference {
            break;
        }
    }

    score *= sum_score;
    println!("right Score: {}", sum_score);
    sum_score = 0;

    // Check up
    for i in (0..(x as usize)).rev() {
        let cmp_tree = camp.get(i).unwrap().get(y as usize).unwrap();

        sum_score += 1;

        if cmp_tree >= reference {
            break;
        }
    }

    score *= sum_score;
    println!("up Score: {}", sum_score);
    sum_score = 0;

    for i in (x as usize + 1)..(camp.len()) {
        let cmp_tree = camp.get(i).unwrap().get(y as usize).unwrap();

        sum_score += 1;
        if cmp_tree >= reference {
            break;
        }
    }

    score *= sum_score;
    println!("down Score: {}", sum_score);
    score
}

pub fn find_max_scenic_score(input: &str) -> u32 {
    let camp = input_to_camp(input);

    let num_rows = camp.len();
    let num_cols = camp[0].len();

    let mut scores = vec![];

    for i in 0..(camp.len()) {
        let current_row = &camp[i];

        for j in 0..(current_row.len()) {
            if is_edge(i as i32, j as i32, num_rows, num_cols) {
                continue;
            }

            scores.push(scenic_score_tile(i as i32, j as i32, &camp));
        }
    }

    scores.iter().max().unwrap().clone()
}

pub fn input_to_camp(input: &str) -> TreeCamp {
    let rows = input.split("\n").collect::<Vec<&str>>();
    let rows: TreeCamp = rows
        .iter()
        .filter(|row| !row.is_empty())
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    rows
}

pub fn find_how_many_visible_trees(input: &str) -> u32 {
    let mut visible_trees = 0;

    let camp: TreeCamp = input_to_camp(input);
    let num_rows = camp.len();
    let num_cols = camp[0].len();

    // Count the inner trees
    for i in 0..(camp.len()) {
        let current_row = &camp[i];

        for j in 0..(current_row.len()) {
            if is_edge(i as i32, j as i32, num_rows, num_cols) {
                visible_trees += 1;
                continue;
            }

            if is_tile_visible(i as i32, j as i32, &camp) {
                visible_trees += 1;
                continue;
            }
        }
    }

    visible_trees
}

#[cfg(test)]
mod tests {

    use utility_2022::{get_input, is_demo_mode};

    use super::*;

    pub mod part1 {

        use super::*;

        #[test]
        fn test_demo_input() {
            if !is_demo_mode() {
                return;
            }

            let input = get_input();

            assert_eq!(find_how_many_visible_trees(&input), 21);
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();

            println!("Answer: {}", find_how_many_visible_trees(&input));
        }
    }

    pub mod part2 {

        use super::*;

        #[test]
        fn test_demo_input() {
            if !is_demo_mode() {
                return;
            }

            let input = get_input();

            let camp = input_to_camp(&input);
            assert_eq!(scenic_score_tile(3, 2, &camp), 8, "3,2 is incorrect");
            assert_eq!(find_max_scenic_score(&input), 8, "Camp is incorrect");
        }

        #[test]
        fn test_input() {
            if is_demo_mode() {
                return;
            }

            let input = get_input();

            println!("Answer pt2: {}", find_max_scenic_score(&input));
        }
    }
}
