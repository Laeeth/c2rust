//! Miscellaneous utility functions.
use rustc::hir::def::Def;
use rustc::hir::def_id::DefId;
use smallvec::SmallVec;

pub mod cursor;
pub mod dataflow;


/// Move the lone item out of a 1-element container.
pub trait Lone<T> {
    fn lone(self) -> T;
}

impl<T> Lone<T> for T {
    fn lone(self) -> T {
        self
    }
}

impl<T> Lone<T> for Vec<T> {
    fn lone(mut self) -> T {
        assert!(self.len() == 1);
        self.pop().unwrap()
    }
}

impl<T> Lone<T> for SmallVec<[T; 1]> {
    fn lone(mut self) -> T {
        assert!(self.len() == 1);
        self.pop().unwrap()
    }
}


/// Extension trait for `rustc::hir::def::Def`, providing the `opt_def_id()` method.
pub trait HirDefExt {
    fn opt_def_id(&self) -> Option<DefId>;
}

impl HirDefExt for Def {
    fn opt_def_id(&self) -> Option<DefId> {
        match *self {
            Def::Mod(did) |
            Def::Struct(did) |
            Def::Union(did) |
            Def::Enum(did) |
            Def::Variant(did) |
            Def::Trait(did) |
            Def::Existential(did) |
            Def::TyAlias(did) |
            Def::ForeignTy(did) |
            Def::TraitAlias(did) |
            Def::AssociatedTy(did) |
            Def::AssociatedExistential(did) |
            Def::TyParam(did) |
            Def::Fn(did) |
            Def::Const(did) |
            Def::Static(did, _) |
            Def::StructCtor(did, _) |
            Def::VariantCtor(did, _) |
            Def::SelfCtor(did) |
            Def::Method(did) |
            Def::AssociatedConst(did) |
            Def::Macro(did, _) => Some(did),

            // Local variables stopped having DefIds at some point and switched to NodeId
            Def::PrimTy(_) |
            Def::SelfTy(_, _) |
            Def::ToolMod |
            Def::Local(_) |
            Def::Upvar(_, _, _) |
            Def::Label(_) |
            Def::NonMacroAttr(_) |
            Def::Err => None
        }
    }
}
