extern crate dotenv;
extern crate postgres;
extern crate rand;

use std::env;
use dotenv::dotenv;
use postgres::{Connection, TlsMode};
use rand::Rng;


fn establis_connection() -> postgres::Result<Connection> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    return Connection::connect(db_url, TlsMode::None);
}

fn get_counter(conn: &Connection) -> i32 {
    let q = "SELECT counter FROM access_counter;";
    let rows = conn.query(q, &[]).unwrap();
    let value: i32 = rows.get(0).get(0);
    return value
}

fn update_counter(conn: &Connection, num: i32) {
   let q = format!("UPDATE access_counter set counter = {}", num);
   conn.execute(&q, &[]).unwrap();
}

fn randi() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen();
}

pub fn count() -> i32 {
    let conn = match establis_connection() {
        Ok(conn) => conn,
        _ => return randi().abs(),
    };
    let mut counter = get_counter(&conn);
    counter += 1;
    update_counter(&conn, counter);
    return counter;
}

