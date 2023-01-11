use std::io;

fn main() {
    let player_names: Vec<String> = get_players();

    for player in player_names.iter() {
        println!("{player}");
    }
    game_loop();
}

fn get_players() -> Vec<String> {
    println!("Enter number of players: ");

    let mut player_count_line = String::new();
    let mut player_count: u8 = 0;
    let mut player_names: Vec<String> = Vec::new();


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

        player_names.push(player_name);


        i += 1;
    }

    return player_names;

}

fn game_loop() {

    
}
