//! The full project list.

use libs::data::PROJECTS;
use topcoat::{
    Result,
    router::page,
    view::{
        View,
        view,
    },
};

use crate::components::project_card;

/// Lists every project.
#[page]
pub async fn page() -> Result<impl View> {
    Ok(view! {
        <div class="w-full py-6">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 px-4 lg:px-0">
                for project in PROJECTS {
                    project_card(project: project)
                }
            </div>
        </div>
    })
}
