#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate chrono;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate uuid;

mod models;

use models::person::Person;
use models::point::Point;

use std::thread;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use uuid::Uuid;
use chrono::offset::Utc;

type DbConnection = r2d2::PooledConnection<PostgresConnectionManager>;

fn main() {
    let manager =
        PostgresConnectionManager::new("posgres://postgres@localhost/rustweb", TlsMode::None)
            .unwrap();

    let pool = r2d2::Pool::new(manager).unwrap();
    let mut handles = vec![];

    {
        let pool = pool.clone();
        let handle = thread::spawn(move || {
            let conn = pool.get().unwrap();
            create_table(conn);
            println!("Table creation thread finished!");
        });
        handles.push(handle);
    }

    {
        let pool = pool.clone();
        let handle = thread::spawn(move || {
            let conn = pool.get().unwrap();
            insert_person(conn);
            println!("Person insertion thread finished!");
        });
        handles.push(handle);
    }

    {
        let pool = pool.clone();
        let handle = thread::spawn(move || {
            let conn = pool.get().unwrap();
            for row in &conn.query("SELECT id, name, data, previous_time FROM person", &[])
                .unwrap()
            {
                let person = Person {
                    id: row.get(0),
                    name: row.get(1),
                    data: row.get(2),
                    previous_time: row.get(3),
                };
                println!("Found person {:?}", person);
                println!(
                    "{}'s email {}",
                    person.name,
                    person.data.unwrap()["contacts"]["email"]
                );
                println!("{}'s last lunch: {}", person.name, person.previous_time);
            }
        });
        handles.push(handle);
    }

    serialize_deserialize();

    for handle in handles {
        handle.join().unwrap();
    }
}

fn create_table(conn: DbConnection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id              UUID PRIMARY KEY,
            name            VARCHAR NOT NULL,
            previous_time   TIMESTAMP WITH TIME ZONE,
            data            JSONB
        )",
        &[],
    ).unwrap();
}

fn insert_person(conn: DbConnection) {
    let me = Person {
        id: Uuid::new_v4(),
        name: "John".to_string(),
        previous_time: Utc::now(),
        data: Some(json!({
            "tags": ["employee", "future_ceo"],
            "contacts": {
                "email": "john.doe@mail.com"
            }
        })),
    };

    conn.execute(
        "INSERT INTO person (id, name, data, previous_time) VALUES ($1, $2, $3, $4)",
        &[&me.id, &me.name, &me.data, &me.previous_time],
    ).unwrap();
}

fn serialize_deserialize() {
    // Sample json serialization
    let point = Point { x: 1, y: 2 };

    // Convert from Point to a JSON string
    let serialized = serde_json::to_string(&point).unwrap();
    // serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert from JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    // deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}
