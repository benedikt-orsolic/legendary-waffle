mod player_score_board;
use player_score_board::PlayerScoreBoard;
use yahtzee;


fn main() {
    let mut player_names = yahtzee::get_players();
    yahtzee::clear_screen();
    yahtzee::PlayerScoreBoard::print_player_score_board(&player_names);

    yahtzee::game_loop(&mut player_names);
}

