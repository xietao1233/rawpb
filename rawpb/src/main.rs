use clap::Parser;
use rawpb_core::parse_to_pretty;
use std::io::{Read, Write};

type IoResult<T> = Result<T, std::io::Error>;

enum InputFormatType {
    Binary,
    HexString,
    Base64,
}

impl InputFormatType {
    pub fn new(fmt: &str) -> Self {
        if fmt == "h" {
            Self::HexString
        } else if fmt == "B" {
            Self::Base64
        } else {
            Self::Binary
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sets a input file
    #[arg(short, long, default_value_t = String::new())]
    input_file: String,

    /// Sets a output file
    #[arg(short, long, default_value_t = String::new())]
    output_file: String,

    /// 'b' is binary, 'h' is hex string, 'B' is base64 string.
    #[arg(short, long, default_value_t = String::from("b"))]
    format_string: String,

    /// 's' is String, 'o' is Object. "String" and "Object", which is first
    #[arg(short, long, default_value_t = String::from("s"))]
    who_is_first: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let input_file = args.input_file;
    let output_file = args.output_file;
    let input_fmt = args.format_string;
    let wif = args.who_is_first;

    match (
        std::fs::File::open(std::path::Path::new(&input_file)),
        std::fs::File::create(std::path::Path::new(&output_file)),
    ) {
        (Ok(ref mut f), Ok(ref mut of)) => {
            parse_data(f, of, InputFormatType::new(input_fmt.as_ref()), wif == "o")?;
        }
        (Ok(ref mut f), Err(_)) => {
            // println!("output file error: {:?}, 已重定向到stdout.", err);
            let mut of = std::io::stdout();
            parse_data(
                f,
                &mut of,
                InputFormatType::new(input_fmt.as_ref()),
                wif == "o",
            )?;
        }
        (Err(_), Ok(ref mut of)) => {
            // println!("input file error: {:?}, 已重定向到stdin.", err);
            let mut f = std::io::stdin();
            parse_data(
                &mut f,
                of,
                InputFormatType::new(input_fmt.as_ref()),
                wif == "o",
            )?;
        }
        _ => {
            // println!("no input file.");
            let mut f = std::io::stdin();
            let mut of = std::io::stdout();
            parse_data(
                &mut f,
                &mut of,
                InputFormatType::new(input_fmt.as_ref()),
                wif == "o",
            )?;
        }
    }

    Ok(())
}

fn parse_data(
    f: &mut impl Read,
    of: &mut impl Write,
    fmt: InputFormatType,
    sif: bool,
) -> IoResult<()> {
    let mut data = Vec::new();
    f.read_to_end(&mut data)?;
    let _data = match fmt {
        InputFormatType::HexString => {
            // 去除末尾的回车键字符
            data.pop();
            hex::decode(data).expect("输入的hex字符串格式错误!")
        }
        InputFormatType::Base64 => {
            use base64::prelude::*;

            // 去除末尾的回车键字符
            data.pop();
            BASE64_STANDARD.decode(data).expect("输入的base64格式错误!")
        }
        _ => data,
    };
    match parse_to_pretty(_data.as_ref(), sif) {
        Ok(d) => {
            of.write_all(d.as_bytes())?;
        }
        Err(err) => {
            panic!("解析pb错误: {:?}", err)
        }
    }

    Ok(())
}
