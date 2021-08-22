use rawpb_core::{converter::build_json, parser};

#[test]
fn test_parse_object_in_object1() {
    let data = hex::decode("0A5C0A013810A6B7ABDC8A8080041A093132372E302E302E31222444344138303743452D314530322D344635352D413543382D32373233374635304545353028838008320B332E312E372E36363239353A0AE68891E79A8469506164420012080A067765776F726B1893CB9A8706").unwrap();
    let json_data = parser::parse_pb_data(data.as_ref(), true).expect("parse error...");
    let value = build_json(json_data).expect("转换json失败!");
    let result = serde_json::to_string(&value).expect("json格式化失败!");
    let correct_result = "{\"1\":{\"1\":\"8\",\"2\":2251802691689382,\"3\":\"127.0.0.1\",\"4\":\"D4A807CE-1E02-4F55-A5C8-27237F50EE50\",\"5\":131075,\"6\":\"3.1.7.66295\",\"7\":\"我的iPad\",\"8\":\"\"},\"2\":\"\\n\\u0006wework\",\"3\":1625728403}";
    assert_eq!(result, correct_result);
}

#[test]
fn test_parse_object_in_object2() {
    let data = hex::decode("0A107145FD538F31D840CCAC689ADA493E5C12A0019A856F03738C091900BECE72ED2EF41C50C9B56AA401C94FA6C0A3BEC9C3692A9B39340BCCABC8762BE27D0FAE61F84FA047E11D68C5248E8A9ADC2B90DFF4173BB39905A5A7F9AE4316646D78EEA1E960FFF6BC6E2F3F8DF7CB0062FA61BCB37BF7A38C2F1176F53F2D255043ACF518CDC55BBE877878420464F9ABB8B6D93053AD6DE498BCFAE7A0B6169508A32382A643F416D77113F7F4FF53B4D6A6F6871A760A6A0A0408011000122033DB1CBC4BD258C8A43C5A68BD26237063A3D6B3205FA20EA5D6147562252C441A401A2444096E53EEFF0BC53D99D1B7341AF94BEA8FB3CF8426E261184BA1E3209455F7CAF691F67D6C73B17D2EA5C85056B1A8B50820C34FD7A1EEE4A606282F2D22080A060800100018002A00").unwrap();
    let json_data = parser::parse_pb_data(data.as_ref(), true).expect("parse error...");
    let value = build_json(json_data).expect("转换json失败!");
    let result = serde_json::to_string(&value).expect("json格式化失败!");
    let correct_result = "{\"1\":{\"1\":\"8\",\"2\":2251802691689382,\"3\":\"127.0.0.1\",\"4\":\"D4A807CE-1E02-4F55-A5C8-27237F50EE50\",\"5\":131075,\"6\":\"3.1.7.66295\",\"7\":\"我的iPad\",\"8\":\"\"},\"2\":\"\\n\\u0006wework\",\"3\":1625728403}";
    assert_eq!(result, correct_result);
}
