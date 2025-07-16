import json
import urllib.request

from flask import request

from app.config import settings
from app.dto import LocationData
from app.exception import IPLocationError


def get_ip_address():
    forwarded = request.headers.getlist("X-Forwarded-For")
    return forwarded[0] if forwarded else request.remote_addr


def get_ip_location(ip_address: str):
    url = f"{settings.ip_geolocation_url}/{ip_address}"

    try:
        with urllib.request.urlopen(url) as response:
            res = json.loads(response.read().decode())
    except Exception as e:
        raise IPLocationError(e, 500)

    if res.get("status") != "success":
        raise IPLocationError("IP not found", 404)

    return LocationData(
        country=res.get("country"),
        region=res.get("regionName"),
        city=res.get("city"),
        latitude=res.get("lat"),
        longitude=res.get("lon"),
    )
