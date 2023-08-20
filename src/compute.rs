use std::collections::HashMap;
use std::ops::Index;

use log::info;
use log::Level;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Variable {id: String},
    Function {id: String, args: Vec<String>, definition: bool},
    Int(i32),
    Float(f32),
    UnknownToken(String),
    Newline,
    OpenParen { id: Option<i64>, function: bool },
    CloseParen { id: Option<i64>, function: bool },
    Power,
    Multiply,
    Divide,
    Add,
    Subtract,
    Equals,
    Space,
    Stop,
    Do,
    Comma,
    Dimension,
    E,
}


impl Token {
    fn identifier_builtin(str: String) -> Token {
        match str.as_str() {
            "\n" => Token::Newline,
            "DO" => Token::Do,
            "DIMENSION" => Token::Dimension,
            "E" => Token::E,
            "STOP" => Token::Stop,
            _ => Token::Identifier(str),
        }
    }

    fn number(str: String) -> Token {
        if str.contains(".") {
            let float = str.parse::<f32>();
            match float {
                Ok(f) => Token::Float(f),
                Err(_) => Token::UnknownToken(str),
            }
        } else {
            let int = str.parse::<i32>();
            match int {
                Ok(i) => Token::Int(i),
                Err(_) => Token::UnknownToken(str),
            }
        }
    }

