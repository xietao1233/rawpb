use error::RunningResult;

pub(crate) mod converter;
pub(crate) mod error;
pub(crate) mod parser;
pub(crate) mod pb_item;
pub(crate) mod utils;

pub fn parse_to_pretty(data: &[u8]) -> RunningResult<String> {
    let pb_items = self::parser::parse_pb_data(data)?;
    let obj = self::converter::build_json(pb_items)?;
    let mut result = serde_json::to_string_pretty(&obj).unwrap_or(String::new());
    result.push('\n');

    Ok(result)
}
