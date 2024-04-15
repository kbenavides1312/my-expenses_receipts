import json
from flask_restful import Resource

class Status(Resource):
    def get(self):
        return {'msg': 'Status Online'}, 200