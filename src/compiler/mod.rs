use inkwell::{context::Context, AddressSpace};

use crate::parser::ast::Statement;

pub fn compile(statements: Vec<Statement>) -> () {
    let context = Context::create();
    let module = context.create_module("frag");
    let float_type = context.f64_type();
    let bool_type = context.bool_type();
    let string_type = context.i8_type().ptr_type(AddressSpace::default());
    let color_type = context.struct_type(&[float_type.into(), float_type.into(), float_type.into(), float_type.into()], false);
}