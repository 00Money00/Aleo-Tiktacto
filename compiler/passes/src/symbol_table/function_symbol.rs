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

use leo_ast::{Function, FunctionInput, Type};
use leo_span::Span;

use crate::SymbolTable;

/// Metadata associated with the finalize block.
#[derive(Debug, Clone)]
pub struct FinalizeData {
    /// The inputs to the finalize block.
    pub(crate) input: Vec<FunctionInput>,
    /// The output type of the finalize block.
    pub(crate) output_type: Type,
    /// The span of the finalize block.
    pub(crate) span: Span,
}

/// An entry for a function in the symbol table.
#[derive(Clone, Debug)]
pub struct FunctionSymbol {
    /// The index associated with the scope in the parent symbol table.
    pub(crate) id: usize,
    /// The output type of the function.
    pub(crate) output_type: Type,
    /// The `Span` associated with the function.
    pub(crate) span: Span,
    /// The inputs to the function.
    pub(crate) input: Vec<FunctionInput>,
    /// Metadata associated with the finalize block.
    pub(crate) finalize: Option<FinalizeData>,
}

impl SymbolTable {
    pub(crate) fn new_function_symbol(id: usize, func: &Function) -> FunctionSymbol {
        FunctionSymbol {
            id,
            output_type: func.output_type.clone(),
            span: func.span,
            input: func.input.clone(),
            finalize: func.finalize.as_ref().map(|finalize| FinalizeData {
                input: finalize.input.clone(),
                output_type: finalize.output_type.clone(),
                span: finalize.span,
            }),
        }
    }
}
