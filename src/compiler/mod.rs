use gccjit::Context;

use crate::parser::ast::Statement;

pub fn compile(statements: Vec<Statement>) -> () {
    let context = Context::default();
    let float_type = context.new_type::<f64>();
    let bool_type = context.new_type::<bool>();
    let string_type = context.new_type::<char>().make_pointer();
    let color_type = context.new_struct_type(
        None,
        "Color",
        &[
            context.new_field(None, float_type, "x"),
            context.new_field(None, float_type, "y"),
            context.new_field(None, float_type, "z"),
            context.new_field(None, float_type, "a"),
        ],
    );
}