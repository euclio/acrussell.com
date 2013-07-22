from collections import namedtuple
import datetime
import glob
import httplib
import itertools
import os
import re

from flask import *
import jinja2
from jinja2.filters import do_truncate, do_striptags

import markdown
import yaml

DEBUG = True
GOOGLE_SITE_VERIFICATION = os.environ.get('GOOGLE_SITE_VERIFICATION',
                                          '404.html')
app = Flask(__name__)
app.config.from_object(__name__)
app.jinja_env.trim_blocks = True
app.jinja_env.autoescape = False 
class Post(object):
    """Represents one post on the blog.

    Parameters:
        title : The "raw title" of the blog post. Used as a fallback when no
        title metadata is found.

        date : The "raw date" of the blog post. Used as a fallback when no date
        metadata is found.

        content : The text of the blog post. Can contain html tags.

        metadata : Optional. A dictionary containing additional data describing
        the post. Accepted fields are 'title', 'date', 'categories', and 'tags'

    """

    # Regex matching valid blog post files
    FILE_PATTERN = re.compile(r'(\d{4})-(\d{2})-(\d{2})-(.+)\.mdown\Z')

    def __init__(self, title, date, content, metadata):
        self._title = title
        self._date = date
        self._content = content
        self.metadata = metadata or {}
        self.url = url_for(
                'show_post',  year=self._date.year, month=self._date.month,
                day=self._date.day, title=self._title)

    @property
    def title(self):
        """Returns a displayable title for the post."""
        try:
            return self.metadata['title']
        except KeyError:
            return self._title.replace('-', ' ').title()

    @property
    def preview(self):
        """HTML representing a short preview of the post.

        Contains the first 200 characters of the post's content, followed by a
        link to the post itself.

        """
        preview_text = do_striptags(self._content)
        link = '... <a href="{}">Continue Reading&rarr;</a>'.format(self.url)
        preview_html = do_truncate(preview_text, length=200, end=link)
        return preview_html

    @property
    def content(self):
        """The content of the blog post."""
        return self._content

    @property
    def date(self):
        """Returns a formatted version of the date."""
        return self._date.strftime('%B %-d, %Y')


@app.errorhandler(httplib.NOT_FOUND)
def page_not_found(e):
    return render_template('not_found.html'), httplib.NOT_FOUND

@app.route('/' + GOOGLE_SITE_VERIFICATION)
@app.route('/robots.txt')
@app.route('/favicon.ico')
def static_from_root():
    """Serves files expected to be at the web root out of the static folder."""
    return send_from_directory(app.static_folder, request.path[1:])


@app.route('/')
def index():
    recent_posts = itertools.islice(blog_posts(), 4)
    return render_template('index.html', recent_posts=recent_posts)

@app.route('/about')
def about_me():
    slideshow_path = url_for('static', filename='images/slideshow/')
    image_names = os.listdir(os.getcwd() + slideshow_path)
    image_urls = [slideshow_path + name for name in image_names]
    return render_template('about.html', image_urls=image_urls)

@app.route('/blog/')
def blog():
    return render_template('blog.html', posts=blog_posts())

def get_post(year, month, day, title):
    """Returns a Post representing a given blog post.

    Each post is uniquely identified by its date and title.

    """
    posts_directory = url_for('static', filename='blog/')
    file_name = '{:04d}-{:02d}-{:02d}-{}.mdown'.format(year, month, day, title)
    abs_path = os.getcwd() + posts_directory + file_name
    return parse_post(abs_path)

def parse_post(abs_path):
    """Parses a Post object from a file name."""
    END_METADATA = '!END\n'

    with open(abs_path) as f:
        lines = f.readlines()

    metadata_yaml = None
    try:
        metadata_end_index = lines.index(END_METADATA)
    except ValueError:
        # If no metadata is found, then the entire file is Markdown
        content_markdown = ''.join(lines)
    else:
        metadata_yaml = ''.join(lines[:metadata_end_index])
        content_markdown = ''.join(lines[metadata_end_index+1:])

    metadata = yaml.load(metadata_yaml) if metadata_yaml else None
    content = markdown.markdown(content_markdown)

    # Extract the raw data from the file name
    file_name = abs_path.rsplit('/', 1)[-1]
    year, month, day, title = Post.FILE_PATTERN.match(file_name).groups()
    date = datetime.date(int(year), int(month), int(day))

    return Post(date=date, content=content, title=title, metadata=metadata)

def blog_posts():
    """Generates all blog posts from the blog/ directory."""
    posts_directory = os.getcwd() + url_for('static', filename='blog/')
    post_file_names = glob.glob(posts_directory + '*.mdown')
    for file_name in sorted(post_file_names, reverse=True):
        yield parse_post(file_name)

@app.route('/blog/<int:year>/<int:month>/<int:day>/<title>')
def show_post(year, month, day, title):
    return render_template('blog_post.html',
            post=get_post(year, month, day, title))

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
        'language', 'repo', 'available', 'needs_fs'])

    projects = []
    projects.append(Project(
            name='Doodler', language='Java', repo='doodler',
            available=True, needs_fs=True,
            location=url_for('serve_jnlp', folder='doodler', repo='doodler'),
            description = ('A drawing application. Sketch a doodle, and save '
            'it to your computer for later. There is a variety of tools '
            'including different shapes and colors.')))
    projects.append(Project(
            name='Learn A Language', language='Java', repo='learnalanguage',
            available=False, needs_fs=True,
            location=None,
            description = ('A program that aims to teach beginners the Java '
                'programming language. It walks users through making their '
                'first Java programs in an interactive terminal, and allows '
                'them to compile and run their code while receiving '
                'feedback. Inspired by <a href="'
                'http://codecademy.com">Codecademy</a>.')))
    return projects

@app.route('/static/bin/<folder>/<repo>.jnlp')
def serve_jnlp(folder, repo):
    projects = get_projects()
    project = next((project for project in projects if project.repo == repo))
    return render_template('bin/template.jnlp', project=project)

@app.route('/projects')
def projects():
    return render_template('projects.html', projects=get_projects())

@app.route('/resume')
def resume():
    return render_template('resume.html')

if __name__ == '__main__':
    app.run()
