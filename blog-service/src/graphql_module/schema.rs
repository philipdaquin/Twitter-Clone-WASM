use async_graphql::{Subscription, MergedObject, Schema, SchemaBuilder};
use super::{
    // comments::resolvers::{CommentQuery, CommentMutation},
    posts::resolver::{PostMutation, PostQuery, Subscription}
};

#[derive(MergedObject, Default)]
pub struct Query(PostQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(PostMutation);
pub type AppSchema = Schema<Query, Mutation, Subscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, Subscription>;
