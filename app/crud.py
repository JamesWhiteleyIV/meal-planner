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


# RECIPE

def create_recipe(db: Session, recipe: schemas.RecipeCreate):
    db_recipe = models.Recipe(**recipe.dict())
    db.add(db_recipe)
    db.commit()
    db.refresh(db_recipe)
    return db_recipe


'''
def read_ingredients(db: Session, skip: int = 0, limit: int = 100):
    return db.query(models.Ingredient).offset(skip).limit(limit).all()

'''

def read_recipe_by_name(db: Session, name: str):
    return db.query(models.Recipe).filter(models.Recipe.name == name).first()


def add_tag_to_recipe(db: Session, recipe_id: int, tag_id: int):
    db_recipe_tag = models.RecipeTag(recipe_id=recipe_id, tag_id=tag_id)
    db.add(db_recipe_tag)
    db.commit()
    db.refresh(db_recipe_tag)
    return db_recipe_tag


# TODO:   
# remove_tag_from_recipe
# remove_ingredient_from_recipe

'''

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
'''
