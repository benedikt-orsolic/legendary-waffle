use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod player_score_board;
use player_score_board::PlayerScoreBoard;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titconsole_logles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/yahtzee" view=|cx| view! { cx, <YahtzeeBoard/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <a href="/yahtzee">{"Yahtzee"}</a>
    }
}

#[component]
fn YahtzeeBoard(cx: Scope) -> impl IntoView {
    let initBoard: Vec<PlayerScoreBoard> = Vec::new();
    let (board, set_board) = create_signal(cx, initBoard);

    set_board.update(|b| b.push(PlayerScoreBoard::new("Jhon".into())));
    set_board.update(|b| b.push(PlayerScoreBoard::new("Wayn".into())));
    set_board.update(|b| b.push(PlayerScoreBoard::new("Bain".into())));
    // set_board.update(|b| b.push(PlayerScoreBoard::new("Main".into())));

    let initPlayer: usize = 0;
    let (curr_player, set_curr_player) = create_signal(cx, initPlayer);

    let initDice: [u8; 5] = [1, 2, 3, 4, 5];
    let (dice, set_dice) = create_signal(cx, initDice);

    let initRoll: u8 = 0;
    let (roll, set_roll) = create_signal(cx, initRoll);

    provide_context(cx, board);
    provide_context(cx, curr_player);

    view! { cx,
        <table style="border: 5px; border-style: solid; margin-left: 10vw; margin-right: 10vw; margin-top: 10vh; margin-bottom: 10vh">
            <NamesRow />
            <tbody>
                <ScoreRow row_name="Acees" score_fn=Box::new(|p: PlayerScoreBoard| p.aces.clone()) />
                <ScoreRow row_name="Twos" score_fn=Box::new(|p: PlayerScoreBoard| p.twos.clone()) />
                <ScoreRow row_name="Threes" score_fn=Box::new(|p: PlayerScoreBoard| p.threes.clone()) />
                <ScoreRow row_name="Fours" score_fn=Box::new(|p: PlayerScoreBoard| p.fours.clone()) />
                <ScoreRow row_name="Fives" score_fn=Box::new(|p: PlayerScoreBoard| p.fives.clone()) />
                <ScoreRow row_name="Sixes" score_fn=Box::new(|p: PlayerScoreBoard| p.sixes.clone()) />
                <hr />
                <ScoreRow row_name="Upper score" score_fn=Box::new(|p: PlayerScoreBoard| p.upper_section_score().clone()) />
                <hr />
                <ScoreRow row_name="Three of a kind" score_fn=Box::new(|p: PlayerScoreBoard| p.three_of_a_kind.clone()) />
                <ScoreRow row_name="Four of a kind" score_fn=Box::new(|p: PlayerScoreBoard| p.four_of_a_kind.clone()) />
                <ScoreRow row_name="Full house" score_fn=Box::new(|p: PlayerScoreBoard| p.full_house.clone()) />
                <ScoreRow row_name="Small strait" score_fn=Box::new(|p: PlayerScoreBoard| p.small_straight.clone()) />
                <ScoreRow row_name="Large strait" score_fn=Box::new(|p: PlayerScoreBoard| p.large_straight.clone()) />
                <ScoreRow row_name="Yahtzee" score_fn=Box::new(|p: PlayerScoreBoard| p.yahtzee.clone()) />
                <ScoreRow row_name="Chance" score_fn=Box::new(|p: PlayerScoreBoard| p.chance.clone()) />
                <hr />
                <ScoreRow row_name="Lower score" score_fn=Box::new(|p: PlayerScoreBoard| p.lower_section_score().clone()) />
                <hr />
                <ScoreRow row_name="Upper score" score_fn=Box::new(|p: PlayerScoreBoard| p.total_score().clone()) />
                <hr />
            </tbody>

            <div>
                <p>{"Current roll"}</p><strong>{roll.get().to_string()}</strong>
                <p>{dice()[0]}</p>
                <p>{dice.get()[1]}</p>
                <p>{dice.get()[2]}</p>
                <p>{dice.get()[3]}</p>
                <p>{dice.get()[4]}</p>
                <button on:click = move |_| 
                    set_roll.update(|r| *r += 1)
                >{"Reroll"}</button>
            </div>

        </table>
    }
}

#[component]
fn NamesRow(cx: Scope) -> impl IntoView {
    let board = use_context::<ReadSignal<Vec<PlayerScoreBoard>>>(cx).expect("PlayerScoreBoard Vec");
    let curr_player = use_context::<ReadSignal<usize>>(cx).expect("Current player");

    view! { cx,
        <thead>
            <th></th>
            <For
                each=board
                key=|player| player.name.clone()
                view=move |cx, player |  {

                    let mut back_ground = "white";
                    if player.name == board.get()[curr_player.get()].name {
                        back_ground = "green";
                    }

                    let style:String = String::new() + "background: " + back_ground;

                    view! {cx,
                        <th style=style>{player.name.clone()}</th>
                    }
                }
            />
        </thead>
    }
}

#[component]
fn ScoreRow(
    cx: Scope,
    row_name: &'static str,
    score_fn: Box<dyn Fn(PlayerScoreBoard) -> i32>,
) -> impl IntoView {
    let board = use_context::<ReadSignal<Vec<PlayerScoreBoard>>>(cx).expect("PlayerScoreBoard Vec");
    let curr_player = use_context::<ReadSignal<usize>>(cx).expect("Current player");

    view! { cx,
        <tr>
            <td>{row_name}</td>
            <For
                each=board
                key=|player| player.name.clone()
                view=move |cx, player |  {
                    let mut button_disable = true;
                    if player.name == board.get()[curr_player.get()].name {
                        button_disable = false;
                    }

                    let score = score_fn(player);
                    let mut btnState = "/".to_string();
                    if score != -1 {
                        btnState = score.to_string();
                    }
                    


                    view! {cx,
                            <td>
                                <button disabled=button_disable>
                                    {btnState}
                                </button>
                            </td>
                    }
                }
            />
        </tr>
    }
}
