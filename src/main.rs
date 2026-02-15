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
        for picture in &project.pictures {
            website.add_page(Page::new_webp(format!("/{}.webp",picture), Pages::get(&format!("{picture}.webp")).unwrap().data.to_vec()));
        }
    }

    website.start([0, 0, 0, 0], 8080).await;
}

#[derive(rust_embed::RustEmbed)]
#[folder = "pages/"]
struct Pages;

fn create_central_manager() -> Arc<CentralManager> {
    let mut project_manager = ProjectManager::new();

    let category_manager = Arc::new(CategoryManager {
        categories: vec![
            Category::new_project("GAME", &mut project_manager, CategoryId(0), vec![
                Project::new("TOP DOWN SCHOOL", "top_down_school", "At computer science A level they asked me to do a mini-nea so I decided to do this. It is coded in python which even though I don't really know, I set an ambitious goal. Of trying to make a top down strategy game with pathfinding and enemy ai when you click the troop and tell it where to go", "https://github.com/drachens1/TopDownSchool4"),
                Project::new("BETTER CIV", "civ", "Better civ was meant to be my answer to civlisation but the game engine effectively failed, although I made an entire packets and client server communication in this.", "https://github.com/drachens1/BetterCiv"),
                Project::new_pictures("A CORPORATE WORLD", vec!["corporate_world", "corporate_old", "corporate_oldest_country", "corporate_world_oldest"], "\
                <p></p>This project is built using Rust and WGSL (via Glfw/Wgpu). This has been a six-month deep dive into the mechanics of game engines and GPU programming. It has undergone multiple rendering overhauls as I’ve experimented with visual styles; currently, I’m focusing on an ultra-modern, executive-room aesthetic. You can see the evolution of this above. On the left is the current look of the game, but the background and chair colours will change; it also has multiple animations that I cannot show with images. \
                <p></p>It is my biggest project by far, in terms of scope and number of lines of code (coming in at 27,783). It has a built-in modding api, that uses a snapshot system to communicate with the engine and simulation, where the modding api is free of dependencies to minimize the size of mods. It also has a mini economic system that I have not implemented visually.\
                <p></p>Due to how satisfying I find doing minor optimizations (and the necessity since I don’t have a good pc) it has led to the game having minimal gpu and cpu usage, it has also led to me understanding many weird aspects of gpu coding with wgpu, like the 4 byte alignment rule and when and where it is correct to use f16’s, this has also meant I have coded many of the Vec and Mat’s for f16’s so that It doesn’t get translated every render frame and instead almost all the rendering code is in f16’s. ", "https://example.com"),
                Project::new("WORLD PEACE 2", "world_peace", "This is one of my older java projects, in 2d with my very own game engine and it is a port of the world peace 1, that I coded in lib gdx, it also has a further port called RustifiedWorldPeace.", "https://github.com/drachens1/WorldPeace2"),
                Project::new_pictures("BAYOGRADE", vec!["bayograde", "bayograde_factions", "bayograde_game_creation", "bayogradee_troops", "bayograde_modifiers"], "
                <p>The monumental coding project that got me into coding. Bayograde is a complete war game inside Minecraft with troops, research factories and entire alliance systems. I created it to combine my passions of war games, Minecraft and coding. It is my oldest project that I have had on GitHub since I started it in August 2024.</p>
<p>It is written in java using the Minestom framework and it totals 20,000 source code lines according to Statistic; I worked on it with another person although they didn’t commit that much.</p>
<p>It is still a very nostalgic project for me since it also taught me a lot about modelling in blockbench, although it isn’t my oldest coding project, but it is the oldest one, I still have access to.</p>", "https://github.com/drachens1/Bayograde"),
                Project::new("RUSTIFIEDWORLDPEACE", "rustified_world_peace", "This was one of my first bigger, rust projects. And it included porting my world peace into rust on macroquad. Their are different versions their is one of it in macroquad, and one in my own 2d game engine.", "https://github.com/drachens1/RustifiedWorldPeace3"),
            ]),

            Category::new_project("WEB", &mut project_manager, CategoryId(1), vec![
                Project::new_pictures("BAYOGRADE WEB", vec!["bayograde_web", "bayograde_purchase"], "So, this was an add-on to the bayograde minecraft server. It has a fully working shop system where you can buy ranks that the server would then send securely to itself but the minecraft side and it fully worked using the stripe api, but I always just used the developer debugging system so I imagine it would work in reality.", "https://github.com/drachens1/BayogradeWeb"),
                Project::new("FLASHCARDS CROSSPLATFORM", "flashcards_cross_platform", "This was mainly an extension of my android flashcards platform to see if I could code it cross platform by using the same central code and then merely changing how it is represented. It ended up just having a working login system with encryption and databasess.", "https://github.com/drachens1/FlashcardsCrossPlatform"),
                Project::new("PORTFOLIO", "project", "This..", "https://github.com/drachens1/Portfolio"),
                Project::new("WEBSITE BUILDER", "website_builder", "Website builder is just a very easy way to add different pages in a project, it is coded in rust and pre-compresses all the pages so that it can send them off quicker.", "https://github.com/drachens1/EasyWebsiteBuilder2"),
            ]),

            Category::new_project("SOFTWARE", &mut project_manager, CategoryId(2), vec![
                Project::new("FLASHCARDS ANDROID", "flashcards", "A system with flashcards and many different question types where you could save the flashcards on your device and it had a folder system that you could easily navigate, and it used my old Java save-io not my newer rust one.", "https://github.com/drachens1/Flashcards"),
                Project::new("UI DESIGNER", "ui_designer", "This was another thing I coded in the hopes of saving time that instead only costed time as I realised I would much rather hard code my UI and build up that system, and give it to the modding api as a support rather than making a json system or with my binary. It was also inferior compared to many other ui software like figma.", "https://github.com/drachens1/UIDesigner"),
                Project::new("NOTEPAD", "notepad", "This was my first android development project, the goal was just to make a very simple working notepad, and it succeeded, I did use my save io system instead of txt's because I thought the savings in memory would more than make up for it not being shareable.", "https://github.com/drachens1/Notepad"),
                Project::new("CORPORATE WORLD BUILDER", "corporate_world_builder", "This is a settings builder for a corporate world made with macroquad, It basically just offers a UI way to add buildings and configure all the research and products needed for a corporate world.", "https://example.com"),
            ]),

            Category::new_project("OTHER", &mut project_manager, CategoryId(3), vec![
                Project::new("YT BLOCKER", "yt", "This is a web extension that on firefox blocks many of the more addictive qualities of youtube. I coded it because I wanted to get more coding done and less youtube watching.", "https://github.com/drachens1/YT-Blocker"),
                //Project::new("Save IO - Rust", "save_io_rust", "This is my far superior and advanced save io system since not only does it use variable integers it also uses variable floats, is optimized for float arrays and hasá better and quicker compression system.", "https://example.com"),
                Project::new("SAVE IO - JAVA", "save_io_java", "This is my older save io system and it uses variable integers to save memory, it is way better for smaller projects and saves allot of memory.", "https://github.com/drachens1/Bayograde2/blob/master/src/main/java/org/drachens/files/filetypes/SaveIO.java"),
                Project::new("WGSL TRANSLATOR", "wgsl", "What this does is, you input rust code into the macro for how you want a shader to work then the macro generates a wgsl file of what the wgsl side is and it generates an entire rust class with an impl than supplies the intiial creation and render part. I thought, at the time that this project would save me tones of time since I could write it in rust and both the wgsl and the rust side would work but it was an extremely hard project that barely works and would output inefficient wgsl code anyway since their are many ways to write optimal code depending on the situation.", "https://github.com/drachens1/WgslWriter"),
                Project::new("OTHER MACROS", "other_macros", "This includes my, getters / setters and my #[Inline. Firstly the #[Inline] simply just makes every fn in the impl have #[inline(always)]. Secondly the geeters just creates gets for each one with 3 options of get mut or just get, also the setter does an equivalent.", "https://example.com"),
                //Project::new("Example Onomics", "example_onomics", "...", "https://example.com"),
                Project::new("CONTINENTAL MOD", "continental_mod", "Intended to be a minecraft mod that enabled continental which was later renamed to bayograde, to allow for fully complex models and for the game to be far better", "https://github.com/drachens1/ContinentalMod"),
                Project::new("HYPRLAND INSTALLER", "hyprland", "I use arch linux on my OS and I reinstall it ever so often so I thought that it would be nice to keep my configurations between installs and save me a significant amount of time. So it simply just uses commands to install and replace the hyprland config file. It is slightly outdated.", "https://github.com/drachens1/HyprlandInstaller"),
            ]),

            Category::new_project("ENGINES", &mut project_manager, CategoryId(4), vec![
                Project::new("SIMPLE ENGINE", "simple_engine", "Simple engine, was one of my first game projects and it uses lwjgl which effectively just has glsl and a few other helpful things for java.", "https://github.com/drachens1/SimpleEngine"),
                Project::new("RUST ENGINE FRAME", "engine_frame", "...", "https://example.com"),
                Project::new("GENESIS", "genesis_engine", "One of my first simple 2d engines, made in java", "https://github.com/drachens1/GenesisEngine"),
                Project::new("CIV ENGINE", "civ_engine", "Civ engine also uses lwjgl library like simple engine but it has been changed to be purely 3d and for a civ 6 style game, but lwjgl seems to have a bug that means the window never opens on NVIDIA linux devices so I cannot view it.", "https://github.com/drachens1/CivEngine"),
                Project::new("RUST ENGINES (1-6)", "rust_engine", "My rust engines, are a series of engines that I used to learn and understand how game engines function and work, they gradually get more complex for example moving more and more to the GPU and more 3d. It also demonstrates my switch to wgpu instead of glsl. As wgpu is a more modern library that scales significantly better.", "https://github.com/drachens1/RustEngine2"),
            ]),
        ],
    });

    Arc::new(CentralManager {
        project_manager: Arc::new(project_manager),
        category_manager,
    })
}

// pub fn flush_image(entry: String, quality: f32) {
//     let path = Path::new(&entry);
//
//     if path.extension().and_then(|e| e.to_str()) == Some("webp") {
//         println!("Processing {:?}", path);
//
//         let bytes = fs::read(&path).expect("Failed to read WebP file");
//         let img = image::load_from_memory_with_format(&bytes, ImageFormat::WebP)
//           .expect("Failed to decode WebP");
//
//         let (orig_width, orig_height) = img.dimensions();
//         let target_width = if orig_width < 300 {
//             300
//         } else if orig_width > 400 {
//             400
//         } else {
//             orig_width
//         };
//
//         let target_height = (orig_height as f32 * target_width as f32 / orig_width as f32).round() as u32;
//
//         let img = img.resize(target_width, target_height, FilterType::Lanczos3);
//
//         let rgba = img.to_rgba8();
//         let encoder = Encoder::from_rgba(rgba.as_raw(), img.width(), img.height());
//         let compressed = encoder.encode(quality);
//
//         fs::write(&path, &*compressed).expect("Failed to write WebP file");
//     } else {
//         println!("Skipped {:?}, not a .webp file", path);
//     }
// }
