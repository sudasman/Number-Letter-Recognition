mod about;
mod draw;
mod home;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpListener;
use topcoat::{
    Result,
    tailwind,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    router::{
        Path, RouterBuilderDiscoverExt, error::redirect, module_router, page, tower::TowerService, Slot, layout,
    },
    runtime::{ RuntimeLayer },
    view::{View, component, view},
};
use tower_http::services::ServeDir;

//Topcoat uses module based routing similiar to nextjs
//the folders must match the parent root

pub async fn new() -> Result<()> {


 topcoat::start(
    module_router!()
        .discover()
        .assets(AssetBundle::load().unwrap())
        .layer(RuntimeLayer::new())
        .build()
)
.await?;

    Ok(())
}

//when get is invoked redirect
#[page]
async fn redirect_page(cx: &Cx) -> Result<()> {
    Err(redirect("/home").into())
}

#[layout]
async fn layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head>
                topcoat::runtime::script() 
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
            </head>
            <body>
                (slot)
            </body>
        </html>
    })
}
