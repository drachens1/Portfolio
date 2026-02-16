use crate::projects::Project;

#[inline(always)]
pub fn create_project_html(
	project: &Project,
	_i: usize,
	header: &str,
	footer: &str,
) -> String {
	let name = project.name.as_str();
	let description = project.description.as_str();
	let link = project.link.as_str();

	let link_html = if name == "A CORPORATE WORLD" {
		r#""#
	} else {
		&format!(r#"<b><a href="{link}" class="center-nav-link">Github Src Code</a></b>"#)
	};

	let images_html = project.pictures.iter()
		.map(|img| {
			format!(r#"<img class="gallery-image" src="/{}.webp" alt="{name}" width="400" height="300" loading="lazy" decoding="async">"#, img)
		})
		.collect::<Vec<_>>()
		.join("\n");

	format!(r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{name}</title>
		<link rel="stylesheet" href="/styles.css">
</head>
<body>
{header}
<main>
    <div class="left">
        <a href="../" class="return">return</a>
    </div>

    <div class="center">
        <div class="vertical">
            <h1><u>{name}</u></h1>

            <div class="gallery">
                {images_html}
            </div>

            {link_html}
            <p>{description}</p>
        </div>
    </div>
</main>
{footer}
</body>
</html>
"##)
}
