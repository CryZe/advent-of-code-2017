use std::collections::HashMap;

pub enum Dir {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Debug)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub value: u32,
}

impl Block {
    pub fn new(x: i32, y: i32, value: u32) -> Self {
        Self { x, y, value }
    }

    pub fn move_to(&self, dir: Dir) -> Self {
        use self::Dir::*;

        match dir {
            Right => Self {
                x: self.x + 1,
                y: self.y,
                value: 0,
            },
            Up => Self {
                x: self.x,
                y: self.y + 1,
                value: 0,
            },
            Left => Self {
                x: self.x - 1,
                y: self.y,
                value: 0,
            },
            Down => Self {
                x: self.x,
                y: self.y - 1,
                value: 0,
            },
        }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn neighbor_positions(&self) -> [(i32, i32); 8] {
        [
            (self.x - 1, self.y + 1),
            (self.x, self.y + 1),
            (self.x + 1, self.y + 1),
            (self.x - 1, self.y + 0),
            (self.x + 1, self.y + 0),
            (self.x - 1, self.y - 1),
            (self.x, self.y - 1),
            (self.x + 1, self.y - 1),
        ]
    }

    pub fn calculate_value(&self, map: &HashMap<(i32, i32), Block>) -> u32 {
        let value = self.neighbor_positions()
            .iter()
            .filter_map(|n| map.get(n))
            .map(|b| b.value)
            .sum();

        // println!("{:?}", self.position());
        // println!("{:?}", map);
        // println!("{}", value);

        value
    }
}

pub fn day3_2(input: u32) -> Option<u32> {
    let square_one = Block::new(0, 0, 1);

    let corners = |level: i32| {
        (
            (level, level),
            (-level, level),
            (-level, -level),
            (level, -level),
        )
    };

    let mut map = HashMap::<(i32, i32), Block>::new();

    let check = |b: &Block| b.value > input;

    let mut current_pos = square_one.position();

    map.insert(square_one.position(), square_one);

    for level in 1u32.. {
        let mut next = map.get(&current_pos).unwrap().move_to(Dir::Right);

        next.value = next.calculate_value(&map);

        if check(&next) {
            return Some(next.value);
        }

        current_pos = next.position();

        map.insert(next.position(), next);

        let current_corners = corners(level as i32);

        let current_y = map.get(&current_pos).unwrap().y;
        let target_y = (current_corners.0).1;

        for y in current_y..target_y {
            next = map.get(&current_pos).unwrap().move_to(Dir::Up);
            next.value = next.calculate_value(&map);

            if check(&next) {
                return Some(next.value);
            }

            current_pos = next.position();

            map.insert(next.position(), next);
        }

        let current_x = map.get(&current_pos).unwrap().x;
        let target_x = (current_corners.1).0;

        for x in (target_x..current_x).rev() {
            next = map.get(&current_pos).unwrap().move_to(Dir::Left);
            next.value = next.calculate_value(&map);

            if check(&next) {
                return Some(next.value);
            }

            current_pos = next.position();

            map.insert(next.position(), next);
        }

        let current_y = map.get(&current_pos).unwrap().y;
        let target_y = (current_corners.2).1;

        for y in (target_y..current_y).rev() {
            next = map.get(&current_pos).unwrap().move_to(Dir::Down);
            next.value = next.calculate_value(&map);

            if check(&next) {
                return Some(next.value);
            }

            current_pos = next.position();

            map.insert(next.position(), next);
        }

        let current_x = map.get(&current_pos).unwrap().x;
        let target_x = (current_corners.3).0;

        for x in current_x..target_x {
            next = map.get(&current_pos).unwrap().move_to(Dir::Right);
            next.value = next.calculate_value(&map);

            if check(&next) {
                return Some(next.value);
            }

            current_pos = next.position();

            map.insert(next.position(), next);
        }
    }

    None
}
