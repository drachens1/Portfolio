mod projects;
mod project_html;
mod categories_html;

use std::fmt::format;
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
        website.add_page(Page::new_png_from_file(format!("{}",project.pictures), format!("pages/{}.png", project.pictures)));
    }

    website.start([0, 0, 0, 0], 8080).await;
}

fn create_central_manager() -> Arc<CentralManager> {
    let project_manager = Arc::new(ProjectManager {
        projects: vec![
            //Game
            Project {//2
                name: "Top Down School".into(),
                pictures: "project".into(),
                description: "At computer science A level they asked me to do a mini-nea so I decided to do this. It is coded in python which even though I don't really know, I set an ambitious goal. Of trying to make a top down strategy game with pathfinding and enemy ai when you click the troop and tell it where to go".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//4
                name: "Bayograde 2".into(),
                pictures: "project".into(),
                description: "This is an example project.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//6
                name: "Better Civ".into(),
                pictures: "project".into(),
                description: "Better civ was meant to be my answer to civlisation but the game engine effectively failed, although I made an entire packets and client server communication in this.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//0
                name: "A Corporate World".into(),
                pictures: "project".into(),
                description: "This is my main project and the biggest one I have ever done. It includes an entire 3d game engine as well as full UI support. It has went through many iterations of the game engine as shown by the images, and now it is going through a hologram look without UI in the same way. It has taken me since the start of Y12, to get to where I am at with near daily commits. It also has a fully mod api system that uses snapshots, and commands. It also has its own macro's that I reuse in other projects, for example my SaveIO system and accompanying macro that offer a binary alternative to the common json parsers, because in many cases the size matters more than its readability".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//1
                name: "Corporate World Builder".into(),
                pictures: "project".into(),
                description: "This is a settings builder for a corporate world made with macroquad, It basically just offers a UI way to add buildings and configure all the research and products needed for a corporate world.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//3
                name: "World Peace 2".into(),
                pictures: "project".into(),
                description: "This is one of my older java projects, in 2d with my very own game engine and it is a port of the world peace 1, that I coded in lib gdx, it also has a further port called RustifiedWorldPeace.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//5
                name: "Bayograde".into(),
                pictures: "project".into(),
                description: "This was the main project that actually got me into coding allot, since I always loved minecraft and war games, so I thought why not make a war game in minecraft. How it works is it uses a resource pack for the models and you use many different items to control your troops, design different troop designs research and go to war with other countries. It is also coded in minestom which is basically a very lightweight server software for minecraft.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//7
                name: "RustifiedWorldPeace".into(),
                pictures: "project".into(),
                description: "This was one of my first bigger, rust projects. And it included porting my world peace into rust on macroquad. Their are different versions their is one of it in macroquad, and one in my own 2d game engine.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },

            //Web

            Project {//8
                name: "Bayograde Web".into(),
                pictures: "project".into(),
                description: "So, this was an add-on to the bayograde minecraft server. It has a fully working shop system where you can buy ranks that the server would then send securely to itself but the minecraft side and it fully worked using the stripe api, but I always just used the developer debugging system so I imagine it would work in reality.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//9
                name: "Flashcards CrossPlatform".into(),
                pictures: "project".into(),
                description: "This was mainly an extension of my android flashcards platform to see if I could code it cross platform by using the same central code and then merely changing how it is represented. It ended up just having a working login system with encryption and databasess.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//10
                name: "Portfolio".into(),
                pictures: "project".into(),
                description: "This project.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//11
                name: "Website Builder".into(),
                pictures: "project".into(),
                description: "Website builder is just a very easy way to add different pages in a project, it is coded in rust and pre-compresses all the pages so that it can send them off quicker.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },

            //Software
            Project {//12
                name: "Flashcards Android".into(),
                pictures: "project".into(),
                description: "A system with flashcards and many different question types where you could save the flashcards on your device and it had a folder system that you could easily navigate, and it used my old Java save-io not my newer rust one.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//13
                name: "UI Designer".into(),
                pictures: "project".into(),
                description: "This was another thing I coded in the hopes of saving time that instead only costed time as I realised I would much rather hard code my UI and build up that system, and give it to the modding api as a support rather than making a json system or with my binary. It was also inferior compared to many other ui software like figma.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//14
                name: "Notepad".into(),
                pictures: "project".into(),
                description: "This was my first android development project, the goal was just to make a very simple working notepad, and it succeeded, I did use my save io system instead of txt's because I thought the savings in memory would more than make up for it not being shareable.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },

            //Other
            Project {//15
                name: "Save IO - Rust".into(),
                pictures: "project".into(),
                description: "This is my far superior and advanced save io system since not only does it use variable integers it also uses variable floats, is optimized for float arrays and hasá better and quicker compression system.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),

            },
            Project {//16
                name: "Save IO - Java".into(),
                pictures: "project".into(),
                description: "This is my older save io system and it uses variable integers to save memory, it is way better for smaller projects and saves allot of memeory".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),

            },
            Project {//17
                name: "Wgsl translator".into(),
                pictures: "project".into(),
                description: "What this does is, you input rust code into the macro for how you want a shader to work then the macro generates a wgsl file of what the wgsl side is and it generates an entire rust class with an impl than supplies the intiial creation and render part. I thought, at the time that this project would save me tones of time since I could write it in rust and both the wgsl and the rust side would work but it was an extremely hard project that barely works and would output inefficient wgsl code anyway since their are many ways to write optimal code depending on the situation.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//18
                name: "Other macros".into(),
                pictures: "project".into(),
                description: "This includes my, getters / setters and my #[Inline. Firstly the #[Inline] simply just makes every fn in the impl have #[inline(always)]. Secondly the geeters just creates gets for each one with 3 options of get mut or just get, also the setter does an equivalent.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//19
                name: "Example Onomics".into(),
                pictures: "project".into(),
                description: "".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//20
                name: "Continental Mod".into(),
                pictures: "project".into(),
                description: "Intended to be a minecraft mod that enabled continental which was later renamed to bayograde, to allow for fully complex models and for the game to be far better".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },
            Project {//21
                name: "HyprlandInstaller".into(),
                pictures: "project".into(),
                description: "I use arch linux on my OS and I reinstall it ever so often so I thought that it would be nice to keep my configurations between installs and save me a significant amount of time. So it simply just uses commands to install and replace the hyprland config file. It is slightly outdated.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            },

            //Engines
            Project {//22
                name: "Rust Engine 1-5".into(),
                pictures: "project".into(),
                description: "This is a summary of all my rust engines and the justifications I had for each one. The first one was made, as a simple 2d engine mainly on the cpu. The second one was a more advanced 3d engine that was mainly on the cpu, then the third one was mainly on the gpu and 3d, And the fourth one was my wgpu rendering engine as I had learnt about it while looking for  engine software I should use. And as the scope of my plans grew I decided to settle on wgpu as it is more scalable and modern supporting many things glsl didn't support and although it requires more initial memory that would more than be made up for.".into(),
                link: "https://example.com".into(),
                category_id: CategoryId(0),
            }
        ],
    });

    //let mut project_manager = ProjectManager::new();

    let category_manager = Arc::new(CategoryManager {
        categories: vec![
            Category::new("Game", vec![0,1,2,3,4,5,6]),
            Category::new("Web", vec![8,9,10]),
            Category::new("Software", vec![11]),
            Category::new("Other", vec![12,13,14,15])
        ],
    });

    Arc::new(CentralManager {
        project_manager: project_manager,
        category_manager,
    })
}
