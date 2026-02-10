mod projects;

use std::sync::Arc;
use EasyWebsiteBuilder2::page::Page;
use EasyWebsiteBuilder2::website::Website;
use crate::projects::{Category, CategoryId, CategoryManager, CentralManager, Project, ProjectId, ProjectManager};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut website = Website::new(
        Page::new_from_html_str("/", include_str!("../pages/landing.html"), 0),
        Page::new_from_html_str("/404", include_str!("../pages/404.html"), 0));
    website
      .add_page(Page::new_css("/styles.css", include_str!("../pages/styles.css")));


    let manager = create_central_manager();

    for (id, category) in manager.category_manager.categories.iter().enumerate() {
        let mut html = String::with_capacity(1024);
        html.push_str(&format!("<h1>Category: {}</h1><ul>", category.name));

        for project_id in &category.projects {
            if let Some(project) = manager.project_manager.get(*project_id) {
                html.push_str(&format!(r#"<li><a href="/project/{}">{}</a></li>"#, project_id.0, project.name));
            }
        }

        html.push_str("</ul>");
        website.add_page(Page::new_from_html_str(&format!("/category/{}", id), &html, 0));
    }

    for (id, project) in manager.project_manager.projects.iter().enumerate() {
        let category_name = manager
          .category_manager
          .get(project.category_id)
          .map(|c| c.name.as_str())
          .unwrap_or("Unknown");

        let html = format!(
            r#"
            <h1>{name}</h1>
            <p><strong>Category:</strong> {category}</p>
            <p>{description}</p>
            <p><a href="{link}" target="_blank">Visit project</a></p>
            "#,
            name = project.name,
            category = category_name,
            description = project.description,
            link = project.link,
        );

        website.add_page(Page::new_from_html_str(&format!("/project/{}", id), &html, 0));
    }


    website.start([0, 0, 0, 0], 8080).await;
}

fn create_central_manager() -> Arc<CentralManager> {
    let project_manager = Arc::new(ProjectManager {
        projects: vec![
            Project {
                name: "Example Project".into(),
                pictures: "example.png".into(),
                description: "This is an example project.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
        ],
    });

    let category_manager = Arc::new(CategoryManager {
        categories: vec![
            Category {
                name: "Web".into(),
                projects: vec![ProjectId(0)],
            },
        ],
    });

    Arc::new(CentralManager {
        project_manager,
        category_manager,
    })
}

