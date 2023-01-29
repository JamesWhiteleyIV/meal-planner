from fastapi import FastAPI, APIRouter, Query
from pydantic import BaseModel, ValidationError, HttpUrl
from typing import Optional, Sequence
from ingredient import Ingredient
from tag import Tag

api_router = APIRouter(prefix="/recipes", tags=["recipes"])


RECIPES = [
    {
        "id": 1,
        "label": "Chicken Vesuvio",
        "instructions": ["add oil to pan", "toss dat chicken"],
        "notes": ["better luck next time"],
        # "ingredients": 
        "tags": ["chicken", "parmosean"]
    },
    {
        "id": 2,
        "label": "Chicken Paprikash",
        "instructions": ["add oil to pan", "toss dat chicken"],
        "notes": ["better luck next time"],
        # "ingredients": 
        "tags": ["chicken", "parmosean"]
    },
    {
        "id": 3,
        "label": "Cauliflower and Tofu Curry Recipe",
        "instructions": ["add oil to pan", "toss dat chicken"],
        "notes": ["better luck next time"],
        # "ingredients": 
        "tags": ["chicken", "parmosean"]
    },
]


class Recipe(BaseModel):
    id: int
    label: str
    instructions: Sequence[str] 
    notes: Sequence[str]
    ingredient: Sequence[Ingredient]
    tags: Sequence[Tag]


class Recipes(BaseModel):
    results: Sequence[Recipe] 


class RecipeCreate(BaseModel):
    label: str


@api_router.get("/{recipe_id}", status_code=200, response_model=Recipe)
def get_recipe(*, 
recipe_id: int) -> dict: 
    result = [recipe for recipe in RECIPES if recipe["id"] == recipe_id]

    if not result:
        raise HTTPException(
        status_code=404, detail=f"Recipe with ID {recipe_id} not found"
    )
    return result[0]


@api_router.get("/", status_code=200, response_model=Recipes)
def get_recipes(
    *,
    keyword: Optional[str] = Query(None, min_length=3, example="chicken"), 
    tag: Optional[str] = Query(None, min_length=1, example="chicken"), 
    max_results: Optional[int] = 10
) -> dict:
    if keyword is None:
        return {"results": RECIPES[:max_results]}

    results = filter(lambda recipe: keyword.lower() in recipe["label"].lower(), RECIPES)
    return {"results": list(results)[:max_results]}


@api_router.post("/", status_code=201, response_model=Recipe)
def create_recipe(*, recipe_in: RecipeCreate) -> dict: 
    new_entry_id = len(RECIPES) + 1
    recipe_entry = Recipe(
        id=new_entry_id,
        label=recipe_in.label,
        source=recipe_in.source,
        url=recipe_in.url,
    )
    RECIPES.append(recipe_entry.dict())  

    return recipe_entry


