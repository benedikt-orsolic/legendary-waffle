use rand::Rng;
use std::io::{self, Write};
mod player_score_board;
pub use player_score_board::PlayerScoreBoard;

pub fn clear_screen() {
    print!("\x1B[2J\x1B[H")
}

pub fn get_players() -> Vec<PlayerScoreBoard> {
    let mut player_count_line = String::new();
    let mut player_count: u8 = 0;
    let mut player_score_board: Vec<PlayerScoreBoard> = Vec::new();

    let min_player_count = 1;
    let max_player_count = 4;

    while player_count < min_player_count || player_count > max_player_count {
        print!("Enter number of players: ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut player_count_line) {
            Ok(_) => print!(""),
            Err(err) => println!("Something went wrong {err}"),
        };

        match player_count_line.trim().parse::<u8>() {
            Ok(count) => player_count = count,
            Err(e) => println!("Something went wrong:  {e}"),
        }
    }

    let mut i: usize = 0;
    while i < player_count.into() {
        let mut player_name = String::new();
        print!("Enter name for player {}: ", i + 1);
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut player_name) {
            Ok(_) => print!(""),
            Err(err) => println!("Something went wrong {err}"),
        };

        // Remove trailing new line
        player_name.pop();

        player_score_board.push(PlayerScoreBoard::new(player_name));

        i += 1;
    }

    return player_score_board;
}

pub fn game_loop(player_board: &mut Vec<PlayerScoreBoard>) {
    let mut round: u8 = 0;
    let max_rounds: u8 = 11;

    while round < max_rounds {
        play_round(player_board);
        round += 1;
    }
}

pub fn play_round(player_board: &mut Vec<PlayerScoreBoard>) {
    let mut i: usize = 0;
    while i < player_board.len() {
        clear_screen();
        PlayerScoreBoard::print_player_score_board(player_board);
        println!("{}", player_board[i].name);
        let selected_dice: [u8; 5] = get_selected_dice();
        while !select_category(&mut player_board[i], selected_dice) {}
        i += 1;
    }
}

pub fn get_selected_dice() -> [u8; 5] {
    let mut i: usize = 0;
    let mut dice: [u8; 5] = [0, 0, 0, 0, 0];
    let mut selected_dice: [bool; 5] = [true, true, true, true, true];

    while i < 3 {
        let mut j: usize = 0;

        while j < 5 {
            if selected_dice[j] {
                dice[j] = get_rng_dice();
            }
            j += 1;
        }

        if selected_dice.iter().position(|&x| x == true) == None {
            break;
        }

        print_dice(dice);
        if i < 2 {
            selected_dice = select_dice_to_reroll();
        }
        i += 1;
    }

    return dice;
}

pub fn select_category(player: &mut PlayerScoreBoard, dice: [u8; 5]) -> bool {
    println!("Select category");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Faild to get category");

    if input.contains("A") {
        if player.aces >= 0 {
            return false;
        }
        player.aces = get_dice_sum_for_val(1, dice);
        return true;
    }

    if input.contains("B") {
        if !player.twos < 0 {
            return false;
        }
        player.twos = get_dice_sum_for_val(2, dice);
        return true;
    }

    if input.contains("C") {
        if !player.threes < 0 {
            return false;
        }
        player.threes = get_dice_sum_for_val(3, dice);
        return true;
    }

    if input.contains("D") {
        if !player.fours < 0 {
            return false;
        }
        player.fours = get_dice_sum_for_val(4, dice);
        return true;
    }

    if input.contains("E") {
        if !player.fives < 0 {
            return false;
        }
        player.fives = get_dice_sum_for_val(5, dice);
        return true;
    }

    if input.contains("F") {
        if !player.sixes < 0 {
            return false;
        }
        player.sixes = get_dice_sum_for_val(6, dice);
        return true;
    }

    return false;
}

pub fn get_dice_sum_for_val(value: u8, dice: [u8; 5]) -> i32 {
    let mut sum: i32 = 0;
    let mut i = 0;
    while i < 5 {
        if dice[i] == value {
            sum += 1;
        }
        i += 1;
    }
    return sum;
}

pub fn print_dice(dice: [u8; 5]) {
    println!();
    let mut i: usize = 0;
    while i < 5 {
        print!("{}    ", dice[i]);
        i += 1;
    }
    println!();
    println!("A    B    C    D    E");
    println!();
}

pub fn select_dice_to_reroll() -> [bool; 5] {
    let mut input = String::new();
    let mut selected_dice: [bool; 5] = [false, false, false, false, false];

    println!("Select dice to rerol");

    io::stdin()
        .read_line(&mut input)
        .expect("Faild to read dice selection");

    if input.contains("A") {
        selected_dice[0] = true;
    }
    if input.contains("B") {
        selected_dice[1] = true;
    }
    if input.contains("C") {
        selected_dice[2] = true;
    }
    if input.contains("D") {
        selected_dice[3] = true;
    }
    if input.contains("E") {
        selected_dice[4] = true;
    }

    return selected_dice;
}

pub fn get_rng_dice() -> u8 {
    let rng = rand::thread_rng().gen_range(1..=6);
    return rng;
}
