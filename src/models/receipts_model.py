import os
import pymongo
import logging

from models.database import Database

class ReceiptModel(): # pylint: disable=R0904

    def __init__(self):
        self.dao = Database(os.environ["RECEIPTS_COLLECTION"])

    def create_receipt(self, receipt):
        return self.dao.insert_doc(receipt)

    def get_all_receipts(self):
        return self.dao.find_docs({})

    def get_receipt(self, id):
        return self.dao.get_doc(id)

    def update_receipt(self, id, receipt):
        return self.dao.update_doc(id, receipt)

    def delete_receipt(self, id):
        return self.dao.delete_doc(id)