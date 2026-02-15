mod projects;
mod project_html;
mod categories_html;

use crate::categories_html::create_category_html;
use crate::project_html::create_project_html;
use crate::projects::{Category, CategoryId, CategoryManager, CentralManager, Project, ProjectManager};
use std::sync::Arc;
use EasyWebsiteBuilder2::page::Page;
use EasyWebsiteBuilder2::website::Website;

#[tokio::main]
async fn main() {
    let header = include_str!("header");
    let footer = include_str!("footer");

    let manager = create_central_manager();

    let category = create_category_html(&manager.category_manager.categories, &manager.project_manager.projects, header, footer);

    let mut website = Website::new(
        Page::new_from_html_str("/", &category, 0),
        Page::new_from_html_str("/404", include_str!("../pages/404.html"), 0));
    website
      .add_page(Page::new_css("/styles.css", include_str!("../pages/styles.css")))
      .add_page(Page::new_from_html_str("/about", include_str!("../pages/about.html"), 0));

    for (id, project) in manager.project_manager.projects.iter().enumerate() {
        website.add_page(Page::new_from_html_str(&format!("/project/{}", id), &create_project_html(project, id, header, footer), 0));
        website.add_page(Page::new_png_from_file(format!("/{}.png",project.pictures), format!("pages/{}.png", project.pictures)));
    }

    website.start([0, 0, 0, 0], 8080).await;
}

