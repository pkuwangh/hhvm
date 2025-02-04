(*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 *)

module LMap = Local_id.Map
module KMap = Typing_continuations.Map

(** A generic exception for all refactor sound dynamic specific failures
   Relationship with shape_analysis: Shape_analysis_exn *)
exception Refactor_sd_exn of string

type analysis_mode =
  | FlagTargets
      (** Flag all possible targets without performing any analysis *)
  | DumpConstraints  (** Dump constraints generated by analysing the program *)
  | SimplifyConstraints
      (** Partially solve key constraints within functions and methods and
          report back whether a function is ever upcasted to dynamic. *)
  | SolveConstraints
      (** Globally solve the key constraints and report back whether a
          function is ever upcasted to dynamic *)

type refactor_mode =
  | Class  (** Locate upcasts of a specific Class *)
  | Function  (** Locate upcasts of a specific Function *)

type options = {
  analysis_mode: analysis_mode;
  refactor_mode: refactor_mode;
}

type entity_ =
  | Literal of Pos.t
  | Variable of int
[@@deriving eq, ord]

type entity = entity_ option

(** Relationship with shape_analysis: constraint_ constructors are different *)
type constraint_ =
  | Introduction of Pos.t
      (** Records introduction of an instance of function pointer *)
  | Upcast of entity_ * Pos.t
      (** Records existance and position of upcast dynamic *)
  | Subset of entity_ * entity_
      (** Records that the first entity is assigned to the second *)
  | Called of Pos.t
      (** Records that the entity_ was a function pointer that was called *)

(** Relationship with shape_analysis: shape_result *)
type refactor_sd_result =
  | Exists_Upcast of Pos.t
  | No_Upcast

(** Local variable environment. Its values are `entity`, i.e., `entity_
    option`, so that we can avoid pattern matching in constraint extraction. *)
type lenv = entity LMap.t KMap.t

type env = {
  constraints: constraint_ list;  (** Append-only set of constraints *)
  lenv: lenv;  (** Local variable information *)
  tast_env: Tast_env.env;
      (** TAST env associated with the definition being analysed *)
}

module PointsToSet : Set.S with type elt = entity_ * entity_

module EntityMap : Map.S with type key = entity_

module EntitySet : Set.S with type elt = entity_
