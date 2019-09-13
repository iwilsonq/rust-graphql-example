table! {
    members (id) {
        id -> Int4,
        name -> Varchar,
        knockouts -> Int4,
        team_id -> Int4,
    }
}

table! {
    teams (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(members -> teams (team_id));

allow_tables_to_appear_in_same_query!(
    members,
    teams,
);