fn create_central_manager() -> Arc<CentralManager> {
    let mut project_manager = ProjectManager::new();

    let category_manager = Arc::new(CategoryManager {
        categories: vec![
            Category::new_project("Game", &mut project_manager, CategoryId(0), vec![
                Project::new("Top Down School", "top_down_school", "At computer science A level they asked me to do a mini-nea so I decided to do this. It is coded in python which even though I don't really know, I set an ambitious goal. Of trying to make a top down strategy game with pathfinding and enemy ai when you click the troop and tell it where to go", "https://example.com"),
                Project::new("Bayograde 2", "project", "...", "https://example.com"),
                Project::new("Better Civ", "civ", "Better civ was meant to be my answer to civlisation but the game engine effectively failed, although I made an entire packets and client server communication in this.", "https://example.com"),
                Project::new("A Corporate World", "corporate_world", "This is my main project and the biggest one I have ever done. It includes an entire 3d game engine as well as full UI support. It has went through many iterations of the game engine as shown by the images, and now it is going through a hologram look without UI in the same way. It has taken me since the start of Y12, to get to where I am at with near daily commits. It also has a fully mod api system that uses snapshots, and commands. It also has its own macro's that I reuse in other projects, for example my SaveIO system and accompanying macro that offer a binary alternative to the common json parsers, because in many cases the size matters more than its readability", "https://example.com"),
                Project::new("Corporate World Builder", "corporate_world_builder", "This is a settings builder for a corporate world made with macroquad, It basically just offers a UI way to add buildings and configure all the research and products needed for a corporate world.", "https://example.com"),
                Project::new("World Peace 2", "world_peace", "This is one of my older java projects, in 2d with my very own game engine and it is a port of the world peace 1, that I coded in lib gdx, it also has a further port called RustifiedWorldPeace.", "https://example.com"),
                Project::new("Bayograde", "bayograde", "This was the main project that actually got me into coding allot, since I always loved minecraft and war games, so I thought why not make a war game in minecraft. How it works is it uses a resource pack for the models and you use many different items to control your troops, design different troop designs research and go to war with other countries. It is also coded in minestom which is basically a very lightweight server software for minecraft.", "https://example.com"),
                Project::new("RustifiedWorldPeace", "rustified_world_peace", "This was one of my first bigger, rust projects. And it included porting my world peace into rust on macroquad. Their are different versions their is one of it in macroquad, and one in my own 2d game engine.", "https://example.com"),
            ]),

            Category::new_project("Web", &mut project_manager, CategoryId(1), vec![
                Project::new("Bayograde Web", "bayograde_web", "So, this was an add-on to the bayograde minecraft server. It has a fully working shop system where you can buy ranks that the server would then send securely to itself but the minecraft side and it fully worked using the stripe api, but I always just used the developer debugging system so I imagine it would work in reality.", "https://example.com"),
                Project::new("Flashcards CrossPlatform", "flashcards_cross_platform", "This was mainly an extension of my android flashcards platform to see if I could code it cross platform by using the same central code and then merely changing how it is represented. It ended up just having a working login system with encryption and databasess.", "https://example.com"),
                Project::new("Portfolio", "project", "...", "https://example.com"),
                Project::new("Website Builder", "website_builder", "Website builder is just a very easy way to add different pages in a project, it is coded in rust and pre-compresses all the pages so that it can send them off quicker.", "https://example.com"),
            ]),

            Category::new_project("Software", &mut project_manager, CategoryId(2), vec![
                Project::new("Flashcards Android", "flashcards", "A system with flashcards and many different question types where you could save the flashcards on your device and it had a folder system that you could easily navigate, and it used my old Java save-io not my newer rust one.", "https://example.com"),
                Project::new("UI Designer", "ui_designer", "This was another thing I coded in the hopes of saving time that instead only costed time as I realised I would much rather hard code my UI and build up that system, and give it to the modding api as a support rather than making a json system or with my binary. It was also inferior compared to many other ui software like figma.", "https://example.com"),
                Project::new("Notepad", "notepad", "This was my first android development project, the goal was just to make a very simple working notepad, and it succeeded, I did use my save io system instead of txt's because I thought the savings in memory would more than make up for it not being shareable.", "https://example.com"),
            ]),

            Category::new_project("Other", &mut project_manager, CategoryId(3), vec![
                Project::new("YT Blocker", "yt", "This is a web extension that on firefox blocks many of the more addictive qualities of youtube. I coded it because I wanted to get more coding done and less youtube watching.", "https://example.com"),
                Project::new("Save IO - Rust", "save_io_rust", "This is my far superior and advanced save io system since not only does it use variable integers it also uses variable floats, is optimized for float arrays and hasá better and quicker compression system.", "https://example.com"),
                Project::new("Save IO - Java", "save_io_java", "This is my older save io system and it uses variable integers to save memory, it is way better for smaller projects and saves allot of memory.", "https://example.com"),
                Project::new("Wgsl translator", "wgsl", "What this does is, you input rust code into the macro for how you want a shader to work then the macro generates a wgsl file of what the wgsl side is and it generates an entire rust class with an impl than supplies the intiial creation and render part. I thought, at the time that this project would save me tones of time since I could write it in rust and both the wgsl and the rust side would work but it was an extremely hard project that barely works and would output inefficient wgsl code anyway since their are many ways to write optimal code depending on the situation.", "https://example.com"),
                Project::new("Other macros", "other_macros", "This includes my, getters / setters and my #[Inline. Firstly the #[Inline] simply just makes every fn in the impl have #[inline(always)]. Secondly the geeters just creates gets for each one with 3 options of get mut or just get, also the setter does an equivalent.", "https://example.com"),
                Project::new("Example Onomics", "example_onomics", "...", "https://example.com"),
                Project::new("Continental Mod", "continental_mod", "Intended to be a minecraft mod that enabled continental which was later renamed to bayograde, to allow for fully complex models and for the game to be far better", "https://example.com"),
                Project::new("HyprlandInstaller", "hyprland", "I use arch linux on my OS and I reinstall it ever so often so I thought that it would be nice to keep my configurations between installs and save me a significant amount of time. So it simply just uses commands to install and replace the hyprland config file. It is slightly outdated.", "https://example.com"),
            ]),
        ],
    });

    Arc::new(CentralManager {
        project_manager: Arc::new(project_manager),
        category_manager,
    })
}
