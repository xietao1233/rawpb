use rawpb_core::parser;
use rawpb_core::pb_item::{PbItem, ProtoType};

#[test]
fn test_parse_string_and_variant_pb() {
    let data = hex::decode("0A084A6F686E20446F65107B1A106A6F686E406578616D706C652E636F6D").unwrap();
    let result = parser::parse_pb_data(data.as_ref(), true).expect("parser error...");
    assert_eq!(
        result,
        vec![
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
            }
        ]
    );
}

#[test]
fn test_parse_fixed32_and_fixed64_pb() {
    let data = hex::decode("0A084A6F686E20446F652D7B000000E103D202964900000000").unwrap();
    let result = parser::parse_pb_data(data.as_ref(), true).expect("parse error...");
    assert_eq!(
        result,
        vec![
            PbItem {
                item_type: ProtoType::String("John Doe".to_string()),
                item_index: 1,
            },
            PbItem {
                item_type: ProtoType::Fixed32(123),
                item_index: 5,
            },
            PbItem {
                item_type: ProtoType::Fixed64(1234567890),
                item_index: 60,
            }
        ],
    );
}

#[test]
fn test_parse_repeated_pb() {
    let data = hex::decode("0A084A6F686E20446F652D7B0000005203546F6D52054A65727279").unwrap();
    let result = parser::parse_pb_data(data.as_ref(), true).expect("parse error...");
    assert_eq!(
        result,
        vec![
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
            }
        ]
    );
}
