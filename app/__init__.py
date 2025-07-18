from flask import Flask
from flask_cors import CORS
from flask_restful import Api

from app.routes import location_routes


def create_app():
    app = Flask(__name__)
    CORS(app)

    api = Api(app)

    location_routes(api)

    return app
