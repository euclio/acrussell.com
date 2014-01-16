from glob import glob
import yaml
import os.path

_projects = None


class ProjectsNotParsedError(Exception):
    """Raised when the application requests project information without parsing
    it from the filesystem first."""


def parse_projects(directory):
    """Parse all of the projects described by the YAML in a directory."""
    file_names = glob(os.path.join(directory, '*.yaml'))
    global _projects
    _projects = [parse_project(file_name) for file_name in file_names]


def parse_project(file_name):
    """Return an object that contains the same information as the YAML."""
    with open(file_name) as f:
        project = yaml.load(f)
    return project


def get_projects():
    """Returns all projects that have been parsed."""
    global _projects
    if _projects is None:
        raise ProjectsNotParsedError
    return _projects
