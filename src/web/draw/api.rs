use topcoat::{
    Result,
    context::Cx,
    router::{route, content::Json, RouterBuilderDiscoverExt, module_router, page, tower::TowerService},
    view::{View, component, view},
};

use serde;

#[derive(serde::Serialize)]
struct Prediction {
    prediction: String,
}

#[page]
async fn home(cx : &Cx) -> Result<impl View> {
    //maybe add button that redirects back to /draw
    Ok(view! {
       <h1> "This is an api end point, go back to /draw" </h1>
    })
}

#[route(POST)]
async fn predict(cx: &Cx, Json(grid): Json<Vec<f64>>) -> Result<Json<Prediction>> {
    let prediction = String::from("test");
    dbg!(grid);
    Ok(Json(Prediction{prediction}))
}
