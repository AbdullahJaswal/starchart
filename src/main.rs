use crate::stardata::celestial_object_dao::fetch_celestial_objects;

mod stardata;

fn main() {
    let co_vec = fetch_celestial_objects(100);

    if let Ok(objects) = co_vec {
        for obj in &objects {
            println!("{:?}", obj);
        }
    }
}
