/**
 * Copyright (c) 2016, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree. An additional
 * directory.
 *
 **
 *
 * THIS FILE IS @generated; DO NOT EDIT IT
 * To regenerate this file, run
 *
 *   buck run //hphp/hack/src:generate_full_fidelity
 *
 **
 *
 */

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TriviaKind {
    WhiteSpace,
    EndOfLine,
    DelimitedComment,
    SingleLineComment,
    Unsafe,
    UnsafeExpression,
    FixMe,
    IgnoreError,
    FallThrough,
    ExtraTokenError,
    AfterHaltCompiler,
}

impl TriviaKind {
    pub fn to_string(&self) -> &str {
        match self {
            TriviaKind::WhiteSpace => "whitespace",
            TriviaKind::EndOfLine => "end_of_line",
            TriviaKind::DelimitedComment => "delimited_comment",
            TriviaKind::SingleLineComment => "single_line_comment",
            TriviaKind::Unsafe => "unsafe",
            TriviaKind::UnsafeExpression => "unsafe_expression",
            TriviaKind::FixMe => "fix_me",
            TriviaKind::IgnoreError => "ignore_error",
            TriviaKind::FallThrough => "fall_through",
            TriviaKind::ExtraTokenError => "extra_token_error",
            TriviaKind::AfterHaltCompiler => "after_halt_compiler",
        }
    }
}
