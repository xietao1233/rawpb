use super::error::{ParserError, RunningResult};
use super::pb_item::{PbItem, ProtoType};
use std::io::{Cursor, Read};

/// read a variant
///
/// # Examples
///
/// ```
/// use rawpb_core::parser::read_to_positive;
///
/// let data = hex::decode("FFFFFFFFFFFFFFFFFF01").unwrap();
/// let result = read_to_positive(&mut std::io::Cursor::new(&data)).unwrap();
/// assert_eq!(result, u64::max_value());
/// ```
pub fn read_to_positive(cur: &mut Cursor<&[u8]>) -> RunningResult<u64> {
    let val = cur
        .get_ref()
        .iter()
        .skip(cur.position() as usize)
        .take_while(|p| ((**p) as i8) < 0)
        .collect::<Vec<&_>>();
    if val.len() <= 9 {
        let mut buf = vec![0; val.len() + 1];
        cur.read(&mut buf)?;
        Ok(buf
            .into_iter()
            .rev()
            .fold(0_u64, |a, b| (a << 7) + (b & 0x7f) as u64))
    } else {
        Err(ParserError::new("解析错误：无效的Variant类型数字！").into())
    }
}

/// 解析pb数据为json
///
pub fn parse_pb_data(data: &[u8]) -> RunningResult<Vec<PbItem>> {
    let mut result = Vec::new();
    let mut buf = Cursor::new(data);

    loop {
        let val = read_to_positive(&mut buf)?;
        let (_index, _type) = (val >> 3, val & 0x7);
        let mut pb_item = PbItem::new(_index, None);
        result.push(match _type {
            0 => {
                pb_item.item_type = ProtoType::Variant(read_to_positive(&mut buf)?);
                pb_item
            }
            1 => {
                let mut _buf = [0; 8];
                buf.read(&mut _buf)?;
                pb_item.item_type = ProtoType::Fixed64(u64::from_le_bytes(_buf));
                pb_item
            }
            2 => {
                let buf_len = read_to_positive(&mut buf)?;
                let mut _buf = vec![0; buf_len as usize];
                buf.read(&mut _buf)?;
                if let Ok(ret) = String::from_utf8(_buf.clone()) {
                    pb_item.item_type = ProtoType::String(ret);
                } else if let Ok(obj) = parse_pb_data(_buf.as_ref()) {
                    pb_item.item_type = ProtoType::Object(obj);
                } else {
                    pb_item.item_type = ProtoType::Array(_buf);
                }
                pb_item
            }
            5 => {
                let mut _buf = [0; 4];
                buf.read(&mut _buf)?;
                pb_item.item_type = ProtoType::Fixed32(u32::from_le_bytes(_buf));
                pb_item
            }
            _ => {
                return Err(ParserError::new(format!("解析错误: 未知的类型: {:?}", val & 7)).into())
            }
        });
        if buf.position() == data.len() as u64 {
            break;
        }
    }
    Ok(result)
}