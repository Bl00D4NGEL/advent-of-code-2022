use serde_json::Value;

fn main() {
    let contents = include_str!("./input.txt");
    let lines = contents
        .split("\n")
        .into_iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let correctly_sorted_indexes_sum = lines
        .chunks(2)
        .enumerate()
        .filter_map(|(idx, chunk)| -> Option<usize> {
            match are_values_sorted_correctly(
                &parse_json(chunk.get(0)?),
                &parse_json(chunk.get(1)?),
            ) {
                None => None,
                Some(v) => {
                    if v {
                        Some(idx + 1)
                    } else {
                        None
                    }
                }
            }
        })
        .sum::<usize>();

    println!("Day 13 part 1: {}", correctly_sorted_indexes_sum);
}

fn parse_json(value: &&str) -> Value {
    serde_json::from_str(&value).unwrap()
}

fn are_values_sorted_correctly(first: &Value, second: &Value) -> Option<bool> {
    if first.is_number() && second.is_number() {
        return are_numbers_sorted_correctly(first.as_i64().unwrap(), second.as_i64().unwrap());
    }

    if first.is_array() && second.is_array() {
        return are_arrays_sorted_correctly(
            first.as_array().unwrap().to_owned(),
            second.as_array().unwrap().to_owned(),
        );
    }

    if first.is_number() && second.is_array() {
        println!("First is number {:?}, second is array {:?}", first, second);
        // dbg!(compare_number_to_array(first, second));
        return are_values_sorted_correctly(&convert_to_array(first), second);
    }

    println!("First is array {:?}, second is number {:?}", first, second);
    // dbg!(compare_array_to_number(first, second));
    return are_values_sorted_correctly(first, &convert_to_array(second));
}

fn convert_to_array(val: &Value) -> Value {
    Value::Array(vec![val.to_owned()])
}

fn are_numbers_sorted_correctly(first: i64, second: i64) -> Option<bool> {
    if first == second {
        None
    } else {
        Some(first < second)
    }
}

fn are_arrays_sorted_correctly(first: Vec<Value>, second: Vec<Value>) -> Option<bool> {
    // if second.len() == 0 {
    //     return false;
    // }

    for (idx, first_val) in first.iter().enumerate() {
        let second_val = match second.get(idx) {
            None => return Some(false), // Right side ran out of items first -> List is NOT sorted correctly
            Some(v) => v,
        };

        let is_valid = are_values_sorted_correctly(first_val, second_val);
        if is_valid.is_some() {
            return is_valid;
        }
    }

    dbg!(first.len(), second.len());

    if first.len() < second.len() {
        Some(true)
    } else {
        None
    }
}
