use crate::types::*;

/// The `Visitor` trait defines the interface for visiting the AST.
make_visitor! {
    trait VisitMut is mut;
}
