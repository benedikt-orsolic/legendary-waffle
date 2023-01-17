use std::io;

struct PlayerScoreBoard {
    name: String,
    
    // Upper section
    aces: i32,
    twos: i32,
    threes: i32,
    fours: i32,
    fives: i32,
    sixes: i32,

    // Lower section
    three_of_a_kind: i32,
    four_of_a_kind: i32,
    full_house: i32,
    small_straight: i32,
    large_straight: i32,
    yahtzee: i32,
    chance: i32,
}

impl PlayerScoreBoard {
    fn upper_section_score(&self) -> i32 {
        let mut total: i32 = 0;
        if self.aces >= 0 {
            total += self.aces;
        }
        if self.threes >= 0 {
            total += self.threes;
        }
        if self.fours >= 0 {
            total += self.fours;
        }
        if self.fives >= 0 {
            total += self.fives;
        }
        if self.sixes >= 0 {
            total += self.sixes;
        }

        return total;
    }

    fn lower_section_score(&self) -> i32 {
        let mut total: i32 = 0;
        if self.three_of_a_kind >= 0 {
            total += self.three_of_a_kind;
        }
        if self.four_of_a_kind >= 0 {
            total += self.four_of_a_kind;
        }
        if self.full_house >= 0 {
            total += self.full_house;
        }
        if self.small_straight >= 0 {
            total += self.small_straight;
        }
        if self.large_straight >= 0 {
            total += self.large_straight;
        }
        if self.yahtzee >= 0 {
            total += self.yahtzee;
        }
        if self.chance >= 0 {
            total += self.chance;
        }

        return total;
    }

    fn total_score(&self) -> i32{
        self.upper_section_score() + self.lower_section_score()
    }
}

fn print_player_score_board(player_score_board: Vec<PlayerScoreBoard>) {
    clear_screen();

    let w: usize = 15;
    let total_width: usize = (player_score_board.len() + 1) * (w+3);

    let mut i: usize = 0;
    print!("{:>w$} |", "name");
    while i < player_score_board.len() {
        print!("{:>w$} | ", player_score_board[i].name);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!("{:>w$} |", "aces");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].aces;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");
    
    i = 0;
    print!("{:>w$} |", "twos");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].twos;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!("{:>w$} |", "threes");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].threes;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!("{:>w$} |", "fours");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].fours;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!("{:>w$} |", "fives");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].fives;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!("{:>w$} |", "sixes");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].sixes;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!("{:>w$} |", "Total upper");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].upper_section_score();
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:=<total_width$}", "");

    i = 0;
    print!("{:>w$} |", "Three of a kind");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].three_of_a_kind;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!("{:>w$} |", "Four of a kind");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].four_of_a_kind;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!("{:>w$} |", "Small straight");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].small_straight;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!("{:>w$} |", "Large straight");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].large_straight;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!("{:>w$} |", "Yahtzee");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].yahtzee;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");

    i = 0;
    print!("{:>w$} |", "Chance");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].chance;
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:-<total_width$}", "");
    
    i = 0;
    print!("{:>w$} |", "Lower section");
    while i < player_score_board.len() {
        let val: i32 = player_score_board[i].lower_section_score();
        print_value_segment(val, w);
        i += 1;
    }
    println!();
    println!("{:=<total_width$}", "");
    
    i = 0;
    print!("{:>w$} |", "Total");
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
    let player_names: Vec<PlayerScoreBoard> = get_players();
    print_player_score_board(player_names);

    game_loop();
}

fn get_players() -> Vec<PlayerScoreBoard> {
    println!("Enter number of players: ");

    let mut player_count_line = String::new();
    let mut player_count: u8 = 0;
    let mut player_score_board: Vec<PlayerScoreBoard> = Vec::new();


    let min_player_count = 1;
    let max_player_count = 4;


    while player_count < min_player_count || player_count > max_player_count {

        match io::stdin().read_line(&mut player_count_line){
            Ok (_) => print!(""),
            Err(err) => println!("Something went wrong {err}"), 
        };

        match player_count_line.trim().parse::<u8>(){
            Ok(count) => player_count = count,
            Err(e) => println!("Something went wrong:  {e}"),
        }
        println!("player count: {player_count}");
    }


    let mut i: usize = 0;
    while i < player_count.into() {
        
        let mut player_name = String::new();
        println!("Enter name for player {}", i+1);
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

fn game_loop() {

    
}
