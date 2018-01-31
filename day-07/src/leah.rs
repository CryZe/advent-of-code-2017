use std::collections::HashMap;
use fxhash::FxHashMap;

#[derive(Debug)]
struct Node<'a> {
    weight: usize,
    childen: Option<Vec<&'a str>>,
}

pub fn part1(input: &str) -> &str {
    let mut current = "";
    let mut map = FxHashMap::default();

    for line in input.lines() {
        let mut split = line.splitn(2, " ");
        let name = split.next().unwrap();
        let rem = split.next().unwrap().split("-> ").skip(1).next();
        if let Some(children) = rem {
            for child in children.split(", ") {
                map.insert(child, name);
            }
            current = name;
        }
    }

    while let Some(next) = map.get(current) {
        current = next;
    }

    current
}

pub fn day(input: &str) -> usize {
    let start = part1(input);
    let mut map = HashMap::new();

    for line in input.lines() {
        let mut split = line.splitn(2, " (");
        let name = split.next().unwrap();
        let mut weight = split.next().unwrap().split(")");
        let weight_val = weight.next().unwrap().parse::<usize>().unwrap();
        let mut rem = weight.next().unwrap().split("-> ").skip(1);
        let mut node = Node {
            weight: weight_val,
            childen: if let Some(children) = rem.next() {
                Some(children.split(", ").collect::<Vec<&str>>())
            } else {
                None
            },
        };
        map.insert(name, node);
    }

    check_children(&map, start).err().unwrap()
}

fn check_children(map: &HashMap<&str, Node>, name: &str) -> Result<(usize, usize), usize> {
    if let Some(node) = map.get(name) {
        if let Some(ref children) = node.childen {
            let mut c_iter = children
                .iter()
                .map(|&child| check_children(&map, child))
                .collect::<Vec<_>>();

            // if there's an error pass it down
            if let Some(&err) = c_iter.iter().find(|c| !c.is_ok()) {
                return err;
            }

            let c_nums = c_iter.iter().map(|c| c.unwrap()).collect::<Vec<_>>();

            let sum: usize = c_nums.iter().map(|&(sum, _)| sum).sum();

            // check different
            if !(c_nums[0].0 * c_nums.len() == sum) {
                let mut flag = 0;

                let mut wrong = (0, 0);
                let mut correct = (0, 0);

                for &e in c_nums.iter() {
                    let len = c_nums
                        .iter()
                        .filter(|&n| n.0 != e.0)
                        .collect::<Vec<_>>()
                        .len();
                    if len > 1 && flag & 2 == 0 {
                        flag = flag << 1;
                        wrong = e;
                    }
                    if len == 1 && flag & 4 == 0 {
                        flag = flag << 2;
                        correct = e;
                    }
                    if flag == 6 {
                        break;
                    }
                }

                let diff = wrong.1 as i32 - (wrong.0 as i32 - correct.0 as i32);

                return Err(diff as usize);
            }

            Ok((node.weight + sum, node.weight))
        } else {
            Ok((node.weight, node.weight))
        }
    } else {
        panic!("Can't find node {:?}", name)
    }
}
