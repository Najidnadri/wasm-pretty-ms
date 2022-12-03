use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct PrettyMsOption {
    pub colon_notation: bool,
    pub compact: bool,
    pub format_sub_milliseconds: bool,
    pub separate_milliseconds: bool,
    pub verbose: bool,
    pub seconds_decimal_digits: Option<u32>,
    pub milliseconds_decimal_digits: Option<u32>,
}

impl Default for PrettyMsOption {
    fn default() -> Self {
        PrettyMsOption {
            colon_notation: false,
            compact: false,
            format_sub_milliseconds: false,
            separate_milliseconds: false,
            verbose: false,
            seconds_decimal_digits: None,
            milliseconds_decimal_digits: None
        }
    }
}