use clap::{App, Arg};
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        .arg(
            Arg::with_name("who_is_first")
                .short("w")
                .long("first")
                .value_name("WHO_IS_FIRST")
                .help("\"s\" is String, \"o\" is Object. \"String\" and \"Object\", which is first")
                .takes_value(true),
        )
        .get_matches();

    let input_file = matches.value_of("input_file").unwrap_or("");
    let output_file = matches.value_of("output_file").unwrap_or("");
    let input_fmt = matches.value_of("format_string").unwrap_or("b");
    let wif = matches.value_of("who_is_first").unwrap_or("s");

    match (
        std::fs::File::open(std::path::Path::new(input_file)),
        std::fs::File::create(std::path::Path::new(output_file)),
    ) {
        (Ok(ref mut f), Ok(ref mut of)) => {
            parse_data(f, of, InputFormatType::new(input_fmt), wif == "o")?;
        }
        (Ok(ref mut f), Err(_)) => {
            // println!("output file error: {:?}, ???????????????stdout.", err);
            let mut of = std::io::stdout();
            parse_data(f, &mut of, InputFormatType::new(input_fmt), wif == "o")?;
        }
        (Err(_), Ok(ref mut of)) => {
            // println!("input file error: {:?}, ???????????????stdin.", err);
            let mut f = std::io::stdin();
            parse_data(&mut f, of, InputFormatType::new(input_fmt), wif == "o")?;
        }
        _ => {
            // println!("no input file.");
            let mut f = std::io::stdin();
            let mut of = std::io::stdout();
            parse_data(&mut f, &mut of, InputFormatType::new(input_fmt), wif == "o")?;
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
            // ??????????????????????????????
            data.pop();
            hex::decode(data).expect("?????????hex?????????????????????!")
        }
        InputFormatType::Base64 => {
            // ??????????????????????????????
            data.pop();
            base64::decode(data).expect("?????????base64????????????!")
        }
        _ => data,
    };
    match parse_to_pretty(_data.as_ref(), sif) {
        Ok(d) => {
            of.write_all(d.as_bytes())?;
        }
        Err(err) => {
            panic!("??????pb??????: {:?}", err)
        }
    }

    Ok(())
}
