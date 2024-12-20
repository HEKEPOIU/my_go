use logos::Logos;

mod parse_token;
use parse_token::{
    none_data_parse, newline_parse, parse_float, parse_identifier, parse_interger, parse_rune,
    parse_string,
};

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
    #[regex(r"\n", newline_parse)]
    Newline,
    #[regex(r"//.*\n?", logos::skip)] // Single Line Comment
    #[regex(r"/\*([^*]|\*+[^*/])*\*+/", logos::skip)] // Multi Line Comment
    Error,
    // Tokens can be literal strings, of any length.
    #[regex("(?&letter)((?&letter)|(?&decimal_digit))*", parse_identifier)]
    Identifier(ParseData<String>),

    #[regex("(?&decimal_digits)*\\.(?&decimal_digits)+", parse_float)]
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

    #[regex("var", none_data_parse)]
    TVar(ParseData<()>),

    #[regex("if", none_data_parse)]
    TIf(ParseData<()>),

    #[regex("else", none_data_parse)]
    TElse(ParseData<()>),

    #[regex("for", none_data_parse)]
    TFor(ParseData<()>),

    #[regex("const", none_data_parse)]
    TConst(ParseData<()>),
    
    #[regex("func", none_data_parse)]
    TFunc(ParseData<()>),

    #[regex("=", none_data_parse)]
    TEq(ParseData<()>),

    #[regex(":=", none_data_parse)]
    TShortDecl(ParseData<()>),


    #[regex("<", none_data_parse)]
    TLess(ParseData<()>),

    #[regex("<=", none_data_parse)]
    TLessEq(ParseData<()>),

    #[regex(">", none_data_parse)]
    TGreater(ParseData<()>),

    #[regex(">=", none_data_parse)]
    TGreaterEq(ParseData<()>),

    #[regex("==", none_data_parse)]
    TLogiEq(ParseData<()>),

    #[regex("!", none_data_parse)]
    TLogiNot(ParseData<()>),

    #[regex("!=", none_data_parse)]
    TLogiNotEq(ParseData<()>),

    #[regex("&&", none_data_parse)]
    TLogiand(ParseData<()>),

    #[regex(r"\|\|", none_data_parse)]
    TLogior(ParseData<()>),

    #[regex(r"\+", none_data_parse)]
    TAdd(ParseData<()>),

    #[regex(r"\+\+", none_data_parse)]
    TAddone(ParseData<()>),

    #[regex(r"\+=", none_data_parse)]
    TAddEq(ParseData<()>),

    #[regex("-", none_data_parse)]
    TSub(ParseData<()>),

    #[regex("--", none_data_parse)]
    TSubone(ParseData<()>),

    #[regex(r"-=", none_data_parse)]
    TSubEq(ParseData<()>),

    #[regex(r"\*", none_data_parse)]
    TMult(ParseData<()>),

    #[regex(r"\*=", none_data_parse)]
    TMultEq(ParseData<()>),

    #[regex("/", none_data_parse)]
    TDiv(ParseData<()>),

    #[regex("/=", none_data_parse)]
    TDivEq(ParseData<()>),

    #[regex("%", none_data_parse)]
    TMod(ParseData<()>),

    #[regex("%=", none_data_parse)]
    TModEq(ParseData<()>),

    #[regex("&", none_data_parse)]
    TBitand(ParseData<()>),

    #[regex(r"\|", none_data_parse)]
    TBitor(ParseData<()>),

    #[regex(r"\^", none_data_parse)]
    TBitxor(ParseData<()>),

    #[regex("~", none_data_parse)]
    TBitNot(ParseData<()>),

    #[regex(r"\{", none_data_parse)]
    TLCBrack(ParseData<()>),

    #[regex(r"\}", none_data_parse)]
    TRCBrack(ParseData<()>),

    #[regex(r"\(", none_data_parse)]
    TLRBrack(ParseData<()>),

    #[regex(r"\)", none_data_parse)]
    TRRBrack(ParseData<()>),

    #[regex(r"\[", none_data_parse)]
    TLSBrack(ParseData<()>),

    #[regex(r"\]", none_data_parse)]
    TRSBrack(ParseData<()>),


    #[regex(";", none_data_parse)]
    TSemi(ParseData<()>),

    #[regex(":", none_data_parse)]
    TColon(ParseData<()>),

    #[regex(",", none_data_parse)]
    TComma(ParseData<()>),

    #[regex(r"\.", none_data_parse )]
    TDot(ParseData<()>),
}

type ParsePos = (Line, Column);
type Line = usize;
type Column = usize;

#[derive(Debug, PartialEq)]
pub struct ParseData<T = ()> {
    loc: ParsePos,
    data: T,
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
    fn test_lexer_keyword(){
        let input_f =
            read_file_to_string_helper("src/lex/testcase/keyword.txt").expect("Failed to read file");

        let mut lex = MyGoToken::lexer(&input_f);
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TVar(ParseData{data:(),loc:(0,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TIf(ParseData{data:(),loc:(1,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TElse(ParseData{data:(),loc:(2,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TFor(ParseData{data:(),loc:(3,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TConst(ParseData{data:(),loc:(4,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TFunc(ParseData{data:(),loc:(5,0)}))));
    }

    #[test]
    fn test_lexer_operator_and_punctuation(){
        let input_f =
            read_file_to_string_helper("src/lex/testcase/opandpun.txt").expect("Failed to read file");

        let mut lex = MyGoToken::lexer(&input_f);
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TAdd(ParseData{data:(),loc:(0,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TSub(ParseData{data:(),loc:(1,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TMult(ParseData{data:(),loc:(2,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TDiv(ParseData{data:(),loc:(3,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TMod(ParseData{data:(),loc:(4,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TBitand(ParseData{data:(),loc:(5,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TBitor(ParseData{data:(),loc:(6,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TBitxor(ParseData{data:(),loc:(7,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TLCBrack(ParseData{data:(),loc:(8,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TRCBrack(ParseData{data:(),loc:(9,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TEq(ParseData{data:(),loc:(10,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TLRBrack(ParseData{data:(),loc:(11,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TRRBrack(ParseData{data:(),loc:(12,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TLSBrack(ParseData{data:(),loc:(13,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TRSBrack(ParseData{data:(),loc:(14,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TLogiNot(ParseData{data:(),loc:(15,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TBitNot(ParseData{data:(),loc:(16,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TAddEq(ParseData{data:(),loc:(17,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TSubEq(ParseData{data:(),loc:(18,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TMultEq(ParseData{data:(),loc:(19,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TDivEq(ParseData{data:(),loc:(20,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TModEq(ParseData{data:(),loc:(21,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TAddone(ParseData{data:(),loc:(22,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TSubone(ParseData{data:(),loc:(23,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TLogiand(ParseData{data:(),loc:(24,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TLogior(ParseData{data:(),loc:(25,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TLogiEq(ParseData{data:(),loc:(26,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TLess(ParseData{data:(),loc:(27,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TGreater(ParseData{data:(),loc:(28,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TSemi(ParseData{data:(),loc:(29,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TColon(ParseData{data:(),loc:(30,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TComma(ParseData{data:(),loc:(31,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TDot(ParseData{data:(),loc:(32,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TShortDecl(ParseData{data:(),loc:(33,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TGreaterEq(ParseData{data:(),loc:(34,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TLessEq(ParseData{data:(),loc:(35,0)}))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::TLogiNotEq(ParseData{data:(),loc:(36,0)}))));
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
