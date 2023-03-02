use std::collections::HashMap;

mod input;

fn main() {
    let calory_sheet = input::input;

    let elfs_calory_map = get_elfs_calory_map(calory_sheet);
    print!("max: {}", get_max_elf(elfs_calory_map));
}

fn get_elfs_calory_map(calory_sheet: &str) -> HashMap<u32, u32> {
    let mut elfs_calory_map = HashMap::new();
    let mut current_elf = 1;
    let mut current_sum = 0;

    for line in calory_sheet.split(
        "
",
    ) {
        let line_is_empty = line == "";
        println!("line: {}, inEmpty: {}", line, line_is_empty);
        match line {
            "" => {
                elfs_calory_map.insert(current_elf, current_sum);

                current_elf += 1;
                current_sum = 0
            }
            stringue => {
                let parse_result = stringue.parse::<u32>();
                match parse_result {
                    Ok(u) => current_sum += u,
                    Err(why) => println!("sad"), //println!("{}, {}", stringue, why),
                }
            }
            _ => panic!("unreachable"),
        }
    }
    return elfs_calory_map;
}

fn get_max_elf(elfs_calory_map: HashMap<u32, u32>) -> u32 {
    let mut max = elfs_calory_map
      .values()
      .cloned()
      .collect::<Vec<u32>>();
      //.sort()
      // .iter()
      // .rev()
      // .take(3)
      // .sum();

      max.sort();

let meme = max.iter().rev().take(3).sum();
    return meme
}
