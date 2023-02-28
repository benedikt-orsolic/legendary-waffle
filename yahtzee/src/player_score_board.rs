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

    pub fn total_score(&self) -> i32{
        self.upper_section_score() + self.lower_section_score()
    }


    pub fn new(player_name: String) -> PlayerScoreBoard {
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
}

