use gccjit::Context;

use crate::parser::ast::Node;

pub fn compile(statements: Vec<Node>) -> () {
    let context = Context::default();
    let float_ty = context.new_type::<f64>();
    let bool_ty = context.new_type::<bool>();
    let string_ty = context.new_type::<char>().make_pointer();
    let color_struct = context.new_struct_type(
        None,
        "Color",
        &[
            context.new_field(None, float_ty, "x"),
            context.new_field(None, float_ty, "y"),
            context.new_field(None, float_ty, "z"),
            context.new_field(None, float_ty, "a"),
        ],
    );

    let frag = context.new_function(
        None,
        gccjit::FunctionType::Exported,
        color_struct.as_type(),
        &[context.new_parameter(None, float_ty, "position")],
        "frag",
        false,
    );

    // context.
}
