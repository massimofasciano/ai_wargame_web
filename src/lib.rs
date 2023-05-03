use std::io::Write;

use wasm_bindgen::prelude::*;

use ai_wargame::{Game, heuristics, GameOptions};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
    #[wasm_bindgen(js_namespace = document, js_name = write)]
    fn document_write(s: &str);
    #[wasm_bindgen(js_name = update_board)]
    fn update_board_js(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (console_log(&format_args!($($t)*).to_string()))
}

fn update_board(s: &str) {
    // let window = web_sys::window().expect("no global `window` exists");
    // let document = window.document().expect("should have a document on window");
    // let element = document.get_element_by_id("board").expect("should find board");
    // element.set_text_content(Some(s));
    update_board_js(s);
}

#[wasm_bindgen]
pub fn web_ai_wargame() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    update_board("Hello");

    // return;

    let mut options = GameOptions::default();
    options.max_depth = Some(6);
    options.max_moves = Some(150);
    // options.max_seconds = Some(5.0);
    {
        use heuristics::*;
        let _h1 = units_health_weights_bias(10,10,100) * 10
                                + ai_distance(2,1)
                                - game_moves();
        options.heuristics.set_attack_heuristics(_h1);
    }
    options.mutual_damage = true;
    options.move_only_forward = true;
    options.adjust_max_depth = true;
    options.debug = true;
       
    let mut game = Game::new(options);

    loop {
        pretty_print(&game);

        if let Some(winner) = game.end_game_result() {
            console_log!("{} wins in {} moves!", winner, game.total_moves());
            break;
        }

        computer_play_turn(&mut game);
    }
}

pub fn computer_play_turn(game: &mut Game) {
    let (score,best_action,elapsed_seconds,avg_depth) = game.suggest_action();
    if let Some(best_action) = best_action {
        if let Ok((player, action, outcome)) = game.play_turn_from_action(best_action) {
        } else {
            panic!("play turn should work");
        }
    } else {
        game.set_deadlock(true);
    }
}

pub fn pretty_print(game: &Game) {
    let mut buffer = Vec::new();
    game.pretty_print_info(&mut buffer).expect("should work in a vec buffer");
    buffer.flush().unwrap();
    let info = String::from_utf8_lossy(&buffer);
    console_log!("{}",info);
    buffer.clear();
    game.pretty_print_board(&mut buffer).expect("should work in a vec buffer");
    buffer.flush().unwrap();
    let board = String::from_utf8_lossy(&buffer);
    console_log!("{}",board);
    update_board(board.as_ref());
}
