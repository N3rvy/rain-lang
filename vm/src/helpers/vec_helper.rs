use common::{helper::Helper, external_functions::IntoExternalFunctionRunner, types::LangVector, lang_value::LangValue};

pub fn default_int_helper() -> Helper {
    let mut helper = Helper::new();

    helper.register("length", length.external());
    helper.register("add", add.external());

    helper
}

fn length(vec: LangVector) -> i32 {
    vec.len() as i32
}

fn add(vec: LangVector, item: LangValue) {
    vec.push(item);
}