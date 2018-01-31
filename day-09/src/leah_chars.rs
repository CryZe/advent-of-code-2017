pub fn part1(input: &str) -> usize {
    let mut chars = input.chars();

    let mut is_garbage = false;
    let mut counter = 0;
    let mut depth = 0;

    while let Some(c) = chars.next() {
        if is_garbage {
            match c {
                '!' => {
                    chars.next();
                }
                '>' => {
                    is_garbage = false;
                }
                _ => {}
            }
            continue;
        }
        match c {
            '{' => {
                depth += 1;
                counter += depth;
            }
            '}' => {
                depth -= 1;
            }
            '<' => {
                is_garbage = true;
            }
            _ => {}
        }
    }

    counter
}
