import os

from flask.ext.assets import Bundle, Environment

scss = Bundle(
    'scss/main.scss',
    filters='pyscss',
    depends=('scss/**/*.scss')
)

css = Bundle(
    scss,
    filters='autoprefixer',
    output='gen/main.css'
)

code_snippets = Bundle(
    'js/github_snippets.js',
    filters='uglifyjs',
    output='gen/snippets.js'
)

def init_app(app):
    assets = Environment(app)

    assets.config['autoprefixer_bin'] = os.path.join(
        app.config['APP_ROOT'], 'node_modules', 'autoprefixer', 'autoprefixer')
    assets.config['uglifyjs_bin'] = os.path.join(
        app.config['APP_ROOT'], 'node_modules', 'uglify-js', 'bin', 'uglifyjs')

    assets.register('css', css)
    assets.register('snippets', code_snippets)
