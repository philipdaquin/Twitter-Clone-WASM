use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
use super::modules;


#[derive(MergedObject, Default)]
pub struct Query;

#[derive(MergedObject, Default)]
pub struct Mutation;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;
