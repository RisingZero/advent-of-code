use regex::Regex;

const REGEX1: &str = r"mul\((?<l_op>\d{1,3}),(?<r_op>\d{1,3})\)";
const REGEX2: &str =
    r"(?<mul>mul)\((?<l_op>\d{1,3}),(?<r_op>\d{1,3})\)|(?<do>do)\(\)|(?<dont>don't)\(\)";

const INPUT: &str = include_str!("input.txt");

fn main() {
    let result1 = solve1(INPUT);
    println!("Part 1:\nresult = {}", result1);

    let result2 = solve2(INPUT);
    println!("Part 2:\nresult = {}", result2);
}

fn solve1(input: &str) -> u32 {
    let re = Regex::new(REGEX1).unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let l_op: u32 = cap["l_op"].parse().unwrap();
            let r_op: u32 = cap["r_op"].parse().unwrap();
            l_op * r_op
        })
        .sum()
}

fn solve2(input: &str) -> u32 {
    let re = Regex::new(REGEX2).unwrap();
    re.captures_iter(input)
        .scan(1, |active, cap| {
            let op_res = match cap.name("mul") {
                Some(_) => {
                    let l_op: u32 = cap["l_op"].parse().unwrap();
                    let r_op: u32 = cap["r_op"].parse().unwrap();
                    l_op * r_op * *active
                }
                None => {
                    if let Some(_) = cap.name("do") {
                        *active = 1;
                    }
                    if let Some(_) = cap.name("dont") {
                        *active = 0;
                    }
                    0
                }
            };
            Some(op_res)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

    const TEST_INPUT2: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn test1() {
        assert_eq!(solve1(TEST_INPUT1), 161);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(TEST_INPUT2), 48);
    }
}
