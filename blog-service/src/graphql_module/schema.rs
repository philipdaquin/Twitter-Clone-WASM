use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
use super::modules::{
    comments::resolvers::{CommentQuery, CommentMutation},
    posts::resolver::{PostMutation, PostQuery}
};

#[derive(MergedObject, Default)]
pub struct Query(PostQuery, CommentQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(PostMutation, CommentMutation);
pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;
