use std::collections::HashMap;
use std::fs;

pub fn part_two() -> i64 {
    let input = fs::read_to_string("src/_2023/15.txt").unwrap();

    let mut boxes: HashMap<i32, Vec<(&str, i32)>> = HashMap::new();

    let input = input
        .split(",")
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();

    for t in input {
        let t = t.trim();
        if t.chars().last() == Some('-') {
            let label = t.replace("-", "");
            let box_number = hash(&label);

            if let Some(lenses) = boxes.get_mut(&box_number) {
                lenses.retain(|(l, _)| l != &label);
            }
        } else {
            let parts = t.split("=").collect::<Vec<&str>>();
            if parts.len() != 2 {
                continue;
            }
            let label = parts[0];
            let lens = (label, parts[1].parse::<i32>().unwrap());
            let box_number = hash(label);

            // if the boxes[box_number] is empty, create a new vector
            // otherwise, if the lens is not in the vector, add it
            // otherwise upday the lens

            if boxes.contains_key(&box_number) {
                let mut found = false;
                for i in 0..boxes[&box_number].len() {
                    if boxes[&box_number][i].0 == lens.0 {
                        boxes.get_mut(&box_number).unwrap()[i].1 = lens.1;
                        found = true;
                        break;
                    }
                }
                if !found {
                    boxes.get_mut(&box_number).unwrap().push(lens);
                }
            } else {
                boxes.insert(box_number, vec![lens]);
            }
        }
    }

    let mut total = 0;
    for (b, bx) in boxes {
        let box_number = b + 1;
        for (i, (_, v)) in bx.iter().enumerate() {
            let pos_number = (i + 1) as i32;
            total += box_number * pos_number * v;
        }
    }

    total as i64
}

fn hash(label: &str) -> i32 {
    let mut total = 0;
    for c in label.chars() {
        if c as i32 == 10 {
            continue;
        }
        total += c as i32;
        total *= 17;
        total %= 256;
    }
    total
}
