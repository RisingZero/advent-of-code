const INPUT: &str = include_str!("input.txt");

fn main() {
    let result1 = solve1(INPUT);
    println!("Part 1:\nresult = {}", result1);

    let result2 = solve2(INPUT);
    println!("Part 2:\nresult = {}", result2);
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn check_report(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true;
    }

    let d1 = report
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<i32>>();
    d1.iter().all(|d| d.abs() >= 1 && d.abs() <= 3)
        && d1
            .windows(2)
            .all(|pair| pair[0].signum() == pair[1].signum())
}

fn solve1(input: &str) -> u32 {
    parse_input(input)
        .iter()
        .map(|report| check_report(report) as u32)
        .sum()
}

fn solve2(input: &str) -> u32 {
    parse_input(input)
        .iter()
        .map(|report| match check_report(report) {
            true => 1,
            false => {
                for i in 0..report.len() {
                    if check_report(
                        &report[..i]
                            .iter()
                            .chain(report.iter().skip(i + 1))
                            .copied()
                            .collect::<Vec<i32>>(),
                    ) {
                        return 1;
                    }
                }
                return 0;
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "#;

    #[test]
    fn test1() {
        assert_eq!(solve1(TEST_INPUT), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(TEST_INPUT), 4);
    }
}
