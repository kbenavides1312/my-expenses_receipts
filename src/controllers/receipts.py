import os
import json
import urllib3
import logging
import xmltodict
from flask import request
from flask_restful import Resource, reqparse
from models.receipts_model import ReceiptModel

http = urllib3.PoolManager()


class Receipt(Resource):
    
    def get(self):
        logging.info("getting all receipts")
        return ReceiptModel().get_all_receipts(),200
    
    def post(self):
        data = request.data
        print(request.headers)
        content_type = request.headers.get('Content-Type').lower()
        if content_type not in ["application/json", "application/xml"]:
            return {"Content-Type header must be either application/json or application/xml"}, 400
        if content_type == "application/xml":
            json_data = xmltodict.parse(data)
        elif content_type == "application/json":
            json_data = json.loads(request.data)
        ReceiptModel().create_receipt(json_data)
        store_id = int(json_data["FacturaElectronica"]["Emisor"]["Identificacion"]["Numero"])
        user_id = 0
        for detailLine in json_data["FacturaElectronica"]["DetalleServicio"].values():
            if type(detailLine) is dict:
                detailLine = [detailLine]
            for item in detailLine:
                response = http.request('PATCH',
                    url=os.environ["INVENTORY_SERVICE_URL"],
                    headers={'Content-Type': 'application/json'},
                    body=json.dumps({
                        **item,
                        "StoreId": store_id,
                        "UserId": user_id,
                        "Codigo": f'{item["Codigo"]}-{item.get("CodigoComercial", {}).get("Codigo", "0")}',
                    }),
                )
            print(f'item: {item.get("Codigo", "Unknown")} => response: {response.status}, {response.data}')

        response_headers = {
            'Content-Type': 'application/json',
            'Access-Control-Allow-Origin': 'http://localhost:3000',
        }
        return {"status": "completed"}, 200, response_headers

