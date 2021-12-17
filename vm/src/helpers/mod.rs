pub mod int_helper;

use common::{helper::HelperRegistry, lang_value::LangValueDiscriminant};

use self::int_helper::default_int_helper;


pub trait DefaultHelperRegistry {
    fn default() -> Self;
}

impl DefaultHelperRegistry for HelperRegistry {
    fn default() -> Self {
        let mut registry = HelperRegistry::new();
        
        registry.register_helper(LangValueDiscriminant::Int, default_int_helper());
            
        registry
    }
}