use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq)]
#[logos(error = MyGOError)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(subpattern letter = r"(\p{L}|_)")] // Any unicode letter or _
#[logos(subpattern punctuation = r#"\p{P}&&[^"]&&[^']"#)] // Any unicode letter or _
#[logos(subpattern decimal_digit = r"[0-9]")]
#[logos(subpattern decimal_digits = r"(?&decimal_digit)(?:_?)(?&decimal_digit)*")]
#[logos(subpattern escaped_char = r#"\\[nrtvfab\'\"]"#)]
// Note That \b not support by rust(?, so use character code(\x08) instand.
#[logos(subpattern unicode_value  = r"[(?&letter)(?&escaped_char)(?&decimal_digit)(?&punctuation)]")]
pub enum MyGoToken {
    #[regex(r"//.*\n?", logos::skip)] // Single Line Comment
    #[regex(r"/\*([^*]|\*+[^*/])*\*+/", logos::skip)] // Multi Line Comment
    Error,
    // Tokens can be literal strings, of any length.
    #[regex("(?&letter)((?&letter)|(?&decimal_digit))*", |lex| lex.slice().to_owned())]
    Identifier(String),

    #[regex("(?&decimal_digits)*\\.(?&decimal_digits)*", |lex| lex.slice().replace('_',"").parse::<f64>().unwrap())]
    FloatNumber(f64), // Save as f64, convert to actual type later.

    #[regex("(?&decimal_digits)", |lex| lex.slice().replace('_',"").parse::<PlatformInt>().unwrap())]
    Integer(PlatformInt),

    #[regex("\'((?&letter)|(?&escaped_char)|(?&decimal_digit)|(?&punctuation))\'", parse_rune)]
    Rune(char),

    #[regex("\"(?&unicode_value)*\"", |lex| lex.slice().replace("\"","").to_owned())]
    String(String),
}

fn parse_rune(lex: &mut Lexer<MyGoToken>) -> Result<char, MyGOError> {
    let char = lex.slice().replace('\'', "");
    match char.as_str() {
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
        _ => Err(MyGOError::InvalidRune) // shouldn't happen, because if not match above, it won't
                                         // get into here.
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum MyGOError {
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
        let input = "
            abc123
            _abc
            // This is a comment
        ";

        let mut lex = MyGoToken::lexer(input);
        let should_be1 = MyGoToken::Identifier(String::from("abc123"));
        let should_be2 = MyGoToken::Identifier(String::from("_abc"));

        assert_eq!(lex.next(), Some(Ok(should_be1)));
        assert_eq!(lex.next(), Some(Ok(should_be2)));
        assert_eq!(lex.next(), None);
    }
    #[test]
    fn test_lexer_number() {
        let input = "
            1234
            1_2
            1.2
            1.0
            1_2.34
            .34
        ";

        let mut lex = MyGoToken::lexer(input);

        assert_eq!(lex.next(), Some(Ok(MyGoToken::Integer(1234))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::Integer(12))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::FloatNumber(1.2))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::FloatNumber(1.0))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::FloatNumber(12.34))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::FloatNumber(0.34))));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_lexer_rune() {
        let mut input_f =
            read_file_to_string("src/lex/testcase/rune.txt").expect("Failed to read file");

        let mut lex = MyGoToken::lexer(&input_f);

        assert_eq!(lex.next(), Some(Ok(MyGoToken::Rune('a'))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::Rune('ä'))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::Rune('本'))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::Rune('\n'))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::Rune('\r'))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::Rune('\t'))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::Rune('\x0B'))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::Rune('\x0C'))));
        assert_eq!(lex.next(), Some(Ok(MyGoToken::Rune('\x08'))));
        assert_eq!(lex.next(), Some(Err(MyGOError::UnKnownToken)));
    }

    fn read_file_to_string(filename: &str) -> io::Result<String> {
        let mut file = File::open(filename)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    #[test]
    fn test_lexer_string_from_file() {
        let input =
            read_file_to_string("src/lex/testcase/string.txt").expect("Failed to read file");

        let mut lex = MyGoToken::lexer(&input);

        assert_eq!(lex.next(), Some(Ok(MyGoToken::String("abc".to_string()))));
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::String("hello,world".to_string())))
        );
        assert_eq!(
            lex.next(),
            Some(Ok(MyGoToken::String(r"中文\n".to_string())))
        );
    }
}
