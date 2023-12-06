use std::collections::HashSet;
use log::debug;
use utils::get_file_string;

fn parse_int(
    char_row: i32,
    char_col: i32,
    mat: &[&[char]],
    parsed: &mut HashSet<(usize, usize, usize)>,
) -> Option<u64> {
    // caller should guarantee that row/col are valid idx for mat
    let row = char_row as usize;
    let col = char_col as usize;

    // if !mat[row][col].is_numeric() {
    //     return 0;
    // }

    // we wont parse vertical numbers for now
    let mut st = 0;
    for i in (0..=col).rev() {
        if !mat[row][i].is_numeric() {
            st = i + 1;
            break;
        }
    }

    let mut end = mat[row].len()-1;
    for i in col..mat[row].len() {
        if !mat[row][i].is_numeric() {
            end = i - 1;
            break;
        }
    }

    if parsed.contains(&(row, st, end)) {
        debug!("returned 0 because already processed {:?}", (st, end));
        return None;
    }

    // println!("st {} end {}", st, end);

    let k: String = mat[row][st..=end].iter().collect();

    parsed.insert((row, st, end));

    Some(k.parse().unwrap())
}

fn ints_around(char_row: i32, char_col: i32, mat: &[&[char]]) -> Vec<u64> {
    //   8 1 2
    //    \|/
    // 7 - # - 3
    //    /|\
    //   6 5 4
    let idxs = [
        (char_row - 1, char_col),
        (char_row - 1, char_col + 1),
        (char_row, char_col + 1),
        (char_row + 1, char_col + 1),
        (char_row + 1, char_col),
        (char_row + 1, char_col - 1),
        (char_row, char_col - 1),
        (char_row - 1, char_col - 1),
    ];
    let mut res = vec![];
    let mut parsed = HashSet::new();
    debug!("row {} col {} symbol {}", char_row, char_col, mat[char_row as usize][char_col as usize]);

    for idx in idxs {
        let (row, col) = idx;
        if row < 0 || !(0..mat.len()).contains(&(row as usize)) {
            debug!("skipped because of row");
            continue;
        }
        if col < 0 || !(0..mat[0].len()).contains(&(col as usize)) {
            debug!("skipped because of col");
            continue;
        }

        if !mat[row as usize][col as usize].is_numeric() {
            debug!("skipped because of not numeric");
            continue;
        }

        debug!("dir row {} col {}", row, col);

        if let Some(num) = parse_int(row, col, mat, &mut parsed) {
            res.push(num);
        }
    }
    res
}

fn assert_square_mat(mat: &[&[char]]) {
    let rows = mat.len();
    for row in mat {
        assert_eq!(rows, row.len());
    }
}

fn sum_of_part_numbers_part1(s: String) -> u64 {
    let arr: Vec<Vec<char>> = s.lines().map(|e| e.chars().collect()).collect();
    let view: Vec<&[char]> = arr.iter().map(|v| &v[..]).collect();
    let view = &view[..];
    assert_square_mat(view);
    let mut res = 0;
    for row in 0..view.len() {
        for col in 0..view[0].len() {
            if !view[row][col].is_numeric() && view[row][col] != '.' {
                // println!("found at row {} col {}", row, col);
                res += ints_around(row as i32, col as i32, view).iter().sum::<u64>();
            }
        }
    }
    res
}

fn engine_parts_part2(s: String) -> u64 {
    let arr: Vec<Vec<char>> = s.lines().map(|e| e.chars().collect()).collect();
    let view: Vec<&[char]> = arr.iter().map(|v| &v[..]).collect();
    let view = &view[..];
    assert_square_mat(view);
    let mut res = 0;
    for row in 0..view.len() {
        for col in 0..view[0].len() {
            if view[row][col] == '*' {
                // println!("found at row {} col {}", row, col);
                let ints = ints_around(row as i32, col as i32, view);
                if ints.len() == 2 {
                    res += ints.iter().fold(1, |acc, x| acc * x);
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_range() {
        let mut count = 0;
        for _i in 10..0 {
            count += 1;
        }
        assert!(count > 0);
    }
}

fn main() {
    env_logger::init();
    let s = get_file_string();
    println!("part1 {}", sum_of_part_numbers_part1(s.clone()));
    println!("part2 {}", engine_parts_part2(s));
}
