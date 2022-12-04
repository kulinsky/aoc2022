use std::collections::HashSet;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn problem_0(arr: Vec<&str>) -> usize {
    let mut total = 0;

    for s in arr {
        let m = s.len() / 2;

        let s1: HashSet<char> = s[..m].chars().collect();
        let s2: HashSet<char> = s[m..].chars().collect();

        let res: Vec<&char> = s1.intersection(&s2).into_iter().take(1).collect();
        total += LETTERS.chars().position(|x| x == *res[0]).unwrap() + 1;
    }

    total
}

pub fn problem_1(arr: Vec<&str>) -> usize {
    let mut total = 0;

    let mut i = 0;

    while i < arr.len() {
        let s1: HashSet<char> = arr[i].chars().collect();
        let s2: HashSet<char> = arr[i + 1].chars().collect();
        let s3: HashSet<char> = arr[i + 2].chars().collect();

        let z: Vec<&char> = s1
            .iter()
            .filter(|k| s2.contains(k))
            .filter(|k| s3.contains(k))
            .collect();

        for c in z {
            total += LETTERS.chars().position(|x| x == *c).unwrap() + 1;
        }

        i += 3;
    }

    total
}

#[test]
fn test_0() {
    assert_eq!(
        problem_0(vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        ]),
        157
    );
}

#[test]
fn test_1() {
    assert_eq!(
        problem_1(vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        ]),
        70
    );
}
