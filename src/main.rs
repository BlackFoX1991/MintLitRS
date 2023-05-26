/*
trait Container {
    fn print(&self) -> String;
}

struct KeyNode {
    value: String,
    subnodes: Vec<Box<dyn Container>>,
}

impl KeyNode {
    fn new(val: &str) -> KeyNode {
        KeyNode {
            value: val.to_string(),
            subnodes: Vec::new(),
        }
    }


    fn add_sub(&mut self, keynd: Box<dyn Container>) {
        self.subnodes.push(keynd);
    }
    fn iter(&self) -> std::slice::Iter<'_, Box<dyn Container>> {
        self.subnodes.iter()
    }
}

impl Container for KeyNode {
    fn print(&self) -> String {
        self.value.clone()
    }
}

struct EmptyContainer;

impl Container for EmptyContainer {
    fn print(&self) -> String {
        String::new()
    }
}
*/

#![allow(non_snake_case)]
#![allow(dead_code)]

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::vec;
use std::fmt;

#[derive(Debug, Clone, Copy)]
enum TTypes
{
    INVALID = 0,
    IDENT_,
    STRING_,
    COLON,
    LBRACE,
    RBRACE,
    SEPA,
}

impl fmt::Display for TTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TTypes::INVALID => write!(f, "Invalid"),
            TTypes::COLON => write!(f, "Colon"),
            TTypes::LBRACE => write!(f, "L-Brace"),
            TTypes::RBRACE => write!(f, "R-Brace"),
            TTypes::IDENT_ => write!(f, "Identifier"),
            TTypes::STRING_ => write!(f, "String"),
            TTypes::SEPA => write!(f,"Separator")
        }
    }
}

trait Token
{
    fn print(&self) -> String;
}

struct Token_
{
    TokenLit: TTypes,
    Value: String,
    Line:i64
}

impl Token for Token_
{
    fn print(&self) -> String {
        self.Value.clone()
    }
}

impl Token_ {
    fn new(val: String, Tk: TTypes, line:i64) -> Token_
    {
        Token_ {
            Value: val.to_string(),
            TokenLit: Tk,
            Line : line
        }
    }
}


fn main() {
    /*
    let keynode1 = KeyNode::new("Node 1");
    let keynode2 = KeyNode::new("Node 2");

    let mut parent = KeyNode::new("Parent Node");
    parent.add_sub(Box::new(keynode1));
    parent.add_sub(Box::new(keynode2));

    for subnode in parent.iter() {
        println!("{}", subnode.print());
    }

     */

    let file = BufReader::new(File::open("sample.txt").expect("Unable to open file"));

    // List of Tokens
    let mut Tokens: Vec<Token_> = vec![];

    // Temporary String for keyvalue
    let mut stringMode = false;

    // Temporary Keyname
    let mut Tempname = String::from("");

    // Current Line
    let mut ln = 1;

    for line in file.lines() {
        for ch in line.expect("Unable to read line").chars() {
            if stringMode {
                if ch == '\'' {
                    Tokens.push(Token_::new(Tempname.to_string(), TTypes::STRING_,ln));
                    Tempname.clear();
                    stringMode = false;
                } else { Tempname.push(ch); }
            } else {
                if ch.is_alphabetic() || ch == '_'
                {
                    Tempname.push(ch);
                } else {
                    if !Tempname.is_empty() {
                        Tokens.push(Token_::new(Tempname.to_string(), TTypes::IDENT_,ln));
                        Tempname.clear();
                    }
                    match ch
                    {
                        '\'' => stringMode = true,
                        '{' => Tokens.push(Token_::new(ch.to_string(), TTypes::LBRACE,ln)),
                        '}' => Tokens.push(Token_::new(ch.to_string(), TTypes::RBRACE,ln)),
                        ':' => Tokens.push(Token_::new(ch.to_string(), TTypes::COLON,ln)),
                        ',' => Tokens.push(Token_::new(ch.to_string(), TTypes::SEPA,ln)),
                        _ =>
                            {
                                if !ch.is_whitespace() {
                                    Tokens.push(Token_::new(ch.to_string(), TTypes::INVALID,ln));
                                } else {
                                    if ch == '\r' {
                                        ln += 1;
                                    }
                                }
                            }
                    }
                }
            }
        }
    }

    'tlabel:for sb in Tokens {
        println!("Tokenizer -> {} is {:?}", sb.print(), sb.TokenLit.to_string());

        match sb.TokenLit {
            TTypes::INVALID => {
                println!("Invalid Token '{}' in Line {}.",sb.Value.to_string(),sb.Line);
                break 'tlabel;
            }
            _ =>
                {}
        }
    }
}
