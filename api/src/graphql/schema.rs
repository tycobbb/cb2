use juniper::{ RootNode, EmptyMutation };
use graphql::query::Query;

pub struct Schema(pub RootNode<
    'static,
    Query,
    EmptyMutation<()>
>);

impl Schema {
    pub fn new() -> Schema {
        Schema(
            RootNode::new(
                Query::new(),
                EmptyMutation::<()>::new()
            )
        )
    }
}
