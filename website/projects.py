from glob import glob
import yaml
import os.path

_projects = []


def parse_projects(directory):
    """Parse all of the projects described by the YAML in a directory."""
    file_names = glob(os.path.join(directory, '*.yaml'))
    for file_name in file_names:
        _projects.append(parse_project(file_name))


def parse_project(file_name):
    """Return an object that contains the same information as the YAML."""
    with open(file_name) as f:
        project = yaml.load(f)
    return project


def get_projects():
    """Returns all projects that have been parsed."""
    return _projects
