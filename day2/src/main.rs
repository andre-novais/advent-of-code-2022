use std::{collections::HashMap, slice::Windows};

mod input;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
enum Game_signs {
    Rock,
    Paper,
    Scisor,
}
fn deriveScore(input: &str) {
    let losing_hash: HashMap<&Game_signs, &Game_signs> = HashMap::from([
        (&Game_signs::Rock, &Game_signs::Scisor),
        (&Game_signs::Scisor, &Game_signs::Paper),
        (&Game_signs::Paper, &Game_signs::Rock),
    ]);

    let winning_hash: HashMap<&Game_signs, &Game_signs> = HashMap::from([
        (&Game_signs::Rock, &Game_signs::Paper),
        (&Game_signs::Scisor, &Game_signs::Rock),
        (&Game_signs::Paper, &Game_signs::Scisor),
    ]);

    let command_lines = input.split(
        "
",
    );
    let mut sum_scores = 0;
    for line in command_lines {
        let commands = line.split(" ").collect::<Vec<&str>>();

        let oponent_command = parse_command(commands[0]);
        let my_command = derive_command_from_strategy(
            &oponent_command,
            commands[1],
            &winning_hash,
            &losing_hash,
        );

        println!(
            "
          raw match: {},
          translated match: {},
          sign score: {},
          match score: {}
        ",
            format!("{} vs {}", commands[0], commands[1]),
            format!("{:?} vs {:?}", oponent_command, my_command),
            sign_score(&my_command),
            battle_score(&oponent_command, &my_command)
        );
        let round_score = sign_score(&my_command) + battle_score(&oponent_command, &my_command);
        sum_scores += round_score
    }

    println!("{sum_scores}");
}

fn derive_command_from_strategy(
    oponent_command: &Game_signs,
    strategy: &str,
    winning_hash: &HashMap<&Game_signs, &Game_signs>,
    losing_hash: &HashMap<&Game_signs, &Game_signs>,
) -> Game_signs {
    match strategy {
        "X" => losing_hash[oponent_command].clone(),
        "Y" => oponent_command.clone(),
        "Z" => winning_hash[oponent_command].clone(),
        _ => panic!("unreachable"),
    }
}

fn parse_command(str: &str) -> Game_signs {
    match str {
        "A" | "X" => return Game_signs::Rock,
        "B" | "Y" => return Game_signs::Paper,
        "C" | "Z" => return Game_signs::Scisor,
        why => {
            panic!("unragchae   {}", why)
        }
    }
}

fn sign_score(command: &Game_signs) -> u32 {
    match command {
        Game_signs::Rock => return 1,
        Game_signs::Paper => return 2,
        Game_signs::Scisor => return 3,
    }
}

fn battle_score(command1: &Game_signs, command2: &Game_signs) -> u32 {
    if command1 == command2 {
        return 3;
    }

    let winning_hash = HashMap::from([
        (&Game_signs::Rock, &Game_signs::Scisor),
        (&Game_signs::Scisor, &Game_signs::Paper),
        (&Game_signs::Paper, &Game_signs::Rock),
    ]);

    if winning_hash[command2] == command1 {
        return 6;
    }

    return 0;
}

fn main() {
    let strategy_sheet: &str = input::INPUT;

    deriveScore(strategy_sheet)
}
