use topcoat::{
    Result,
    context::Cx,
    router::{RouterBuilderDiscoverExt, module_router, page, tower::TowerService},
    view::{View, component, view},
};

#[page]
async fn home(cx : &Cx) -> Result<impl View> {
    let reader = std::io::BufReader::new(std::fs::File::open("image.json")?);
    let image = serde_json::from_reader::<_, crate::Image>(reader)?;
    Ok(view! {
     <p1> $(1 + 2) </p1>
    <h1>"Main Logic"</h1>
    } )
}
