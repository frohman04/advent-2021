use itertools::Itertools;

fn main() {
    let depths = std::fs::read_to_string("src/bin/day01.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| val.parse::<u16>().ok().unwrap())
                .collect::<Vec<u16>>()
        })
        .expect("Unable to open file");
    println!(
        "{:?}",
        depth_increases(depths)
    );
}

fn depth_increases(depths: Vec<u16>) -> usize {
    depths.iter().tuple_windows().filter(|(p, _, _, n)| n > p).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(depth_increases(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 5)
    }
}
