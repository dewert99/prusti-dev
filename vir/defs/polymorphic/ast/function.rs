// © 2019, ETH Zurich
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::polymorphic::ast::*;
use std::collections::HashMap;
use std::fmt;

use super::super::super::{legacy, converter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub formal_args: Vec<LocalVar>,
    pub return_type: Type,
    pub pres: Vec<Expr>,
    pub posts: Vec<Expr>,
    pub body: Option<Expr>,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "function {}(", self.name)?;
        let mut first = true;
        for arg in &self.formal_args {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", arg)?;
            first = false
        }
        writeln!(f, "): {}", self.return_type)?;
        for pre in &self.pres {
            writeln!(f, "  requires {}", pre)?;
        }
        for post in &self.posts {
            writeln!(f, "  ensures {}", post)?;
        }
        if let Some(ref body) = self.body {
            writeln!(f, "{{")?;
            writeln!(f, "\t{}", body)?;
            write!(f, "}}")?;
        }
        write!(f, "")
    }
}

impl From<Function> for legacy::Function {
    fn from(function: Function) -> legacy::Function {
        legacy::Function {
            name: function.name,
            formal_args: function.formal_args.into_iter().map(|formal_arg| legacy::LocalVar::from(formal_arg)).collect(),
            return_type: legacy::Type::from(function.return_type),
            pres: function.pres.into_iter().map(|pre| legacy::Expr::from(pre)).collect(),
            posts: function.posts.into_iter().map(|post| legacy::Expr::from(post)).collect(),
            body: function.body.map(|body_expr| legacy::Expr::from(body_expr)),
        }
    }
}

impl converter::Generic for Function {
    fn substitute(self, map: &HashMap<TypeVar, Type>) -> Self {
        let mut function = self;
        function.formal_args = function.formal_args.into_iter().map(|formal_arg| formal_arg.substitute(map)).collect();
        function.return_type = function.return_type.substitute(map);
        function.pres = function.pres.into_iter().map(|pre| pre.substitute(map)).collect();
        function.posts = function.posts.into_iter().map(|post| post.substitute(map)).collect();
        function.body = function.body.map(|body_expr| body_expr.substitute(map));
        function
    }
}