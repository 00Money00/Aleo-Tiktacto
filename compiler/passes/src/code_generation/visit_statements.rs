// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::CodeGenerator;

use leo_ast::{
    AssignStatement, Block, ConditionalStatement, ConsoleFunction, ConsoleStatement, DecrementStatement,
    DefinitionStatement, Expression, FinalizeStatement, IncrementStatement, IterationStatement, Mode, ReturnStatement,
    Statement,
};

use itertools::Itertools;

impl<'a> CodeGenerator<'a> {
    fn visit_statement(&mut self, input: &'a Statement) -> String {
        match input {
            Statement::Assign(stmt) => self.visit_assign(stmt),
            Statement::Block(stmt) => self.visit_block(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Console(stmt) => self.visit_console(stmt),
            Statement::Decrement(stmt) => self.visit_decrement(stmt),
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Finalize(stmt) => self.visit_finalize(stmt),
            Statement::Increment(stmt) => self.visit_increment(stmt),
            Statement::Iteration(stmt) => self.visit_iteration(stmt),
            Statement::Return(stmt) => self.visit_return(stmt),
        }
    }

    fn visit_return(&mut self, input: &'a ReturnStatement) -> String {
        let (operand, mut expression_instructions) = self.visit_expression(&input.expression);
        // TODO: Bytecode functions have an associated output mode. Currently defaulting to private since we do not yet support this at the Leo level.
        let types = self.visit_return_type(&self.current_function.unwrap().output_type, Mode::Private);
        let instructions = operand
            .split('\n')
            .into_iter()
            .zip(types.iter())
            .map(|(operand, type_)| format!("    output {} as {};\n", operand, type_))
            .join("");

        expression_instructions.push_str(&instructions);

        expression_instructions
    }

    fn visit_definition(&mut self, _input: &'a DefinitionStatement) -> String {
        // TODO: If SSA is made optional, then conditionally enable codegen for DefinitionStatement
        // let (operand, expression_instructions) = self.visit_expression(&input.value);
        // self.variable_mapping.insert(&input.variable_name.name, operand);
        // expression_instructions
        unreachable!("DefinitionStatement's should not exist in SSA form.")
    }

    fn visit_increment(&mut self, _input: &'a IncrementStatement) -> String {
        todo!()
    }

    fn visit_decrement(&mut self, _input: &'a DecrementStatement) -> String {
        todo!()
    }

    fn visit_finalize(&mut self, _input: &'a FinalizeStatement) -> String {
        todo!()
    }

    fn visit_assign(&mut self, input: &'a AssignStatement) -> String {
        match &input.place {
            Expression::Identifier(identifier) => {
                let (operand, expression_instructions) = self.visit_expression(&input.value);
                self.variable_mapping.insert(&identifier.name, operand);
                expression_instructions
            }
            _ => unimplemented!(
                "Code generation for the left-hand side of an assignment is only implemented for `Identifier`s."
            ),
        }
    }

    fn visit_conditional(&mut self, _input: &'a ConditionalStatement) -> String {
        // TODO: Once SSA is made optional, create a Leo error informing the user to enable the SSA pass.
        unreachable!("`ConditionalStatement`s should not be in the AST at this phase of compilation.")
    }

    fn visit_iteration(&mut self, _input: &'a IterationStatement) -> String {
        // TODO: Once loop unrolling is made optional, create a Leo error informing the user to enable the loop unrolling pass..
        unreachable!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    fn visit_console(&mut self, input: &'a ConsoleStatement) -> String {
        let mut generate_assert_instruction = |name: &str, left: &'a Expression, right: &'a Expression| {
            let (left_operand, left_instructions) = self.visit_expression(left);
            let (right_operand, right_instructions) = self.visit_expression(right);
            let assert_instruction = format!("    {} {} {};\n", name, left_operand, right_operand);

            // Concatenate the instructions.
            let mut instructions = left_instructions;
            instructions.push_str(&right_instructions);
            instructions.push_str(&assert_instruction);

            instructions
        };
        match &input.function {
            ConsoleFunction::Assert(expr) => {
                let (operand, mut instructions) = self.visit_expression(expr);
                let assert_instruction = format!("    assert.eq {} true;\n", operand);

                instructions.push_str(&assert_instruction);
                instructions
            }
            ConsoleFunction::AssertEq(left, right) => generate_assert_instruction("assert.eq", left, right),
            ConsoleFunction::AssertNeq(left, right) => generate_assert_instruction("assert.neq", left, right),
        }
    }

    pub(crate) fn visit_block(&mut self, input: &'a Block) -> String {
        // For each statement in the block, visit it and add its instructions to the list.
        input.statements.iter().map(|stmt| self.visit_statement(stmt)).join("")
    }
}
