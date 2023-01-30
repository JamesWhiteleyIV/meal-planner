from fastapi import FastAPI, APIRouter, Query, Depends, HTTPException
from pydantic import BaseModel, ValidationError
from typing import Optional, Sequence
from sqlalchemy.orm import Session
import crud, models, schemas
from database import SessionLocal, engine
from unit import Unit


router = APIRouter(
    prefix="/ingredients",
    tags=["ingredients"],
    #dependencies=[Depends(get_token_header)],
)

@router.post("/", response_model=schemas.Ingredient)
def create_ingredient(ingredient: schemas.IngredientCreate, db: Session = Depends(get_db)):
    db_ingredient = crud.read_ingredient_by_name(db, name=ingredient.name)
    if db_ingredient:
        raise HTTPException(status_code=400, detail="Ingredient already exists.")
    try:
        Unit.get_unit(ingredient.unit)
    except ValueError:
        raise HTTPException(status_code=400, detail=f"Invalid unit: '{ingredient.unit}'")

    return crud.create_ingredient(db=db, ingredient=ingredient)


@router.put("/{ingredient_id}", response_model=schemas.Ingredient)
def update_ingredient(ingredient_id: int, ingredient: schemas.IngredientCreate, db: Session = Depends(get_db)):
    if ingredient.unit is not None:
        try:
            Unit.get_unit(ingredient.unit)
        except ValueError:
            raise HTTPException(status_code=400, detail=f"Invalid unit: '{ingredient.unit}'")

    return crud.update_ingredient(db=db, ingredient=ingredient, ingredient_id=ingredient_id)


@router.get("/", status_code=200, response_model=list[schemas.Ingredient])
def read_ingredients(skip: int = 0, limit: int = 100, db: Session = Depends(get_db)):
    ingredients = crud.read_ingredients(db, skip=skip, limit=limit)
    return ingredients
 

@router.delete("/{ingredient_id}", status_code=200)
def delete_ingredient(ingredient_id: int, db: Session = Depends(get_db)):
    is_deleted = crud.delete_ingredient(db, ingredient_id)
    if not is_deleted:
        raise HTTPException(status_code=400, detail=f"could not delete ingredient with id: {ingredient_id}; (it probably does not exist anymore)")
    return {"details": f"deleted ingredient with id: {ingredient_id}"}

