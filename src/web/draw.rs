use topcoat::{
    Result,
    context::Cx,
    asset::{Asset, asset},
    router::{RouterBuilderDiscoverExt, module_router, page, tower::TowerService},
    view::{View, component, view},
};

const CANVAS: Asset = asset!("src/web/draw.js");

#[page]
async fn home(cx : &Cx) -> Result<impl View> {
    Ok(view! {
        <head> <script src=(CANVAS)></script> </head>
        <main class="mx-auto max-w-5xl px-4 py-6 sm:py-10">
            <h1 class="mb-1 font-serif text-2xl font-semibold">
                "Pixel canvas"
            </h1>

            //outer
            <div class="grid grid-cols-1 gap-5 md:grid-cols-[minmax(0,1fr)_240px]">

            //inner
                <div class="rounded-lg border border-stone-300 bg-white p-3 dark:border-zinc-700">
                    //canvas
                    <canvas
                        id="draw"
                        width="560"
                        height="560"
                        aria-label="Drawing area"
                        class="block aspect-square w-full touch-none cursor-crosshair rounded border border-stone-300 bg-white"
                    />
                </div>

                <div class="flex flex-col gap-4">

                    <div class="rounded-lg border border-stone-300 bg-white p-3 dark:border-zinc-700">
                        <p class="mb-2 text-sm text-stone-500">
                            "Grid preview"
                        </p>

                        <canvas
                            id="preview"
                            aria-label="Pixel grid preview"
                            class="block aspect-square w-full rounded border border-stone-300 bg-white [image-rendering:pixelated]"
                        />
                    </div>

                    <div class="flex flex-col gap-4 rounded-lg border border-stone-300 bg-white p-3 dark:border-zinc-700">

                        <label class="flex items-center justify-between gap-2 text-sm">
                            "Grid size"

                            <select
                                id="size"
                                class="rounded border border-stone-300 bg-white px-2 py-1"
                            >
                                <option selected = "true">"28"</option>
                            </select>
                        </label>

                        <label class="flex items-center justify-between gap-2 text-sm">
                            "Brush"

                            <input
                                id="brush"
                                type="range"
                                min="6"
                                max="60"
                                value="28"
                                class="w-28 accent-blue-600"
                            />
                        </label>

                        <div class="flex flex-wrap gap-2">
                            <button
                                id="clear"
                                type="button"
                                class="rounded border border-stone-300 px-4 py-2 hover:bg-stone-100 dark:hover:bg-zinc-700"
                            >
                                "Clear"
                            </button>

                            <button
                                id="copy"
                                type="button"
                                class="rounded bg-blue-600 px-4 py-2 text-white hover:bg-blue-700"
                            >
                                "Copy grid"
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <div class="mt-5 rounded-lg border border-stone-300 bg-white p-3 dark:border-zinc-700">
                <p class="mb-2 text-sm text-stone-500">
                    "Grid output: rows of values from 0 (blank) to 1 (full ink)"
                </p>

                //1d vec of pixels
                <textarea
                    id="out"
                    readonly=""
                    class="h-32 w-full resize-y rounded border border-stone-300 bg-stone-50 p-2 font-mono text-xs leading-relaxed dark:bg-zinc-900"
                />
            </div>
        </main>
    } )
}
