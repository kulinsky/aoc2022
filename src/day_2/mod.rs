use std::fs;

pub enum PlayOption {
    Rock,
    Paper,
    Scissors,
}

impl PlayOption {
    fn get_score(&self) -> i32 {
        match &self {
            PlayOption::Rock => 1,
            PlayOption::Paper => 2,
            PlayOption::Scissors => 3,
        }
    }
}

impl From<&str> for PlayOption {
    fn from(v: &str) -> Self {
        match v {
            "A" => PlayOption::Rock,
            "B" => PlayOption::Paper,
            "C" => PlayOption::Scissors,
            "X" => PlayOption::Rock,
            "Y" => PlayOption::Paper,
            "Z" => PlayOption::Scissors,
            _ => unreachable!(),
        }
    }
}

pub enum PlayResult {
    Win,
    Draw,
    Lose,
}

impl From<&str> for PlayResult {
    fn from(v: &str) -> Self {
        match v {
            "X" => PlayResult::Lose,
            "Y" => PlayResult::Draw,
            "Z" => PlayResult::Win,
            _ => unreachable!(),
        }
    }
}

fn get_result_for_b(a: &PlayOption, b: &PlayOption) -> PlayResult {
    match (a, b) {
        (PlayOption::Rock, PlayOption::Rock) => PlayResult::Draw,
        (PlayOption::Rock, PlayOption::Paper) => PlayResult::Win,
        (PlayOption::Rock, PlayOption::Scissors) => PlayResult::Lose,
        (PlayOption::Paper, PlayOption::Rock) => PlayResult::Lose,
        (PlayOption::Paper, PlayOption::Paper) => PlayResult::Draw,
        (PlayOption::Paper, PlayOption::Scissors) => PlayResult::Win,
        (PlayOption::Scissors, PlayOption::Rock) => PlayResult::Win,
        (PlayOption::Scissors, PlayOption::Paper) => PlayResult::Lose,
        (PlayOption::Scissors, PlayOption::Scissors) => PlayResult::Draw,
    }
}

fn get_res_score(opt: &PlayOption, res: &PlayResult) -> i32 {
    match res {
        PlayResult::Win => 6 + opt.get_score(),
        PlayResult::Draw => 3 + opt.get_score(),
        PlayResult::Lose => opt.get_score(),
    }
}

pub fn problem_1(arr: Vec<(PlayOption, PlayOption)>) -> i32 {
    let mut total = 0;

    for (a, b) in &arr {
        let res = get_result_for_b(a, b);
        total += get_res_score(b, &res);
    }

    total
}

fn get_option(a: &PlayOption, res: &PlayResult) -> PlayOption {
    match (a, res) {
        (PlayOption::Rock, PlayResult::Win) => PlayOption::Paper,
        (PlayOption::Rock, PlayResult::Draw) => PlayOption::Rock,
        (PlayOption::Rock, PlayResult::Lose) => PlayOption::Scissors,
        (PlayOption::Paper, PlayResult::Win) => PlayOption::Scissors,
        (PlayOption::Paper, PlayResult::Draw) => PlayOption::Paper,
        (PlayOption::Paper, PlayResult::Lose) => PlayOption::Rock,
        (PlayOption::Scissors, PlayResult::Win) => PlayOption::Rock,
        (PlayOption::Scissors, PlayResult::Draw) => PlayOption::Scissors,
        (PlayOption::Scissors, PlayResult::Lose) => PlayOption::Paper,
    }
}

pub fn problem_2(arr: Vec<(PlayOption, PlayResult)>) -> i32 {
    let mut total = 0;

    for (a, res) in &arr {
        let b = get_option(a, res);
        let tmp = get_result_for_b(a, &b);
        total += get_res_score(&b, &tmp);
    }

    total
}

#[test]
fn test_0() {
    let data = vec![
        (PlayOption::Rock, PlayOption::Paper),
        (PlayOption::Paper, PlayOption::Rock),
        (PlayOption::Scissors, PlayOption::Scissors),
    ];

    let ans = problem_1(data);

    assert_eq!(ans, 15);
}

#[test]
fn test_1() {
    let data = vec![
        (PlayOption::Rock, PlayResult::Draw),
        (PlayOption::Paper, PlayResult::Lose),
        (PlayOption::Scissors, PlayResult::Win),
    ];

    let ans = problem_2(data);

    assert_eq!(ans, 12);
}

pub fn get_data_1(filename: &str) -> Vec<(PlayOption, PlayOption)> {
    let binding = fs::read_to_string(filename).unwrap();
    let contents: Vec<&str> = binding.split_terminator("\n").collect();

    let mut res = Vec::new();

    for c in contents {
        let s: Vec<&str> = c.split(" ").collect();
        res.push((PlayOption::from(s[0]), PlayOption::from(s[1])));
    }

    res
}

pub fn get_data_2(filename: &str) -> Vec<(PlayOption, PlayResult)> {
    let binding = fs::read_to_string(filename).unwrap();
    let contents: Vec<&str> = binding.split_terminator("\n").collect();

    let mut res = Vec::new();

    for c in contents {
        let s: Vec<&str> = c.split(" ").collect();
        res.push((PlayOption::from(s[0]), PlayResult::from(s[1])));
    }

    res
}
