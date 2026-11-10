use topcoat::{
    Result,
    router::{RouterBuilderDiscoverExt, module_router, page, tower::TowerService},
    view::{View, component, view},
};

#[page]
async fn home() -> Result<impl View> {
    let reader = std::io::BufReader::new(std::fs::File::open("image.json")?);
    let image = serde_json::from_reader::<_, crate::Image>(reader)?;
    Ok(view! { 
        <h1>"Main Logic"</h1>
                 <div class="grid" style="white-space: pre;">
                    for row in &image.image {
                        <div class="flex">
                            (row)
                        </div>
                    }
                </div>
        <p>"Label: " (image.label)</p> 
            }
        )
}
