from typing import Optional

from pydantic import BaseModel


class LocationData(BaseModel):
    country: Optional[str]
    region: Optional[str]
    city: Optional[str]
    latitude: Optional[float]
    longitude: Optional[float]
