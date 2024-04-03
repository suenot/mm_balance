use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct Human {
  id: String,
  name: String,
}

pub struct Query;

#[juniper::graphql_object]
impl Query {
  fn human(&self, id: String) -> Human {
    Human {
      id,
      name: "Luke Skywalker".to_owned(),
    }
  }
}

pub type Schema = juniper::RootNode<'static, Query, juniper::EmptyMutation<()>>;