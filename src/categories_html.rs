use crate::projects::{Category, Project};

#[inline(always)]
pub fn create_category_carousel_html(
	category: &Category,
	projects: &Vec<Project>,
	i: usize,
) -> String {
	let mut project_html = String::new();

	let priority = if i == 0 {
		r#"fetchpriority="high""#
	} else if i == 1 {
		r#"fetchpriority="medium""#
	} else {
		r#"loading="lazy" fetchpriority="low""#
	};

	for project_id in &category.projects {
		let project = &projects[project_id.0 as usize];
		let name = &project.name;
		let id = project_id.0 as usize;
		let image = project.pictures.get(0).unwrap();

		project_html.push_str(&format!(
			r#"
			<a class="project-card" href="/project/{id}">
					<div class="image-wrapper">
							<img src="/{image}.webp" alt="{name}" width="400" height="300" {priority} decoding="async">
					</div>
					<span>{name}</span>
			</a>
        "#
		));
	}

	let name = &category.name;

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

	for category in categories {
		carousel_html.push_str(&create_category_carousel_html(category, projects, 0));
	}

	format!(
		r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Projects</title>
		<link rel="stylesheet" href="/styles.css">
</head>
<body>
{header}
<main>
    {carousel_html}
</main>
{footer}
</body>
</html>
"#
	)
}
