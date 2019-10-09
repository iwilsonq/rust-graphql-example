extern crate dotenv;

use diesel::prelude::*;
use juniper::RootNode;

use crate::db::PgPool;
use crate::schema::members;

#[derive(Clone)]
pub struct Context {
  pub db: PgPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
  fn members(context: &Context) -> Vec<Member> {
    use crate::schema::members::dsl::*;
    let connection = context.db.get().unwrap();;
    members
      .limit(100)
      .load::<Member>(&connection)
      .expect("Error loading members")
  }

  fn teams(context: &Context) -> Vec<Team> {
    use crate::schema::teams::dsl::*;
    let connection = context.db.get().unwrap();;
    teams
      .limit(10)
      .load::<Team>(&connection)
      .expect("Error loading teams")
  }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
  fn create_member(context: &Context, data: NewMember) -> Member {
    let connection = context.db.get().unwrap();;
    diesel::insert_into(members::table)
      .values(&data)
      .get_result(&connection)
      .expect("Error saving new post")
  }
}

#[derive(Queryable)]
pub struct Member {
  pub id: i32,
  pub name: String,
  pub knockouts: i32,
  pub team_id: i32,
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "members"]
pub struct NewMember {
  pub name: String,
  pub knockouts: i32,
  pub team_id: i32,
}

#[juniper::object(description = "A member of a team")]
impl Member {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }

  pub fn knockouts(&self) -> i32 {
    self.knockouts
  }

  pub fn team_id(&self) -> i32 {
    self.team_id
  }
}

#[derive(Queryable)]
pub struct Team {
  pub id: i32,
  pub name: String,
}

#[juniper::object(Context = Context, description = "A team of members")]
impl Team {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }

  pub fn members(&self, context: &Context) -> Vec<Member> {
    use crate::schema::members::dsl::*;
    let connection = context.db.get().unwrap();
    members
      .filter(team_id.eq(self.id))
      .limit(100)
      .load::<Member>(&connection)
      .expect("Error loading members")
  }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}
