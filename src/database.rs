use crate::model::{NewTable, Table};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("could not initiate db pool")
}

pub fn read_last_table(
    table_name: &str,
    pool: &Pool,
) -> QueryResult<Table> {
    use crate::schema::tables::dsl::*;
    let con: &PgConnection = &pool.get().expect("could not get db con from pool");
    tables
        .filter(name.eq(&table_name))
        .order(ts.desc())
        .first::<Table>(con)
}


pub fn create_table(new_table: NewTable, pool: Pool) -> QueryResult<Table> {
    let con: &PgConnection = &pool.get().expect("could not get db con from pool");

    diesel::insert_into(crate::schema::tables::table)
        .values(&new_table)
        .get_result(con)
}

pub fn drop_table_named(table_name: &str, pool: Pool) -> QueryResult<usize> {
    use crate::schema::tables::dsl::*;
    let con: &PgConnection = &pool.get().expect("could not get db con from pool");

    diesel::delete(tables.filter(name.eq(&table_name))).execute(con)
}
