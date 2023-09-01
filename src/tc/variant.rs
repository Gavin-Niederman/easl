use rusttyc::{Arity, Constructable, Partial, Variant};

use super::TypeCheckerError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Infer,
    String,
    Int,
    Color,
    Bool,
    Unit,
    Array(Box<Type>),
    Fun { input: Box<Type>, output: Box<Type> },
}

impl Variant for Type {
    type Err = TypeCheckerError;
    fn arity(&self) -> Arity {
        match self {
            Self::Fun { .. } => Arity::Fixed(2),
            Self::Array(_) => Arity::Fixed(1),
            _ => Arity::Fixed(0),
        }
    }

    fn top() -> Self {
        Self::Infer
    }

    fn meet(lhs: Partial<Self>, rhs: Partial<Self>) -> Result<Partial<Self>, Self::Err> {
        let mut least_arity = 0;
        let variant = match (lhs.variant, rhs.variant) {
            (Self::Infer, x) | (x, Self::Infer) => Ok(x),
            (Self::String, Self::String) => Ok(Self::String),
            (Self::Int, Self::Int) => Ok(Self::Int),
            (Self::Color, Self::Color) => Ok(Self::Color),
            (Self::Bool, Self::Bool) => Ok(Self::Bool),
            (Self::Unit, Self::Unit) => Ok(Self::Unit),
            (Self::Array(lhs), Self::Array(rhs)) => {
                least_arity = 1;
                Ok(Self::Array(Box::new(
                    Self::meet(
                        Partial {
                            variant: *lhs,
                            least_arity: 0,
                        },
                        Partial {
                            variant: *rhs,
                            least_arity: 0,
                        },
                    )?
                    .variant,
                )))
            }
            (Self::Fun { .. }, Self::Fun { .. }) => Err(TypeCheckerError::MetFunctions),
            (lhs, rhs) => Err(TypeCheckerError::IncompatibleTypes { lhs, rhs }),
        }?;

        Ok(Partial {
            variant,
            least_arity: 0,
        })
    }
}

// impl Constructable for Type {
//     type Type = gccjit::Type<'static>;
//     fn construct(
//         &self,
//         children: &[Self::Type],
//     ) -> Result<Self::Type, <Self as rusttyc::ContextSensitiveVariant>::Err> {
//         let ctx = gccjit::Context::default();
//         Ok(match self {
//             Type::Array(_) => todo!(),
//             Type::String => todo!(),
//             Type::Int => ctx.new_type::<f64>(),
//             Type::Color => todo!(),
//             Type::Bool => ctx.new_type::<bool>(),
//             Type::Unit => ctx.new_type::<()>(),
//             Type::Fun { input, output } => ctx.new_function_pointer_type(
//                 None,
//                 output.construct(&[])?,
//                 &[input.construct(&[])?],
//                 false,
//             ),
//             Type::Infer => Err(TypeCheckerError::UnknownType {})?,
//         })
//     }
// }
