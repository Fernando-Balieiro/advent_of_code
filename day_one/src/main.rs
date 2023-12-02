fn main() {
    let input = include_str!("./input/day_one/input.txt");

    println!("{}", part1(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let digits = line.chars().filter(|n| n.is_digit(10)).collect::<Vec<_>>();
            let first_digit = digits.first()?;
            let last_digit = digits.last()?;
            format!("{first_digit}{last_digit}").parse::<usize>().ok()
        })
        .sum()
}

#[test]
fn test_part_1() {
    let input = "1abc2
		pqr3stu8vwx
		a1b2c3d4e5f
		treb7uchet";

    assert_eq!(part1(input), 142)
}
