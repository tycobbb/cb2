use db::schema::games;

#[derive(GraphQLObject, Serialize, Queryable)]
pub struct Game {
    pub id:   i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="games"]
pub struct NewGame<'a> {
    pub name: &'a str,
}
