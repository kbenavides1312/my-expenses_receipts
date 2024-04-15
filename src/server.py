import os
from flask import Flask
from flask_restful import Api
from flask_jwt_extended import jwt_required, JWTManager

from controllers.status import Status
from controllers.receipts import Receipt

app = Flask(__name__)
api = Api(app)

api.app.config['JWT_SECRET_KEY'] = os.environ["JWT_SECRET_KEY"]
api.app.config['JWT_TOKEN_LOCATION'] = ["headers"]
jwt = JWTManager(api.app)

api.add_resource(Status, '/status')
api.add_resource(Receipt, '/api/receipts')


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8080)