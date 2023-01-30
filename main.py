# uvicorn main:app --reload
# https://christophergs.com/tutorials/ultimate-fastapi-tutorial-pt-8-project-structure-api-versioning/

from fastapi import FastAPI, APIRouter, Query
from pydantic import BaseModel, ValidationError, HttpUrl
from typing import Optional, Sequence

app = FastAPI(
    title="Recipe API", openapi_url="/openapi.json"
)

import ingredient
import recipe
import tag
from unit import Units

api_router = APIRouter()
app.include_router(api_router)
app.include_router(ingredient.api_router)
app.include_router(recipe.api_router)
app.include_router(tag.api_router)


@api_router.get("/", status_code=200)
def root() -> dict:
    return {"msg": "Hello, World!"}


