//! The blog index.

pub mod slug;

use topcoat::{
    Result,
    router::{
        href,
        page,
    },
    view::{
        View,
        view,
    },
};

use crate::posts;

/// Lists every post, newest first.
#[page]
pub async fn page() -> Result<impl View> {
    Ok(view! {
        <div class="w-full py-6">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 px-4 lg:px-0">
                for (index, post) in posts::all().iter().enumerate() {
                    let number = posts::all().len() - index;

                    <a
                        href=(href!(slug::page, slug::Slug(post.slug.clone())))
                        class="border border-fg p-4 hover:bg-purple-light transition-scale hover:scale-105 lg:hover:scale-110 duration-100 hover:z-10"
                    >
                        <div class="mb-1">
                            <div class="flex justify-between">
                                <h1 class="text-xl font-semibold text-fg mb-3">(&post.title)</h1>
                                <p class="text-fg opacity-60">"#"(number)</p>
                            </div>
                            <p class="text-fg opacity-70 mb-3">(&post.brief)</p>
                            <p class="text-fg opacity-60">(&post.date)</p>
                        </div>
                    </a>
                }
            </div>
        </div>
    })
}
