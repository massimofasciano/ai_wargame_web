use wasm_bindgen::prelude::*;

use ai_wargame::{Game, GameOptions, Coord, Dim};

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
#[derive(Debug,Clone,Default)]
pub struct WebGame {
    game: Game
}

#[wasm_bindgen]
impl WebGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut options = GameOptions::default();
        options.debug = true;
        let game = Game::new(options);
        Self { game }
    }
    pub fn info_string(&self) -> String {
        let mut buffer = Vec::new();
        self.game.pretty_print_info(&mut buffer).expect("should work in a vec buffer");
        String::from_utf8_lossy(&buffer).to_string()
    }
    pub fn board_string(&self) -> String {
        let mut buffer = Vec::new();
        self.game.pretty_print_board(&mut buffer).expect("should work in a vec buffer");
        String::from_utf8_lossy(&buffer).to_string()
    }
    pub fn damage_table_string(&self, legend: &str) -> String {
        self.game.html_damage_table_string(Some(legend))
    }
    pub fn repair_table_string(&self, legend: &str) -> String {
        self.game.html_repair_table_string(Some(legend))
    }
    pub fn html_string(&self, css_class: String, fn_click: String) -> String {
        self.game.to_html_board_string(css_class, fn_click)
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
        String::from_utf8_lossy(&buffer).to_string()
    }
    pub fn display_coord(from_row: Dim, from_col: Dim) -> String {
        let from = Coord::from_tuple((from_row,from_col));
        from.to_string()
    }
    pub fn player_play_turn(&mut self, from_row: Dim, from_col: Dim, to_row: Dim, to_col: Dim) -> Option<String> {
        let from = Coord::from_tuple((from_row,from_col));
        let to = Coord::from_tuple((to_row,to_col));
        console_log!("User entered move from {from} to {to}");
        let mut buffer = Vec::new();
        if self.game.human_play_turn_from_coords(Some(&mut buffer),from,to).expect("should work in a vec buffer") {
            Some(String::from_utf8_lossy(&buffer).to_string())
        } else {
            console_log!("Invalid move from {from} to {to}");
            None
        }
    }
}

