from fastapi import FastAPI, APIRouter, Query
from pydantic import BaseModel
from typing import Optional, Sequence

api_router = APIRouter(prefix="/tags", tags=["tags"])

# TODO: Get tags   GET /tags
# TODO: Create     POST /tags/{name}

if __name__ == "__main__":
    MOCK_TAGS = [
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

    for idx in range(len(MOCK_TAGS)):
        print("----------------------------")
        t = {"id": idx+1, "name": MOCK_TAGS[idx]}
        print(Tag(**t))