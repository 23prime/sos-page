extern crate dotenv;
extern crate postgres;

use std::env;
use dotenv::dotenv;
use postgres::{Connection, TlsMode};


pub fn establis_connection() -> Connection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    return Connection::connect(db_url, TlsMode::None).unwrap();
}

pub fn get_counter(conn: &Connection) -> i32 {
    let q = "SELECT counter FROM access_counter;";
    let rows = conn.query(q, &[]).unwrap();
    let value: i32 = rows.get(0).get(0);
    return value
}

pub fn update_counter(conn: &Connection, num: i32) {
   let q = format!("UPDATE access_counter set counter = {}", num);
   conn.execute(&q, &[]).unwrap();
}

pub fn count() -> i32 {
    let conn = establis_connection();
    let mut counter = get_counter(&conn);
    counter += 1;
    update_counter(&conn, counter);
    return counter;
}

