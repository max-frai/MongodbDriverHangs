use bson::doc;
use std::sync::Arc;

use mongodb::sync::{Client, Database};

pub fn process_tasks(db: Arc<Database>) {
    let reviews_collection = db.collection("reviews");

    let mut iterator = 0;
    for _ in 0..40 {
        println!("Iterator: {}", iterator);
        iterator += 1;

        let filter = doc! {
            "id" : 1
        };

        dbg!(&filter);
        let result = reviews_collection.find_one(filter, None).unwrap();
        dbg!(&result);
        println!("Db query finished");
    }
}

#[actix_web::main]
async fn main() {
    let client: Arc<Client> = Arc::new(Client::with_uri_str("mongodb://localhost:27018/").unwrap());
    let db = Arc::new(client.database("reviews_system"));

    process_tasks(db)
}
