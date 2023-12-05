use std::collections::HashSet;

use utils::get_file_string;

fn parse_int(
    char_row: i32,
    char_col: i32,
    mat: &[&[char]],
    parsed: &mut HashSet<(usize, usize)>,
) -> u64 {
    // caller should guarantee that row/col are valid idx for mat
    let row = char_row as usize;
    let col = char_col as usize;

    if !mat[row][col].is_numeric() {
        return 0;
    }


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

    if parsed.contains(&(st, end)) {
        return 0;
    }

    // println!("st {} end {}", st, end);

    let k: String = dbg!(mat[row][st..=end].iter().collect());

    parsed.insert((st, end));

    k.parse().unwrap()
}

fn sum_around(char_row: i32, char_col: i32, mat: &[&[char]]) -> u64 {
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
    let mut res = 0;
    let mut parsed = HashSet::new();

    for idx in idxs {
        let (row, col) = idx;
        if row < 0 || !(0..mat.len()).contains(&(row as usize)) {
            continue;
        }
        if col < 0 || !(0..mat[0].len()).contains(&(col as usize)) {
            continue;
        }

        if !mat[row as usize][col as usize].is_numeric() {
            continue;
        }

        // println!("row {} col {}", row, col);

        res += parse_int(row, col, mat, &mut parsed);
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
                res += dbg!(sum_around(row as i32, col as i32, view));
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
        for i in 10..0 {
            count += 1;
        }
        assert!(count > 0);
    }
}

fn main() {
    let s = get_file_string();
    println!("part1 {}", sum_of_part_numbers_part1(s.clone()));
}
