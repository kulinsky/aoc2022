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

pub enum PlayResult {
    Win,
    Draw,
    Lose,
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
