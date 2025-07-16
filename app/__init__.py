from flask import Flask
from flask_restful import Api

from app.routes import location_routes


def create_app():
    app = Flask(__name__)

    api = Api(app)

    location_routes(api)

    return app
