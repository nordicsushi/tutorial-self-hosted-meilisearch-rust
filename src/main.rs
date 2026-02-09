#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    id: i64,
    title: String,
    poster: String,
    overview: String,
    release_date: i64,
    genres: Vec<String>,
}

use meilisearch_sdk::{client::*, indexes::*, search::*, settings::*};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::prelude::*};
use tokio::time::{Duration, sleep};

struct TaskId(u32);

impl AsRef<u32> for TaskId {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

#[tokio::main]
async fn main() {
    let client = Client::new("http://localhost:7700", Some("aSampleMasterKey")).unwrap();

    // Reading and parsing the file
    let mut file = File::open("movies.json").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let movies_docs: Vec<Movie> = serde_json::from_str(&content).unwrap();

    // Adding documents
    let task_info = client
        .index("movies")
        .add_documents(&movies_docs, None)
        .await
        .unwrap();

    println!("{:#?}", task_info);

    sleep(Duration::from_secs(5)).await;

    let result = client.get_task(task_info).await.unwrap();
    println!("{:#?}", result);

    let results: SearchResults<Movie> = client
        .index("movies")
        .search()
        .with_query("botman")
        .execute()
        .await
        .unwrap();

    println!("{:#?}", results);
}
