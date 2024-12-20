use logos::{Lexer, Skip};

use super::{ MyGOError, MyGoToken, ParseData, ParsePos, PlatformInt};

pub fn parse_rune(lex: &mut Lexer<MyGoToken>) -> Result<ParseData<char>, MyGOError> {
    let char = &lex.slice()[1..lex.slice().len() - 1];
    let data = match char {
        r"\n" => Ok('\n'),
        r"\r" => Ok('\r'),
        r"\t" => Ok('\t'),
        r"\v" => Ok('\x0B'),
        r"\f" => Ok('\x0C'),
        r"\a" => Ok('\x07'),
        r"\\\\" => Ok('\\'),
        r"\'" => Ok('\''),
        r#"\""# => Ok('\"'),
        r"\b" => Ok('\x08'),
        c if char.chars().count() == 1 => Ok(c.parse().unwrap()),
        _ => Err(MyGOError::InvalidRune), // shouldn't happen, because if not match above, it won't
                                          // get into here.
    }?;
    Ok(ParseData {
        data,
        loc: get_parse_loc(lex),
    })
}

pub fn parse_string(lex: &mut Lexer<MyGoToken>) -> ParseData<String> {
    let data = lex.slice()[1..lex.slice().len() - 1].to_string();
    ParseData {
        data,
        loc: get_parse_loc(lex),
    }
}

pub fn parse_identifier(lex: &mut Lexer<MyGoToken>) -> ParseData<String> {
    let data = lex.slice().to_string();
    ParseData {
        data,
        loc: get_parse_loc(lex),
    }
}

pub fn parse_interger(lex: &mut Lexer<MyGoToken>) -> Result<ParseData<PlatformInt>, MyGOError> {
    let data = lex
        .slice()
        .replace('_', "")
        .parse::<PlatformInt>()
        .map_err(|_| MyGOError::InvalidInterger)?;
    Ok(ParseData {
        data,
        loc: get_parse_loc(lex),
    })
}

pub fn parse_float(lex: &mut Lexer<MyGoToken>) -> Result<ParseData<f64>, MyGOError> {
    let data = lex
        .slice()
        .replace('_', "")
        .parse::<f64>()
        .map_err(|_| MyGOError::Invalidfloat)?;
    Ok(ParseData {
        data,
        loc: get_parse_loc(lex),
    })
}

fn get_parse_loc(lex: &Lexer<MyGoToken>) -> ParsePos {
    let line = lex.extras.0;
    let column = lex.span().start - lex.extras.1;
    (line, column)
}



pub fn newline_parse(lex: &mut Lexer<MyGoToken>) -> Skip {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
    Skip
}
