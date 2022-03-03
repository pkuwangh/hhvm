// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use crate::{Prefix, RelativePath, ToOxidized};
use eq_modulo_pos::EqModuloPos;
use intern::string::BytesId;
use oxidized::file_pos_small::FilePosSmall;
use oxidized::pos_span_raw::PosSpanRaw;
use oxidized::pos_span_tiny::PosSpanTiny;
use std::fmt;
use std::hash::Hash;

pub use oxidized::file_pos_large::FilePosLarge;

pub trait Pos:
    Eq
    + Hash
    + Clone
    + std::fmt::Debug
    + for<'a> From<&'a oxidized::pos::Pos>
    + for<'a> From<&'a oxidized_by_ref::pos::Pos<'a>>
    + EqModuloPos
    + 'static
{
    /// Make a new instance. If the implementing Pos is stateful,
    /// it will call cons() to obtain interned values to construct the instance.
    fn mk(cons: impl FnOnce() -> (RelativePath, FilePosLarge, FilePosLarge)) -> Self;

    fn from_ast(pos: &oxidized::pos::Pos) -> Self {
        Self::mk(|| {
            let PosSpanRaw { start, end } = pos.to_raw_span();
            (pos.filename().into(), start, end)
        })
    }

    fn from_decl(pos: &oxidized_by_ref::pos::Pos<'_>) -> Self {
        Self::mk(|| {
            let PosSpanRaw { start, end } = pos.to_raw_span();
            (pos.filename().into(), start, end)
        })
    }

    fn to_oxidized<'a>(&self, arena: &'a bumpalo::Bump) -> &'a oxidized_by_ref::pos::Pos<'a>;
}

/// Represents a closed-ended range [start, end] in a file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PosImpl {
    Small {
        prefix: Prefix,
        suffix: BytesId,
        span: Box<(FilePosSmall, FilePosSmall)>,
    },
    Large {
        prefix: Prefix,
        suffix: BytesId,
        span: Box<(FilePosLarge, FilePosLarge)>,
    },
    Tiny {
        prefix: Prefix,
        suffix: BytesId,
        span: PosSpanTiny,
    },
}

static_assertions::assert_eq_size!(PosImpl, u128);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BPos(PosImpl);

impl Pos for BPos {
    fn mk(cons: impl FnOnce() -> (RelativePath, FilePosLarge, FilePosLarge)) -> Self {
        let (file, start, end) = cons();
        Self::new(file, start, end)
    }

    fn to_oxidized<'a>(
        &self,
        _arena: &'a bumpalo::Bump,
    ) -> &'a oxidized_by_ref::pos_or_decl::PosOrDecl<'a> {
        unimplemented!()
    }
}

impl BPos {
    pub fn new(file: RelativePath, start: FilePosLarge, end: FilePosLarge) -> Self {
        let prefix = file.prefix();
        let suffix = file.suffix();
        if let Some(span) = PosSpanTiny::make(&start, &end) {
            return BPos(PosImpl::Tiny {
                prefix,
                suffix,
                span,
            });
        }
        let (lnum, bol, offset) = start.line_beg_offset();
        if let Some(start) = FilePosSmall::from_lnum_bol_offset(lnum, bol, offset) {
            let (lnum, bol, offset) = end.line_beg_offset();
            if let Some(end) = FilePosSmall::from_lnum_bol_offset(lnum, bol, offset) {
                let span = Box::new((start, end));
                return BPos(PosImpl::Small {
                    prefix,
                    suffix,
                    span,
                });
            }
        }
        let span = Box::new((start, end));
        BPos(PosImpl::Large {
            prefix,
            suffix,
            span,
        })
    }

    pub fn file(&self) -> RelativePath {
        match self.0 {
            PosImpl::Small { prefix, suffix, .. }
            | PosImpl::Large { prefix, suffix, .. }
            | PosImpl::Tiny { prefix, suffix, .. } => RelativePath::from_bytes_id(prefix, suffix),
        }
    }
}

impl fmt::Debug for BPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut do_fmt = |start_line, start_col, end_line, end_col| {
            if start_line == end_line {
                write!(
                    f,
                    "Pos({:?}, {}:{}-{})",
                    &self.file(),
                    &start_line,
                    &start_col,
                    &end_col,
                )
            } else {
                write!(
                    f,
                    "Pos({:?}, {}:{}-{}:{})",
                    &self.file(),
                    &start_line,
                    &start_col,
                    &end_line,
                    &end_col,
                )
            }
        };
        match &self.0 {
            PosImpl::Small { span, .. } => {
                let (start, end) = &**span;
                do_fmt(start.line(), start.column(), end.line(), end.column())
            }
            PosImpl::Large { span, .. } => {
                let (start, end) = &**span;
                do_fmt(start.line(), start.column(), end.line(), end.column())
            }
            PosImpl::Tiny { span, .. } => {
                let span = span.to_raw_span();
                do_fmt(
                    span.start.line(),
                    span.start.column(),
                    span.end.line(),
                    span.end.column(),
                )
            }
        }
    }
}

