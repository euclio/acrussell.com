import datetime
import http.client
import itertools
import os

from flask import render_template, request, send_from_directory, url_for

from . import blog

import website
from website import app

# Read data from the filesystem
blog.parse_posts(
    os.path.join(os.getcwd(), app.static_folder, 'blog'))


@app.errorhandler(http.client.NOT_FOUND)
def page_not_found(e):
    return render_template('not_found.html'), http.client.NOT_FOUND


@app.route('/robots.txt')
@app.route('/favicon.ico')
def static_from_root():
    """Serves files expected to be at the web root out of the static folder."""
    return send_from_directory(app.static_folder, request.path[1:])


@app.route('/')
def index():
    recent_posts = itertools.islice(blog.posts(), 3)
    return render_template('index.html', posts=recent_posts)


@app.route('/about')
def about_me():
    # We need to find all the slideshow images and list them
    slideshow_url = url_for('static', filename='images/slideshow/')
    slideshow_rel_path = os.path.normpath(slideshow_url.strip('/'))
    slideshow_dir = os.path.join(os.getcwd(), slideshow_rel_path)
    image_urls = [slideshow_url + image for image in os.listdir(slideshow_dir)]
    return render_template('about.html', image_urls=image_urls)


@app.route('/blog/')
def render_blog():
    return render_template('blog.html', posts=blog.posts())


@app.route('/blog/<int:year>/<int:month>/<int:day>/<title>')
def show_post(year, month, day, title):
    date = datetime.date(year, month, day)
    return render_template('blog_post.html', post=blog.get_post(date, title))


@app.route('/projects')
def render_projects():
    return render_template('projects.html', projects=website.projects)


@app.route('/resume')
def resume():
    return render_template('resume.html')
