mod web;
use machine_learning::number_letter_recognition;
use topcoat::{
    Result,
    router::module_router,
};

#[tokio::main]
async fn main() {
    let _ = web::new().await;
}
