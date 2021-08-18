use clap::{App, Arg, SubCommand};
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

fn main() -> IoResult<()> {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets a output file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Sets a input file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("format_string")
                .short("f")
                .long("format")
                .value_name("FORMAT_STRING")
                .help("\"b\" is binary, \"h\" is hex string, \'B\" is base64 string.")
                .takes_value(true),
        )
        .get_matches();

    let input_file = matches.value_of("input_file").unwrap_or("");
    let output_file = matches.value_of("output_file").unwrap_or("");
    let input_fmt = matches.value_of("format_string").unwrap_or("b");

    match (
        std::fs::File::open(std::path::Path::new(input_file)),
        std::fs::File::create(std::path::Path::new(output_file)),
    ) {
        (Ok(ref mut f), Ok(ref mut of)) => {
            parse_data(f, of, InputFormatType::new(input_fmt))?;
        }
        (Ok(ref mut f), Err(err)) => {
            println!("output file error: {:?}, 已重定向到stdout.", err);
            let mut of = std::io::stdout();
            parse_data(f, &mut of, InputFormatType::new(input_fmt))?;
        }
        (Err(err), Ok(ref mut of)) => {
            println!("input file error: {:?}, 已重定向到stdin.", err);
            let mut f = std::io::stdin();
            parse_data(&mut f, of, InputFormatType::new(input_fmt))?;
        }
        _ => {
            println!("no input file.");
            let mut f = std::io::stdin();
            let mut of = std::io::stdout();
            parse_data(&mut f, &mut of, InputFormatType::new(input_fmt))?;
        }
    }

    Ok(())
}

fn parse_data(f: &mut impl Read, of: &mut impl Write, fmt: InputFormatType) -> IoResult<()> {
    let mut data = Vec::new();
    f.read_to_end(&mut data)?;
    let _data = match fmt {
        InputFormatType::HexString => hex::decode(data).expect("输入的hex字符串格式错误!"),
        InputFormatType::Base64 => base64::decode(data).expect("输入的base64格式错误!"),
        _ => data,
    };
    match parse_to_pretty(_data.as_ref()) {
        Ok(d) => {
            of.write_all(d.as_bytes())?;
        }
        Err(err) => {
            panic!("解析pb错误: {:?}", err)
        }
    }

    Ok(())
}