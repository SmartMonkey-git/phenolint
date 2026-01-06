use crate::tree::scopes::ScopeLayer;
use crate::tree::traits::NodeRepository;

pub trait QueryStrategy {
    type Output;
    #[allow(unused)]
    fn query(node_repo: &impl NodeRepository) -> Self::Output;
}

macro_rules! impl_query_strategy_tuple {
    () => {
        impl QueryStrategy for () {
            type Output = ();
            fn query(_node_repo: &impl NodeRepository) -> Self::Output {

            }
        }
    };

    ($head:ident $(, $tail:ident)*) => {
        impl_query_strategy_tuple!($($tail),*);

        impl<$head, $($tail),*> QueryStrategy for ($head, $($tail,)*)
        where
            $head: QueryStrategy,
            $($tail: QueryStrategy),*
        {
            type Output = ($head::Output, $($tail::Output,)*);

            fn query(node_repo: &impl NodeRepository) -> Self::Output {
                (
                    $head::query(node_repo),
                    $($tail::query(node_repo),)*
                )
            }
        }
    };
}

impl_query_strategy_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);

#[allow(unused)]
pub(crate) trait QueryPresentation<Input> {
    fn present(query_res: Input) -> Self
    where
        Self: Sized;
}

pub trait ScopeDefinition {
    fn layer() -> ScopeLayer;

    fn partitioning_fields() -> &'static [&'static str] {
        &[]
    }
}
