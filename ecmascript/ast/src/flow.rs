#![allow(clippy::vec_box)]
#![allow(missing_copy_implementations)]
use crate::{
    expr::Expr,
    ident::Ident,
    lit::{Bool, Number, Str},
    module::ModuleItem,
    pat::{ArrayPay, AssignPat, ObjectPat, RestPat},
};
use serde::{
    de::{self, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;
use string_enum::StringEnum;
#[cfg(feature = "fold")]
use swc_common::Fold;
use swc_common::{ast_node, Span};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialze)]
#[cfd_attr(feature = "fold", derive(Fold))]
pub enum KeywordTypeKind {
    #[serde(rename = "any")] 
    AnyTypeAnnotation,

    #[serde(rename = "boolean")]
    BooleanTypeAnnotation,

    #[serde(rename = "string")]
    StringTypeAnnotation,

    #[serde(rename = "number")]
    NumberTypeAnnotation,
    
    #[serde(rename = "null")]
    NullLiteralTypeAnnotation,

    #[serde(rename = "*")]
    ExistsTypeAnnotation,

    #[serde(rename = "void")]
    VoidTypeAnnotation,

    #[serde(rename = "mixed")]
    MixedTypeAnnotation,

    #[serde(rename = "empty")]
    EmptyTypeAnnotation,
}

#[ast_node("ThisTypeAnnotation")]
#[derive(Copy, Eq, Hash)]
pub struct ThisTypeAnnotation {
    pub span: Span,
}

#[ast_node("BooleanLiteralTypeAnnotation")]
#[derive(Eq, Hash)]
pub struct BooleanLiteralTypeAnnotation {
    pub span: Span,
    pub value: Box<Expr>,
}

#[ast_node("InferredPredicate")]
#[derive(Copy, Eq, Hash)]
pub struct InferredPredicate {
    pub span: Span,
}

#[ast_node("DeclaredPredicate")]
#[derive(Eq, Hash)]
pub struct DeclaredPredicate {
    pub span: Span,
    pub value: Box<Expr>,
}

#[ast_node]
#[derive(Eq, Hash)]
pub enum InferredOrDeclaredPredicate {
    #[tag("InferredPredicate")]
    InferredPredicate(InferredPredicate),

    #[tag("DeclaredPredicate")]
    DeclaredPredicate(DeclaredPredicate),
}

#[ast_node("DeclareClass")]
#[derive(Eq, Hash)]
pub struct DeclareClass {
    pub span: Span,
    pub declare: bool,
    pub id: Ident,
}

#[ast_node("DeclareFunction")]
#[derive(Eq, Hash)]
pub struct DeclareFunction {
    pub span: Span,
    pub declare: bool,
    pub id: Ident,
    pub predicate: InferredOrDeclaredPredicate, 
}
