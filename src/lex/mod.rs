use logos::{Lexer, Logos, Skip};

#[derive(Logos, Debug, PartialEq)]
#[logos(error = MyGOError)]
#[logos(extras = (usize, usize))]
#[logos(skip r"[ \t\f]+")]
#[logos(subpattern letter = r"(\p{L}|_)")] // Any unicode letter or _
#[logos(subpattern punctuation = r#"\p{P}&&[^"]&&[^']"#)] // Any unicode letter or _
#[logos(subpattern decimal_digit = r"[0-9]")]
#[logos(subpattern decimal_digits = r"(?&decimal_digit)(?:_?)(?&decimal_digit)*")]
#[logos(subpattern escaped_char_raw = r#"\\[nrtvfab\'\"]"#)]
#[logos(subpattern escaped_char = r"\s")]
// Note That \b not support by rust(?, so use character code(\x08) instand.
#[logos(subpattern unicode_value  = r"[(?&letter)(?&escaped_char_raw)(?&escaped_char)(?&decimal_digit)(?&punctuation)]")]
#[logos(subpattern interpreted_string_lit = r#""(?&unicode_value)*""#)]
#[logos(subpattern raw_string_lit = r"`((?&unicode_value)|\n)*`")]
pub enum MyGoToken {
    #[regex(r"\n", newline_callback)]
    Newline,
    #[regex(r"//.*\n?", logos::skip)] // Single Line Comment
    #[regex(r"/\*([^*]|\*+[^*/])*\*+/", logos::skip)] // Multi Line Comment
    Error,
    // Tokens can be literal strings, of any length.
    #[regex("(?&letter)((?&letter)|(?&decimal_digit))*", parse_identifier)]
    Identifier(ParseData<String>),

    #[regex("(?&decimal_digits)*\\.(?&decimal_digits)*", parse_float)]
    FloatNumber(ParseData<f64>), // Save as f64, convert to actual type later.

    #[regex("(?&decimal_digits)", parse_interger)]
    Integer(ParseData<PlatformInt>),

    #[regex(
        "\'((?&letter)|(?&escaped_char_raw)|(?&decimal_digit)|(?&punctuation))\'",
        parse_rune
    )]
    Rune(ParseData<char>),

    #[regex("((?&interpreted_string_lit)|(?&raw_string_lit))", parse_string)]
    String(ParseData<String>),
}

type ParsePos = (usize, usize);

#[derive(Debug, PartialEq)]
pub struct ParseData<T> {
    loc: ParsePos,
    data: T,
}

fn get_parse_loc(lex: &Lexer<MyGoToken>) -> ParsePos {
    let line = lex.extras.0;
    let column = lex.span().start - lex.extras.1;
    (line, column)
}

fn parse_rune(lex: &mut Lexer<MyGoToken>) -> Result<ParseData<char>, MyGOError> {
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

fn parse_string(lex: &mut Lexer<MyGoToken>) -> ParseData<String> {
    let data = lex.slice()[1..lex.slice().len() - 1].to_string();
    ParseData {
        data,
        loc: get_parse_loc(lex),
    }
}

fn parse_identifier(lex: &mut Lexer<MyGoToken>) -> ParseData<String> {
    let data = lex.slice().to_string();
    ParseData {
        data,
        loc: get_parse_loc(lex),
    }
}

fn parse_interger(lex: &mut Lexer<MyGoToken>) -> Result<ParseData<PlatformInt>, MyGOError> {
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

fn parse_float(lex: &mut Lexer<MyGoToken>) -> Result<ParseData<f64>, MyGOError> {
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

fn newline_callback(lex: &mut Lexer<MyGoToken>) -> Skip {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
    Skip
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum MyGOError {
    Invalidfloat,
    InvalidInterger,
    InvalidRune,
    #[default]
    UnKnownToken,
}

#[cfg(target_pointer_width = "32")]
type PlatformInt = i32;

#[cfg(target_pointer_width = "64")]
type PlatformInt = i64;

mod tests {
    use std::{
        fs::File,
        io::{self, Read},
    };

    use super::*;

    #[test]
    fn test_lexer_id() {
        let input_f = read_file_to_string_helper("src/lex/testcase/identifier.txt")
            .expect("Failed to read file");

        let mut lex = MyGoToken::lexer(&input_f);
        let should_be1 = MyGoToken::Identifier(ParseData {
            data: String::from("abc123"),
            loc: (0, 0),
        });
        let should_be2 = MyGoToken::Identifier(ParseData {
            data: String::from("_abc"),
            loc: (1, 0),
        });

        assert_eq!(lex.next(), Some(Ok(should_be1)));
        assert_eq!(lex.next(), Some(Ok(should_be2)));
        assert_eq!(lex.next(), None);
    }
    #[test]
    fn test_lexer_number() {
        let input_f =
            read_file_to_string_helper("src/lex/testcase/number.txt").expect("Failed to read file");

        let mut lex = MyGoToken::lexer(&input_f);

        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::Integer(ParseData {
                data: 1234,
                loc: (0, 0)
            })))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::Integer(ParseData {
                data: 12,
                loc: (1, 0)
            })))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::FloatNumber(ParseData {
                data: 1.2,
                loc: (2, 0)
            })))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::FloatNumber(ParseData {
                data: 1.0,
                loc: (3, 0)
            })))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::FloatNumber(ParseData {
                data: 12.34,
                loc: (4, 0)
            })))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::FloatNumber(ParseData {
                data: 0.34,
                loc: (5, 0)
            })))
        );
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_lexer_rune() {
        let input_f =
            read_file_to_string_helper("src/lex/testcase/rune.txt").expect("Failed to read file");

        let mut lex = MyGoToken::lexer(&input_f);

        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::Rune(ParseData {
                data: 'a',
                loc: (0, 0)
            })))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::Rune(ParseData {
                data: 'ä',
                loc: (1, 0)
            })))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::Rune(ParseData {
                data: '本',
                loc: (2, 0)
            })))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::Rune(ParseData {
                data: '\n',
                loc: (3, 0)
            })))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::Rune(ParseData {
                data: '\r',
                loc: (4, 0)
            })))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::Rune(ParseData {
                data: '\t',
                loc: (5, 0)
            })))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::Rune(ParseData {
                data: '\x0B',
                loc: (6, 0)
            })))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::Rune(ParseData {
                data: '\x0C',
                loc: (7, 0)
            })))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::Rune(ParseData {
                data: '\x08',
                loc: (8, 0)
            })))
        );
        assert_eq!(lex.next(), Some(Err(MyGOError::UnKnownToken)));
        assert_eq!(lex.extras.0, 9);
    }

    fn read_file_to_string_helper(filename: &str) -> io::Result<String> {
        let mut file = File::open(filename)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    #[test]
    fn test_lexer_string_from_file() {
        let input =
            read_file_to_string_helper("src/lex/testcase/string.txt").expect("Failed to read file");

        let mut lex = MyGoToken::lexer(&input);

        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::String(ParseData {
                data: "abc".to_string(),
                loc: (0, 0)
            })))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::String(ParseData {
                data: "hello,world".to_string(),
                loc: (1, 0)
            })))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::String(ParseData {
                data: r"中文\n".to_string(),
                loc: (2, 0)
            })))
        );

        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::String(ParseData {
                data: "\\n\n\\n".to_string(),
                loc: (3, 0)
            })))
        );

        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::String(ParseData {
                data: "abc\n    123".to_string(),
                loc: (4, 0)
            })))
        );
    }
}
