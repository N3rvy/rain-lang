#![feature(assert_matches)]

#[cfg(test)]
mod tests {
    use reverse::IntoExternalFunctionRunner;
    use std::assert_matches::assert_matches;
    use reverse::{LangValue, Scope, evaluate_scope};

    #[test]
    fn basic() {
        let script = r#"
        var sum = func(a, b) {
            return a + b
        }
        
        return sum(20, 10)
        "#;
        
        let result = reverse::evaluate(script.to_string());
        
        assert_matches!(result, Ok(LangValue::Int(30)))
    }
    
    #[test]
    fn external() {
        let script = r#"
        return add2(2)
        "#;
        
        let mut scope = Scope::new(None);
        scope.declare_ext_func("add2", ext_add2.external());
        // scope.declare_ext_func("sum", ext_sum.external());
        
        let result = evaluate_scope(script.to_string(), &mut scope);
        
        assert_matches!(result, Ok(LangValue::Int(4)))
    }
    
    fn ext_add2(i: i32) -> i32 {
        i + 2
    }
    
    fn ext_sum(a: i32, b: i32) -> i32 {
        a + b
    }
}