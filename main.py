# uvicorn main:app --reload

from fastapi import FastAPI, APIRouter, Query, Depends, HTTPException
from pydantic import BaseModel, ValidationError
from typing import Optional, Sequence
from sqlalchemy.orm import Session
import crud, models, schemas
from database import SessionLocal, engine
from unit import Unit


app = FastAPI(
    title="Meal Planner API", openapi_url="/openapi.json"
)

models.Base.metadata.create_all(bind=engine)

def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()


@app.get("/", status_code=200)
def root() -> dict:
    return {"msg": "Hello, World!"}


@app.post("/tags", response_model=schemas.Tag)
def create_tag(tag: schemas.TagCreate, db: Session = Depends(get_db)):
    db_tag = crud.get_tag_by_name(db, name=tag.name)
    if db_tag:
        raise HTTPException(status_code=400, detail="Tag already exists.")
    return crud.create_tag(db=db, tag=tag)


@app.get("/tags", status_code=200, response_model=list[schemas.Tag])
def read_tags(skip: int = 0, limit: int = 100, db: Session = Depends(get_db)):
    tags = crud.get_tags(db, skip=skip, limit=limit)
    return tags
    '''
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
    '''

@app.post("/ingredients", response_model=schemas.Ingredient)
def create_ingredient(ingredient: schemas.IngredientCreate, db: Session = Depends(get_db)):
    db_ingredient = crud.get_ingredient_by_name(db, name=ingredient.name)
    if db_ingredient:
        raise HTTPException(status_code=400, detail="Ingredient already exists.")
    try:
        Unit.get_unit(ingredient.unit)
    except ValueError:
        raise HTTPException(status_code=400, detail=f"Invalid unit: '{ingredient.unit}'")

    return crud.create_ingredient(db=db, ingredient=ingredient)


@app.get("/ingredients", status_code=200, response_model=list[schemas.Ingredient])
def read_ingredients(skip: int = 0, limit: int = 100, db: Session = Depends(get_db)):
    ingredients = crud.get_ingredients(db, skip=skip, limit=limit)
    return ingredients
 
    '''
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
    '''


