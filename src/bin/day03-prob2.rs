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

enum RatingType {
    O2,
    CO2,
}

fn calc_consumption_int(report: &[Vec<bool>], rating_type: RatingType) -> u32 {
    let mut matching = report.to_owned();
    for i in 0..matching[0].len() {
        let mut set_reports: Vec<Vec<bool>> = Vec::with_capacity(matching.len());
        let mut unset_reports: Vec<Vec<bool>> = Vec::with_capacity(matching.len());
        for num in matching {
            if num[i] {
                set_reports.push(num);
            } else {
                unset_reports.push(num);
            }
        }

        match rating_type {
            RatingType::O2 => {
                if set_reports.len() >= unset_reports.len() {
                    matching = set_reports;
                } else {
                    matching = unset_reports;
                }
            }
            RatingType::CO2 => {
                if set_reports.len() < unset_reports.len() {
                    matching = set_reports;
                } else {
                    matching = unset_reports;
                }
            }
        }

        if matching.len() == 1 {
            break;
        }
    }

    assert_eq!(
        matching.len(),
        1,
        "Should only have a single matching rating"
    );
    let rating_raw = matching.first().unwrap();

    rating_raw
        .iter()
        .rev()
        .enumerate()
        .map(|(i, is_set)| if *is_set { 2u32.pow(i as u32) } else { 0 })
        .sum()
}

fn calc_consumption(report: Vec<Vec<bool>>) -> (u32, u32) {
    (
        calc_consumption_int(&report, RatingType::O2),
        calc_consumption_int(&report, RatingType::CO2),
    )
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
    fn test_calc_consumption_int_o2() {
        assert_eq!(
            calc_consumption_int(
                &vec![
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
                ],
                RatingType::O2
            ),
            23
        )
    }

    #[test]
    fn test_calc_consumption_int_co2() {
        assert_eq!(
            calc_consumption_int(
                &vec![
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
                ],
                RatingType::CO2
            ),
            10
        )
    }
}
