use crate::{
    error::{ParserError, RunningResult},
    pb_item::{PbItem, ProtoType},
};
use serde_json::{Map, Number, Value};

fn insert_value<S, T>(obj: &mut Map<String, Value>, key: String, v: T) -> RunningResult<()>
where
    S: std::convert::From<T> + std::convert::Into<Value>,
{
    if let Some(val) = obj.get_mut(&key) {
        if val.is_array() {
            val.as_array_mut().unwrap().push(S::from(v).into())
        } else {
            let _arr = vec![val.clone(), S::from(v).into()];
            obj.insert(key, Value::Array(_arr));
        }
    } else {
        obj.insert(key, S::from(v).into());
    }

    Ok(())
}

fn insert_json_value(obj: &mut Map<String, Value>, item: &PbItem) -> RunningResult<()> {
    let key = item.item_index.to_string();

    match item.item_type.clone() {
        ProtoType::Variant(n) | ProtoType::Fixed64(n) => insert_value::<Number, u64>(obj, key, n),
        ProtoType::String(s) => insert_value::<String, String>(obj, key, s),
        ProtoType::Array(a) => insert_value::<Vec<u8>, Vec<u8>>(obj, key, a),
        ProtoType::Object(o) => {
            insert_value::<Map<String, Value>, Map<String, Value>>(obj, key, build_json(o)?)
        }
        ProtoType::Fixed32(n) => insert_value::<Number, u32>(obj, key, n),
        _ => return Err(ParserError::new("解析错误: 未知类型！").into()),
    }
}

pub fn build_json(items: Vec<PbItem>) -> RunningResult<Map<String, Value>> {
    let mut result = Map::new();
    items.iter().for_each(|item| {
        insert_json_value(&mut result, item).expect("解析错误: 转换为json失败!");
    });

    Ok(result)
}
