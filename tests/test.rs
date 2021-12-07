extern crate reverse;

#[cfg(test)]
mod tests {
    use reverse::ast::node::ASTChild;
    use reverse::parser::parse;
    use reverse::tokenizer::tokenize;
    use reverse::ast::node::ASTNode;
    use reverse::common::lang_value::LangValue;

    #[test]
    fn basic() {
        let script = "var i = func { 10 }";
        let tokens = tokenize::tokenize(script.to_string()).unwrap();
        let root = parse::parse(tokens);
        
        print_node(&root, 0);
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
                match value {
                    LangValue::String(string) => print_indented(string, ind + 1),
                    LangValue::Int(int) => print_indented(&int.to_string(), ind + 1),
                    LangValue::Float(float) => print_indented(&float.to_string(), ind + 1),
                    LangValue::Number(number) => print_indented(&number.to_string(), ind + 1),
                    LangValue::Bool(bool) => print_indented(&bool.to_string(), ind + 1),
                    LangValue::Function(body) => {
                        print_indented(&"[Function]:".to_string(), ind + 1);
                        
                        for child in body {
                            print_node(child, ind + 2);
                        }
                    },
                };
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