    fn tokenize_symbols(str: String) -> Token {
        match str.as_str() {
            "(" => Token::OpenParen { id: None, function: false },
            ")" => Token::CloseParen { id: None, function: false },
            "*" => Token::Multiply,
            "\n" => Token::Newline,
            "/" => Token::Divide,
            "+" => Token::Add,
            "," => Token::Comma,
            " " => Token::Space,
            "-" => Token::Subtract,
            "=" => Token::Equals,
            _ => Token::UnknownToken(str),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum CharType {
    Letter,
    Digit,
    Operator,
}



fn combine_and_expand(tokens: &mut Vec<Token>) {
    if tokens.len() == 0 {
        return;
    }
    let mut i = 0;
    let mut bedmas = 0;
    while i < tokens.len() {
        let mut del = vec![];
        let mut changed = true;
        if i < tokens.len() - 1 && i > 0 {
            match (&tokens[i - 1], &tokens[i], &tokens[i + 1], bedmas) {
                (Token::Float(x), Token::E, Token::Int(y), 0)=> {
                    tokens[i] = Token::Float(x * (10.0_f32).powi(y.clone()));
                }
                (Token::Int(x), Token::Power, Token::Int(y), 0) => {
                    tokens[i] = Token::Int(x.pow(y.clone() as u32));
                }
                (Token::Int(x), Token::Multiply, Token::Int(y), 1) => {
                    tokens[i] = Token::Int(x * y);
                }
                (Token::Float(x), Token::Multiply, Token::Float(y), 1) => {
                    tokens[i] = Token::Float(x * y);
                }
                (Token::Float(x), Token::Add, Token::Float(y), 2) => {
                    tokens[i] = Token::Float(x+y);
                }
                (Token::Float(x), Token::Subtract, Token::Float(y), 2) => {
                    tokens[i] = Token::Float(x-y);
                }
                (Token::Float(x), Token::Divide, Token::Float(y), 1) => {
                    tokens[i] = Token::Float(x/y);
                }
                (Token::Int(x), Token::Add, Token::Int(y), 2) => {
                    tokens[i] = Token::Int(x+y);
                }
                (Token::Int(x), Token::Subtract, Token::Int(y), 2) => {
                    tokens[i] = Token::Int(x-y);
                }
                (Token::Int(x), Token::Divide, Token::Int(y), 1) => {
                    tokens[i] = Token::Int(x/y);
                }
                // must be last option
                (Token::OpenParen { id: a, function: false}, _, Token::CloseParen { id: b, function: false}, _) if a==b => {
                    bedmas = 0;
                }
                _ => {
                    changed = false;
                }
            }
            if changed {
                del.push(i - 1);
                del.push(i + 1);
                i = 0;
            }
        }
        if i>0 {
            changed = true;
        match (&tokens[i-1], &tokens[i]) {
                (Token::Multiply, Token::Multiply) => {
                    tokens[i] = Token::Power;
                }
                _ => {
                    changed = false;
                }
            }
            if changed {
                del.push(i - 1);
                i = 0;
            }
        }
        if del == vec![] {
            i += 1;
            if i==tokens.len() {
                bedmas += 1;
                i = 0;
            }
            if bedmas == 5 {
                break;
            }
        }
        del.sort();
        for i in del.iter().rev() {
            tokens.remove(*i);
        }
        
    }
}

fn id_brackets(tokens: &mut Vec<Token>) {
    let mut id_1 = 0;
    let mut id_2 = 0;
    let mut functions = vec![];
    for i in 0..tokens.len() {
        match tokens[i] {
            Token::OpenParen { id: None, function:_} => {
                id_1 += 1;
                if id_1 == 1 {
                    id_2 += 10000000;
                }
                let func = !(i == 0 || !matches!(tokens[i - 1], Token::Identifier(_)));
                if func {
                    functions.push(id_1 + id_2);
                }
                tokens[i] = Token::OpenParen {
                    id: Some(id_1 + id_2),
                    function: func,
                };
            }
            Token::CloseParen { id: None, function:_} => {
                tokens[i] = Token::CloseParen {
                    id: Some(id_2 + id_1),
                    function: functions.contains(&(id_2 + id_1)),
                };
                id_1 -= 1;
            }
            _ => {}
        }
    }
}

fn get_char_type(c: char) -> CharType {
    if c.is_alphabetic() {
        return CharType::Letter;
    } else if c.is_numeric() || c == '.' {
        return CharType::Digit;
    } else {
        return CharType::Operator;
    }
}

fn inline_functions(tokens:&mut Vec<Token>) {
    let mut id = 0;
    // let mut current_args = vec![];

    let mut functions: HashMap<String, (Vec<Token>, Vec<Token>)> = HashMap::new();
    let mut sub_in = vec![];
    for i in 1..tokens.len() {
        match (&tokens[i-1], &tokens[i]) {
            (Token::Identifier(a), Token::OpenParen {id: Some(b), function: true}) => {
                let mut inside = vec![];
                if functions.contains_key(a) {
                    let (args, mut inside): (Vec<Token>, Vec<Token>) = functions.get(a).unwrap().clone();
                    if let Some(index) = tokens.iter().position(|r| r == &Token::CloseParen {id: Some(b.clone()), function: true}) {
                        let mut arg_num = 0;
                        let mut current_args: Vec<Token> = vec![];
                        for i in tokens.iter().skip(i+1) {
                            match i {
                                Token::CloseParen { id: Some(close_id), function: true } if close_id == b => {
                                    inside = inside.iter().flat_map(|x| match x {
                                        Token::Identifier(_) if x == &args[arg_num] => current_args.clone(),
                                        _ => vec![x.clone()]
                                    }).collect::<Vec<Token>>();
                                    info!("{:?}, {:?}", arg_num, args[arg_num]);
                                    break;
                                }
                                Token::Comma => {
                                    
                                    inside = inside.iter().flat_map(|x| match x {
                                        Token::Identifier(_) if x == &args[arg_num] => current_args.clone(),
                                        _ => vec![x.clone()]
                                    }).collect::<Vec<Token>>();
                                    
                                    current_args = vec![];
                                    arg_num += 1;
                                }
                                i => {
                                    current_args.push(i.clone());
                                }
                            }
                        }
                        
                        for i in tokens.iter().skip(i+1) {
                            if *i == Token::Newline {
                                break;
                            }
                            
                        }
                        sub_in.push((i-1, index, inside.clone()))

                    }
                }
                if let Some(index) = tokens.iter().position(|r| r == &Token::CloseParen {id: Some(b.clone()), function: true}) {
                    if index < tokens.len()-1 && tokens[index+1] == Token::Equals {
                        let args = tokens[i+1..index].to_vec().iter().filter(|x| !matches!(x, Token::Comma)).map(|x| x.clone()).collect::<Vec<Token>>();
                        for i in tokens.iter().skip(index+2) {
                            if *i == Token::Newline {
                                break;
                            }
                            inside.push(i.clone());
                        }
                        functions.insert(a.clone(), (args, inside));
                    }
                }


            }
            _ => {}
        }

    }

    for i in sub_in.iter().rev() {
        tokens.splice(i.0..i.1+1, i.2.clone());
    }
}

pub fn tokenize(mut in_string: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    in_string = in_string.to_uppercase();
    in_string = in_string.replace("\r\n", "\n");
    let chars = in_string.chars();

    let mut string = String::new();
    let mut number = String::new();
    for c in chars {
        let c_type = get_char_type(c);
        info!("{:?},{}", c_type, c);

        if c_type != CharType::Letter && string.len() > 0 {
            println!("string: {}", string);
            tokens.push(Token::identifier_builtin(string));
            string = String::new();
        } else if CharType::Letter == c_type {
            string.push(c);
        }

        if c_type != CharType::Digit && number.len() > 0 {
            tokens.push(Token::number(number));
            number = String::new();
        } else if CharType::Digit == c_type {
            number.push(c);
        }
        if c_type == CharType::Operator {
            tokens.push(Token::tokenize_symbols(c.to_string()));
        }
    }
    if number.len() > 0 {
        tokens.push(Token::number(number));
    }
    if string.len() > 0 {
        tokens.push(Token::identifier_builtin(string));
    }

    return tokens;
}

pub fn process(str: String) -> Vec<Token> {
    let mut tokens = tokenize(str);
    tokens = tokens.iter().filter(|x| !matches!(x, Token::Space)).map(|x| x.clone()).collect();
    id_brackets(&mut tokens);
    combine_and_expand(&mut tokens);
    for i in 0..5 {
        inline_functions(&mut tokens);
    }
    combine_and_expand(&mut tokens);
    return tokens;
}
