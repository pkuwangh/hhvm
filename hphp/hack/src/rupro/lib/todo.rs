// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

#[derive(Debug)]
pub enum RuproTodo {
    /// AST nodes or variants of AST nodes that are not yet supported.
    AST,
    /// An error should be registered in certain situations.
    MissingError,
    /// Variance annotations, or propagation (during type inference) is not yet
    /// implemented.
    Variance,
    /// Memoization is not yet implemented.
    Memoization,
    /// Holes are not yet produced.
    Holes,
    /// Bidirectional type checking is not yet implemented.
    ///
    /// In the OCaml code, bidirectional type checking is implemented via
    /// the `~expected` argument that is threaded throughout `typing.ml`.
    BidirectionalTC,
    /// Coeffects related logic is not yet implemented.
    Coeffects,
    /// Fake members and refinements are not yet implemented.
    FakeMembersAndRefinement,
    /// Awaitable is not yet impelemented.
    Awaitable,
    /// Using variables are not yet implemented.
    UsingVar,
    /// Dependent type of a local expression is not yet implemented.
    LocalExprId,
    /// Readonly is not yet implemented.
    Readonly,
    /// Check whether local variables are defined when accessing them.
    ///
    /// Currently, we don't do this. See `typing.ml`'s `check_defined`
    /// parameter to learn when the check should occur.
    CheckLocalDefined,
    /// Extra checks for inout parameters are not supported.
    InoutParameters,
    /// Disposable feature is not yet implemented.
    Disposable,
    /// No calls into subtyping from within typing yet.
    SubtypeCheck,
    /// No legacy or sound dynamic.
    Dynamic,
    /// No flow-based continuation typing yet.
    Flow,
    /// Terminality checks are not implemented, see `Typing_func_terminality.ml`.
    Terminality,
    /// Some logic that is specialized for hhi files is not implemented.
    Hhi,
    /// Logic that registered information in the environment to be saved in the
    /// `SavedEnv` is not yet implemented.
    SavedEnv,
    /// The `DisableTypecheckerInternal` attribute is not yet implemented.
    DisableTypecheckerInternalAttribute,
    /// Abstract functions/methods are not yet implemented.
    Abstract,
    /// Generators are not yet implemented.
    Generators,
}

macro_rules! rupro_todo_assert {
  ($cond:expr, $todo:ident, $($arg:tt)+) => {
    if !($cond) {
        panic!(
            "rupro_todo_mark: {:?}: {}",
            $crate::todo::RuproTodo::$todo,
            std::format_args!($($arg)+)
        );
    }
  };
  ($cond:expr, $todo:ident) => {
    if !($cond) {
        panic!(
            "rupro_todo_mark: {:?}",
            $crate::todo::RuproTodo::$todo,
        );
    }
  };
}

macro_rules! rupro_todo_mark {
  ($todo:ident, $($arg:tt)+) => {
    let _ = $crate::todo::RuproTodo::$todo;
    assert!(true, $($arg)+);
  };
  ($todo:ident) => {
    let _ = $crate::todo::RuproTodo::$todo;
  };
}

macro_rules! rupro_todo {
  ($todo:ident, $($arg:tt)+) => {
    panic!(
      "rupro_todo: {:?}: {}",
      $crate::todo::RuproTodo::$todo,
      std::format_args!($($arg)+)
    )
  };
  ($todo:ident) => {
    panic!(
      "rupro_todo: {:?}",
      $crate::todo::RuproTodo::$todo,
    )
  };
}
