use std::io::{self, Write};
use rand::Rng;
mod player_score_board;
use player_score_board::PlayerScoreBoard;

fn print_player_score_board(player_score_board: &Vec<PlayerScoreBoard>) {
    clear_screen();

    let w: usize = 15;
    let total_width: usize = (player_score_board.len() + 1) * (w+3) + 1;

    let mut i: usize = 0;
    print!("   {:>w$} |", "name");
    while i < player_score_board.len() {
        print!("{:>w$} | ", player_score_board[i].name);
        i += 1;
    }
    println!();

    println!("{:-<total_width$}", "");
    i = 0;
    print!(" A {:>w$} |", "aces");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].aces;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");
    
    i = 0;
    print!(" B {:>w$} |", "twos");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].twos;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!(" C {:>w$} |", "threes");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].threes;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!(" D {:>w$} |", "fours");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].fours;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!(" E {:>w$} |", "fives");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].fives;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!(" F {:>w$} |", "sixes");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].sixes;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!("   {:>w$} |", "Total upper");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].upper_section_score();
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:=<total_width$}", "");

    i = 0;
    print!(" G {:>w$} |", "Three of a kind");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].three_of_a_kind;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!(" H {:>w$} |", "Four of a kind");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].four_of_a_kind;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!(" J {:>w$} |", "Small straight");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].small_straight;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!(" K {:>w$} |", "Large straight");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].large_straight;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!(" L {:>w$} |", "Yahtzee");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].yahtzee;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!(" M {:>w$} |", "Chance");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].chance;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");
    
    i = 0;
    print!("   {:>w$} |", "Lower section");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].lower_section_score();
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:=<total_width$}", "");

    print!("   {:>w$} |", "Total");
    i = 0;
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].total_score();
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:=<total_width$}", "");
}

fn print_value_segment(val: i32, width: usize) {
        if val < 0 {
            print!("{:>width$} | ", "/");
        }else{
            print!("{:>width$} | ", val);
        }
}

fn create_player_score_board(player_name: String) -> PlayerScoreBoard {
    let player_score_board: PlayerScoreBoard = PlayerScoreBoard {
        name : player_name,

        aces : -1,
        twos : -1,
        threes : -1,
        fours : -1,
        fives : -1,
        sixes : -1,

        three_of_a_kind : -1,
        four_of_a_kind : -1,
        full_house : -1,
        small_straight : -1,
        large_straight : -1,
        yahtzee : -1,
        chance : -1,
    };

    return player_score_board;
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H")
}

fn main() {
    let mut player_names: Vec<PlayerScoreBoard> = get_players();
    print_player_score_board(&player_names);

    game_loop(&mut player_names);
}

fn get_players() -> Vec<PlayerScoreBoard> {

    let mut player_count_line = String::new();
    let mut player_count: u8 = 0;
    let mut player_score_board: Vec<PlayerScoreBoard> = Vec::new();


    let min_player_count = 1;
    let max_player_count = 4;


    while player_count < min_player_count || player_count > max_player_count {
        
        print!("Enter number of players: ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut player_count_line){
            Ok (_) => print!(""),
            Err(err) => println!("Something went wrong {err}"), 
        };

        match player_count_line.trim().parse::<u8>(){
            Ok(count) => player_count = count,
            Err(e) => println!("Something went wrong:  {e}"),
        }
    }


    let mut i: usize = 0;
    while i < player_count.into() {
        
        let mut player_name = String::new();
        print!("Enter name for player {}: ", i+1);
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut player_name){
            Ok (_) => print!(""),
            Err(err) => println!("Something went wrong {err}"), 
        };

        // Remove trailing new line
        player_name.pop();

        player_score_board.push(create_player_score_board(player_name));


        i += 1;
    }
    
    return player_score_board;
}

fn game_loop(player_board: &mut Vec<PlayerScoreBoard>) {

    let mut round: u8 = 0;
    let max_rounds: u8 = 11;

    while round < max_rounds {

        play_round(player_board);
        round += 1;
    }
}

fn play_round(player_board: &mut Vec<PlayerScoreBoard>) {
    
    let mut i: usize = 0;
    while i < player_board.len() {
        clear_screen();
        print_player_score_board(&player_board);
        println!("{}", player_board[i].name);
        let selected_dice: [u8; 5] = get_selected_dice();
        while !select_category(&mut player_board[i], selected_dice) {};
        i += 1;
    }
}

fn get_selected_dice()-> [u8; 5] {

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






























fn select_category(player: &mut PlayerScoreBoard, dice: [u8; 5]) -> bool {
    println!("Select category");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Faild to get category");

    if input.contains("A") {
        if player.aces >= 0 { return false; }
        player.aces = get_dice_sum_for_val(1, dice);
        return true;
    }

    if input.contains("B") { 
        if !player.twos < 0 { return false; }
        player.twos = get_dice_sum_for_val(2, dice);
        return true;
    }

    if input.contains("C") {
        if !player.threes < 0 { return false; }
        player.threes = get_dice_sum_for_val(3, dice);
        return true;
    }

    if input.contains("D") {
        if !player.fours < 0 { return false; }
        player.fours = get_dice_sum_for_val(4, dice);
        return true;
    }

    if input.contains("E") {
        if !player.fives < 0 { return false; }
        player.fives = get_dice_sum_for_val(5, dice);
        return true;
    }
        
    if input.contains("F") {
        if !player.sixes < 0 { return false; }
        player.sixes = get_dice_sum_for_val(6, dice);
        return true;
    }

    return false;       
    
}

fn get_dice_sum_for_val(value: u8, dice: [u8; 5]) -> i32 {
    let mut sum: i32 = 0;
    let mut i = 0;
    while i < 5 {
        if dice[i] == value { sum += 1; }
        i += 1;
    }
    return sum;
}

fn print_dice(dice: [u8; 5]){
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

fn select_dice_to_reroll() -> [bool; 5]{ 
    let mut input = String::new();
    let mut selected_dice: [bool; 5] = 
        [false, false, false, false, false];

    println!("Select dice to rerol");

    io::stdin()
        .read_line(&mut input)
        .expect("Faild to read dice selection");

    if input.contains("A") { selected_dice[0] = true; }
    if input.contains("B") { selected_dice[1] = true; }
    if input.contains("C") { selected_dice[2] = true; }
    if input.contains("D") { selected_dice[3] = true; }
    if input.contains("E") { selected_dice[4] = true; }

    return selected_dice;
}


fn get_rng_dice() -> u8 {
    let rng = rand::thread_rng().gen_range(1..=6);
    return rng;
}
