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

#[ast_node("ThisTypeAnnotation")]
#[derive(Copy, Eq, Hash)]
pub struct ThisTypeAnnotation {
    pub span: Span,
}

#[ast_node("TypeAnnotation")]
#[derive(Eq, Hash)]
pub struct TypeAnn {
    pub span: Span,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: Box<FlowType>,
}

#[ast_node("TypeParamDecl")]
#[derive(Eq, Hash)]
pub struct TypeParamDecl {
    pub span: Span,
    #[serde(rename = "parameters")]
    pub params: Vec<TypeParam>,
}

#[ast_node("TypeParameter")]
#[derive(Eq, Hash)]
pub struct TypeParam {
    pub span: Span,
    pub name: Ident,

    #[serde(default)]
    pub bound: Option<Box<FlowType>>,

    #[serde(default)]
    pub default: Option<Box<FlowType>>,
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
    pub predicate: bool,
    pub id: Ident,
    pub predicate: InferredOrDeclaredPredicate,
}

#[ast_node("InferredPredicate")]
#[derive(Eq, Hash)]
pub struct InferredPredicate {
    pub span: Span,
}

#[ast_node("DeclaredPredicate")]
#[derive(Eq, Hash)]
pub struct DeclaredPredicate {
    pub span: Span,
    pub value: Box<Expr>,
}

#[ast_node("DeclareInterface")]
#[derive(Eq, Hash)]
pub struct DeclareInterface {
    pub span: Span,
}

#[ast_node("DeclareModule")]
#[derive(Eq, Hash)]
pub struct DeclareModule {
    pub span: Span,
    pub id: Ident,
    pub body: Option<ModuleBlock>,
}

#[ast_node("DeclareModuleExports")]
#[derive(Eq, Hash)]
pub struct DeclareModuleExports {
    pub span: Span,
    pub type_ann: Option<FlowType>,
}

#[ast_node("DeclareTypeAlias")]
#[derive(Eq, Hash)]
pub struct DeclareTypeAlias {
    pub span: Span,
    pub type_params: Option<TypeParamDecl>,
    pub type_ann: Option<FlowType>,
}

#[ast_node("ModuleBlock")]
#[derive(Eq, Hash)]
pub struct ModuleBlock {
    pub span: Span,
    pub body: Vec<ModuleItem>,
}

#[ast_node("DeclareOpaqueType")]
#[derive(Eq, Hash)]
pub struct DeclareOpaqueType {
    pub span: Span,
    pub id: Ident,
    pub type_ann: Option<FlowType>,
}

#[ast_node("FunctionTypeAnnotation")]
#[derive(Eq, Hash)]
pub struct FunctionTypeAnnotation {
    pub span: Span,
    pub id: Ident,
    pub params: Vec<TypeParam>,
    pub type_params: Option<TypeParamDecl>,
}

// ================
// Flow types
// ================

#[ast_node]
#[derive(Ee, Hash)]
pub enum FlowType {
    #[tag("KeywordType")]
    KeywordType(KeywordType),

    #[tag("ThisTypeAnnotation")]
    ThisTypeAnnotation(ThisTypeAnnotation),

    #[tag("FunctionTypeAnnotation")]
    FunctionTypeAnnotation(FunctionTypeAnnotation),


}

#[ast_node("KeywordType")]
#[derive(Eq, Hash)]
pub struct KeywordType {
    pub span: Span,
    pub kind: KeywordTypeKind,
}

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
