mod projects;
mod project_html;
mod categories_html;

use std::sync::Arc;
use EasyWebsiteBuilder2::page::Page;
use EasyWebsiteBuilder2::website::Website;
use crate::categories_html::create_category_html;
use crate::project_html::create_project_html;
use crate::projects::{Category, CategoryId, CategoryManager, CentralManager, Project, ProjectId, ProjectManager};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut website = Website::new(
        Page::new_from_html_str("/", include_str!("../pages/landing.html"), 0),
        Page::new_from_html_str("/404", include_str!("../pages/404.html"), 0));
    website
      .add_page(Page::new_css("/styles.css", include_str!("../pages/styles.css")))
      .add_page(Page::new_from_html_str("/about", include_str!("../pages/about.html"), 0))
      .add_page(Page::new_png("/project.png", Vec::from(include_bytes!("../pages/project.png"))));

    let header = include_str!("header");
    let footer = include_str!("footer");

    let manager = create_central_manager();

    let category = create_category_html(&manager.category_manager.categories, &manager.project_manager.projects, header, footer);
    website.add_page(Page::new_from_html_str("/projects", &category, 0));

    for (id, project) in manager.project_manager.projects.iter().enumerate() {
        website.add_page(Page::new_from_html_str(&format!("/project/{}", id), &create_project_html(project, id, header, footer), 0));
    }

    website.start([0, 0, 0, 0], 8080).await;
}

fn create_central_manager() -> Arc<CentralManager> {
    let project_manager = Arc::new(ProjectManager {
        projects: vec![
            //Game

            Project {
                name: "A Corporate World".into(),
                pictures: "project".into(),
                description: "This is an example project.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {
                name: "Corporate World Builder".into(),
                pictures: "project".into(),
                description: "This is an example project.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {
                name: "Top Down School".into(),
                pictures: "project".into(),
                description: "This is an example project.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {
                name: "World Peace 2".into(),
                pictures: "project".into(),
                description: "This is an example project.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {
                name: "Bayograde 2".into(),
                pictures: "project".into(),
                description: "This is an example project.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {
                name: "Bayograde 1".into(),
                pictures: "project".into(),
                description: "This is an example project.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {
                name: "Better Civ".into(),
                pictures: "project".into(),
                description: "This is an example project.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },

            //Web

            Project {
                name: "Bayograde Web".into(),
                pictures: "project".into(),
                description: "This is an example project.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {
                name: "Flashcards CrossPlatform".into(),
                pictures: "project".into(),
                description: "This is an example project.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {
                name: "Portfolio".into(),
                pictures: "project".into(),
                description: "This is an example project.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {
                name: "Website Builder".into(),
                pictures: "project".into(),
                description: "This is an example project.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },

            //Software

            Project {
                name: "Flashcards Android".into(),
                pictures: "project".into(),
                description: "This is an example project.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },

            //Engines

        ],
    });

    let category_manager = Arc::new(CategoryManager {
        categories: vec![
            Category::new("Game", vec![0,1,2,3,4,5,6]),
            Category::new("Web", vec![8,9,10]),
            Category::new("Software", vec![11]),
        ],
    });

    Arc::new(CentralManager {
        project_manager,
        category_manager,
    })
}

