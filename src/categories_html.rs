use crate::projects::{Category, Project};

#[inline(always)]
pub fn create_category_carousel_html(
	category: &Category,
	projects: &Vec<Project>,
	i: usize,
) -> String {
	let mut project_html = String::new();

	for project_id in &category.projects {
		let project = &projects[project_id.0 as usize];
		let name = project.name.clone();
		let id = project_id.0 as usize;
		let image = project.pictures.clone();

		project_html.push_str(&format!(
			r#"
        <a class="project-card" href="/project/{id}">
        <img src="/{image}.png" alt="{name}">
        <span>{name}</span>
      	</a>
      "#
		));
	}

	let name = category.name.clone();

	format!(
		r#"
        <section class="category">
          <div class="center">
            <div class="wrapped-container">
              <h2>{name}</h2>
              <div class="carousel-wrapper">
                <div class="carousel">
									{project_html}
									{project_html}
                </div>
              </div>
            </div>
          </div>
        </section>
    "#
	)
}

#[inline(always)]
pub fn create_category_html(
	categories: &Vec<Category>,
	projects: &Vec<Project>,
	header: &str,
	footer: &str,
) -> String {
	let mut carousel_html = String::new();

	for (i, category) in categories.iter().enumerate() {
		carousel_html.push_str(&create_category_carousel_html(category, projects, i));
	}

	format!(
		r#"
        <!DOCTYPE html>
        <html lang="en">
          <head>
            <meta charset="UTF-8">
            <title>Projects</title>
            <link rel="stylesheet" href="styles.css">
          </head>
          <body>
					{header}
					{carousel_html}
					{footer}
          </body>
        </html>
        "#
	)
}
