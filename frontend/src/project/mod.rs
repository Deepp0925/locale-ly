use crate::utils::loading::LoadingState;

use locales::Locales;
mod locales;
/// Manages the project state open in the application
///
#[derive(Debug, Clone)]
pub struct Project {
    /// project name
    name: String,
    /// project path
    path: String,
    /// project locales
    locales: Vec<Locales>,
}

#[derive(Debug, Clone)]
pub struct ProjectState {
    /// project currently open
    pub project: LoadingState<Project>,
}

impl ProjectState {
    pub fn new() -> Self {
        Self {
            project: LoadingState::None,
        }
    }

    /// check if a project is currently open
    pub fn is_project_open(&self) -> bool {
        matches!(&self.project, LoadingState::Loaded(_))
    }

    /// get the current project
    pub fn get_project(&self) -> Option<&Project> {
        match &self.project {
            LoadingState::Loaded(project) => Some(project),
            _ => None,
        }
    }

    /// get the current project mut reference
    pub fn get_project_mut(&mut self) -> Option<&mut Project> {
        match &mut self.project {
            LoadingState::Loaded(project) => Some(project),
            _ => None,
        }
    }
}
