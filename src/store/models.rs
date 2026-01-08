use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    prelude::*,
    sql_types::Text,
};

pub struct PathBuf(std::path::PathBuf);

impl<DB> Queryable<Text, DB> for PathBuf
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    type Row = String;

    fn build(s: String) -> deserialize::Result<Self> {
        Ok(PathBuf(std::path::PathBuf::from(s)))
    }
}

impl From<PathBuf> for std::path::PathBuf {
    fn from(value: PathBuf) -> Self {
        value.0
    }
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::store::schema::config)]
pub struct NewConfig {
    pub installation_directory: Option<String>,
    pub temp_directory: Option<String>,
}

#[derive(Selectable, Queryable)]
#[diesel(table_name = crate::store::schema::config)]
pub struct Config {
    pub id: i32,
    #[diesel(deserialize_as = PathBuf)]
    pub installation_directory: std::path::PathBuf,
    #[diesel(deserialize_as = PathBuf)]
    pub temp_directory: std::path::PathBuf,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::store::schema::repositories)]
pub struct NewRepository<'a> {
    pub owner: &'a str,
    pub name: &'a str,
    pub package: &'a str,
    pub location: &'a str,
    pub tag: &'a str,
    pub locked: bool,
}

#[derive(Selectable, Queryable)]
#[diesel(table_name = crate::store::schema::repositories)]
pub struct Repository {
    pub id: i32,
    pub owner: String,
    pub name: String,
    pub package: String,
    #[diesel(deserialize_as = PathBuf)]
    pub location: std::path::PathBuf,
    pub tag: String,
    pub locked: bool,
}
