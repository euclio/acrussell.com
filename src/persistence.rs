//! Data to be used with a persistent router.

use iron::typemap::Key;

use projects::Project;

/// Contains all projects that will be displayed on the site.
#[derive(Copy, Clone)]
pub struct Projects;

impl Key for Projects {
    type Value = Vec<Project>;
}
