use wasm_bindgen::prelude::*;

use ai_wargame::{Game, GameOptions, Coord, Dim, CoordPair, Action};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (console_log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    Ok(())
}

#[wasm_bindgen(js_name = Coord)]
#[derive(Debug,Clone,Copy)]
pub struct JsCoord {
   pub row: Dim,
   pub col: Dim,
}

impl From<Coord> for JsCoord {
    fn from(value: Coord) -> Self {
        Self { row: value.row, col: value.col }
    }
}

#[wasm_bindgen(js_name = CoordPair)]
#[derive(Debug,Clone,Copy)]
pub struct JsCoordPair {
    pub from: JsCoord,
    pub to: JsCoord,
}

impl From<CoordPair> for JsCoordPair {
    fn from(value: CoordPair) -> Self {
        Self { from: value.from.into(), to: value.to.into() }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug,Clone)]
pub struct MoveResult {
    pub coords: Option<JsCoordPair>,
    pub string: String,
}

#[wasm_bindgen(js_name = Game)]
#[derive(Debug,Clone,Default)]
pub struct JsGame {
    game: Game
}

#[wasm_bindgen(js_class = Game)]
impl JsGame {
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
    pub fn text_board_string(&self) -> String {
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
    pub fn html_board_string(&self, css_class: String, id: String, fn_click: String) -> String {
        self.game.to_html_board_string(css_class, id, fn_click)
    }
    pub fn display_coord(from_row: Dim, from_col: Dim) -> String {
        let from = Coord::from_tuple((from_row,from_col));
        from.to_string()
    }
    pub fn has_winner(&self) -> Option<String> {
        if let Some(winner) = self.game.end_game_result() {
            Some(format!("{} wins in {} moves!", winner, self.game.total_moves()))
        } else {
            None
        }
    }
    pub fn computer_play_turn(&mut self) -> MoveResult {
        let mut buffer = Vec::new();
        let coords = self.game.computer_play_turn(Some(&mut buffer)).expect("should work in a vec buffer")
            .and_then(Action::into_coord_pair).map(Into::into);
        MoveResult { coords,
            string: String::from_utf8_lossy(&buffer).to_string()
        }
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
    pub fn auto_adjust_max_depth(&mut self, auto: bool) {
        let mut options = self.game.clone_options();
        options.adjust_max_depth = auto;
        self.game.set_options(options);
    }
    pub fn set_max_depth(&mut self, max_depth : usize) {
        let mut options = self.game.clone_options();
        options.max_depth=Some(max_depth);
        self.game.set_options(options);
        console_log!("Max depth set to {max_depth}");
    }
    pub fn set_max_moves(&mut self, max_moves : usize) {
        let mut options = self.game.clone_options();
        options.max_moves=Some(max_moves);
        self.game.set_options(options);
        console_log!("Max moves set to {max_moves}");
    }
    pub fn set_max_seconds(&mut self, max_seconds: f32) {
        let mut options = self.game.clone_options();
        options.max_seconds=Some(max_seconds);
        self.game.set_options(options);
        console_log!("Max seconds set to {max_seconds}");
    }
    pub fn set_rand_traversal(&mut self, rand_traversal : bool) {
        let mut options = self.game.clone_options();
        options.rand_traversal=rand_traversal;
        self.game.set_options(options);
        console_log!("Random traversal set to {rand_traversal}");
    }
    pub fn set_alpha_beta(&mut self, alpha_beta : bool) {
        let mut options = self.game.clone_options();
        options.pruning=alpha_beta;
        self.game.set_options(options);
        console_log!("Alpha-Beta pruning set to {alpha_beta}");
    }
    pub fn set_heuristics_e1(&mut self) {
        let mut options = self.game.clone_options();
        options.heuristics.set_e1();
        self.game.set_options(options);
        console_log!("Activated e1 heuristic.");
    }
    pub fn set_heuristics_e2(&mut self) {
        let mut options = self.game.clone_options();
        options.heuristics.set_e2();
        self.game.set_options(options);
        console_log!("Activated e2 heuristic.");
    }
    pub fn set_heuristics_e3e4(&mut self) {
        let mut options = self.game.clone_options();
        options.heuristics.set_e3e4();
        self.game.set_options(options);
        console_log!("Activated e3/e4 heuristics.");

    }
    pub fn moves_played(&self) -> usize {
        self.game.total_moves()
    }
}
