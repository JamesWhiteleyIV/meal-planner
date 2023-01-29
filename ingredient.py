from fastapi import FastAPI, APIRouter, Query
from pydantic import BaseModel, ValidationError
from typing import Optional, Sequence

api_router = APIRouter(prefix="/ingredients", tags=["ingredients"])


class Ingredient(BaseModel):
    id: int
    unit: str
    amount: float
    protein: float
    carbs: float
    fat: float


if __name__ == "__main__":
    DEFAULT_INGREDIENTS = [
        "instant pot",
        "poultry",
        "beef",
        "fish",
        "legumes",
        "rice",
        "grains",
        "noodles",
        "sauce",
        "smoothie",
        "oven",
    ]



