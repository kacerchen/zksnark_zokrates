use crate::flat_absy::flat_parameter::FlatParameter;
use crate::flat_absy::flat_variable::FlatVariable;
use crate::flat_absy::*;
use crate::helpers::{DirectiveStatement, Helper};
use crate::types::signature::Signature;
use crate::types::Type;
use std::collections::HashMap;
use zokrates_field::field::Field;

fn use_variable(
    layout: &mut HashMap<String, FlatVariable>,
    name: String,
    index: &mut usize,
) -> FlatVariable {
    let var = FlatVariable::new(*index);
    layout.insert(name, var);
    *index = *index + 1;
    var
}

pub fn split<T: Field>() -> FlatProg<T> {
    let nbits = T::get_required_bits();

    let mut counter = 0;

    let mut layout = HashMap::new();

    let arguments = vec![FlatParameter {
        id: FlatVariable::new(0),
        private: true,
    }];

    // o0, ..., o253 = ToBits(i0)

    let directive_inputs = vec![FlatExpression::Identifier(use_variable(
        &mut layout,
        format!("i0"),
        &mut counter,
    ))];
    let directive_outputs: Vec<FlatVariable> = (0..T::get_required_bits())
        .map(|index| use_variable(&mut layout, format!("o{}", index), &mut counter))
        .collect();

    let helper = Helper::bits();

    let signature = Signature {
        inputs: vec![Type::FieldElement],
        outputs: vec![Type::FieldElementArray(nbits)],
    };

    let outputs = directive_outputs
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= T::get_required_bits() - nbits)
        .map(|(_, o)| FlatExpression::Identifier(o.clone()))
        .collect();

    // o253, o252, ... o{253 - (nbits - 1)} are bits
    let mut statements: Vec<FlatStatement<T>> = (0..nbits)
        .map(|index| {
            let bit = FlatExpression::Identifier(FlatVariable::new(T::get_required_bits() - index));
            FlatStatement::Condition(
                bit.clone(),
                FlatExpression::Mult(box bit.clone(), box bit.clone()),
            )
        })
        .collect();

    // sum check: o253 + o252 * 2 + ... + o{253 - (nbits - 1)} * 2**(nbits - 1)
    let mut lhs_sum = FlatExpression::Number(T::from(0));

    for i in 0..nbits {
        lhs_sum = FlatExpression::Add(
            box lhs_sum,
            box FlatExpression::Mult(
                box FlatExpression::Identifier(FlatVariable::new(T::get_required_bits() - i)),
                box FlatExpression::Number(T::from(2).pow(i)),
            ),
        );
    }

    statements.push(FlatStatement::Condition(
        lhs_sum,
        FlatExpression::Mult(
            box FlatExpression::Identifier(FlatVariable::new(0)),
            box FlatExpression::Number(T::from(1)),
        ),
    ));

    statements.insert(
        0,
        FlatStatement::Directive(DirectiveStatement {
            inputs: directive_inputs,
            outputs: directive_outputs,
            helper: helper,
        }),
    );

    statements.push(FlatStatement::Return(FlatExpressionList {
        expressions: outputs,
    }));

    FlatProg {
        functions: vec![FlatFunction {
            id: String::from("main"),
            arguments,
            statements,
            signature,
        }],
    }
}

pub fn cast<T: Field>(from: &Type, to: &Type) -> FlatFunction<T> {
    let mut counter = 0;

    let mut layout = HashMap::new();

    let arguments = (0..from.get_primitive_count())
        .enumerate()
        .map(|(index, _)| FlatParameter {
            id: FlatVariable::new(index),
            private: true,
        })
        .collect();

    let binding_inputs: Vec<_> = (0..from.get_primitive_count())
        .map(|index| use_variable(&mut layout, format!("i{}", index), &mut counter))
        .collect();
    let binding_outputs: Vec<FlatVariable> = (0..to.get_primitive_count())
        .map(|index| use_variable(&mut layout, format!("o{}", index), &mut counter))
        .collect();

    let outputs = binding_outputs
        .iter()
        .map(|o| FlatExpression::Identifier(o.clone()))
        .collect();

    let bindings: Vec<_> = match (from, to) {
        (Type::Boolean, Type::FieldElement) => binding_outputs
            .into_iter()
            .zip(binding_inputs.into_iter())
            .map(|(o, i)| FlatStatement::Definition(o, i.into()))
            .collect(),
        _ => panic!(format!("can't cast {} to {}", from, to)),
    };

    let signature = Signature {
        inputs: vec![from.clone()],
        outputs: vec![to.clone()],
    };

    let statements = bindings
        .into_iter()
        .chain(std::iter::once(FlatStatement::Return(FlatExpressionList {
            expressions: outputs,
        })))
        .collect();

    FlatFunction {
        id: format!("_{}_to_{}", from, to),
        arguments,
        statements,
        signature,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zokrates_field::field::FieldPrime;

    #[cfg(test)]
    mod cast {
        use super::*;

        #[test]
        fn bool_to_field() {
            let b2f: FlatFunction<FieldPrime> = cast(&Type::Boolean, &Type::FieldElement);
            assert_eq!(b2f.id, String::from("_bool_to_field"));
            assert_eq!(
                b2f.arguments,
                vec![FlatParameter::private(FlatVariable::new(0))]
            );
            assert_eq!(b2f.statements.len(), 2); // 1 definition, 1 return
            assert_eq!(
                b2f.statements[0],
                FlatStatement::Definition(FlatVariable::new(1), FlatVariable::new(0).into())
            );
            assert_eq!(
                b2f.statements[1],
                FlatStatement::Return(FlatExpressionList {
                    expressions: vec![FlatExpression::Identifier(FlatVariable::new(1))]
                })
            );
            assert_eq!(b2f.signature.outputs.len(), 1);
        }
    }

    #[cfg(test)]
    mod split {
        use super::*;

        #[test]
        fn split254() {
            let unpack: FlatProg<FieldPrime> = split();
            let unpack = &unpack.functions[0];

            assert_eq!(unpack.id, String::from("main"));
            assert_eq!(
                unpack.arguments,
                vec![FlatParameter::private(FlatVariable::new(0))]
            );
            assert_eq!(
                unpack.statements.len(),
                FieldPrime::get_required_bits() + 1 + 1 + 1
            ); // 128 bit checks, 1 directive, 1 sum check, 1 return
            assert_eq!(
                unpack.statements[0],
                FlatStatement::Directive(DirectiveStatement::new(
                    (0..FieldPrime::get_required_bits())
                        .map(|i| FlatVariable::new(i + 1))
                        .collect(),
                    Helper::bits(),
                    vec![FlatVariable::new(0)]
                ))
            );
            assert_eq!(
                *unpack.statements.last().unwrap(),
                FlatStatement::Return(FlatExpressionList {
                    expressions: (0..FieldPrime::get_required_bits())
                        .map(|i| FlatExpression::Identifier(FlatVariable::new(i + 1)))
                        .collect()
                })
            );
        }
    }
}
