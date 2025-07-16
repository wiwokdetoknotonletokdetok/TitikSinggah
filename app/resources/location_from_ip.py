from flask_restful import Resource

from app.dto import WebResponse
from app.exception import IPLocationError
from app.service import get_ip_address
from app.service.ip_service import get_ip_location


class LocationFromIP(Resource):

    def get(self):
        try:
            ip_address = get_ip_address()
            location_data = get_ip_location(ip_address)
            return WebResponse.builder().data(location_data).build().dict(), 200
        except IPLocationError as e:
            return WebResponse.builder().errors(e.message).build().dict(), e.status_code
