pub fn part_a(input: &str) -> usize {
    let text = input.trim();
    for (n, window) in text
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .windows(4)
        .enumerate()
    {
        let mut unique_letters = Vec::new();

        for letter in window {
            if unique_letters.contains(letter) {
                continue;
            }

            unique_letters.push(letter.to_string());

            if unique_letters.len() == 4 {
                return n + 4;
            }
        }
    }

    0
}

pub fn part_b(input: &str) -> usize {
    let text = input.trim();
    for (n, window) in text
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .windows(14)
        .enumerate()
    {
        let mut unique_letters = Vec::new();

        for letter in window {
            if unique_letters.contains(letter) {
                continue;
            }

            unique_letters.push(letter.to_string());

            if unique_letters.len() == 14 {
                return n + 14;
            }
        }
    }

    0
}

#[cfg(test)]
#[cfg(test)]
mod test {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1920)
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 2334)
    }
}
