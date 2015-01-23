from flask import Flask
from flask.ext.assets import Environment

app = Flask(__name__)
app.config.from_object(__name__)
app.jinja_env.trim_blocks = True
app.jinja_env.autoescape = False        # For HTML in blog posts
app.debug = True

from website import assets
assets.init_app(app)

import website.views
