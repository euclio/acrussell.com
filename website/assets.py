from flask.ext.assets import Bundle, Environment

css = Bundle(
    'css/base.css',
    'css/blog.css',
    'css/projects.css',
    'css/resume.css',
    'css/slideshow.css',
)

def init_app(app):
    assets = Environment(app)

    assets.register('css', css)
