use topcoat::{
    Result,
    router::{RouterBuilderDiscoverExt, module_router, page, tower::TowerService},
    view::{View, component, view},
};

#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <h1>"Home"</h1> })
}
