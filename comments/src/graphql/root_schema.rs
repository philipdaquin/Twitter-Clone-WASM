use async_graphql::{EmptySubscription, 
    MergedObject, Schema, SchemaBuilder, EmptyMutation};
use super::modules::schema::{CommentQuery, CommentMutation, CommentSubscription};

#[derive(MergedObject, Default)]
pub struct Query(CommentQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(CommentMutation);



pub type AppSchema = Schema<Query, Mutation, CommentSubscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, CommentSubscription>;
