use crate::rules::traits::LintData;
use crate::tree::node_repository::{FromLintData, NodeRepository};
use crate::tree::pointer::Pointer;
use serde_json::Value;
use std::borrow::Cow;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::Range;

pub trait Node {
    fn value_at(&'_ self, ptr: &Pointer) -> Option<Cow<'_, Value>>;
    fn span_at(&self, ptr: &Pointer) -> Option<&Range<usize>>;
    fn pointer(&self) -> &Pointer;
}

pub struct DataAccess<'a, ScopeType, Data>
where
    ScopeType: Scoped<ScopeType>,
    Data: FromLintData,
{
    pub inner: &'a Data,
    _scope: PhantomData<ScopeType>,
}

impl<'a, ScopeType, Data> DataAccess<'a, ScopeType, Data>
where
    ScopeType: Scoped<ScopeType>,
    Data: FromLintData,
{
    pub fn new(inner: &'a Data) -> Self {
        Self {
            inner,
            _scope: PhantomData,
        }
    }
}

impl<'a, ScopeType, Data> LintData<'a> for DataAccess<'a, ScopeType, Data>
where
    ScopeType: Scoped<ScopeType>,
    Data: FromLintData,
{
    fn fetch(board: &'a NodeRepository) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}

pub trait Scoped<ScopeType> {
    fn scope_id() -> &'static u8;
}

pub struct Scope<ScopeType>(PhantomData<ScopeType>);

pub struct Series;

impl Display for Series {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "series")
    }
}
pub struct Case;

impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "case")
    }
}

impl Scoped<Scope<Series>> for Scope<Series> {
    fn scope_id() -> &'static u8 {
        &1
    }
}
impl Scoped<Scope<Case>> for Scope<Case> {
    fn scope_id() -> &'static u8 {
        &0
    }
}
