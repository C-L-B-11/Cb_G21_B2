use logos::{Lexer, Logos, Source};
use std::fmt::{Display, Formatter};

/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Token enum for capturing of link URLs and Texts
#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
    // TODO: Capture link definitions
    #[regex("href=\"[^<]*<",extract_link_info)]
    Link((LinkUrl, LinkText)),

    #[regex(".|ä|Ä|ö|Ö|ü|Ü|\n|ß|\n|<|>", logos::skip)]
    Ignored,

    // Catch any error    
    #[error]
    Error,
}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    let slice =lex.slice();
    let input: String = slice[..slice.len() - 1].to_string();
    let input2: String = input[6..].to_string();
    let index2= input2.find("\"").unwrap();
    let linkurl: String = input2[0..index2].to_string();
    let index3 = input2.find(">").unwrap();
    let linktext: String = input2[index3+1..].to_string();

    (LinkUrl(linkurl),LinkText(linktext))
}


