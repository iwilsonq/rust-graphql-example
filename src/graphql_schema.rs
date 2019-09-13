extern crate dotenv;

use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use juniper::RootNode;

use crate::schema::members;

pub struct QueryRoot;

fn establish_connection() -> PgConnection {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[juniper::object]
impl QueryRoot {
  fn members() -> Vec<Member> {
    use crate::schema::members::dsl::*;
    let connection = establish_connection();
    members
      .limit(100)
      .load::<Member>(&connection)
      .expect("Error loading members")
  }

  fn teams() -> Vec<Team> {
    use crate::schema::teams::dsl::*;
    let connection = establish_connection();
    teams
      .limit(10)
      .load::<Team>(&connection)
      .expect("Error loading teams")
  }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
  fn create_member(data: NewMember) -> Member {
    let connection = establish_connection();
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

#[juniper::object(description = "A team of members")]
impl Team {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }

  pub fn members(&self) -> Vec<Member> {
    use crate::schema::members::dsl::*;
    let connection = establish_connection();
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
