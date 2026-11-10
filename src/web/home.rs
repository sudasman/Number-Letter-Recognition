use topcoat::{
    Result,
    context::Cx,
    router::{RouterBuilderDiscoverExt, module_router, page, tower::TowerService},
    view::{View, component, view},
};

#[page]
async fn home(cx : &Cx) -> Result<impl View> {
    Ok(view! { <h1>"Home"</h1> })
}
