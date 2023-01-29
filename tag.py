from fastapi import FastAPI, APIRouter, Query
from pydantic import BaseModel
from typing import Optional, Sequence

api_router = APIRouter(prefix="/tags", tags=["tags"])


TAGS = [
    {
        "id": 1,
        "label": "chicken"
        },
    {
        "id": 2,
        "label": "beef"
        },
    {
        "id": 3,
        "label": "fish"
        },
]


class Tag(BaseModel):
    id: int
    label: str