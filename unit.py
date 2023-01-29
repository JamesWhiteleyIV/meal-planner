from fastapi import FastAPI, APIRouter, Query
from pydantic import BaseModel, ValidationError, HttpUrl
from typing import Optional, Sequence

api_router = APIRouter(prefix="/units", tags=["units"])


UNITS = [
    {
        "id": 1,
        "label": "ml",
    },
    {
        "id": 2,
        "label": "g",
    },
    {
        "id": 2,
        "label": "oz",
    },
]


class Unit(BaseModel):
    id: int
    label: str