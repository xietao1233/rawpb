use error::RunningResult;

pub mod converter;
pub(crate) mod error;
pub mod parser;
pub mod pb_item;
pub mod utils;

pub fn parse_to_pretty(data: &[u8], sif: bool) -> RunningResult<String> {
    let pb_items = self::parser::parse_pb_data(data, sif)?;
    let obj = self::converter::build_json(pb_items)?;
    let mut result = serde_json::to_string_pretty(&obj).unwrap_or(String::new());
    result.push('\n');

    Ok(result)
}
