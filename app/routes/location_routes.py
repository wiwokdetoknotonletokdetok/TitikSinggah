from app.resources import LocationFromIP


def location_routes(api):
    api.add_resource(LocationFromIP, "/locations/me")
