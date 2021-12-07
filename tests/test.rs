extern crate reverse;

#[cfg(test)]
mod tests {
    use reverse::ast::node::ASTChild;
    use reverse::parser::parse;
    use reverse::tokenizer::tokenize;
    use reverse::ast::node::ASTNode;
    use reverse::vm::scope::Scope;
    use reverse::vm::vm::evaluate;
    use reverse::vm::vm::EvalResult;

    #[test]
    fn basic() {
        let script = "10 + 10";
        let tokens = tokenize::tokenize(script.to_string()).unwrap();
        let root = parse::parse(tokens);
        
        print_node(&root, 0);
        
        let value = evaluate(&root, &mut Scope::new(None));
        
        match value {
            EvalResult::None => (),
            EvalResult::Some(value) => print_indented(&value.to_string(), 0),
            EvalResult::Err(err) => println!("{}", err),
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
            ASTNode::FunctionInvok { variable } => {
                println!("FunctionInvok:");
                print_node(variable, ind + 1);
            },
            ASTNode::Literal { value } => {
                println!("Literal:");
                print_indented(&value.to_string(), ind + 1);
            },
            ASTNode::MathOperation { operation, left, right } => {
                println!("MathOperator:");
                print_node(left, ind + 1);
                print_node(right , ind + 1);
            },
            ASTNode::BoolOperation { operation, left, right } => {
                println!("BoolOperator:");
                print_node(left, ind + 1);
                print_node(right , ind + 1);
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