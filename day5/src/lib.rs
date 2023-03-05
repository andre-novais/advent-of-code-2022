#[derive(Debug, Clone)]
struct Stack {
    crates: Vec<String>,
}

struct MoveOrder {
    times: u32,
    from: u32,
    to: u32,
}

impl Stack {
    fn pop(&mut self) -> String {
        return self.crates.pop().unwrap();
    }

    fn push(&mut self, crate_char: String) {
        self.crates.push(crate_char)
    }
}

fn execute_move_order_9000(order: MoveOrder, stacks: &mut Vec<Stack>) {
    let stack_from = &mut stacks[order.from as usize - 1];

    let mut moved = stack_from
        .crates
        .drain((stack_from.crates.len() - order.times as usize)..stack_from.crates.len())
        .collect::<Vec<String>>();

    moved.reverse();

    let stack_to = &mut stacks[order.to as usize - 1];

    for c in moved {
        stack_to.push(c)
    }
}

fn execute_move_order_9001(order: MoveOrder, stacks: &mut Vec<Stack>) {
    let stack_from = &mut stacks[order.from as usize - 1];

    let moved = stack_from
        .crates
        .drain((stack_from.crates.len() - order.times as usize)..stack_from.crates.len())
        .collect::<Vec<String>>();

    let stack_to = &mut stacks[order.to as usize - 1];

    for c in moved {
        stack_to.push(c)
    }
}

fn get_final_result(stacks: Vec<Stack>) -> String {
    let mut result = String::new();

    for stack in stacks {
        if let Some(char) = stack.crates.last() {
            result.push_str(&char.to_string())
        }
    }

    return result;
}

fn parse_starting_stacks(starting_stacks: &str) -> Vec<Stack> {
    let input = starting_stacks.split("\n");

    let mut stacks: Vec<Stack> = Vec::new();
    let mut strings = Vec::new();

    for line in input {
        let chars = line.chars().collect::<Vec<char>>();
        let mut temp_vec = Vec::new();

        for four_chars in chars.chunks(4) {
            let elem = four_chars.into_iter().collect::<String>();
            let normalized_elem = elem.replace("[", "").replace("]", "").replace(" ", "");

            temp_vec.push(normalized_elem);
        }

        strings.push(temp_vec);
    }

    let mut transposed_vec = Vec::new();
    for _ in 0..strings[0].len() {
        transposed_vec.push(Vec::new())
    }

    for i in 0..strings.len() {
        for j in (0..strings[0].len()).rev() {
            if strings[i][j] != "" {
                transposed_vec[j].push(strings[i][j].clone())
            }
        }
    }

    for vec in &mut transposed_vec {
        vec.reverse();
    }

    for vec in transposed_vec {
        let stack = Stack {
            crates: vec[1..vec.len()].to_vec(),
        };

        stacks.push(stack)
    }

    return stacks;
}

fn parse_move_orders(input: &str) -> Vec<MoveOrder> {
    return input
        .trim()
        .split("\n")
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|args| MoveOrder {
            times: args[1].parse::<u32>().unwrap(),
            from: args[3].parse::<u32>().unwrap(),
            to: args[5].parse::<u32>().unwrap(),
        })
        .collect();
}

pub fn part_a(starting_stacks: &str, input: &str) -> String {
    let mut stacks = parse_starting_stacks(starting_stacks);
    let move_orders = parse_move_orders(input);

    for order in move_orders {
        execute_move_order_9000(order, &mut stacks)
    }

    return get_final_result(stacks);
}

pub fn part_b(starting_stacks: &str, input: &str) -> String {
    let mut stacks = parse_starting_stacks(starting_stacks);
    let move_orders = parse_move_orders(input);

    for order in move_orders {
        execute_move_order_9001(order, &mut stacks)
    }

    return get_final_result(stacks);
}

#[cfg(test)]
#[cfg(test)]
mod test {
    #[test]
    fn part_a() {
        assert_eq!(
            super::part_a(include_str!("stacks.txt"), include_str!("input.txt")),
            "NTWZZWHFV".to_string()
        )
    }

    #[test]
    fn part_b() {
        assert_eq!(
            super::part_b(include_str!("stacks.txt"), include_str!("input.txt")),
            "BRZGFVBTJ".to_string()
        )
    }
}
