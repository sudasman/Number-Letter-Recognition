mod web;
use machine_learning::number_letter_recognition;

#[tokio::main]
async fn main(){
    // number_letter_recognition::sample().expect("Err");
   let _ = web::new().await;
}
