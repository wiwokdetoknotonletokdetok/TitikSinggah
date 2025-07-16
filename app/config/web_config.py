from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    ip_geolocation_url: str

    class Config:
        env_file = ".env"


settings = Settings()
