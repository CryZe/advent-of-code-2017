pub fn part1(input: &str) -> usize {
    let mut split = input.split("");

    let mut is_garbage = false;
    let mut counter = 0;
    let mut depth = 0;

    while let Some(s) = split.next() {
        let s: &str = s;
        if s.is_empty() {
            continue;
        };
        if is_garbage {
            match s.as_ref() {
                "!" => {
                    split.next();
                }
                ">" => {
                    is_garbage = false;
                }
                _ => {}
            }
            continue;
        }
        match s.as_ref() {
            "{" => {
                depth += 1;
                counter += depth;
            }
            "}" => {
                depth -= 1;
            }
            "<" => {
                is_garbage = true;
            }
            _ => {}
        }
    }

    counter
}
