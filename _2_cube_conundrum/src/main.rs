use utils::get_file_string;

#[derive(Default)]
struct Game {
    r: u64,
    g: u64,
    b: u64,
}

fn parse_game(s: &str) -> (u64, Vec<Game>) {
    let parts: Vec<&str> = s.split(":").collect();
    let id: u64 = parts[0].split(" ").nth(1).unwrap().parse().unwrap();
    let mut g = vec![];
    for gameline in parts[1].split(";") {
        let mut game = Game::default();
        let cubes: Vec<&str> = gameline.split(", ").collect();
        for cube in cubes {
            let mut p = cube.trim().split(" ");
            let count: u64 = p.nth(0).unwrap().parse().unwrap();
            let color = p.nth(0).unwrap();
            match color {
                "blue" => game.b = count,
                "red" => game.r = count,
                "green" => game.g = count,
                _ => panic!("unknown color"),
            };
        }
        g.push(game);
    }
    (id, g)
}

fn is_games_possible(games: Vec<Game>, max_r: u64, max_g: u64, max_b: u64) -> bool {
    for game in games {
        if game.r > max_r || game.g > max_g || game.b > max_b {
            return false;
        }
    }
    return true;
}

fn possible_games_sum_part1(s: String, max_r: u64, max_g: u64, max_b: u64) -> u64 {
    let mut res = 0;
    for l in s.lines() {
        let (id, games) = parse_game(l);
        if is_games_possible(games, max_r, max_g, max_b) {
            res += id;
        }
    }
    res
}

fn sum_of_power_of_min_cubes(s: String) -> u64 {
    let mut res = 0;
    for l in s.lines() {
        let (_, games) = parse_game(l);
        let mut max_r = 1;
        let mut max_g = 1;
        let mut max_b = 1;
        for game in games {
            if game.r > max_r {
                max_r = game.r;
            }
            if game.g > max_g {
                max_g = game.g;
            }
            if game.b > max_b {
                max_b = game.b;
            }
        }
        res += max_r * max_g * max_b;
    }
    res
}

fn main() {
    let s = get_file_string();
    println!("part1 {}", possible_games_sum_part1(s.clone(), 12, 13, 14));
    println!("part2 {}", sum_of_power_of_min_cubes(s));
}
