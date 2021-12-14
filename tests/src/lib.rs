#![feature(assert_matches)]

#[cfg(test)]
mod tests {
    use std::{assert_matches::assert_matches};
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
        
        let ext: fn(i32) -> i32 = ext_add2;
        
        let mut scope = Scope::new(None);
        scope.declare_ext_func("add2".to_string(), ext);
        
        let result = evaluate_scope(script.to_string(), &mut scope);
        
        assert_matches!(result, Ok(LangValue::Int(4)))
    }
    
    fn ext_add2(i: i32) -> i32 {
        i + 2
    }
}