pub fn part_a(input: &str) -> i64 {
    let mut letters: Vec<String> = Vec::new();

    for letter in b'a'..=b'z' {
        letters.push((letter as char).to_string())
    }

    for letter in b'A'..=b'Z' {
        letters.push((letter as char).to_string())
    }

    let mut sum = 0;

    for line in input.trim().split("\n") {
        //vJrwpWtwJgWrhcsFMMfFFhFp
        let (compart1, compart2) = line.split_at(line.len() / 2);

        let mut duplicates = Vec::new();

        for c in compart2.chars() {
            if compart1.contains(&c.to_string()) {
                duplicates.push(c.to_string())
            }
        }

        duplicates.dedup();

        sum += duplicates
            .into_iter()
            .map(|letter| letters.iter().position(|c| &letter == c).unwrap() as i32)
            .map(|num| num + 1)
            .sum::<i32>();
    }

    sum.into()
}

pub fn part_b(input: &str) -> i64 {
    let mut letters: Vec<String> = Vec::new();

    for letter in b'a'..=b'z' {
        letters.push((letter as char).to_string())
    }

    for letter in b'A'..=b'Z' {
        letters.push((letter as char).to_string())
    }

    let mut sum = 0;

    let mut tempLines: Vec<&str> = Vec::new();

    for line in input.trim().split("\n") {
        //println!("tempLines: \n{}\n", tempLines.join("\n"));
        match tempLines.len() {
            2 => {
                tempLines.push(line);
                // println!(
                //     "lines: \n{}, \n{}, \n{}",
                //     tempLines[0], tempLines[1], tempLines[2]
                // );

                let duplicate = tempLines[2]
                    .chars()
                    .into_iter()
                    .find(|c| {
                        tempLines[0].contains(&c.to_string())
                            && tempLines[1].contains(&c.to_string())
                    })
                    .unwrap()
                    .to_string();

                sum += 1 + letters.iter().position(|c| &duplicate == c).unwrap() as i32;

                tempLines.clear();
            }
            _ => tempLines.push(line),
        }
    }

    sum.into()
}

#[cfg(test)]
#[cfg(test)]
mod test {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 0)
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 0)
    }
}
