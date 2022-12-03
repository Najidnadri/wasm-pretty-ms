use option::PrettyMsOption;
use parse::ParsedMs;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

mod parse;
mod option;

const SECOND_ROUNDING_EPSILON: f64 = 0.000_000_1;

#[wasm_bindgen(js_name = prettyMilliseconds)]
pub fn pretty_milliseconds(milliseconds: JsValue, option: JsValue) -> JsValue {
    let mut result: Vec<String> = Vec::new();
    let mut result_string = String::new();
    let mut option: PrettyMsOption = serde_wasm_bindgen::from_value(option).unwrap_or(PrettyMsOption::default());
    let milliseconds = milliseconds.as_f64().unwrap(); 

    if option.colon_notation {
        option.compact = false;
        option.format_sub_milliseconds = false;
        option.separate_milliseconds = false;
        option.verbose = false;
    }

    if option.compact {
        option.seconds_decimal_digits = Some(0);
        option.milliseconds_decimal_digits = Some(0);
    }

    let parsed = ParsedMs::new(milliseconds);
    add(&mut result, (parsed.days / 360.0).trunc(), "year", "y", &option);
    add(&mut result, parsed.days % 360.0, "day", "d", &option);
    add(&mut result, parsed.hours, "hour", "h", &option);
    add(&mut result, parsed.minutes, "minute", "m", &option);
    
    
    if option.separate_milliseconds || option.format_sub_milliseconds || (!option.colon_notation && milliseconds < 1000.0) {
        add(&mut result, parsed.seconds, "second", "s", &option); 
        match option.format_sub_milliseconds {
            true => {
                add(&mut result, parsed.milliseconds, "millisecond", "ms", &option);
                add(&mut result, parsed.microseconds, "microsecond", "μs", &option);
                add(&mut result, parsed.nanoseconds, "nanosecond", "ns", &option);
            },
            false => {
                let milliseconds_and_below = parsed.milliseconds + (parsed.microseconds / 1000.0) + (parsed.nanoseconds / 1_000_000.0);
                let rounded_milliseconds = match milliseconds_and_below >= 1.0 {
                    true => milliseconds_and_below.round(),
                    false => milliseconds_and_below.ceil(),
                };
                add(&mut result, rounded_milliseconds, "millisecond", "ms", &option);
            }
        }
    } else {
        let seconds = (milliseconds / 1000.0) % 60.0;
        let seconds_decimal_digits = match option.seconds_decimal_digits {
            Some(n) => n,
            None => 1
        };
        let second_str = floor_decimals(seconds, seconds_decimal_digits as f64);
        add(&mut result, second_str, "second", "s", &option);
    }

    if result.len() == 0 {
        result_string.push('0');
        result_string.push_str(if option.verbose {"milliseconds"} else {"ms"});
        return serde_wasm_bindgen::to_value(&result_string).unwrap();
    }

    if option.compact {
        return serde_wasm_bindgen::to_value(&result[0]).unwrap()
    }

    if option.colon_notation {
        serde_wasm_bindgen::to_value(&result.join("")).unwrap()
    }  else {
        serde_wasm_bindgen::to_value(&result.join(" ")).unwrap()
    }
}



fn pluralize(word: &mut String, count: f64) {
    if count != 1.0 {
        word.push('s')
    }
}



fn floor_decimals(num: f64, decimal_digit: f64) -> f64 {
    let multiplier: f64 = (10.0 as f64).powf(decimal_digit);

    let floored_interim_val = ((num * multiplier) + SECOND_ROUNDING_EPSILON).floor();

    let floored_val = floored_interim_val.round() / multiplier;
    floored_val
}


fn add(result: &mut Vec<String>, value: f64, long: &str, short: &str, options: &PrettyMsOption) {
    if (result.len() == 0 || !options.colon_notation ) && value == 0.0 && !(options.colon_notation && short == "m") {
        return;
    }
    let mut long = long.to_string();
    let mut value_str = value.to_string();
    let mut _prefix = "";
    let mut _suffix = String::new();

    if options.colon_notation {
        if result.len() > 0 {_prefix = ":"};
        let whole_digit = value_str.find('.').unwrap_or(value_str.len());
        let min_length = if result.len() > 0 {2} else {1};
        if whole_digit < min_length {
            match min_length - whole_digit {
                1 => value_str.insert(0, '0'),
                2 => value_str.insert_str(0, "00"),
                _ => ()
            }
        }
    } else {
        if options.verbose {
            pluralize(&mut long, value);
            _suffix.push_str(" ");
            _suffix.push_str(&long);
        } else {
            _suffix.push_str(short);
        }
    }

    value_str.insert_str(0, _prefix);
    value_str.push_str(&_suffix);
    result.push(value_str);
}




