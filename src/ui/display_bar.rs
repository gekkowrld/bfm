use std::path::PathBuf;

use crate::ui::files::Message;

#[derive(Debug)]
pub enum Expr {
    StringValue(String),
    NumberValue(u32),
    WhiteSpace(String),
    Colon,
}

#[derive(Debug)]
pub struct Bar {
    pub path: PathBuf,
    pub page: usize,
}

pub fn display_bar<'a>(content: String) -> iced::Element<'a, Message> {
    iced::widget::text_input("Enter file address", &content)
        .on_input(Message::DisplayBarContentChanged)
        .on_submit(Message::DisplayBarContentSubmitted)
        .into()
}

pub fn display_bar_content(content: String) -> Bar {
    let lexed = lex_content(content);
    parse_content(lexed)
}

fn parse_content(lexed: Vec<Expr>) -> Bar {
    let mut is_page: bool = false;
    let mut path = String::new();
    let mut page = 1;

    for x in lexed {
        match x {
            Expr::StringValue(s) if s.trim().to_lowercase() == "page" => {
                is_page = true;
            }
            Expr::NumberValue(n) if is_page => {
                page = n as usize;
                is_page = false;
            }
            Expr::NumberValue(n) if !is_page => {
                path.push_str(&format!("{n}"));
            }
            Expr::StringValue(s) => {
                path.push_str(&s);
            }
            Expr::Colon => {
                if !is_page {
                    path.push(':');
                }
            }
            Expr::WhiteSpace(s) if !is_page => {
                path.push_str(&s);
            }
            // Skip spaces if it is a page
            Expr::WhiteSpace(_) => {}
            Expr::NumberValue(_) => todo!("Handle number in page"),
        }
    }
    let path = PathBuf::from(path.trim().to_string());

    Bar { path, page }
}

fn lex_content(content: String) -> Vec<Expr> {
    let mut exprs: Vec<Expr> = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = content.chars().collect();

    while i < chars.len() {
        let c = chars[i];

        match c {
            '0'..='9' => {
                let mut number_str = String::new();
                while i < chars.len() && chars[i].is_ascii_digit() {
                    number_str.push(chars[i]);
                    i += 1;
                }

                match number_str.trim().parse::<u32>() {
                    Ok(num) => exprs.push(Expr::NumberValue(num)),
                    Err(_) => eprintln!("Error parsing number: {}", number_str),
                }
                continue;
            }
            ':' => {
                exprs.push(Expr::Colon);
                i += 1;
            }
            ' ' | '\t' | '\n' => {
                let mut white_space = String::new();
                while i < chars.len() && chars[i].is_whitespace() {
                    white_space.push(chars[i]);
                    i += 1;
                }

                exprs.push(Expr::WhiteSpace(white_space));
                continue;
            }
            _ => {
                let mut unkown_str = String::new();

                while i < chars.len() && !match_checked(chars[i]) {
                    unkown_str.push(chars[i]);
                    i += 1;
                }

                exprs.push(Expr::StringValue(unkown_str));
                continue;
            }
        }
    }

    exprs
}

/// A helper function to see if the char is checked by the match statement
fn match_checked(c: char) -> bool {
    matches!(c, ':') || c.is_whitespace() || c.is_ascii_digit()
}
