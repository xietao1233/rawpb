#[derive(PartialEq, Clone)]
pub enum ProtoType {
    Unknown,
    Variant(u64),    //Varint int32, int64, uint32, uint64, sint32, sint64, bool, enum
    Fixed64(u64),    // 64-bit fixed64, sfixed64, double
    String(String),  // Length-delimited string, bytes, embedded messages, packed repeated fields
    Arrays(Vec<u8>), // ditto
    Object(Vec<PbItem>), // ditto
    Fixed32(u32),    // 32-bit fixed32, sfixed32, float
}

impl ProtoType {
    pub fn format(&self) -> String {
        match self {
            Self::Unknown => "cannot display for Unknown!".to_string(),
            Self::Variant(n) | Self::Fixed64(n) => format!("{}", n),
            Self::Fixed32(n) => format!("{}", n),
            Self::String(s) => s.to_string(),
            Self::Arrays(a) => hex::encode(&a),
            Self::Object(o) => o
                .iter()
                .map(|x| x.format())
                .collect::<Vec<String>>()
                .join("\n"),
        }
    }
}

impl std::fmt::Display for ProtoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl std::fmt::Debug for ProtoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[derive(PartialEq, Clone)]
pub struct PbItem {
    pub item_type: ProtoType,
    pub item_index: u64,
}

impl PbItem {
    pub fn new(index: u64, _type: Option<ProtoType>) -> Self {
        Self {
            item_index: index,
            item_type: _type.unwrap_or(ProtoType::Unknown),
        }
    }

    pub fn format(&self) -> String {
        format!(
            "{}: {{\n\t{}\n}}\n",
            self.item_index,
            self.item_type.format()
        )
    }
}

impl std::fmt::Display for PbItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl std::fmt::Debug for PbItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}
