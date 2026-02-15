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

	// Generate gallery images
	let images_html = project
		.pictures
		.iter()
		.map(|img| {
			format!(r#"<img class="gallery-image" src="/{}.png" alt="{}">"#, img, name)
		})
		.collect::<Vec<_>>()
		.join("\n");

	format!(r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{name}</title>
    <link rel="stylesheet" href="../styles.css">
</head>
<body>
{header}

<div class="left">
    <p></p>
    <a href="../" class="return">return</a>
</div>

<p></p>

<div class="center">
    <div class="vertical">
        <h1 class="center"><u>{name}</u></h1>

        <div class="gallery">
            {images_html}
        </div>

        <a href="{link}" class="center-nav-link a">Github</a>
        <p class="center">{description}</p>
    </div>
</div>

{footer}
</body>
</html>
"##)
}
