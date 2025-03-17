use crate::window::Message;

#[derive(Debug)]
pub enum Expr {
    StringValue(String),
    NumberValue(u32),
    WhiteSpace(String),
    Colon,
}

#[derive(Debug)]
pub struct Bar {
    pub path: String,
    pub fs: BarFS,
}

#[derive(Debug)]
pub enum BarFS {
    Local,
    FTP,
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
    let mut modifiers = std::collections::HashMap::new();
    let mut current_modifier = None;
    let mut path = String::new();

    for x in lexed {
        match x {
            Expr::StringValue(s) => {
                let s_trimmed = s.trim().to_lowercase();
                if ["page", "fs"].contains(&s_trimmed.as_str()) {
                    current_modifier = Some(s_trimmed);
                } else {
                    if let Some(modifier) = current_modifier.take() {
                        let _ = modifiers.insert(modifier, s);
                    } else {
                        path.push_str(&s);
                    }
                }
            }
            Expr::NumberValue(n) => {
                if let Some(modifier) = current_modifier.take() {
                    let _ = modifiers.insert(modifier, n.to_string());
                } else {
                    path.push_str(&format!("{n}"));
                }
            }
            Expr::Colon => {
                if current_modifier.is_none() {
                    path.push(':');
                }
            }
            Expr::WhiteSpace(s) => {
                if current_modifier.is_none() {
                    path.push_str(&s);
                }
            }
        }
    }

    let fs = match modifiers.get("fs").map(|s| s.as_str()) {
        Some("local") => BarFS::Local,
        Some("ftp") => BarFS::FTP,
        _ => BarFS::Local,
    };

    let path = path.trim().to_string();

    Bar { path, fs }
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
