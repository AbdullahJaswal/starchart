use rusqlite::Connection;
use crate::stardata::celestial_object::CelestialObject;
use crate::stardata::query::astronomical_query::fetch_non_null_astronomical_data;

pub fn fetch_celestial_objects(limit: usize) -> Result<Vec<CelestialObject>, rusqlite::Error> {
    let db = Connection::open("./StellarData.sqlite")?;

    let query = fetch_non_null_astronomical_data(limit);
    let mut stmt = db.prepare(&query)?;

    let celestial_objects_iter = stmt.query_map([], |row| {
        Ok(CelestialObject::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
            row.get(9)?,
            row.get(10)?,
            row.get(11)?,
            row.get(12)?,
        ))
    })?;

    let celestial_objects: rusqlite::Result<Vec<CelestialObject>, rusqlite::Error> = celestial_objects_iter.collect();

    return celestial_objects;
}