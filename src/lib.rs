use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn run() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn solve(json_sudoku: JsValue) -> JsValue {
    let sudoku: Sudoku = json_sudoku.into_serde().expect("Deserialization failed");
    let solution = partial_solution(sudoku, 0);
    if let Some(s) = solution {
        JsValue::from_serde(&s).expect("Serialization failed")
    } else {
        json_sudoku
    }
}

#[derive(Serialize, Deserialize)]
struct Sudoku {
    sudoku: [[u8; 9]; 9],
}

impl Sudoku {
    fn is_set(&self, x: usize, y: usize) -> bool {
        self.sudoku[y][x] > 0
    }

    fn try_value(&self, x: usize, y: usize, val: u8) -> Sudoku {
        let mut sudoku = Sudoku {
            sudoku: self.sudoku.clone(),
        };
        sudoku.sudoku[y][x] = val;
        sudoku
    }

    fn is_valid(&self, x: usize, y: usize) -> bool {
        let lower_x = (x / 3) * 3;
        let lower_y = (y / 3) * 3;

        let value = self.sudoku[y][x];

        for i in lower_x..(lower_x + 3) {
            for j in lower_y..(lower_y + 3) {
                if i != x && j != y && self.sudoku[j][i] == value {
                    return false;
                }
            }
        }

        for i in 0..9 {
            if i != x && self.sudoku[y][i] == value {
                return false;
            }
        }

        for j in 0..9 {
            if j != y && self.sudoku[j][x] == value {
                return false;
            }
        }

        true
    }

    fn from_str(input: &str) -> Sudoku {
        let mut sudoku = [[0; 9]; 9];
        let lines = input.split('\n');
        for (x, line) in lines.into_iter().enumerate() {
            let numbers = line.trim().chars();
            for (y, number) in numbers.enumerate() {
                sudoku[x][y] = number
                    .to_string()
                    .parse()
                    .expect("No valid sudoku provided")
            }
        }
        Sudoku { sudoku }
    }
}

impl ToString for Sudoku {
    fn to_string(&self) -> String {
        let mut out = String::new();
        for y in 0..9 {
            out.push('|');
            for x in 0..9 {
                out.push_str(self.sudoku[y][x].to_string().as_str());
                out.push('|');
            }
            out.push('\n');
        }
        out
    }
}

fn partial_solution(sudoku: Sudoku, index: usize) -> Option<Sudoku> {
    let x = index % 9;
    let y = index / 9;
    let mut solution = None;

    if index == 9 * 9 {
        solution = Some(sudoku)
    } else if sudoku.is_set(x, y) {
        solution = partial_solution(sudoku, index + 1);
    } else {
        for value in 1..10 {
            let trial = sudoku.try_value(x, y, value);
            // clear_screen();
            // println!("{}", trial.to_string());
            if trial.is_valid(x, y) {
                if let Some(result) = partial_solution(trial, index + 1) {
                    solution = Some(result);
                    break;
                }
            }
        }
    }
    solution
}
