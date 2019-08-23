//! Module containing semantic analysis tools to run at compile time
//! The goal is to detect semantic errors such as undefined variables
//! A variable is undefined if it isn't present in the static scope
//!
//! @file semantics.rs
//! @author Thibaut Schaeffer <thibaut@schaeff.fr>
//! @date 2017

use crate::absy::variable::Variable;
use crate::absy::Identifier;
use crate::absy::*;
use crate::typed_absy::*;
use crate::types::Signature;
use std::collections::HashSet;
use std::fmt;
use zokrates_field::field::Field;

use crate::parser::Position;

use crate::types::Type;

use std::hash::{Hash, Hasher};

#[derive(PartialEq, Debug)]
pub struct Error {
    pos: Option<(Position, Position)>,
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let location = self
            .pos
            .map(|p| format!("{}", p.0))
            .unwrap_or("?".to_string());
        write!(f, "{}\n\t{}", location, self.message)
    }
}

pub struct FunctionQuery<'ast> {
    id: Identifier<'ast>,
    inputs: Vec<Type>,
    outputs: Vec<Option<Type>>,
}

impl<'ast> fmt::Display for FunctionQuery<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        r#try!(write!(f, "("));
        for (i, t) in self.inputs.iter().enumerate() {
            r#try!(write!(f, "{}", t));
            if i < self.inputs.len() - 1 {
                r#try!(write!(f, ", "));
            }
        }
        r#try!(write!(f, ") -> ("));
        for (i, t) in self.outputs.iter().enumerate() {
            match t {
                Some(t) => r#try!(write!(f, "{}", t)),
                None => r#try!(write!(f, "_")),
            }
            if i < self.outputs.len() - 1 {
                r#try!(write!(f, ", "));
            }
        }
        write!(f, ")")
    }
}

impl<'ast> FunctionQuery<'ast> {
    fn new(
        id: Identifier<'ast>,
        inputs: &Vec<Type>,
        outputs: &Vec<Option<Type>>,
    ) -> FunctionQuery<'ast> {
        FunctionQuery {
            id,
            inputs: inputs.clone(),
            outputs: outputs.clone(),
        }
    }

    fn match_func(&self, func: &FunctionDeclaration) -> bool {
        self.id == func.id
            && self.inputs == func.signature.inputs
            && self.outputs.len() == func.signature.outputs.len()
            && self.outputs.iter().enumerate().all(|(index, t)| match t {
                Some(ref t) => t == &func.signature.outputs[index],
                _ => true,
            })
    }

    fn match_funcs(&self, funcs: &HashSet<FunctionDeclaration>) -> Vec<FunctionDeclaration> {
        funcs
            .iter()
            .filter(|func| self.match_func(func))
            .cloned()
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct ScopedVariable<'ast> {
    id: Variable<'ast>,
    level: usize,
}

impl<'ast> Hash for ScopedVariable<'ast> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.id.hash(state);
    }
}

impl<'ast> PartialEq for ScopedVariable<'ast> {
    fn eq(&self, other: &ScopedVariable) -> bool {
        self.id.id == other.id.id
    }
}
impl<'ast> Eq for ScopedVariable<'ast> {}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionDeclaration {
    id: String, // we use strings here as flat functions are still using strings
    signature: Signature,
}

// Checker, checks the semantics of a program.
pub struct Checker<'ast> {
    scope: HashSet<ScopedVariable<'ast>>,
    functions: HashSet<FunctionDeclaration>,
    level: usize,
}

