from sqlalchemy.orm import Session
from sqlalchemy import update, delete
import models, schemas

# TAG

def create_tag(db: Session, tag: schemas.TagCreate):
    db_tag = models.Tag(name=tag.name)
    db.add(db_tag)
    db.commit()
    db.refresh(db_tag)
    return db_tag


def read_tags(db: Session, skip: int = 0, limit: int = 100):
    return db.query(models.Tag).offset(skip).limit(limit).all()


def read_tag_by_name(db: Session, name: str):
    return db.query(models.Tag).filter(models.Tag.name == name).first()


def delete_tag(db: Session, tag_id: int):
    result = db.execute(delete(models.Tag).where(models.Tag.id == tag_id))
    db.commit()
    if result.rowcount > 0:
        return True
    return False


# INGREDIENT

def create_ingredient(db: Session, ingredient: schemas.IngredientCreate):
    db_ingredient = models.Ingredient(**ingredient.dict())
    db.add(db_ingredient)
    db.commit()
    db.refresh(db_ingredient)
    return db_ingredient


def read_ingredients(db: Session, skip: int = 0, limit: int = 100):
    return db.query(models.Ingredient).offset(skip).limit(limit).all()


def read_ingredient_by_name(db: Session, name: str):
    return db.query(models.Ingredient).filter(models.Ingredient.name == name).first()


def read_ingredient_by_id(db: Session, ingredient_id: int):
    return db.query(models.Ingredient).filter(models.Ingredient.id == ingredient_id).first()


def update_ingredient(db: Session, ingredient: schemas.IngredientCreate, ingredient_id: int):
    db.execute(update(models.Ingredient).where(models.Ingredient.id == ingredient_id).values(**ingredient.dict()))
    db.commit()
    return read_ingredient_by_id(db, ingredient_id)


def delete_ingredient(db: Session, ingredient_id: int):
    result = db.execute(delete(models.Ingredient).where(models.Ingredient.id == ingredient_id))
    db.commit()
    if result.rowcount > 0:
        return True
    return False

