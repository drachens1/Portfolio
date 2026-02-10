use std::sync::Arc;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CategoryId(pub u8);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ProjectId(pub u8);

pub struct Project {
    pub name: String,
    pub pictures: String,
    pub description: String,
    pub link: String,
    pub category_id: CategoryId,
}

pub struct Category {
    pub name: String,
    pub projects: Vec<ProjectId>
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
    pub fn get(&self, project_id: ProjectId) -> Option<&Project> {
        self.projects.get(project_id.0 as usize)
    }
}

pub struct CentralManager {
    pub project_manager: Arc<ProjectManager>,
    pub category_manager: Arc<CategoryManager>,
}
