# uvicorn main:app --reload

from fastapi import FastAPI, APIRouter, Query, Depends, HTTPException
from pydantic import BaseModel, ValidationError
from typing import Optional, Sequence
from sqlalchemy.orm import Session
import crud, models, schemas
from routers import ingredient
from database import SessionLocal, engine
from unit import Unit



app = FastAPI(
    title="Meal Planner API", openapi_url="/openapi.json"
)
app.include_router(ingredient.router)

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
    db_tag = crud.read_tag_by_name(db, name=tag.name)
    if db_tag:
        raise HTTPException(status_code=400, detail="Tag already exists.")
    return crud.create_tag(db=db, tag=tag)


@app.get("/tags", status_code=200, response_model=list[schemas.Tag])
def read_tags(skip: int = 0, limit: int = 100, db: Session = Depends(get_db)):
    tags = crud.read_tags(db, skip=skip, limit=limit)
    return tags
    '''
    TAGS = [
        "beef",
        "fish",
        "grains",
        "instant pot",
        "legumes",
        "noodles",
        "oven",
        "poultry",
        "rice",
        "sauce",
        "smoothie",
    ]
    '''

@app.delete("/tags/{tag_id}", status_code=200)
def delete_tag(tag_id: int, db: Session = Depends(get_db)):
    is_deleted = crud.delete_tag(db, tag_id)
    if not is_deleted:
        raise HTTPException(status_code=400, detail=f"could not delete tag with id: {tag_id}; (it probably does not exist anymore)")
    return {"details": f"deleted tag with id: {tag_id}"}


 
