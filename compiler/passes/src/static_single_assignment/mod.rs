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

//! The Static Single Assignment pass traversed the AST and converts it into SSA form.
//! See https://en.wikipedia.org/wiki/Static_single-assignment_form for more information.
//! The pass also flattens `ConditionalStatement`s into a sequence of `AssignStatement`s.
//! The pass also rewrites complex `AssignStatement`s into simpler ones. e.g x += 1 -> x = x + 1.
//! The pass also rewrites `ReturnStatement`s into `AssignStatement`s and consolidates the returned values as a single `ReturnStatement` at the end of the function.
//!
//! Consider the following Leo code.
//! ```leo
//! function main(flag: u8, value: u8) -> u8 {
//!     if (flag == 0u8) {
//!         value += 1u8;
//!         return value;
//!     } else {
//!         value += 2u8;
//!     }
//!     return value;
//! }
//! ```
//!
//! The SSA pass produces the following code.
//! ```leo
//! function main(flag: u8, value: u8) -> u8 {
//!     let $cond$0 = flag == 0u8;
//!     let value$1 = value + 1u8;
//!     let $return$2 = value$1;
//!     let value$3 = value + 2u8;
//!     let value$4 = $cond$0 ? value$1 : value$4;
//!     let $return$5 = value$4;
//!     return $cond$0 ? $return$2 : $return$5;
//! ```

mod rename_expression;

mod rename_program;

mod rename_statement;

mod rename_table;
pub(crate) use rename_table::*;

pub mod static_single_assigner;
pub use static_single_assigner::*;

use crate::Pass;

use leo_ast::{Ast, ProgramReconstructor};
use leo_errors::{emitter::Handler, Result};

impl<'a> Pass for StaticSingleAssigner<'a> {
    type Input = (Ast, &'a Handler);
    type Output = Result<Ast>;

    fn do_pass((ast, handler): Self::Input) -> Self::Output {
        let mut reconstructor = StaticSingleAssigner::new(handler);
        let program = reconstructor.reconstruct_program(ast.into_repr());
        handler.last_err()?;

        Ok(Ast::new(program))
    }
}