impl<'ast> Checker<'ast> {
    fn new() -> Checker<'ast> {
        Checker {
            scope: HashSet::new(),
            functions: HashSet::new(),
            level: 0,
        }
    }

    pub fn check<T: Field>(prog: Prog<'ast, T>) -> Result<TypedProg<'ast, T>, Vec<Error>> {
        Checker::new().check_program(prog)
    }

    fn check_program<T: Field>(
        &mut self,
        prog: Prog<'ast, T>,
    ) -> Result<TypedProg<'ast, T>, Vec<Error>> {
        for func in &prog.imported_functions {
            self.functions.insert(FunctionDeclaration {
                id: func.id.to_string(), // we use strings here as flat absy is still string based
                signature: func.signature.clone(),
            });
        }

        let mut errors = vec![];
        let mut checked_functions = vec![];

        for func in prog.functions {
            self.enter_scope();

            let dec = FunctionDeclaration {
                id: func.value.id.to_string(),
                signature: func.value.signature.clone(),
            };

            match self.check_function(func) {
                Ok(checked_function) => {
                    checked_functions.push(checked_function);
                }
                Err(e) => {
                    errors.extend(e);
                }
            };
            self.functions.insert(dec);
            self.exit_scope();
        }

        match self.check_single_main() {
            Ok(()) => {}
            Err(e) => errors.push(e),
        };

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(TypedProg {
            functions: checked_functions,
            imported_functions: prog.imported_functions,
            imports: prog.imports.into_iter().map(|i| i.value).collect(),
        })
    }

    fn check_single_main(&mut self) -> Result<(), Error> {
        match self.functions.iter().filter(|fun| fun.id == "main").count() {
            1 => Ok(()),
            0 => Err(Error {
                pos: None,
                message: format!("No main function found"),
            }),
            n => Err(Error {
                pos: None,
                message: format!("Only one main function allowed, found {}", n),
            }),
        }
    }

    fn check_for_var(&self, var: &VariableNode) -> Result<(), Error> {
        match var.value.get_type() {
            Type::FieldElement => Ok(()),
            t => Err(Error {
                pos: Some(var.pos()),
                message: format!("Variable in for loop cannot have type {}", t),
            }),
        }
    }

    fn check_function<T: Field>(
        &mut self,
        funct_node: FunctionNode<'ast, T>,
    ) -> Result<TypedFunction<'ast, T>, Vec<Error>> {
        let mut errors = vec![];
        let pos = funct_node.pos();
        let funct = funct_node.value;

        assert_eq!(funct.arguments.len(), funct.signature.inputs.len());

        let query = FunctionQuery::new(
            funct.id,
            &funct.signature.inputs,
            &funct
                .signature
                .outputs
                .clone()
                .into_iter()
                .map(|o| Some(o))
                .collect(),
        );

        let candidates = self.find_candidates(&query);

        match candidates.len() {
            1 => {
                errors.push(Error {
                    pos: Some(pos),
                    message: format!(
                        "Duplicate definition for function {} with signature {}",
                        funct.id, funct.signature
                    ),
                });
            }
            0 => {}
            _ => panic!("duplicate function declaration should have been caught"),
        }

        for arg in &funct.arguments {
            self.insert_scope(arg.value.id.value.clone());
        }

        let mut statements_checked = vec![];

        for stat in funct.statements.into_iter() {
            match self.check_statement(stat, &funct.signature.outputs) {
                Ok(statement) => {
                    statements_checked.push(statement);
                }
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(TypedFunction {
            id: funct.id,
            arguments: funct
                .arguments
                .into_iter()
                .map(|a| a.value.into())
                .collect(),
            statements: statements_checked,
            signature: funct.signature,
        })
    }

    fn check_statement<T: Field>(
        &mut self,
        stat: StatementNode<'ast, T>,
        header_return_types: &Vec<Type>,
    ) -> Result<TypedStatement<'ast, T>, Error> {
        let pos = stat.pos();

        match stat.value {
            Statement::Return(list) => {
                let mut expression_list_checked = vec![];
                for e in list.value.expressions {
                    let e_checked = self.check_expression(e)?;
                    expression_list_checked.push(e_checked);
                }

                let return_statement_types: Vec<Type> = expression_list_checked
                    .iter()
                    .map(|e| e.get_type())
                    .collect();

                match return_statement_types == *header_return_types {
                    true => Ok(TypedStatement::Return(expression_list_checked)),
                    false => Err(Error {
                        pos: Some(pos),
                        message: format!(
                            "Expected ({}) in return statement, found ({})",
                            header_return_types
                                .iter()
                                .map(|t| t.to_string())
                                .collect::<Vec<_>>()
                                .join(", "),
                            return_statement_types
                                .iter()
                                .map(|t| t.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    }),
                }
            }
            Statement::Declaration(var) => match self.insert_scope(var.clone().value) {
                true => Ok(TypedStatement::Declaration(var.value.into())),
                false => Err(Error {
                    pos: Some(pos),
                    message: format!("Duplicate declaration for variable named {}", var.value.id),
                }),
            },
            Statement::Definition(assignee, expr) => {
                // we create multidef when rhs is a function call to benefit from inference
                // check rhs is not a function call here
                match expr.value {
					Expression::FunctionCall(..) => panic!("Parser should not generate Definition where the right hand side is a FunctionCall"),
					_ => {}
				}

                // check the expression to be assigned
                let checked_expr = self.check_expression(expr)?;
                let expression_type = checked_expr.get_type();

                // check that the assignee is declared and is well formed
                let var = self.check_assignee(assignee)?;

                let var_type = var.get_type();

                // make sure the assignee has the same type as the rhs
                match var_type == expression_type {
                    true => Ok(TypedStatement::Definition(var, checked_expr)),
                    false => Err(Error {
                        pos: Some(pos),
                        message: format!(
                            "Expression {} of type {} cannot be assigned to {} of type {}",
                            checked_expr, expression_type, var, var_type
                        ),
                    }),
                }
            }
            Statement::Condition(lhs, rhs) => {
                let checked_lhs = self.check_expression(lhs)?;
                let checked_rhs = self.check_expression(rhs)?;

                match (checked_lhs.clone(), checked_rhs.clone()) {
                    (ref l, ref r) if r.get_type() == l.get_type() => {
                        Ok(TypedStatement::Condition(checked_lhs, checked_rhs))
                    }
                    (e1, e2) => Err(Error {
                        pos: Some(pos),
                        message: format!(
                            "Cannot compare {} of type {:?} to {} of type {:?}",
                            checked_lhs,
                            e1.get_type(),
                            checked_rhs,
                            e2.get_type(),
                        ),
                    }),
                }
            }
            Statement::For(var, from, to, statements) => {
                self.enter_scope();

                self.check_for_var(&var)?;

                self.insert_scope(var.clone().value);

                let mut checked_statements = vec![];

                for stat in statements {
                    let checked_stat = self.check_statement(stat, header_return_types)?;
                    checked_statements.push(checked_stat);
                }

                self.exit_scope();
                Ok(TypedStatement::For(
                    var.value.into(),
                    from,
                    to,
                    checked_statements,
                ))
            }
            Statement::MultipleDefinition(assignees, rhs) => {
                match rhs.value {
                    // Right side has to be a function call
                    Expression::FunctionCall(fun_id, arguments) => {
                        // find lhs types
                        let mut vars_types: Vec<Option<Type>> = vec![];
                        let mut var_names = vec![];
                        for assignee in assignees {
                            let (name, t) = match assignee.value {
                    			Assignee::Identifier(name) => {
                    				Ok((name, match self.get_scope(&name) {
					            		None => None,
					            		Some(sv) => Some(sv.id.get_type())
					            	}))
                    			}
                    			ref a => Err(Error {
                                    pos: Some(pos),
 message: format!("Left hand side of function return assignment must be a list of identifiers, found {}", a)})
                    		}?;
                            vars_types.push(t);
                            var_names.push(name);
                        }
                        // find arguments types
                        let mut arguments_checked = vec![];
                        for arg in arguments {
                            let arg_checked = self.check_expression(arg)?;
                            arguments_checked.push(arg_checked);
                        }

                        let arguments_types =
                            arguments_checked.iter().map(|a| a.get_type()).collect();

                        let query = FunctionQuery::new(&fun_id, &arguments_types, &vars_types);
                        let candidates = self.find_candidates(&query);

                        match candidates.len() {
                    		// the function has to be defined
                    		1 => {
                    			let f = &candidates[0];

                                // we can infer the left hand side to be typed as the return values
                    			let lhs: Vec<_> = var_names.iter().enumerate().map(|(index, name)|
                    				Variable::new(*name, f.signature.outputs[index].clone())
                    			).collect();

                                let assignees: Vec<_> = lhs.iter().map(|v| v.clone().into()).collect();

                                let call = TypedExpressionList::FunctionCall(f.id.to_string(), arguments_checked, f.signature.outputs.clone());

                                for var in lhs {
                                    self.insert_scope(var);
                                }

                                Ok(TypedStatement::MultipleDefinition(assignees, call))
                    		},
                    		0 => Err(Error {                         pos: Some(pos),
 message: format!("Function definition for function {} with signature {} not found.", fun_id, query) }),
                    		_ => Err(Error {                         pos: Some(pos),
 message: format!("Function call for function {} with arguments {:?} is ambiguous.", fun_id, arguments_types) }),
                    	}
                    }
                    _ => Err(Error {
                        pos: Some(pos),
                        message: format!("{} should be a FunctionCall", rhs),
                    }),
                }
            }
        }
    }

    fn check_assignee<T: Field>(
        &mut self,
        assignee: AssigneeNode<'ast, T>,
    ) -> Result<TypedAssignee<'ast, T>, Error> {
        let pos = assignee.pos();
        // check that the assignee is declared
        match assignee.value {
            Assignee::Identifier(variable_name) => match self.get_scope(&variable_name) {
                Some(var) => Ok(TypedAssignee::Identifier(
                    crate::typed_absy::Variable::with_id_and_type(
                        variable_name.into(),
                        var.id.get_type(),
                    ),
                )),
                None => Err(Error {
                    pos: Some(assignee.pos()),
                    message: format!("Undeclared variable: {:?}", variable_name),
                }),
            },
            Assignee::ArrayElement(box assignee, box index) => {
                let checked_assignee = self.check_assignee(assignee)?;
                let checked_index = match index {
                    RangeOrExpression::Expression(e) => self.check_expression(e)?,
                    r => unimplemented!(
                        "Using slices in assignments is not supported yet, found {}",
                        r
                    ),
                };

                let checked_typed_index = match checked_index {
                    TypedExpression::FieldElement(e) => Ok(e),
                    e => Err(Error {
                        pos: Some(pos),

                        message: format!(
                            "Expected array {} index to have type field, found {}",
                            checked_assignee,
                            e.get_type()
                        ),
                    }),
                }?;

                Ok(TypedAssignee::ArrayElement(
                    box checked_assignee,
                    box checked_typed_index,
                ))
            }
        }
    }

    fn check_spread_or_expression<T: Field>(
        &mut self,
        spread_or_expression: SpreadOrExpression<'ast, T>,
    ) -> Result<Vec<TypedExpression<'ast, T>>, Error> {
        match spread_or_expression {
            SpreadOrExpression::Spread(s) => {
                let pos = s.pos();

                let checked_expression = self.check_expression(s.value.expression)?;
                match checked_expression {
                    TypedExpression::FieldElementArray(e) => {
                        let size = e.size();
                        Ok((0..size)
                            .map(|i| {
                                FieldElementExpression::Select(
                                    box e.clone(),
                                    box FieldElementExpression::Number(T::from(i)),
                                )
                                .into()
                            })
                            .collect())
                    }
                    e => Err(Error {
                        pos: Some(pos),

                        message: format!(
                            "Expected spread operator to apply on field element array, found {}",
                            e.get_type()
                        ),
                    }),
                }
            }
            SpreadOrExpression::Expression(e) => self.check_expression(e).map(|r| vec![r]),
        }
    }

    fn check_expression<T: Field>(
        &mut self,
        expr: ExpressionNode<'ast, T>,
    ) -> Result<TypedExpression<'ast, T>, Error> {
        let pos = expr.pos();

        match expr.value {
            Expression::BooleanConstant(b) => Ok(BooleanExpression::Value(b).into()),
            Expression::Identifier(name) => {
                // check that `id` is defined in the scope
                match self.get_scope(&name) {
                    Some(v) => match v.id.get_type() {
                        Type::Boolean => Ok(BooleanExpression::Identifier(name.into()).into()),
                        Type::FieldElement => {
                            Ok(FieldElementExpression::Identifier(name.into()).into())
                        }
                        Type::FieldElementArray(n) => {
                            Ok(FieldElementArrayExpression::Identifier(n, name.into()).into())
                        }
                    },
                    None => Err(Error {
                        pos: Some(pos),
                        message: format!("Identifier \"{}\" is undefined", name),
                    }),
                }
            }
            Expression::Add(box e1, box e2) => {
                let e1_checked = self.check_expression(e1)?;
                let e2_checked = self.check_expression(e2)?;

                match (e1_checked, e2_checked) {
                    (TypedExpression::FieldElement(e1), TypedExpression::FieldElement(e2)) => {
                        Ok(FieldElementExpression::Add(box e1, box e2).into())
                    }
                    (t1, t2) => Err(Error {
                        pos: Some(pos),

                        message: format!(
                            "Expected only field elements, found {:?}, {:?}",
                            t1.get_type(),
                            t2.get_type()
                        ),
                    }),
                }
            }
            Expression::Sub(box e1, box e2) => {
                let e1_checked = self.check_expression(e1)?;
                let e2_checked = self.check_expression(e2)?;

                match (e1_checked, e2_checked) {
                    (TypedExpression::FieldElement(e1), TypedExpression::FieldElement(e2)) => {
                        Ok(FieldElementExpression::Sub(box e1, box e2).into())
                    }
                    (t1, t2) => Err(Error {
                        pos: Some(pos),

                        message: format!(
                            "Expected only field elements, found {:?}, {:?}",
                            t1.get_type(),
                            t2.get_type()
                        ),
                    }),
                }
            }
            Expression::Mult(box e1, box e2) => {
                let e1_checked = self.check_expression(e1)?;
                let e2_checked = self.check_expression(e2)?;

                match (e1_checked, e2_checked) {
                    (TypedExpression::FieldElement(e1), TypedExpression::FieldElement(e2)) => {
                        Ok(FieldElementExpression::Mult(box e1, box e2).into())
                    }
                    (t1, t2) => Err(Error {
                        pos: Some(pos),

                        message: format!(
                            "Expected only field elements, found {:?}, {:?}",
                            t1.get_type(),
                            t2.get_type()
                        ),
                    }),
                }
            }
            Expression::Div(box e1, box e2) => {
                let e1_checked = self.check_expression(e1)?;
                let e2_checked = self.check_expression(e2)?;

                match (e1_checked, e2_checked) {
                    (TypedExpression::FieldElement(e1), TypedExpression::FieldElement(e2)) => {
                        Ok(FieldElementExpression::Div(box e1, box e2).into())
                    }
                    (t1, t2) => Err(Error {
                        pos: Some(pos),

                        message: format!(
                            "Expected only field elements, found {:?}, {:?}",
                            t1.get_type(),
                            t2.get_type()
                        ),
                    }),
                }
            }
            Expression::Pow(box e1, box e2) => {
                let e1_checked = self.check_expression(e1)?;
                let e2_checked = self.check_expression(e2)?;

                match (e1_checked, e2_checked) {
                    (TypedExpression::FieldElement(e1), TypedExpression::FieldElement(e2)) => Ok(
                        TypedExpression::FieldElement(FieldElementExpression::Pow(box e1, box e2)),
                    ),
                    (t1, t2) => Err(Error {
                        pos: Some(pos),

                        message: format!(
                            "Expected only field elements, found {:?}, {:?}",
                            t1.get_type(),
                            t2.get_type()
                        ),
                    }),
                }
            }
            Expression::IfElse(box condition, box consequence, box alternative) => {
                let condition_checked = self.check_expression(condition)?;
                let consequence_checked = self.check_expression(consequence)?;
                let alternative_checked = self.check_expression(alternative)?;

                match condition_checked {
                    TypedExpression::Boolean(condition) => {
                        let consequence_type = consequence_checked.get_type();
                        let alternative_type = alternative_checked.get_type();
                        match consequence_type == alternative_type {
                            true => match (consequence_checked, alternative_checked) {
                                (TypedExpression::FieldElement(consequence), TypedExpression::FieldElement(alternative)) => {
                                    Ok(FieldElementExpression::IfElse(box condition, box consequence, box alternative).into())
                                },
                                (TypedExpression::FieldElementArray(consequence), TypedExpression::FieldElementArray(alternative)) => {
                                    Ok(FieldElementArrayExpression::IfElse(box condition, box consequence, box alternative).into())
                                },
                                _ => unimplemented!()
                            }
                            false => Err(Error {
                                pos: Some(pos),
                                message: format!("{{consequence}} and {{alternative}} in `if/else` expression should have the same type, found {}, {}", consequence_type, alternative_type)
                            })
                        }
                    }
                    c => Err(Error {
                        pos: Some(pos),
                        message: format!(
                            "{{condition}} after `if` should be a boolean, found {}",
                            c.get_type()
                        ),
                    }),
                }
            }
            Expression::FieldConstant(n) => Ok(FieldElementExpression::Number(n).into()),
            Expression::FunctionCall(fun_id, arguments) => {
                // check the arguments
                let mut arguments_checked = vec![];
                for arg in arguments {
                    let arg_checked = self.check_expression(arg)?;
                    arguments_checked.push(arg_checked);
                }

                let mut arguments_types = vec![];
                for arg in arguments_checked.iter() {
                    arguments_types.push(arg.get_type());
                }

                // outside of multidef, function calls must have a single return value
                // we use type inference to determine the type of the return, so we don't specify it
                let query = FunctionQuery::new(&fun_id, &arguments_types, &vec![None]);

                let candidates = self.find_candidates(&query);

                match candidates.len() {
                    // the function has to be defined
                    1 => {
                        let f = &candidates[0];
                        // the return count has to be 1
                        match f.signature.outputs.len() {
                            1 => match f.signature.outputs[0] {
                                Type::FieldElement => Ok(FieldElementExpression::FunctionCall(
                                    f.id.to_string(),
                                    arguments_checked,
                                )
                                .into()),
                                Type::FieldElementArray(size) => {
                                    Ok(FieldElementArrayExpression::FunctionCall(
                                        size,
                                        f.id.to_string(),
                                        arguments_checked,
                                    )
                                    .into())
                                }
                                _ => unimplemented!(),
                            },
                            n => Err(Error {
                                pos: Some(pos),

                                message: format!(
                                    "{} returns {} values but is called outside of a definition",
                                    f.id, n
                                ),
                            }),
                        }
                    }
                    0 => Err(Error {
                        pos: Some(pos),

                        message: format!(
                            "Function definition for function {} with signature {} not found.",
                            fun_id, query
                        ),
                    }),
                    _ => panic!("duplicate definition should have been caught before the call"),
                }
            }
            Expression::Lt(box e1, box e2) => {
                let e1_checked = self.check_expression(e1)?;
                let e2_checked = self.check_expression(e2)?;
                match (e1_checked, e2_checked) {
                    (TypedExpression::FieldElement(e1), TypedExpression::FieldElement(e2)) => {
                        Ok(BooleanExpression::Lt(box e1, box e2).into())
                    }
                    (e1, e2) => Err(Error {
                        pos: Some(pos),
                        message: format!(
                            "Cannot compare {} of type {} to {} of type {}",
                            e1,
                            e1.get_type(),
                            e2,
                            e2.get_type()
                        ),
                    }),
                }
            }
            Expression::Le(box e1, box e2) => {
                let e1_checked = self.check_expression(e1)?;
                let e2_checked = self.check_expression(e2)?;
                match (e1_checked, e2_checked) {
                    (TypedExpression::FieldElement(e1), TypedExpression::FieldElement(e2)) => {
                        Ok(BooleanExpression::Le(box e1, box e2).into())
                    }
                    (e1, e2) => Err(Error {
                        pos: Some(pos),
                        message: format!(
                            "Cannot compare {} of type {} to {} of type {}",
                            e1,
                            e1.get_type(),
                            e2,
                            e2.get_type()
                        ),
                    }),
                }
            }
            Expression::Eq(box e1, box e2) => {
                let e1_checked = self.check_expression(e1)?;
                let e2_checked = self.check_expression(e2)?;
                match (e1_checked, e2_checked) {
                    (TypedExpression::FieldElement(e1), TypedExpression::FieldElement(e2)) => {
                        Ok(BooleanExpression::Eq(box e1, box e2).into())
                    }
                    (e1, e2) => Err(Error {
                        pos: Some(pos),
                        message: format!(
                            "Cannot compare {} of type {} to {} of type {}",
                            e1,
                            e1.get_type(),
                            e2,
                            e2.get_type()
                        ),
                    }),
                }
            }
            Expression::Ge(box e1, box e2) => {
                let e1_checked = self.check_expression(e1)?;
                let e2_checked = self.check_expression(e2)?;
                match (e1_checked, e2_checked) {
                    (TypedExpression::FieldElement(e1), TypedExpression::FieldElement(e2)) => {
                        Ok(BooleanExpression::Ge(box e1, box e2).into())
                    }
                    (e1, e2) => Err(Error {
                        pos: Some(pos),
                        message: format!(
                            "Cannot compare {} of type {} to {} of type {}",
                            e1,
                            e1.get_type(),
                            e2,
                            e2.get_type()
                        ),
                    }),
                }
            }
            Expression::Gt(box e1, box e2) => {
                let e1_checked = self.check_expression(e1)?;
                let e2_checked = self.check_expression(e2)?;
                match (e1_checked, e2_checked) {
                    (TypedExpression::FieldElement(e1), TypedExpression::FieldElement(e2)) => {
                        Ok(BooleanExpression::Gt(box e1, box e2).into())
                    }
                    (e1, e2) => Err(Error {
                        pos: Some(pos),
                        message: format!(
                            "Cannot compare {} of type {} to {} of type {}",
                            e1,
                            e1.get_type(),
                            e2,
                            e2.get_type()
                        ),
                    }),
                }
            }
            Expression::Select(box array, box index) => {
                let array = self.check_expression(array)?;
                //let index = self.check_range_or_expression(index)?;

                match index {
                    RangeOrExpression::Range(r) => match array {
                        TypedExpression::FieldElementArray(array) => {
                            let array_size = array.size();

                            let from = r
                                .value
                                .from
                                .map(|v| v.to_dec_string().parse::<usize>().unwrap())
                                .unwrap_or(0);

                            let to = r
                                .value
                                .to
                                .map(|v| v.to_dec_string().parse::<usize>().unwrap())
                                .unwrap_or(array_size);

                            match (from, to, array_size) {
                                (f, _, s) if f > s => Err(Error {
                                    pos: Some(pos),
                                    message: format!(
                                        "Lower range bound {} is out of array bounds [0, {}]",
                                        f, s,
                                    ),
                                }),
                                (_, t, s) if t > s => Err(Error {
                                    pos: Some(pos),
                                    message: format!(
                                        "Higher range bound {} is out of array bounds [0, {}]",
                                        t, s,
                                    ),
                                }),
                                (f, t, _) if f > t => Err(Error {
                                    pos: Some(pos),
                                    message: format!(
                                        "Lower range bound {} is larger than higher range bound {}",
                                        f, t,
                                    ),
                                }),
                                (f, t, _) => Ok(FieldElementArrayExpression::Value(
                                    t - f,
                                    (f..t)
                                        .map(|i| {
                                            FieldElementExpression::Select(
                                                box array.clone(),
                                                box FieldElementExpression::Number(T::from(i)),
                                            )
                                        })
                                        .collect(),
                                )
                                .into()),
                            }
                        }
                        _ => panic!(""),
                    },
                    RangeOrExpression::Expression(e) => match (array, self.check_expression(e)?) {
                        (
                            TypedExpression::FieldElementArray(a),
                            TypedExpression::FieldElement(i),
                        ) => Ok(FieldElementExpression::Select(box a, box i).into()),
                        (a, e) => Err(Error {
                            pos: Some(pos),
                            message: format!(
                                "Cannot access element {} on expression of type {}",
                                e,
                                a.get_type()
                            ),
                        }),
                    },
                }
            }
            Expression::InlineArray(expressions) => {
                // we should have at least one expression
                let size = expressions.len();
                assert!(size > 0);
                // check each expression, getting its type
                let mut expressions_checked = vec![];
                for e in expressions {
                    let e_checked = self.check_spread_or_expression(e)?;
                    expressions_checked.extend(e_checked);
                }

                // we infer the type to be the type of the first element
                let inferred_type = expressions_checked.get(0).unwrap().get_type();

                match inferred_type {
                    Type::FieldElement => {
                        // we check all expressions have that same type
                        let mut unwrapped_expressions = vec![];

                        for e in expressions_checked {
                            let unwrapped_e = match e {
                                TypedExpression::FieldElement(e) => Ok(e),
                                e => Err(Error {
                                    pos: Some(pos),

                                    message: format!(
                                        "Expected {} to have type {}, but type is {}",
                                        e,
                                        inferred_type,
                                        e.get_type()
                                    ),
                                }),
                            }?;
                            unwrapped_expressions.push(unwrapped_e);
                        }

                        Ok(FieldElementArrayExpression::Value(
                            unwrapped_expressions.len(),
                            unwrapped_expressions,
                        )
                        .into())
                    }
                    _ => Err(Error {
                        pos: Some(pos),

                        message: format!(
                            "Only arrays of {} are supported, found {}",
                            Type::FieldElement,
                            inferred_type
                        ),
                    }),
                }
            }
            Expression::And(box e1, box e2) => {
                let e1_checked = self.check_expression(e1)?;
                let e2_checked = self.check_expression(e2)?;
                match (e1_checked, e2_checked) {
                    (TypedExpression::Boolean(e1), TypedExpression::Boolean(e2)) => {
                        Ok(BooleanExpression::And(box e1, box e2).into())
                    }
                    (e1, e2) => Err(Error {
                        pos: Some(pos),

                        message: format!(
                            "cannot apply boolean operators to {} and {}",
                            e1.get_type(),
                            e2.get_type()
                        ),
                    }),
                }
            }
            Expression::Or(box e1, box e2) => {
                let e1_checked = self.check_expression(e1)?;
                let e2_checked = self.check_expression(e2)?;
                match (e1_checked, e2_checked) {
                    (TypedExpression::Boolean(e1), TypedExpression::Boolean(e2)) => {
                        Ok(BooleanExpression::Or(box e1, box e2).into())
                    }
                    (e1, e2) => Err(Error {
                        pos: Some(pos),

                        message: format!("cannot compare {} to {}", e1.get_type(), e2.get_type()),
                    }),
                }
            }
            Expression::Not(box e) => {
                let e_checked = self.check_expression(e)?;
                match e_checked {
                    TypedExpression::Boolean(e) => Ok(BooleanExpression::Not(box e).into()),
                    e => Err(Error {
                        pos: Some(pos),

                        message: format!("cannot negate {}", e.get_type()),
                    }),
                }
            }
        }
    }

    fn get_scope(&self, variable_name: &Identifier<'ast>) -> Option<&ScopedVariable> {
        self.scope.get(&ScopedVariable {
            id: Variable::new(*variable_name, Type::FieldElement),
            level: 0,
        })
    }

    fn insert_scope(&mut self, v: Variable<'ast>) -> bool {
        self.scope.insert(ScopedVariable {
            id: v,
            level: self.level,
        })
    }

    fn find_candidates(&self, query: &FunctionQuery<'ast>) -> Vec<FunctionDeclaration> {
        query.match_funcs(&self.functions)
    }

    fn enter_scope(&mut self) -> () {
        self.level += 1;
    }

    fn exit_scope(&mut self) -> () {
        let current_level = self.level;
        self.scope
            .retain(|ref scoped_variable| scoped_variable.level < current_level);
        self.level -= 1;
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use absy::parameter::Parameter;
    // use zokrates_field::field::FieldPrime;

    // pub fn new_with_args(
    //     scope: HashSet<ScopedVariable>,
    //     level: usize,
    //     functions: HashSet<FunctionDeclaration>,
    // ) -> Checker {
    //     Checker {
    //         scope: scope,
    //         functions: functions,
    //         level: level,
    //     }
    // }

    // #[test]
    // fn undefined_variable_in_statement() {
    //     // a = b
    //     // b undefined
    //     let statement: Statement<FieldPrime> = Statement::Definition(
    //         Assignee::Identifier(String::from("a")),
    //         Expression::Identifier(String::from("b")),
    //     );
    //     let mut checker = Checker::new();
    //     assert_eq!(
    //         checker.check_statement(&statement, &vec![]),
    //         Err(Error {
    //             message: "b is undefined".to_string()
    //         })
    //     );
    // }

    // #[test]
    // fn defined_variable_in_statement() {
    //     // a = b
    //     // b defined
    //     let statement: Statement<FieldPrime> = Statement::Definition(
    //         Assignee::Identifier(String::from("a")),
    //         Expression::Identifier(String::from("b")),
    //     );

    //     let mut scope = HashSet::new();
    //     scope.insert(ScopedVariable {
    //         id: Variable::field_element("a"),
    //         level: 0,
    //     });
    //     scope.insert(ScopedVariable {
    //         id: Variable::field_element("b"),
    //         level: 0,
    //     });
    //     let mut checker = new_with_args(scope, 1, HashSet::new());
    //     assert_eq!(
    //         checker.check_statement(&statement, &vec![]),
    //         Ok(TypedStatement::Definition(
    //             TypedAssignee::Identifier(Variable::field_element("a")),
    //             FieldElementExpression::Identifier(String::from("b")).into()
    //         ))
    //     );
    // }

    // #[test]
    // fn declared_in_other_function() {
    //     // def foo():
    //     //   field a = 1
    //     // def bar():
    //     //   return a
    //     // should fail
    //     let foo_args = Vec::<Parameter>::new();
    //     let mut foo_statements = Vec::<Statement<FieldPrime>>::new();
    //     foo_statements.push(Statement::Declaration(Variable::field_element("a")));
    //     foo_statements.push(Statement::Definition(
    //         Assignee::Identifier(String::from("a")),
    //         Expression::FieldConstant(FieldPrime::from(1)),
    //     ));
    //     let foo = Function {
    //         id: "foo".to_string(),
    //         arguments: foo_args,
    //         statements: foo_statements,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let bar_args = Vec::<Parameter>::new();
    //     let mut bar_statements = Vec::<Statement<FieldPrime>>::new();
    //     bar_statements.push(Statement::Return(ExpressionList {
    //         expressions: vec![Expression::Identifier(String::from("a"))],
    //     }));
    //     let bar = Function {
    //         id: "bar".to_string(),
    //         arguments: bar_args,
    //         statements: bar_statements,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let mut funcs = Vec::<Function<FieldPrime>>::new();
    //     funcs.push(foo);
    //     funcs.push(bar);
    //     let prog = Prog {
    //         functions: funcs,
    //         imports: vec![],
    //         imported_functions: vec![],
    //     };

    //     let mut checker = Checker::new();
    //     assert_eq!(
    //         checker.check_program(prog),
    //         Err(Error {
    //             message: "a is undefined".to_string()
    //         })
    //     );
    // }

    // #[test]
    // fn declared_in_two_scopes() {
    //     // def foo():
    //     //   a = 1
    //     // def bar():
    //     //   a = 2
    //     //   return a
    //     // def main():
    //     //   return 1
    //     // should pass
    //     let foo_args = vec![];
    //     let foo_statements = vec![
    //         Statement::Declaration(Variable::field_element("a")),
    //         Statement::Definition(
    //             Assignee::Identifier(String::from("a")),
    //             Expression::FieldConstant(FieldPrime::from(1)),
    //         ),
    //     ];

    //     let foo = Function {
    //         id: "foo".to_string(),
    //         arguments: foo_args,
    //         statements: foo_statements,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let bar_args = Vec::<Parameter>::new();
    //     let bar_statements = vec![
    //         Statement::Declaration(Variable::field_element("a")),
    //         Statement::Definition(
    //             Assignee::Identifier(String::from("a")),
    //             Expression::FieldConstant(FieldPrime::from(2)),
    //         ),
    //         Statement::Return(ExpressionList {
    //             expressions: vec![Expression::Identifier(String::from("a"))],
    //         }),
    //     ];
    //     let bar = Function {
    //         id: "bar".to_string(),
    //         arguments: bar_args,
    //         statements: bar_statements,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let main_args = vec![];
    //     let main_statements = vec![Statement::Return(ExpressionList {
    //         expressions: vec![Expression::FieldConstant(FieldPrime::from(1))],
    //     })];

    //     let main = Function {
    //         id: "main".to_string(),
    //         arguments: main_args,
    //         statements: main_statements,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let mut funcs = Vec::<Function<FieldPrime>>::new();
    //     funcs.push(foo);
    //     funcs.push(bar);
    //     funcs.push(main);
    //     let prog = Prog {
    //         functions: funcs,
    //         imports: vec![],
    //         imported_functions: vec![],
    //     };

    //     let mut checker = Checker::new();
    //     assert!(checker.check_program(prog).is_ok());
    // }

    // #[test]
    // fn for_index_after_end() {
    //     // def foo():
    //     //   for field i in 0..10 do
    //     //   endfor
    //     //   return i
    //     // should fail
    //     let mut foo_statements = Vec::<Statement<FieldPrime>>::new();
    //     foo_statements.push(Statement::For(
    //         Variable::field_element("i"),
    //         FieldPrime::from(0),
    //         FieldPrime::from(10),
    //         Vec::<Statement<FieldPrime>>::new(),
    //     ));
    //     foo_statements.push(Statement::Return(ExpressionList {
    //         expressions: vec![Expression::Identifier(String::from("i"))],
    //     }));
    //     let foo = Function {
    //         id: "foo".to_string(),
    //         arguments: Vec::<Parameter>::new(),
    //         statements: foo_statements,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let mut checker = Checker::new();
    //     assert_eq!(
    //         checker.check_function(&foo),
    //         Err(Error {
    //             message: "i is undefined".to_string()
    //         })
    //     );
    // }

    // #[test]
    // fn for_index_in_for() {
    //     // def foo():
    //     //   for i in 0..10 do
    //     //     a = i
    //     //   endfor
    //     // should pass
    //     let mut foo_statements = Vec::<Statement<FieldPrime>>::new();
    //     let mut for_statements = Vec::<Statement<FieldPrime>>::new();
    //     for_statements.push(Statement::Declaration(Variable::field_element("a")));
    //     for_statements.push(Statement::Definition(
    //         Assignee::Identifier(String::from("a")),
    //         Expression::Identifier(String::from("i")),
    //     ));
    //     foo_statements.push(Statement::For(
    //         Variable::field_element("i"),
    //         FieldPrime::from(0),
    //         FieldPrime::from(10),
    //         for_statements,
    //     ));

    //     let mut foo_statements_checked = Vec::<TypedStatement<FieldPrime>>::new();
    //     let mut for_statements_checked = Vec::<TypedStatement<FieldPrime>>::new();

    //     for_statements_checked.push(TypedStatement::Declaration(Variable::field_element("a")));

    //     for_statements_checked.push(TypedStatement::Definition(
    //         TypedAssignee::Identifier(Variable::field_element("a")),
    //         FieldElementExpression::Identifier(String::from("i")).into(),
    //     ));

    //     foo_statements_checked.push(TypedStatement::For(
    //         Variable::field_element("i"),
    //         FieldPrime::from(0),
    //         FieldPrime::from(10),
    //         for_statements_checked,
    //     ));

    //     let foo = Function {
    //         id: "foo".to_string(),
    //         arguments: Vec::<Parameter>::new(),
    //         statements: foo_statements,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let foo_checked = TypedFunction {
    //         id: "foo".to_string(),
    //         arguments: Vec::<Parameter>::new(),
    //         statements: foo_statements_checked,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let mut checker = Checker::new();
    //     assert_eq!(checker.check_function(&foo), Ok(foo_checked));
    // }

    // #[test]
    // fn arity_mismatch() {
    //     // def foo():
    //     //   return 1, 2
    //     // def bar():
    //     //   field c = foo()
    //     // should fail
    //     let bar_statements: Vec<Statement<FieldPrime>> = vec![
    //         Statement::Declaration(Variable::field_element("a")),
    //         Statement::MultipleDefinition(
    //             vec![Assignee::Identifier(String::from("a"))],
    //             Expression::FunctionCall("foo".to_string(), vec![]),
    //         ),
    //     ];

    //     let foo = FunctionDeclaration {
    //         id: "foo".to_string(),
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement, Type::FieldElement],
    //         },
    //     };

    //     let mut functions = HashSet::new();
    //     functions.insert(foo);

    //     let bar = Function {
    //         id: "bar".to_string(),
    //         arguments: vec![],
    //         statements: bar_statements,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let mut checker = new_with_args(HashSet::new(), 0, functions);
    //     assert_eq!(
    //         checker.check_function(&bar),
    //         Err(Error {
    //             message:
    //                 "Function definition for function foo with signature () -> (field) not found."
    //                     .to_string()
    //         })
    //     );
    // }

    // #[test]
    // fn multi_return_outside_multidef() {
    //     // def foo():
    //     //   return 1, 2
    //     // def bar():
    //     //   4 == foo()
    //     // should fail
    //     let bar_statements: Vec<Statement<FieldPrime>> = vec![Statement::Condition(
    //         Expression::FieldConstant(FieldPrime::from(2)),
    //         Expression::FunctionCall("foo".to_string(), vec![]),
    //     )];

    //     let foo = FunctionDeclaration {
    //         id: "foo".to_string(),
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement, Type::FieldElement],
    //         },
    //     };

    //     let mut functions = HashSet::new();
    //     functions.insert(foo);

    //     let bar = Function {
    //         id: "bar".to_string(),
    //         arguments: vec![],
    //         statements: bar_statements,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let mut checker = new_with_args(HashSet::new(), 0, functions);
    //     assert_eq!(
    //         checker.check_function(&bar),
    //         Err(Error {
    //             message: "Function definition for function foo with signature () -> (_) not found."
    //                 .to_string()
    //         })
    //     );
    // }

    // #[test]
    // fn function_undefined_in_multidef() {
    //     // def bar():
    //     //   field a = foo()
    //     // should fail
    //     let bar_statements: Vec<Statement<FieldPrime>> = vec![
    //         Statement::Declaration(Variable::field_element("a")),
    //         Statement::MultipleDefinition(
    //             vec![Assignee::Identifier(String::from("a"))],
    //             Expression::FunctionCall("foo".to_string(), vec![]),
    //         ),
    //     ];

    //     let bar = Function {
    //         id: "bar".to_string(),
    //         arguments: vec![],
    //         statements: bar_statements,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let mut checker = new_with_args(HashSet::new(), 0, HashSet::new());
    //     assert_eq!(
    //         checker.check_function(&bar),
    //         Err(Error {
    //             message:
    //                 "Function definition for function foo with signature () -> (field) not found."
    //                     .to_string()
    //         })
    //     );
    // }

    // #[test]
    // fn undefined_variable_in_multireturn_call() {
    //     // def foo(x):
    //     // 	return 1, 2
    //     // def main():
    //     // 	a, b = foo(x)
    //     // 	return 1
    //     // should fail

    //     let foo_statements: Vec<Statement<FieldPrime>> = vec![Statement::Return(ExpressionList {
    //         expressions: vec![
    //             Expression::FieldConstant(FieldPrime::from(1)),
    //             Expression::FieldConstant(FieldPrime::from(2)),
    //         ],
    //     })];

    //     let foo = Function {
    //         id: "foo".to_string(),
    //         arguments: vec![Parameter {
    //             id: Variable::field_element("x"),
    //             private: false,
    //         }],
    //         statements: foo_statements,
    //         signature: Signature {
    //             inputs: vec![Type::FieldElement],
    //             outputs: vec![Type::FieldElement, Type::FieldElement],
    //         },
    //     };

    //     let main_statements: Vec<Statement<FieldPrime>> = vec![
    //         Statement::Declaration(Variable::field_element("a")),
    //         Statement::Declaration(Variable::field_element("b")),
    //         Statement::MultipleDefinition(
    //             vec![
    //                 Assignee::Identifier(String::from("a")),
    //                 Assignee::Identifier(String::from("b")),
    //             ],
    //             Expression::FunctionCall(
    //                 "foo".to_string(),
    //                 vec![Expression::Identifier("x".to_string())],
    //             ),
    //         ),
    //         Statement::Return(ExpressionList {
    //             expressions: vec![Expression::FieldConstant(FieldPrime::from(1))],
    //         }),
    //     ];

    //     let main = Function {
    //         id: "main".to_string(),
    //         arguments: vec![],
    //         statements: main_statements,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement, Type::FieldElement],
    //         },
    //     };

    //     let program = Prog {
    //         functions: vec![foo, main],
    //         imports: vec![],
    //         imported_functions: vec![],
    //     };

    //     let mut checker = new_with_args(HashSet::new(), 0, HashSet::new());
    //     assert_eq!(
    //         checker.check_program(program),
    //         Err(Error {
    //             message: "x is undefined".to_string()
    //         })
    //     );
    // }

    // #[test]
    // fn function_undefined() {
    //     // def bar():
    //     //   1 == foo()
    //     // should fail
    //     let bar_statements: Vec<Statement<FieldPrime>> = vec![Statement::Condition(
    //         Expression::FieldConstant(FieldPrime::from(1)),
    //         Expression::FunctionCall("foo".to_string(), vec![]),
    //     )];

    //     let bar = Function {
    //         id: "bar".to_string(),
    //         arguments: vec![],
    //         statements: bar_statements,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let mut checker = new_with_args(HashSet::new(), 0, HashSet::new());
    //     assert_eq!(
    //         checker.check_function(&bar),
    //         Err(Error {
    //             message: "Function definition for function foo with signature () -> (_) not found."
    //                 .to_string()
    //         })
    //     );
    // }

    // #[test]
    // fn return_undefined() {
    //     // def bar():
    //     //   return a, b
    //     // should fail
    //     let bar_statements: Vec<Statement<FieldPrime>> = vec![Statement::Return(ExpressionList {
    //         expressions: vec![
    //             Expression::Identifier("a".to_string()),
    //             Expression::Identifier("b".to_string()),
    //         ],
    //     })];

    //     let bar = Function {
    //         id: "bar".to_string(),
    //         arguments: vec![],
    //         statements: bar_statements,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement, Type::FieldElement],
    //         },
    //     };

    //     let mut checker = new_with_args(HashSet::new(), 0, HashSet::new());
    //     assert_eq!(
    //         checker.check_function(&bar),
    //         Err(Error {
    //             message: "a is undefined".to_string()
    //         })
    //     );
    // }

    // #[test]
    // fn multi_def() {
    //     // def foo():
    //     //   return 1, 2
    //     // def bar():
    //     //   field a, field b = foo()
    //     //   return a + b
    //     //
    //     // should pass
    //     let bar_statements: Vec<Statement<FieldPrime>> = vec![
    //         Statement::Declaration(Variable::field_element("a")),
    //         Statement::Declaration(Variable::field_element("b")),
    //         Statement::MultipleDefinition(
    //             vec![
    //                 Assignee::Identifier(String::from("a")),
    //                 Assignee::Identifier(String::from("b")),
    //             ],
    //             Expression::FunctionCall("foo".to_string(), vec![]),
    //         ),
    //         Statement::Return(ExpressionList {
    //             expressions: vec![Expression::Add(
    //                 box Expression::Identifier("a".to_string()),
    //                 box Expression::Identifier("b".to_string()),
    //             )],
    //         }),
    //     ];

    //     let bar_statements_checked: Vec<TypedStatement<FieldPrime>> = vec![
    //         TypedStatement::Declaration(Variable::field_element("a")),
    //         TypedStatement::Declaration(Variable::field_element("b")),
    //         TypedStatement::MultipleDefinition(
    //             vec![Variable::field_element("a"), Variable::field_element("b")],
    //             TypedExpressionList::FunctionCall(
    //                 "foo".to_string(),
    //                 vec![],
    //                 vec![Type::FieldElement, Type::FieldElement],
    //             ),
    //         ),
    //         TypedStatement::Return(vec![FieldElementExpression::Add(
    //             box FieldElementExpression::Identifier("a".to_string()),
    //             box FieldElementExpression::Identifier("b".to_string()),
    //         )
    //         .into()]),
    //     ];

    //     let foo = FunctionDeclaration {
    //         id: "foo".to_string(),
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement, Type::FieldElement],
    //         },
    //     };

    //     let mut functions = HashSet::new();
    //     functions.insert(foo);

    //     let bar = Function {
    //         id: "bar".to_string(),
    //         arguments: vec![],
    //         statements: bar_statements,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let bar_checked = TypedFunction {
    //         id: "bar".to_string(),
    //         arguments: vec![],
    //         statements: bar_statements_checked,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let mut checker = new_with_args(HashSet::new(), 0, functions);
    //     assert_eq!(checker.check_function(&bar), Ok(bar_checked));
    // }

    // #[test]
    // fn duplicate_function_declaration() {
    //     // def foo(a, b):
    //     //   return 1
    //     // def foo(c, d):
    //     //   return 2
    //     //
    //     // should fail
    //     let foo2_statements: Vec<Statement<FieldPrime>> = vec![Statement::Return(ExpressionList {
    //         expressions: vec![Expression::FieldConstant(FieldPrime::from(1))],
    //     })];

    //     let foo2_arguments = vec![
    //         Parameter {
    //             id: Variable::field_element("a"),
    //             private: true,
    //         },
    //         Parameter {
    //             id: Variable::field_element("b"),
    //             private: true,
    //         },
    //     ];

    //     let foo1 = FunctionDeclaration {
    //         id: "foo".to_string(),
    //         signature: Signature {
    //             inputs: vec![Type::FieldElement, Type::FieldElement],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let mut functions = HashSet::new();
    //     functions.insert(foo1);

    //     let foo2 = Function {
    //         id: "foo".to_string(),
    //         arguments: foo2_arguments,
    //         statements: foo2_statements,
    //         signature: Signature {
    //             inputs: vec![Type::FieldElement, Type::FieldElement],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let mut checker = new_with_args(HashSet::new(), 0, functions);
    //     assert_eq!(
    //         checker.check_function(&foo2),
    //         Err(Error {
    //             message:
    //                 "Duplicate definition for function foo with signature (field, field) -> (field)"
    //                     .to_string()
    //         })
    //     );
    // }

    // #[test]
    // fn duplicate_main_function() {
    //     // def main(a):
    //     //   return 1
    //     // def main():
    //     //   return 1
    //     //
    //     // should fail
    //     let main1_statements: Vec<Statement<FieldPrime>> =
    //         vec![Statement::Return(ExpressionList {
    //             expressions: vec![Expression::FieldConstant(FieldPrime::from(1))],
    //         })];

    //     let main1_arguments = vec![Parameter {
    //         id: Variable::field_element("a"),
    //         private: false,
    //     }];

    //     let main2_statements: Vec<Statement<FieldPrime>> =
    //         vec![Statement::Return(ExpressionList {
    //             expressions: vec![Expression::FieldConstant(FieldPrime::from(1))],
    //         })];

    //     let main2_arguments = vec![];

    //     let main1 = Function {
    //         id: "main".to_string(),
    //         arguments: main1_arguments,
    //         statements: main1_statements,
    //         signature: Signature {
    //             inputs: vec![Type::FieldElement],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let main2 = Function {
    //         id: "main".to_string(),
    //         arguments: main2_arguments,
    //         statements: main2_statements,
    //         signature: Signature {
    //             inputs: vec![],
    //             outputs: vec![Type::FieldElement],
    //         },
    //     };

    //     let prog = Prog {
    //         functions: vec![main1, main2],
    //         imports: vec![],
    //         imported_functions: vec![],
    //     };

    //     let mut checker = Checker::new();
    //     assert_eq!(
    //         checker.check_program(prog),
    //         Err(Error {
    //             message: "Only one main function allowed, found 2".to_string()
    //         })
    //     );
    // }

    // #[test]
    // fn shadowing_with_same_type() {
    //     //   field a
    //     //	 field a
    //     //
    //     // should fail

    //     let mut checker = Checker::new();
    //     let _: Result<TypedStatement<FieldPrime>, Error> = checker.check_statement(
    //         &Statement::Declaration(Variable::field_element("a")),
    //         &vec![],
    //     );
    //     let s2_checked: Result<TypedStatement<FieldPrime>, Error> = checker.check_statement(
    //         &Statement::Declaration(Variable::field_element("a")),
    //         &vec![],
    //     );
    //     assert_eq!(
    //         s2_checked,
    //         Err(Error {
    //             message: "Duplicate declaration for variable named a".to_string()
    //         })
    //     );
    // }

    // #[test]
    // fn shadowing_with_different_type() {
    //     //   field a
    //     //	 bool a
    //     //
    //     // should fail

    //     let mut checker = Checker::new();
    //     let _: Result<TypedStatement<FieldPrime>, Error> = checker.check_statement(
    //         &Statement::Declaration(Variable::field_element("a")),
    //         &vec![],
    //     );
    //     let s2_checked: Result<TypedStatement<FieldPrime>, Error> =
    //         checker.check_statement(&Statement::Declaration(Variable::boolean("a")), &vec![]);
    //     assert_eq!(
    //         s2_checked,
    //         Err(Error {
    //             message: "Duplicate declaration for variable named a".to_string()
    //         })
    //     );
    // }

    // mod assignee {
    //     use super::*;

    //     #[test]
    //     fn identifier() {
    //         // a = 42
    //         let a = Assignee::Identifier::<FieldPrime>(String::from("a"));

    //         let mut checker: Checker = Checker::new();
    //         checker
    //             .check_statement::<FieldPrime>(
    //                 &Statement::Declaration(Variable::field_element("a")),
    //                 &vec![],
    //             )
    //             .unwrap();

    //         assert_eq!(
    //             checker.check_assignee(&a),
    //             Ok(TypedAssignee::Identifier(Variable::field_element("a")))
    //         );
    //     }

    //     #[test]
    //     fn array_element() {
    //         // field[33] a
    //         // a[2] = 42
    //         let a = Assignee::ArrayElement(
    //             box Assignee::Identifier(String::from("a")),
    //             box Expression::FieldConstant(FieldPrime::from(2)),
    //         );

    //         let mut checker: Checker = Checker::new();
    //         checker
    //             .check_statement::<FieldPrime>(
    //                 &Statement::Declaration(Variable::field_array("a", 33)),
    //                 &vec![],
    //             )
    //             .unwrap();

    //         assert_eq!(
    //             checker.check_assignee(&a),
    //             Ok(TypedAssignee::ArrayElement(
    //                 box TypedAssignee::Identifier(Variable::field_array("a", 33)),
    //                 box FieldElementExpression::Number(FieldPrime::from(2)).into()
    //             ))
    //         );
    //     }
    // }
}
