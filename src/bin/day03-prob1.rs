fn main() {
    let report = std::fs::read_to_string("src/bin/day03.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(parse_line)
                .collect::<Vec<Vec<bool>>>()
        })
        .expect("Unable to open file");
    let (gamma, epsilon) = calc_consumption(report);
    println!("{:?}", gamma * epsilon);
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars()
        .map(|c| match c {
            '0' => false,
            '1' => true,
            _ => panic!("Unknown binary value '{}'", c),
        })
        .collect()
}

fn calc_consumption(report: Vec<Vec<bool>>) -> (u32, u32) {
    let threshold = (report.len() / 2) as u32;

    let mut set_counts: Vec<u32> = vec![0; report[0].len()];
    set_counts.fill(0);
    for num in report {
        for (i, val) in num.iter().enumerate() {
            if *val {
                set_counts[i] += 1;
            }
        }
    }

    let gamma_bits = set_counts
        .into_iter()
        .map(|count| count > threshold)
        .collect::<Vec<bool>>();
    let gamma = gamma_bits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, is_set)| if *is_set { 2u32.pow(i as u32) } else { 0 })
        .sum();

    let epsilon_bits = gamma_bits.into_iter().map(|is_set| !is_set);
    let epsilon = epsilon_bits
        .rev()
        .enumerate()
        .map(|(i, is_set)| if is_set { 2u32.pow(i as u32) } else { 0 })
        .sum();

    (gamma, epsilon)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line12() {
        assert_eq!(parse_line("11110"), vec![true, true, true, true, false])
    }

    #[test]
    fn test_parse_line1() {
        assert_eq!(
            parse_line("100110000110"),
            vec![true, false, false, true, true, false, false, false, false, true, true, false]
        )
    }

    #[test]
    fn test_calc_consumption() {
        assert_eq!(
            calc_consumption(vec![
                parse_line("00100"),
                parse_line("11110"),
                parse_line("10110"),
                parse_line("10111"),
                parse_line("10101"),
                parse_line("01111"),
                parse_line("00111"),
                parse_line("11100"),
                parse_line("10000"),
                parse_line("11001"),
                parse_line("00010"),
                parse_line("01010"),
            ]),
            (22, 9)
        )
    }
}
