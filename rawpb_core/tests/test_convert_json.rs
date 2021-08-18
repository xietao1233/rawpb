use rawpb_core::{
    converter::build_json,
    pb_item::{PbItem, ProtoType},
};

#[test]
fn test_convert_pb() {
    let data = vec![
        PbItem {
            item_type: ProtoType::String("John Doe".to_string()),
            item_index: 1,
        },
        PbItem {
            item_type: ProtoType::Variant(123),
            item_index: 2,
        },
        PbItem {
            item_type: ProtoType::String("john@example.com".to_string()),
            item_index: 3,
        },
    ];
    let value = build_json(data).expect("转换json失败!");
    let result = serde_json::to_string(&value).expect("json格式化失败!");
    // println!("result: {:?}", text);
    let correct_result = "{\"1\":\"John Doe\",\"2\":123,\"3\":\"john@example.com\"}";
    assert_eq!(result, correct_result);
}

#[test]
fn test_convert_pb2() {
    let data = vec![
        PbItem {
            item_type: ProtoType::String("John Doe".to_string()),
            item_index: 1,
        },
        PbItem {
            item_type: ProtoType::Fixed32(123),
            item_index: 5,
        },
        PbItem {
            item_type: ProtoType::String("Tom".to_string()),
            item_index: 10,
        },
        PbItem {
            item_type: ProtoType::String("Jerry".to_string()),
            item_index: 10,
        },
    ];
    let value = build_json(data).expect("转换json失败!");
    let result = serde_json::to_string(&value).expect("json格式化失败!");
    let correct_result = "{\"1\":\"John Doe\",\"5\":123,\"10\":[\"Tom\",\"Jerry\"]}";
    assert_eq!(result, correct_result);
}
