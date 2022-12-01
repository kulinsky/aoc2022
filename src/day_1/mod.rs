use std::fs;

pub fn solve_1(data: Vec<Vec<i32>>) -> i32 {
    let mut ans: Vec<i32> = data.into_iter().map(|x| x.iter().sum()).collect();

    ans.sort();

    ans.iter().rev().take(1).sum()
}

pub fn solve_2(data: Vec<Vec<i32>>) -> i32 {
    let mut ans: Vec<i32> = data.into_iter().map(|x| x.iter().sum()).collect();

    ans.sort();

    ans.iter().rev().take(3).sum()
}

pub fn get_data(filename: &str) -> Vec<Vec<i32>> {
    let binding = fs::read_to_string(filename).unwrap();
    let contents: Vec<&str> = binding.split_terminator("\n\n").collect();

    let mut res = Vec::new();

    for c in contents {
        let inner: Vec<i32> = c
            .split_terminator("\n")
            .map(|x| x.parse().unwrap())
            .collect();

        res.push(inner);
    }

    res
}

#[test]
fn test_0() {
    let data = vec![
        vec![1000, 2000, 3000],
        vec![4000],
        vec![5000, 6000],
        vec![7000, 8000, 9000],
        vec![10000],
    ];
    assert_eq!(solve_1(data), 24000);
}

#[test]
fn test_1() {
    let data = vec![
        vec![1000, 2000, 3000],
        vec![4000],
        vec![5000, 6000],
        vec![7000, 8000, 9000],
        vec![10000],
    ];
    assert_eq!(solve_2(data), 45000);
}
