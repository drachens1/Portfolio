use crate::projects::Project;

#[inline(always)]
pub fn create_project_html(project: &Project, i: usize, header: &str, footer: &str) -> String {
	let name = project.name.as_str();
	let description = project.description.as_str();
	let image = project.pictures.as_str();
	let link = project.link.as_str();

	format!(r#"
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
         <a href="../projects" class = "return">return</a>
         </div>
         <p></p>
         <div class= "center">
         			<div class="vertical">
								<h1 class = "center"><u>{name}</u></h1>
								<a class="project-card">
									<img src="/{image}.png" alt="{name}">
								</a>
								<a href="{link}" class="center-nav-link a">Github</a>
								<p class = "center">{description}</p>
              </div>
         </div>
         {footer}
      </body>
		  </html>
	"#)
}


