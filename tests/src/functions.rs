#[allow(unused_imports)]
use core::{LangError, AnyValue};
#[allow(unused_imports)]
use crate::test_script;


#[test]
fn function() -> Result<(), LangError> {
    test_script("
    func sum(a: int, b: int): int {
        return a + b
    }

    func init(): int {
        return sum(10, 5)
    }
    ",
    AnyValue::Int(15))
}