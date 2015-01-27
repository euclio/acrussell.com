import os

from flask.ext.assets import Bundle, Environment

css = Bundle(
    'css/base.css',
    'css/blog.css',
    'css/projects.css',
    'css/resume.css',
    'css/slideshow.css',
    filters='autoprefixer',
    output='gen/main.css'
)

def init_app(app):
    assets = Environment(app)

    assets.config['autoprefixer_bin'] = os.path.join(
        app.config['APP_ROOT'], 'node_modules', 'autoprefixer', 'autoprefixer')

    assets.register('css', css)
