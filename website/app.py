from collections import namedtuple
import datetime
import httplib
import itertools
import os
from os import path

from flask import Flask, render_template, request, send_from_directory, url_for

import blog


def create_app():
    app = Flask(__name__)
    app.config.from_object(__name__)
    app.jinja_env.trim_blocks = True
    app.jinja_env.autoescape = False
    blog.parse_posts(path.join(os.getcwd(), app.static_folder, 'blog'))
    return app

app = create_app()


@app.errorhandler(httplib.NOT_FOUND)
def page_not_found(e):
    return render_template('not_found.html'), httplib.NOT_FOUND


@app.route('/robots.txt')
@app.route('/favicon.ico')
def static_from_root():
    """Serves files expected to be at the web root out of the static folder."""
    return send_from_directory(app.static_folder, request.path[1:])


@app.route('/')
def index():
    recent_posts = itertools.islice(blog.posts(), 3)
    return render_template('index.html', recent_posts=recent_posts)


@app.route('/about')
def about_me():
    # We need to find all the slideshow images and list them
    slideshow_url = url_for('static', filename='images/slideshow/')
    slideshow_rel_path = path.normpath(slideshow_url.strip('/'))
    slideshow_dir = path.join(os.getcwd(), slideshow_rel_path)
    image_urls = [slideshow_url + image for image in os.listdir(slideshow_dir)]
    return render_template('about.html', image_urls=image_urls)


@app.route('/blog/')
def render_blog():
    return render_template('blog.html', posts=blog.posts())


@app.route('/blog/<int:year>/<int:month>/<int:day>/<title>')
def show_post(year, month, day, title):
    date = datetime.date(year, month, day)
    return render_template('blog_post.html', post=blog.get_post(date, title))


def get_projects():
    """Returns a list of objects representing the current projects."""

    # Simple data wrapper for Projects.
    # Parameters:
    #   name : The full name of the project
    #   location : The location of the project on the filesystem
    #   description : The full description of the project
    #   language : The main programming language of the project
    #   repo : The github repo that contains the project's source
    #   available : True if the site has a binary of the project
    #   needs_fs : True if the project needs access to the filesystem to run
    Project = namedtuple('Project', ['name', 'location', 'description',
                                     'language', 'repo', 'available',
                                     'needs_fs'])

    projects = []
    projects.append(Project(
        name='Doodler', language='Java', repo='doodler',
        available=True, needs_fs=True,
        location=url_for('serve_jnlp', folder='doodler', repo='doodler'),
        description=(
            'A drawing application. Sketch a doodle, and save '
            'it to your computer for later. There is a variety of tools '
            'including different shapes and colors.')))
    projects.append(Project(
        name='Learn A Language', language='Java', repo='learnalanguage',
        available=False, needs_fs=True,
        location=None,
        description=(
            'A program that aims to teach beginners the Java programming '
            'language. It walks users through making their first Java '
            'programs in an interactive terminal, and allows them to compile '
            'and run their code while receiving feedback. Inspired by <a '
            'href="' 'http://codecademy.com">Codecademy</a>.')))
    return projects


@app.route('/static/bin/<folder>/<repo>.jnlp')
def serve_jnlp(folder, repo):
    projects = get_projects()
    project = next(project for project in projects if project.repo == repo)
    return render_template('bin/template.jnlp', project=project)


@app.route('/projects')
def projects():
    return render_template('projects.html', projects=get_projects())


@app.route('/resume')
def resume():
    return render_template('resume.html')

if __name__ == '__main__':
    app.run(debug=False)
