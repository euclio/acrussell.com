import os

from flask import Flask
from flask.ext.assets import Environment

app = Flask(__name__)
app.config.from_object('website.config')
app.jinja_env.trim_blocks = True
app.jinja_env.autoescape = False        # For HTML in blog posts

from website import assets
assets.init_app(app)

import website.projects
projects = website.projects.parse(
    os.path.join(os.getcwd(), 'website', 'config.yaml'))

import website.views
