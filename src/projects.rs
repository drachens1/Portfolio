use std::sync::Arc;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CategoryId(pub u8);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ProjectId(pub u8);

#[derive(Clone)]
pub struct Project {
    pub name: String,
    pub pictures: Vec<String>,
    pub description: String,
    pub link: String,
    pub category_id: CategoryId,
}
impl Project {
    pub fn new(name: impl Into<String>, pictures: impl Into<String>, description: impl Into<String>, link: impl Into<String>) -> Self {
        Project { name: name.into(), pictures: vec![pictures.into()], description: description.into(), link: link.into(), category_id: CategoryId(0) }
    }

    pub fn new_pictures(name: impl Into<String>, pictures: Vec<impl Into<String>>, description: impl Into<String>, link: impl Into<String>) -> Self {
        Project { name: name.into(), pictures: pictures.into_iter().map(|picture| picture.into()).collect(), description: description.into(), link: link.into(), category_id: CategoryId(0) }
    }
}

pub struct Category {
    pub name: String,
    pub projects: Vec<ProjectId>
}
impl Category {
    pub fn new(name: impl Into<String>, projects_u8: Vec<u8>) -> Self {
        let mut projects = Vec::new();
        for project in projects_u8 {
            projects.push(ProjectId(project));
        }
        Category { name: name.into(), projects }
    }

    pub fn new_project(name: impl Into<String>, project_manager: &mut ProjectManager, id: CategoryId, mut projects: Vec<Project>) -> Self {
        let project_ids = projects.iter_mut().map(|project| { project.category_id = id; project_manager.add(project.clone()) }).collect();
        Self {
            name: name.into(),
            projects: project_ids
        }
    }
}

pub struct CategoryManager {
    pub categories: Vec<Category>
}
impl CategoryManager {
    pub fn get(&self, category_id: CategoryId) -> Option<&Category> {
        self.categories.get(category_id.0 as usize)
    }
}

pub struct ProjectManager {
    pub projects: Vec<Project>
}
impl ProjectManager {
    pub fn new() -> Self {
        Self { projects: Vec::new() }
    }

    pub fn get(&self, project_id: ProjectId) -> Option<&Project> {
        self.projects.get(project_id.0 as usize)
    }

    pub fn add(&mut self, project: Project) -> ProjectId {
        let id = ProjectId(self.projects.len() as u8);
        self.projects.push(project);
        id
    }
}

pub struct CentralManager {
    pub project_manager: Arc<ProjectManager>,
    pub category_manager: Arc<CategoryManager>,
}
