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

	let images_html = project.pictures.iter()
		.map(|img| {
			format!(r#"<img class="gallery-image" src="/{}.webp" alt="{name}" width="400" height="300">"#, img)
		})
		.collect::<Vec<_>>()
		.join("\n");

	format!(r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{name}</title>

    <link rel="preload" href="/styles.css" as="style" onload="this.rel='stylesheet'">
    <noscript><link rel="stylesheet" href="/styles.css"></noscript>
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

            <b><a href="{link}" class="center-nav-link">Github Src Code</a></b>
            <p>{description}</p>
        </div>
    </div>
</main>
{footer}
</body>
</html>
"##)
}
