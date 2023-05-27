

#![allow(non_snake_case)]
#![allow(dead_code)]

use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::fs::File;
use std::vec;
use std::fmt;
use std::hash::Hash;
use std::ptr::hash;
use std::thread::current;

use crate::TTypes::INVALID;


/* Token Section */
#[derive(Debug, Clone, Copy, PartialEq)]
enum TTypes
{
    INVALID = 0,
    IDENT_,
    STRING_,
    COLON,
    LBRACE,
    RBRACE,
    SEPA,
    SEMI
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
            TTypes::SEPA => write!(f,"Separator"),
            TTypes::SEMI => write!(f,"Semicolon")
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

/* Token Section end */


fn main() {


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
                        ';' => Tokens.push(Token_::new(ch.to_string(), TTypes::SEMI,ln)),
                        _ =>
                            {
                                if !ch.is_whitespace() {
                                    Tokens.push(Token_::new(ch.to_string(), TTypes::INVALID,ln));
                                } else {
                                    if ch == '\n' {
                                        ln += 1;
                                    }
                                }
                            }
                    }
                }
            }
        }
    }



    let mut currentExpected:[TTypes; 3] = [TTypes::IDENT_,TTypes::INVALID,TTypes::INVALID];
    let mut MyKeys:HashMap<String,String> = HashMap::new();
    let mut curPath:String = String::new();
    let mut lev = 0;


    'tlabel:for sb in Tokens {
       // println!("Tokenizer -> {} is {:?}", sb.print(), sb.TokenLit.to_string());

        if !currentExpected.contains(&sb.TokenLit) {
            println!("Unexpected Token '{:?} in Line {}'.",sb.TokenLit.to_string(),sb.Line);
            break 'tlabel;
        }

        match sb.TokenLit {
            TTypes::INVALID => {
                println!("Invalid Token '{}' in Line {}.",sb.Value.to_string(),sb.Line);
                break 'tlabel;
            },
            TTypes::IDENT_ => {

                curPath = if curPath.is_empty() {
                    format!("{}",sb.Value.to_string())
                }
                else{
                    format!("{}{}",curPath.to_string(),sb.Value.to_string())
                };
                //println!("PATH IS {}.",curPath.to_string());

                currentExpected = [TTypes::INVALID,TTypes::INVALID,TTypes::INVALID];
                currentExpected[0] = TTypes::COLON;
            },
            TTypes::COLON =>{
                currentExpected = [TTypes::INVALID,TTypes::INVALID,TTypes::INVALID];
                currentExpected[0] = TTypes::STRING_;
            },
            TTypes::STRING_ => {
                MyKeys.insert(curPath.to_string(),sb.Value.to_string());
                println!("The Value is -> {}.",sb.Value.to_string());
                currentExpected = [TTypes::INVALID,TTypes::INVALID,TTypes::INVALID];
                currentExpected[0] = TTypes::LBRACE;
                currentExpected[1] = TTypes::SEMI;
            },
            TTypes::LBRACE => {
                lev +=1;
                curPath.push('/');
                currentExpected = [TTypes::INVALID,TTypes::INVALID,TTypes::INVALID];
                if curPath.contains("/"){ currentExpected[0] = TTypes::RBRACE; }
                currentExpected[1] = TTypes::IDENT_;
            },
            TTypes::SEMI => {

                if curPath.contains("/")
                {
                    let myPoint = curPath.rfind("/").unwrap();
                    curPath = curPath[0..myPoint+1].to_string();
                }
                else{
                    curPath.clear();
                }

                currentExpected = [TTypes::INVALID,TTypes::INVALID,TTypes::INVALID];
                if curPath.contains("/") { currentExpected[0] = TTypes::RBRACE;}
                currentExpected[1] = TTypes::IDENT_;

            },
            TTypes::RBRACE =>{

                lev-=1;
                let myPoint = curPath.rfind("/").unwrap();
                curPath = curPath[0..myPoint].to_string();

                if curPath.contains("/") {
                    let myp = curPath.rfind("/").unwrap();
                    curPath = curPath[0..myp+1].to_string();
                }
                else { curPath.clear(); }


                currentExpected = [TTypes::INVALID,TTypes::INVALID,TTypes::INVALID];
                if curPath.contains("/") { currentExpected[0] = TTypes::RBRACE;}
                currentExpected[1] = TTypes::IDENT_;

            }
            _ =>
                {}
        }
    }
    for (key, value) in MyKeys.iter() {
        println!("Key: {}, Value: {}", key, value);
    }
}