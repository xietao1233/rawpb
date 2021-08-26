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
    let json_data = parser::parse_pb_data(data.as_ref(), false).expect("parse error...");
    let value = build_json(json_data).expect("转换json失败!");
    let result = serde_json::to_string(&value).expect("json格式化失败!");
    let correct_result = "{\"1\":\"7145fd538f31d840ccac689ada493e5c\",\"2\":\"9a856f03738c091900bece72ed2ef41c50c9b56aa401c94fa6c0a3bec9c3692a9b39340bccabc8762be27d0fae61f84fa047e11d68c5248e8a9adc2b90dff4173bb39905a5a7f9ae4316646d78eea1e960fff6bc6e2f3f8df7cb0062fa61bcb37bf7a38c2f1176f53f2d255043acf518cdc55bbe877878420464f9abb8b6d93053ad6de498bcfae7a0b6169508a32382a643f416d77113f7f4ff53b4d6a6f687\",\"3\":{\"1\":{\"1\":{\"1\":1,\"2\":0},\"2\":\"33db1cbc4bd258c8a43c5a68bd26237063a3d6b3205fa20ea5d6147562252c44\",\"3\":\"1a2444096e53eeff0bc53d99d1b7341af94bea8fb3cf8426e261184ba1e3209455f7caf691f67d6c73b17d2ea5c85056b1a8b50820c34fd7a1eee4a606282f2d\"},\"4\":{\"1\":{\"1\":0,\"2\":0,\"3\":0}}},\"5\":\"\"}";
    assert_eq!(result, correct_result);
}

#[test]
fn test_parse_object_in_object3() {
    let data = hex::decode("08c3cccef58080800312b0016a47d3b08d9298256f0d009c601222264def233e484ae8e9babfd8e63f83c0f8b77f319918f49f1f72a654be5540cbda8e503e023f87b3450bc72ba19c4da5ee833e5a4655dbe6a8dfd09e290e53bb80fec3d4af39b27cbee34337715abe5fd876b0d8bb10e442db00a36ef2088d32afcc7644655b9cb899904405c03de5790bfda8f5d34609cd4deb4aa42c58213f0aa8c6e68f5a0e5a3b7d4d2ee5500a7e00dc751cbdab1a988222b6ccc5b8b215881ab00129fae3b9711aa392f465e154d0aeed3438f2ad1d586762aa88ba986688cb404f0e3ec91dc1fddf076ef7167a8a7de0c5ff011fdc1491075bd0c6b106320918b8f3352ca3417ec905884fd1984dd04d3b193fb6dc819d38f8cb89f37f70b380ac75463556c10ecde73b228991602f265dd0220a971eb3932f30fa133f48cbe0e1ddca3cb16446d21277d6f3946e742d1b2a15c2ec71508de1be8f6274f1840029c92ac0416293eea64a35cf7412c515c930014a2444344138303743452d314530322d344635352d413543382d3237323337463530454535305000722434343033313942392d433330392d343937352d384642312d333138303735344137423344900100a00101b00100b80100c20100c80100").unwrap();
    let json_data = parser::parse_pb_data(data.as_ref(), false).expect("parse error...");
    let value = build_json(json_data).expect("转换json失败!");
    let result = serde_json::to_string(&value).expect("json格式化失败!");
    let correct_result = "{\"1\":1688850106918467,\"2\":\"6a47d3b08d9298256f0d009c601222264def233e484ae8e9babfd8e63f83c0f8b77f319918f49f1f72a654be5540cbda8e503e023f87b3450bc72ba19c4da5ee833e5a4655dbe6a8dfd09e290e53bb80fec3d4af39b27cbee34337715abe5fd876b0d8bb10e442db00a36ef2088d32afcc7644655b9cb899904405c03de5790bfda8f5d34609cd4deb4aa42c58213f0aa8c6e68f5a0e5a3b7d4d2ee5500a7e00dc751cbdab1a988222b6ccc5b8b21588\",\"3\":\"29fae3b9711aa392f465e154d0aeed3438f2ad1d586762aa88ba986688cb404f0e3ec91dc1fddf076ef7167a8a7de0c5ff011fdc1491075bd0c6b106320918b8f3352ca3417ec905884fd1984dd04d3b193fb6dc819d38f8cb89f37f70b380ac75463556c10ecde73b228991602f265dd0220a971eb3932f30fa133f48cbe0e1ddca3cb16446d21277d6f3946e742d1b2a15c2ec71508de1be8f6274f1840029c92ac0416293eea64a35cf7412c515c9\",\"6\":1,\"9\":\"D4A807CE-1E02-4F55-A5C8-27237F50EE50\",\"10\":0,\"14\":\"440319B9-C309-4975-8FB1-3180754A7B3D\",\"18\":0,\"20\":1,\"22\":0,\"23\":0,\"24\":\"\",\"25\":0}";
    assert_eq!(result, correct_result);
}
