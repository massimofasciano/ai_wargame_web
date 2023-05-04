use wasm_bindgen::prelude::*;

use ai_wargame::{Game, heuristics, GameOptions};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (console_log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    Ok(())
}

#[wasm_bindgen]
pub struct WebGame {
    game: Game
}

#[wasm_bindgen]
impl WebGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut options = GameOptions::default();
        options.max_depth = Some(6);
        options.max_moves = Some(150);
        options.max_seconds = Some(5.0);
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
           
        let game = Game::new(options);

        Self { game }
    }
    pub fn info_string(&self) -> String {
        let mut buffer = Vec::new();
        self.game.pretty_print_info(&mut buffer).expect("should work in a vec buffer");
        // buffer.flush().unwrap();
        String::from_utf8_lossy(&buffer).to_string()

    }
    pub fn board_string(&self) -> String {
        let mut buffer = Vec::new();
        self.game.pretty_print_board(&mut buffer).expect("should work in a vec buffer");
        // buffer.flush().unwrap();
        String::from_utf8_lossy(&buffer).to_string()
    }
    pub fn has_winner(&self) -> Option<String> {
        if let Some(winner) = self.game.end_game_result() {
            Some(format!("{} wins in {} moves!", winner, self.game.total_moves()))
        } else {
            None
        }
    }
    pub fn computer_play_turn(&mut self) -> String {
        let mut buffer = Vec::new();
        self.game.computer_play_turn(Some(&mut buffer)).expect("should work in a vec buffer");
        // buffer.flush().unwrap();
        String::from_utf8_lossy(&buffer).to_string()
    }
}

