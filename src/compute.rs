use std::collections::HashMap;
use sycamore::prelude::ReadSignal;
use log::info;
use log::Level;
use sycamore::web::html::li;

use crate::IO704;
use crate::LineData;

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
    Print,
    Format,
    GoTo,
    SenseLight,
    If,
    Label(i32),
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
            "IF" => Token::If,
            "GoTo" => Token::GoTo,
            "DIMENSION" => Token::Dimension,
            "SenseLight" => Token::SenseLight,
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
    // let mut id = 0;
    // let mut current_args = vec![];
    let mut new_ids = -1;

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
                        new_ids -= 1;
                        sub_in.push((i-1, index, [vec![Token::OpenParen { id: Some(new_ids), function: false }],inside.clone(),vec![Token::CloseParen { id: Some(new_ids), function: false }]].concat()))

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
        if tokens.len() >= i.1+1 {
        tokens.splice(i.0..i.1+1, i.2.clone());
        }else {
        }
    }
}
#[derive(Debug, Clone)]
pub struct DoStatement {
    start: usize,
    end: usize,
    max: i32,
    current: i32,
    variable: String,
    step: i32,
}


/// # Returns (recurse, io, line_num)
pub fn run(tokens: Vec<Token>, io:IO704, mut line_num:usize, mut variables: HashMap<String, Vec<Token>>, mut do_statements: Vec<DoStatement>) -> (bool, IO704, usize, Vec<DoStatement>, HashMap<String, Vec<Token>>, bool) {
    let mut io = io;
    let mut lines: Vec<Vec<Token>> = vec![];
    let mut line = vec![];
    let mut labels:HashMap<i32, usize> = HashMap::new();

    let mut update_io = false;


    for i in 0..tokens.len() {
        match &tokens[i] {
            Token::Newline => {
                line.push(tokens[i].clone());
                lines.push(line.clone());
                line = vec![];
            }
            Token::Label(a) => {
                labels.insert(a.clone(), lines.len());
            }
            _ => {
                line.push(tokens[i].clone());
            }
        }
    }
    lines.push(line.clone());
    let mut return_ = false;
    loop {
        if line_num >= lines.len() {
            break
        }
        let line = lines[line_num].clone();
        let mut index = 0;
        for i in line.iter().skip(1) {
            index +=1;
            match i {
                Token::Identifier(a) => {
                    if variables.contains_key(a) {
                        let new_line: Vec<Token> = variables.get(a).unwrap().clone();                        
                        lines[line_num].splice(index..index+1, new_line.clone());
                        // lines.insert(line_num+1, new_line);
                        return_ = true;
                        combine_and_expand(&mut lines[line_num]);
                    }
                }
                _ => {}
            }
        }

        let line = lines[line_num].clone();

        if line.len() > 1 {
            match (&line[0], &line[1]) {
                (Token::Identifier(a), Token::Equals) => {
                    variables.insert(a.to_owned(), line[2..].iter().filter(|x| !matches!(x, Token::Newline)).map(|x| x.clone()).collect::<Vec<Token>>());
                }
                (Token::SenseLight, Token::Int(a)) if io.sense_lights.len()+1 > *a as usize => {
                    if a > &0 {
                        io.sense_lights[*a as usize-1] = true;
                        return_ = true;
                    }else {
                        io.sense_lights = vec![false; 4];
                        return_ = true;
                    }
                    update_io = true;
                }
                (Token::GoTo, Token::Int(a)) if labels.contains_key(a) => {
                    line_num = labels.get(a).unwrap().clone();
                    return_ = true;
                }
                _ => {}
            }
        }

        if line.len() > 4 {
            match (&line[0], &line[1], &line[2], &line[3], &line[4]) {
                (Token::If, Token::Int(x), Token::Int(a),Token::Int(b), Token::Int(c)) if labels.contains_key(a) && labels.contains_key(b) && labels.contains_key(c) => {
                    match x {
                        0 => {
                            line_num = labels.get(b).unwrap().clone();
                        }
                        _ if x < &1 => {
                            line_num = labels.get(a).unwrap().clone();
                        }
                        _ => {
                            line_num = labels.get(c).unwrap().clone();
                        }

                    }
                }
                
                _ => {}
            }
        }

        if line.len() > 9 {
            match (&line[0], &line[1], &line[2], &line[3], &line[4], &line[5], &line[6], &line[7], &line[8]) {
                (Token::Do, Token::Int(x), Token::Identifier(a), Token::Equals, Token::Int(b), Token::Comma, Token::Int(c), Token::Comma, Token::Int(d)) if labels.contains_key(x) => {
                    let do_statement = DoStatement {
                        start: line_num,
                        end: labels.get(x).unwrap().clone(),
                        max: *c,
                        step: *d,
                        current: *b,
                        variable: a.to_owned(),
                    };
                    variables.insert(a.to_owned(), vec![Token::Int(do_statement.current)]);
                    do_statements.push(do_statement);
                    return_ = true;
                }
                _ => {}
            }
        }

        if line.len() > 7 {
            match (&line[0], &line[1], &line[2], &line[3], &line[4], &line[5], &line[6]) {
                (Token::Do, Token::Int(x), Token::Identifier(a), Token::Equals, Token::Int(b), Token::Comma, Token::Int(c)) if labels.contains_key(x) => {
                    let do_statement = DoStatement {
                        start: line_num,
                        end: labels.get(x).unwrap().clone(),
                        max: *c,
                        step: 1,
                        current: *b,
                        variable: a.to_owned(),
                    };
                    variables.insert(a.to_owned(), vec![Token::Int(do_statement.current)]);
                    do_statements.push(do_statement);
                    return_ = true;
                }
                _ => {}
            }
        }
        if line.len() > 5 {
            match (&line[0], &line[1], &line[2], &line[3], &line[4]) {
                (Token::Do, Token::Int(x), Token::Identifier(a), Token::Equals, Token::Int(b)) if labels.contains_key(x) => {
                    let do_statement = DoStatement {
                        start: line_num,
                        end: labels.get(x).unwrap().clone(),
                        max: *b,
                        step: 1,
                        current: 1,
                        variable: a.to_owned(),
                    };
                    variables.insert(a.to_owned(), vec![Token::Int(do_statement.current)]);
                    do_statements.push(do_statement);
                    return_ = true;
                }
                _ => {}
            }
        }

        if !do_statements.is_empty() {
            let statement = do_statements.last_mut().unwrap();
            if line_num == statement.end {
                statement.current += statement.step;
                if statement.current <= statement.max {
                    line_num = statement.start;
                    variables.insert(statement.variable.to_owned(), vec![Token::Int(statement.current)]);
                    return_ = true;
                }else {
                    do_statements.pop();
                }
            }

        }


        line_num += 1;
        if return_ {
            return (true, io, line_num, do_statements, variables, update_io);
        }
    }

    
    update_io = true;

    return (false, io, line_num, do_statements, variables, update_io);

}

