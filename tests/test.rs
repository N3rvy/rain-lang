extern crate reverse;

#[cfg(test)]
mod tests {
    use reverse::common::lang_value::LangValue;
    use reverse::tokenizer::tokenize::tokenize;
    use reverse::tokenizer::tokens::Token;

    #[test]
    fn basic() {
        let script = "var i = 10 == 10";
        let tokens = tokenize(script.to_string()).unwrap();
        
        for token in tokens {
            let str = match token {
                Token::Function => "Function",
                Token::Variable => "Variable",
                Token::Operator(_) => "Operator",
                Token::BoolOperator(_) => "BoolOperator",
                Token::MathOperator(_) => "MathOperator",
                Token::Symbol(_) => "Symbol",
                Token::Literal(_) => "Literal",
                Token::Parenthesis(_, _) => "Parenthesis",
            };
            
            println!("{}", str);
        }
    }
}