#[derive(Clone)]
pub struct PlayerScoreBoard {
    pub name: String,

    // Upper section
    pub aces: i32,
    pub twos: i32,
    pub threes: i32,
    pub fours: i32,
    pub fives: i32,
    pub sixes: i32,

    // Lower section
    pub three_of_a_kind: i32,
    pub four_of_a_kind: i32,
    pub full_house: i32,
    pub small_straight: i32,
    pub large_straight: i32,
    pub yahtzee: i32,
    pub chance: i32,
}

impl PlayerScoreBoard {
    pub fn upper_section_score(&self) -> i32 {
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

    pub fn lower_section_score(&self) -> i32 {
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

    pub fn total_score(&self) -> i32 {
        self.upper_section_score() + self.lower_section_score()
    }

    pub fn print_player_score_board(player_score_board: &Vec<Self>) {
        let w: usize = 15;
        let total_width: usize = (player_score_board.len() + 1) * (w + 3) + 1;

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
            Self::print_value_segment(val, w);
            i += 1;
        }
        println!();
        println!("{:-<total_width$}", "");

        i = 0;
        print!(" B {:>w$} |", "twos");
        while i < player_score_board.len() {
            let val: i32 = player_score_board[i].twos;
            Self::print_value_segment(val, w);
            i += 1;
        }
        println!();
        println!("{:-<total_width$}", "");

        i = 0;
        print!(" C {:>w$} |", "threes");
        while i < player_score_board.len() {
            let val: i32 = player_score_board[i].threes;
            Self::print_value_segment(val, w);
            i += 1;
        }
        println!();
        println!("{:-<total_width$}", "");

        i = 0;
        print!(" D {:>w$} |", "fours");
        while i < player_score_board.len() {
            let val: i32 = player_score_board[i].fours;
            Self::print_value_segment(val, w);
            i += 1;
        }
        println!();
        println!("{:-<total_width$}", "");

        i = 0;
        print!(" E {:>w$} |", "fives");
        while i < player_score_board.len() {
            let val: i32 = player_score_board[i].fives;
            Self::print_value_segment(val, w);
            i += 1;
        }
        println!();
        println!("{:-<total_width$}", "");

        i = 0;
        print!(" F {:>w$} |", "sixes");
        while i < player_score_board.len() {
            let val: i32 = player_score_board[i].sixes;
            Self::print_value_segment(val, w);
            i += 1;
        }
        println!();
        println!("{:-<total_width$}", "");

        i = 0;
        print!("   {:>w$} |", "Total upper");
        while i < player_score_board.len() {
            let val: i32 = player_score_board[i].upper_section_score();
            Self::print_value_segment(val, w);
            i += 1;
        }
        println!();
        println!("{:=<total_width$}", "");

        i = 0;
        print!(" G {:>w$} |", "Three of a kind");
        while i < player_score_board.len() {
            let val: i32 = player_score_board[i].three_of_a_kind;
            Self::print_value_segment(val, w);
            i += 1;
        }
        println!();
        println!("{:-<total_width$}", "");

        i = 0;
        print!(" H {:>w$} |", "Four of a kind");
        while i < player_score_board.len() {
            let val: i32 = player_score_board[i].four_of_a_kind;
            Self::print_value_segment(val, w);
            i += 1;
        }
        println!();
        println!("{:-<total_width$}", "");

        i = 0;
        print!(" J {:>w$} |", "Small straight");
        while i < player_score_board.len() {
            let val: i32 = player_score_board[i].small_straight;
            Self::print_value_segment(val, w);
            i += 1;
        }
        println!();
        println!("{:-<total_width$}", "");

        i = 0;
        print!(" K {:>w$} |", "Large straight");
        while i < player_score_board.len() {
            let val: i32 = player_score_board[i].large_straight;
            Self::print_value_segment(val, w);
            i += 1;
        }
        println!();
        println!("{:-<total_width$}", "");

        i = 0;
        print!(" L {:>w$} |", "Yahtzee");
        while i < player_score_board.len() {
            let val: i32 = player_score_board[i].yahtzee;
            Self::print_value_segment(val, w);
            i += 1;
        }
        println!();
        println!("{:-<total_width$}", "");

        i = 0;
        print!(" M {:>w$} |", "Chance");
        while i < player_score_board.len() {
            let val: i32 = player_score_board[i].chance;
            Self::print_value_segment(val, w);
            i += 1;
        }
        println!();
        println!("{:-<total_width$}", "");

        i = 0;
        print!("   {:>w$} |", "Lower section");
        while i < player_score_board.len() {
            let val: i32 = player_score_board[i].lower_section_score();
            Self::print_value_segment(val, w);
            i += 1;
        }
        println!();
        println!("{:=<total_width$}", "");

        print!("   {:>w$} |", "Total");
        i = 0;
        while i < player_score_board.len() {
            let val: i32 = player_score_board[i].total_score();
            Self::print_value_segment(val, w);
            i += 1;
        }
        println!();
        println!("{:=<total_width$}", "");
    }

    fn print_value_segment(val: i32, width: usize) {
        if val < 0 {
            print!("{:>width$} | ", "/");
        } else {
            print!("{:>width$} | ", val);
        }
    }

    pub fn new(player_name: String) -> PlayerScoreBoard {
        let player_score_board: PlayerScoreBoard = PlayerScoreBoard {
            name: player_name,

            aces: -1,
            twos: -1,
            threes: -1,
            fours: -1,
            fives: -1,
            sixes: -1,

            three_of_a_kind: -1,
            four_of_a_kind: -1,
            full_house: -1,
            small_straight: -1,
            large_straight: -1,
            yahtzee: -1,
            chance: -1,
        };

        return player_score_board;
    }
}
