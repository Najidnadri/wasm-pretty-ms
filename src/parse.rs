use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen]
pub struct ParsedMs {
    pub days: f64, 
    pub hours: f64,
    pub minutes: f64,
    pub seconds: f64,
    pub milliseconds: f64,
    pub microseconds: f64,
    pub nanoseconds: f64,
}

#[wasm_bindgen]
impl ParsedMs {
    #[wasm_bindgen(constructor)]
    pub fn new(milliseconds: f64) -> ParsedMs {

        ParsedMs {
            days: (milliseconds / 86_400_000.0).trunc(),
            hours: ((milliseconds/ 3600000.0) % 24.0).trunc(),
            minutes: ((milliseconds / 60000.0) % 60.0).trunc(),
            seconds: ((milliseconds / 1000.0) % 60.0).trunc(),
            milliseconds: (milliseconds % 1000.0).trunc(),
            microseconds: ((milliseconds * 1000.0) % 1000.0).trunc(),
            nanoseconds: ((milliseconds * 1_000_000.0) % 1000.0).trunc(),
        }
    }
}


