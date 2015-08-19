from collections import namedtuple
import logging
import os
import os.path

from github import Github, GithubException
import yaml


class Project(object):

    def __init__(self, name, repo):
        self.name = name
        self.owner = repo.owner
        self.url = repo.url
        self.languages = repo.languages
        self.description = repo.description


Repo = namedtuple('Repo', ['owner', 'url', 'languages', 'description'])


def parse(config_file):
    github = Github(
        os.environ['GITHUB_USERNAME'], os.environ['GITHUB_PASSWORD'])
    with open(config_file) as config:
        projects_yaml = yaml.load(config)['projects']

        projects = []

        for project in projects_yaml:
            name = project['name']
            custom_description = project.get('description')
            (username, _, repo_name) = project['repo'].partition('/')
            try:
                gh_repo = github.get_user(login=username).get_repo(repo_name)
                languages = list(gh_repo.get_languages().keys())
                description = custom_description or gh_repo.description
                repo = Repo(username, gh_repo.html_url, languages, description)

            except GithubException as ghe:
                logging.exception(ghe)
                # Attempt to guess some information
                repo = Repo(username, 'http://github.com/' + repo_name,
                            [], custom_description)

            projects.append(Project(name, repo))

    return projects
