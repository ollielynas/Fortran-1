

#[derive(Debug)]
enum Token {
    Identifier(String),
    Int(i32),
    Float(f32),
    UnknownToken(String),
    Newline,
    OpenParen{id: Option<i64>},
    CloseParen{id: Option<i64>},
    Power,
    Multiply,
    Divide,
    Add,
    Subtract,
    Equals,
    Space,
    Stop,
    Do,
    Dimension,
    E,
}



impl Token {

    fn identifier_builtin(str:String) -> Token {

        match str.as_str() {
            "\n" => Token::Newline,
            "DO" => Token::Do,
            "DIMENSION" => Token::Dimension,
            "E" => Token::E,
            "STOP" => Token::Stop,
            _ => Token::Identifier(str),
        }
    }

    fn number(str:String) -> Token {
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


    fn tokenize_symbols(str:String) -> Token {
        match str.as_str() {
            "(" => Token::OpenParen{id:None},
            ")" => Token::CloseParen{id:None},
            "*" => Token::Multiply,
            "/" => Token::Divide,
            "+" => Token::Add,
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
    Space,
    Operator,
}

fn get_char_type(c: char) -> CharType {
    if c.is_alphabetic() {
        return CharType::Letter;
    } else if c.is_numeric() || c == '.' {
        return CharType::Digit;
    }else{
        return CharType::Operator;
    }
}


fn tokenize(in_string: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut in_string = in_string.to_uppercase();
    let chars = in_string.chars();

    let mut string = String::new();
    let mut number = String::new();
    for c in chars {
        let c_type = get_char_type(c);
        println!("{:?},{}", c_type, c);

        if c_type == CharType::Operator {
            tokens.push(Token::tokenize_symbols(c.to_string()));
        }

        if c_type != CharType::Letter && string.len() > 0 {
            println!("string: {}", string);
            tokens.push(Token::identifier_builtin(string));
            string = String::new();
        }else {
            string.push(c);
        }

        if c_type != CharType::Digit && number.len() > 0 {
            tokens.push(Token::number(number));
            number = String::new();
        }else {
            number.push(c);
        }


    }
    
    return tokens
}

fn main() {
    let input = include_str!("input.txt");
    let tokens = tokenize(String::from(input));
    println!("{:#?}", tokens);
}
