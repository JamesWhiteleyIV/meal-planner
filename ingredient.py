from fastapi import FastAPI, APIRouter, Query
from pydantic import BaseModel, ValidationError
from typing import Optional, Sequence
from unit import Unit

api_router = APIRouter(prefix="/ingredients", tags=["ingredients"])



# TODO: GET /ingredients
# TODO: GET /ingredients/{id}
# TODO: POST /ingredients w/ body


if __name__ == "__main__":
    MOCK_INGREDIENTS = [
        {
            "id": 1,
            "name": "pinto beans",
            "amount": 1.00,
            "unit": Unit.CUP.label,
            "calories_kcal": 245,
            "protein_g": 15,
            "carbohydrates_g": 45,
            "fat_g": 1.5,
            "saturated_fat_g": 0.3,
            "potassium_mg": 705,
            "fiber_g": 15,
            "sodium_mg": 15,
            "sugar_g": 1,
            "cholesterol_mg": 0
        },
        {
            "id": 2,
            "name": "black beans"
        }
    ]

    for i in MOCK_INGREDIENTS:
        print("--------------------------")
        print(Ingredient(**i))