pub fn tokenize(mut in_string: String, mut line_data:Vec<LineData>) -> Vec<Token> {
    let mut tokens = Vec::new();
    in_string = in_string.to_uppercase();
    in_string = in_string.replace("\r\n", "\n");
    in_string = in_string.replace("SENSE LIGHT", "SenseLight");
    in_string = in_string.replace("GO TO", "GoTo");
    let chars = in_string.chars();

    let mut string = String::new();
    let mut number = String::new();

    
    
    let mut current_line = 0;

    for c in chars {
        if c == '\n' {
            current_line += 1;
        }
        if current_line >= line_data.len() {
            continue;
        }
        if line_data[current_line].continuation && c == '\n' {
            continue;
        }

        if line_data[current_line].label != 0 {
            tokens.push(Token::Label(line_data[current_line].label));
            line_data[current_line].label = 0;
        } else
            

        if line_data[current_line].comment {
            continue;
        }
        let c_type = get_char_type(c);

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

pub fn process(str: String, line_data:Vec<LineData>) -> Vec<Token> {
    let mut tokens = tokenize(str, line_data);
    tokens = tokens.iter().filter(|x| !matches!(x, Token::Space)).map(|x| x.clone()).collect();
    id_brackets(&mut tokens);
    combine_and_expand(&mut tokens);
    for i in 0..3 {
        inline_functions(&mut tokens);
    }
    combine_and_expand(&mut tokens);
    return tokens;
}
