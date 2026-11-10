mod web;
use machine_learning::number_letter_recognition;
use serde::{Deserialize, Serialize};
use topcoat::{Result, router::module_router};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    image: Vec<String>,
    label: char,
}

#[tokio::main]
async fn main() {
    let (image, label): (Vec<String>, char) =
        machine_learning::number_letter_recognition::sample().expect("Error");
    let temp = Image { image, label };

    let writer = std::io::BufWriter::new(
        std::fs::File::create("image.json").expect("unable to create json file"),
    );

    serde_json::to_writer_pretty(writer, &temp);

    let _ = web::new().await;
}
