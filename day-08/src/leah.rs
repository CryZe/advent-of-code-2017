use std::collections::HashMap;

pub fn part1(input: &str) -> isize {
    let mut map: HashMap<&str, isize> = HashMap::new();

    for line in input.lines() {
        let mut split = line.split_whitespace();
        let reg = split.next().unwrap();
        let op = split.next().unwrap();
        let n = split.next().unwrap().parse::<isize>().unwrap();
        split.next();
        let c_reg = split.next().unwrap();
        let cond = split.next().unwrap();
        let c_n = split.next().unwrap().parse::<isize>().unwrap();

        let c_reg_v = *map.get(c_reg).unwrap_or(&0);

        if !match cond.as_ref() {
            ">" => c_reg_v > c_n,
            "<" => c_reg_v < c_n,
            ">=" => c_reg_v >= c_n,
            "<=" => c_reg_v <= c_n,
            "==" => c_reg_v == c_n,
            "!=" => c_reg_v != c_n,
            _ => panic!("invalid cond"),
        } {
            continue;
        }

        let mut reg_v = map.entry(reg).or_insert(0);

        match op.as_ref() {
            "inc" => *reg_v += n,
            "dec" => *reg_v -= n,
            _ => panic!("invalid op"),
        }
    }

    map.iter()
        .fold(0, |acc, (_, &v)| if v > acc { v } else { acc })
}
