//! Intermediate representation of variables.

use std::borrow::Cow;
use std::io;
use std::str;

use crate::callbacks::{FnMacroInfo, MacroParsingBehavior};
use crate::clang;
use crate::parse::{ClangSubItemParser, ParseError, ParseResult};

use super::context::BindgenContext;
use super::dot::DotAttributes;

/// A `MacroDef` is our intermediate representation of a macro definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacroDef {
    /// A function-like macro.
    Fn(String),
    /// A variable-like macro.
    Var(String),
}

impl MacroDef {
    /// Get the macro name.
    pub fn name(&self) -> &str {
        match self {
            Self::Fn(name) => name,
            Self::Var(name) => name,
        }
    }
}

impl DotAttributes for MacroDef {
    fn dot_attributes<W>(
        &self,
        _ctx: &BindgenContext,
        out: &mut W,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        writeln!(out, "<tr><td>macro</td><td>true</td></tr>")
    }
}

impl ClangSubItemParser for MacroDef {
    fn parse(
        cursor: clang::Cursor,
        ctx: &mut BindgenContext,
    ) -> Result<ParseResult<Self>, ParseError> {
        use clang_sys::CXCursor_MacroDefinition;

        if cursor.kind() != CXCursor_MacroDefinition {
            return Err(ParseError::Continue);
        }

        debug!("MacroDef::parse({:?}, {:?})", cursor, cursor.cur_type());

        match ctx
            .options()
            .last_callback(|c| Some(c.will_parse_macro(&cursor.spelling())))
            .unwrap_or_default()
        {
            MacroParsingBehavior::Ignore => {
                return Err(ParseError::Continue);
            }
            MacroParsingBehavior::Default => (),
        }

        let clang_tokens = cursor.tokens().iter().collect::<Vec<_>>();
        let args_boundary = if cursor.is_macro_function_like() {
            clang_tokens.iter().position(|t| {
                t.kind == clang_sys::CXToken_Punctuation &&
                    t.spelling().to_bytes() == b")"
            })
        } else {
            None
        };
        let mut cmacro_tokens = clang_tokens
            .iter()
            .map(|token| {
                let spelling = token.spelling();
                match spelling.to_str() {
                    Ok(token) => Cow::Borrowed(token),
                    Err(_) => {
                        let token = spelling.to_string_lossy();
                        warn!(
                            "Lossy conversion of non-UTF8 token {:?} to {:?}.",
                            spelling, token
                        );
                        token
                    }
                }
            })
            .collect::<Vec<_>>();

        let name = cmacro_tokens.remove(0);

        let args = if let Some(args_boundary) = args_boundary {
            let args: Vec<_> = cmacro_tokens
                .drain(0..args_boundary)
                .skip(1)
                .take(args_boundary - 2)
                .filter(|token| token != ",")
                .collect();
            Some(args)
        } else {
            None
        };

        let body = cmacro_tokens;

        if let Some(args) = args {
            let args = args.iter().map(|s| s.as_ref()).collect::<Vec<_>>();

            if !ctx.options().parse_callbacks.is_empty() {
                let body = body.iter().map(|s| s.as_ref()).collect::<Vec<_>>();

                let info = FnMacroInfo {
                    name: &name,
                    args: &args,
                    body: &body,
                };

                for callbacks in &ctx.options().parse_callbacks {
                    callbacks.fn_macro(&info);
                }
            }

            if ctx.macro_set.define_fn_macro(name.as_ref(), args, body) {
                // TODO: Redefined macro.
            }

            Ok(ParseResult::New(
                MacroDef::Fn(name.into_owned()),
                Some(cursor),
            ))
        } else {
            if ctx.macro_set.define_var_macro(name.as_ref(), body) {
                // TODO: Redefined macro.
            }

            Ok(ParseResult::New(
                MacroDef::Var(name.into_owned()),
                Some(cursor),
            ))
        }
    }
}
