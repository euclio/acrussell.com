import datetime
import httplib
import itertools
import os
from os import path

from flask import Flask, render_template, request, send_from_directory, url_for

import blog
import projects


def create_app():
    app = Flask(__name__)
    app.config.from_object(__name__)
    app.jinja_env.trim_blocks = True
    app.jinja_env.autoescape = False
    blog.parse_posts(
        path.join(os.getcwd(), app.static_folder, 'blog'))
    projects.parse_projects(
        path.join(os.getcwd(), app.static_folder, 'projects'))
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


@app.route('/projects')
def render_projects():
    return render_template('projects.html', projects=projects.get_projects())


@app.route('/resume')
def resume():
    return render_template('resume.html')

if __name__ == '__main__':
    app.run(debug=False)