impl EqModuloPos for BPos {
    fn eq_modulo_pos(&self, _rhs: &Self) -> bool {
        true
    }
}

impl<'a> From<&'a oxidized::pos::Pos> for BPos {
    fn from(pos: &'a oxidized::pos::Pos) -> Self {
        Self::from_ast(pos)
    }
}

impl<'a> From<&'a oxidized_by_ref::pos::Pos<'a>> for BPos {
    fn from(pos: &'a oxidized_by_ref::pos::Pos<'a>) -> Self {
        Self::from_decl(pos)
    }
}

/// A stateless sentinel Pos.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NPos;

impl Pos for NPos {
    fn mk(_cons: impl FnOnce() -> (RelativePath, FilePosLarge, FilePosLarge)) -> Self {
        NPos
    }

    fn to_oxidized<'a>(&self, _arena: &'a bumpalo::Bump) -> &'a oxidized_by_ref::pos::Pos<'a> {
        oxidized_by_ref::pos::Pos::none()
    }
}

impl EqModuloPos for NPos {
    fn eq_modulo_pos(&self, _rhs: &Self) -> bool {
        true
    }
}

impl<'a> From<&'a oxidized::pos::Pos> for NPos {
    fn from(pos: &'a oxidized::pos::Pos) -> Self {
        Self::from_ast(pos)
    }
}

impl<'a> From<&'a oxidized_by_ref::pos::Pos<'a>> for NPos {
    fn from(pos: &'a oxidized_by_ref::pos::Pos<'a>) -> Self {
        Self::from_decl(pos)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Positioned<S, P> {
    // Caution: field order will matter if we ever derive
    // `ToOcamlRep`/`FromOcamlRep` for this type.
    pos: P,
    id: S,
}

impl<S: fmt::Debug, P: fmt::Debug> fmt::Debug for Positioned<S, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if std::mem::size_of::<P>() == 0 {
            write!(f, "{:?}", &self.id)
        } else {
            f.debug_tuple("").field(&self.pos).field(&self.id).finish()
        }
    }
}

impl<S, P> Positioned<S, P> {
    pub fn new(pos: P, id: S) -> Self {
        Self { pos, id }
    }

    pub fn pos(&self) -> &P {
        &self.pos
    }

    pub fn id_ref(&self) -> &S {
        &self.id
    }
}

impl<S: Copy, P> Positioned<S, P> {
    pub fn id(&self) -> S {
        self.id
    }
}

impl<'a, S: From<&'a str>, P: Pos> From<&'a oxidized::ast_defs::Id> for Positioned<S, P> {
    fn from(pos_id: &'a oxidized::ast_defs::Id) -> Self {
        let oxidized::ast_defs::Id(pos, id) = pos_id;
        Self::new(Pos::from_ast(pos), S::from(id))
    }
}

impl<'a, S: From<&'a str>, P: Pos> From<oxidized_by_ref::ast_defs::Id<'a>> for Positioned<S, P> {
    fn from(pos_id: oxidized_by_ref::ast_defs::Id<'a>) -> Self {
        let oxidized_by_ref::ast_defs::Id(pos, id) = pos_id;
        Self::new(Pos::from_decl(pos), S::from(id))
    }
}

impl<'a, S: From<&'a str>, P: Pos> From<oxidized_by_ref::typing_defs::PosId<'a>>
    for Positioned<S, P>
{
    fn from(pos_id: oxidized_by_ref::typing_defs::PosId<'a>) -> Self {
        let (pos, id) = pos_id;
        Self::new(Pos::from_decl(pos), S::from(id))
    }
}

impl<'a, S: std::convert::AsRef<str>, P: Pos> ToOxidized<'a> for Positioned<S, P> {
    type Output = oxidized_by_ref::typing_reason::PosId<'a>;

    fn to_oxidized(&self, arena: &'a bumpalo::Bump) -> Self::Output {
        (
            self.pos().to_oxidized(arena),
            arena.alloc_str(self.id.as_ref()),
        )
    }
}
