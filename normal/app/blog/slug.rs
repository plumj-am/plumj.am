//! A single blog post.

use libs::data::ME;
use topcoat::{
    Result,
    context::Cx,
    router::{
        href,
        module_param,
        page,
        path_param,
    },
    view::{
        View,
        view,
    },
};

use crate::{
    app::blog,
    components::{
        LOGO,
        credits,
    },
    markdown,
    posts,
};

module_param!(pub slug);

/// Renders the post identified by the URL slug.
#[page]
pub async fn page(cx: &Cx) -> Result<impl View> {
    let slug: &str = path_param::<Slug>(cx);
    let total = posts::all().len();

    Ok(view! {
        <div class="flex flex-col py-6">
            match posts::find(slug) {
                Some((post, index)) => {
                    <div class="border border-fg p-4 relative">
                        <div class="flex justify-between items-start">
                            <h1 class="text-3xl font-bold text-fg mb-3">(&post.title)</h1>
                            <p class="text-lg text-fg opacity-60">"#"(total.saturating_sub(index))</p>
                        </div>
                        <div class="text-md text-fg flex items-end justify-between">
                            <p class="opacity-60 flex flex-col sm:flex-row w-full sm:items-center">
                                (&post.date)
                                " "
                                <span class="text-xs opacity-40 sm:ml-2">"(last edit: "(&post.edited)")"</span>
                            </p>
                            <span class="group flex items-center">
                                <img class="w-4 mr-2 group-hover:scale-400 group-hover:translate-x-[-30px] transition-all duration-300 animate-bounce" src=(LOGO) alt=(ME.name)>
                                <span class="text-purple-light text-sm sm:text-base mr-4">(&post.author)</span>
                            </span>
                        </div>
                    </div>
                    <article class="border-x border-fg px-4 py-6">
                        <div class="markdown">(markdown::to_html(&post.content))</div>
                        credits()
                    </article>
                },
                None => {
                    <div class="border border-fg p-4 relative">
                        <h1 class="text-3xl font-bold text-fg mb-3">"Post Not Found :("</h1>
                    </div>
                    <article class="border-x border-fg px-4 py-6">
                        "This post doesn't exist..."
                    </article>
                },
            }

            <a
                href=(href!(blog::page))
                class="group border border-fg p-4 hover:bg-purple-light hover:scale-105 transition-all duration-100 text-fg"
            >
                "← Back to the blog list."
            </a>
        </div>
    })
}
