use std::io::Write;

use wasm_bindgen::prelude::*;

use ai_wargame::{Game, heuristics, GameOptions, IsUsefulInfo, Coord};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! wprintln {
    ($($t:tt)*) => (doc_string(format!("{}\n",format_args!($($t)*).to_string())).unwrap())
}
macro_rules! wprint {
    ($($t:tt)*) => (doc_string(format_args!($($t)*).to_string()).unwrap())
}

fn doc_string(s: String) -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    body.set_inner_text(s.as_str());

    // Manufacture the element we're gonna append
    // let val = document.create_element("span")?;
    // val.set_text_content(Some(s.as_str()));

    // body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn web_ai_wargame() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

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
       
    let mut game = Game::new(options);

    loop {
        wprintln!("");
        pretty_print(&game);
        wprintln!("");

        if let Some(winner) = game.end_game_result() {
            log(&format!("{} wins in {} moves!", winner, game.total_moves()));
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
    log(&String::from_utf8_lossy(&buffer));
    buffer.clear();
    game.pretty_print_board(&mut buffer).expect("should work in a vec buffer");
    buffer.flush().unwrap();
    wprintln!("{}",String::from_utf8_lossy(&buffer));
}
