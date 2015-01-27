import os

from flask.ext.assets import Bundle, Environment

scss = Bundle(
    'scss/main.scss',
    filters='pyscss'
)

css = Bundle(
    'css/blog.css',
    'css/projects.css',
    'css/resume.css',
    scss,
    filters='autoprefixer',
    output='gen/main.css'
)


def init_app(app):
    assets = Environment(app)

    assets.config['autoprefixer_bin'] = os.path.join(
        app.config['APP_ROOT'], 'node_modules', 'autoprefixer', 'autoprefixer')

    assets.register('css', css)
