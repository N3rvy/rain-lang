extern crate reverse;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use reverse::ast::node::ASTChild;
    use reverse::common::lang_value::LangValue;
    use reverse::common::types::ReturnKind;
    use reverse::parser::parse;
    use reverse::tokenizer::tokenize;
    use reverse::ast::node::ASTNode;
    use reverse::vm::scope::Scope;
    use reverse::vm::vm::evaluate;
    use reverse::vm::vm::EvalResult;
    use crate::reverse::vm::externals::functions::IntoExtFunc;

    #[test]
    fn basic() {
        let script = r#"
        var sum = func(a, b) {
            return a + b
        }
        
        return sum(20, 10)
        "#;
        let tokens = tokenize::tokenize(script.to_string()).unwrap();
        
        println!("Token count: {}", tokens.len());
        for token in &tokens {
            println!("{:?}", token);
        }

        let root = parse::parse(tokens).unwrap();
        
        print_node(&root, 0);
        
        let value = evaluate(&root, &mut Scope::new(None));
        
        match value {
            EvalResult::Ok(value) => println!("Ok {}", value.to_string()),
            EvalResult::Ret(value, kind) => println!("Return ({}) {}", kind_to_string(&kind), value.to_string()),
            EvalResult::Err(err) => println!("Error {}", err),
        }
    }
    
    #[test]
    fn external() {
        let script = r#"
        return getnum()
        "#;
        let tokens = tokenize::tokenize(script.to_string()).unwrap();
        let root = parse::parse(tokens).unwrap();
        
        let get_10: fn() -> LangValue = || LangValue::Int(10);

        let mut scope = Scope::new(None);
        scope.declare_var("getnum".to_string(), LangValue::ExtFunction(Arc::new(get_10.external_func())));

        let value = evaluate(&root, &mut scope);
        
        match value {
            EvalResult::Ok(value) => println!("Ok {}", value.to_string()),
            EvalResult::Ret(value, kind) => println!("Return ({}) {}", kind_to_string(&kind), value.to_string()),
            EvalResult::Err(err) => println!("Error {}", err),
        }
    }
    
    fn kind_to_string(kind: &ReturnKind) -> &'static str  {
        match kind {
            ReturnKind::Return => "Return",
            ReturnKind::Break => "Break",
            ReturnKind::Panic => "Panic",
        }
    }
    
    fn print_node(node: &ASTChild, ind: i32) {
        for _ in 0..ind {
            print!("  ");
        }

        match node.as_ref() {
            ASTNode::Root { body } => {
                println!("Root:");
                for child in body {
                    print_node(child, ind + 1)
                }
            },
            ASTNode::VariableDecl { name, value } => {
                println!("VariableDecl:");
                print_indented(name, ind + 1);
                print_node(value, ind + 1);
            },
            ASTNode::VaraibleRef { name } => {
                println!("VariableRef:");
                print_indented(name, ind + 1);
            },
            ASTNode::FunctionInvok { variable, parameters } => {
                println!("FunctionInvok:");
                print_node(variable, ind + 1);
            },
            ASTNode::Literal { value } => {
                println!("Literal:");
                print_indented(&value.to_string(), ind + 1);
            },
            ASTNode::MathOperation { operation, left, right } => {
                println!("MathOperator():");
                print_node(left, ind + 1);
                print_node(right , ind + 1);
            },
            ASTNode::BoolOperation { operation, left, right } => {
                println!("BoolOperator:");
                print_node(left, ind + 1);
                print_node(right , ind + 1);
            },
            ASTNode::ReturnStatement { value: Some(value), kind } => {
                println!("Return ({}):", kind_to_string(kind));
                print_node(value, ind + 1);
            },
            ASTNode::ReturnStatement { value: None, kind } => {
                println!("Return ({})", kind_to_string(kind));
            },
            ASTNode::IfStatement { condition, body } => {
                println!("IfStatement:");
                print_node(condition, ind + 1);
                print_indented(&"Body:".to_string(), ind + 1);
                for child in body {
                    print_node(child, ind + 2);
                }
            },
            ASTNode::ForStatement { left, right, body, iter_name } => {
                println!("ForStatement:");
                print_node(left, ind + 1);
                print_node(right, ind + 1);
                print_indented(&format!("iter_name: {}", iter_name), ind + 1);
                print_indented(&"Body:".to_string(), ind + 1);
                for child in body {
                    print_node(child, ind + 2);
                }
            },
            ASTNode::WhileStatement { condition, body } => {
                println!("WhileStatement:");
                print_node(condition, ind + 1);
                print_indented(&"Body:".to_string(), ind + 1);
                for child in body {
                    print_node(child, ind + 2);
                }
            },
            ASTNode::VariableAsgn { name, value } => {
                println!("VariableAsgn:");
                print_indented(name, ind + 1);
                print_node(value, ind + 1);
            },
        }
    }
    
    fn print_indented(string: &String, ind: i32) {
        for _ in 0..ind {
            print!("  ");
        }

        println!("{}", string); 
    }
}