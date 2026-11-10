use topcoat::{
    Result,
    context::Cx,
    router::{route, content::Json, RouterBuilderDiscoverExt, module_router, page, tower::TowerService},
    view::{View, component, view},
};
use machine_learning::number_letter_recognition;

use serde;

#[derive(serde::Serialize)]
struct Prediction {
    prediction: char,
}

#[page]
async fn home(cx : &Cx) -> Result<impl View> {
    //maybe add button that redirects back to /draw
    Ok(view! {
       <h1> "This is an api end point, go back to /draw" </h1>
    })
}

#[route(POST)]
async fn predict(cx: &Cx, Json(grid): Json<Vec<Vec<f64>>>) -> Result<Json<Prediction>> {
    let prediction = number_letter_recognition::process_image(&grid);
    Ok(Json(Prediction{prediction}))
}
