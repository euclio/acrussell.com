from flask import Flask

app = Flask(__name__)
app.config.from_object(__name__)
app.jinja_env.trim_blocks = True
app.jinja_env.autoescape = False        # For HTML in blog posts

import website.views
