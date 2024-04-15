import os
import pymongo
import logging


class Database(): # pylint: disable=R0904

    def __init__(self, collection):
        mongo_client = pymongo.MongoClient(os.environ["DB_URL"])
        self.dao = mongo_client[os.environ["DB_NAME"]][collection]

    def insert_doc(self, doc):
        self.dao.insert_one(doc)

    def get_doc(self, id):
        query = {"_id": id}
        return self.dao.find_one(query)
    
    def find_docs(self, query):
        return list(self.dao.find(query, projection={'_id': False}) )

    def update_doc(self, id, doc):
        query = {"_id": id}
        new_values = { "$set": doc }
        self.dao.update_one(query, new_values)

    def delete_doc(self, id):
        query = {"_id": id}
        self.dao.delete_one(query)
