# cd app
# uvicorn main:app --reload

from fastapi import FastAPI, Depends, HTTPException
import os
from sqlalchemy.orm import Session
import crud, models, schemas
from database import SessionLocal, engine
from unit import Unit
import json


app = FastAPI(
    title="Meal Planner API", openapi_url="/openapi.json"
)

def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()


def setup():
    db = SessionLocal()
    models.Base.metadata.drop_all(bind=engine)
    models.Base.metadata.create_all(bind=engine)

    with open("tags.json", 'r') as fp:
        tag_names = json.loads(fp.read())
    for tag_name in tag_names:
        db_tag = crud.read_tag_by_name(db, name=tag_name)
        if db_tag:
            continue
        crud.create_tag(db=db, tag=schemas.TagCreate(name=tag_name))

    ingredients = []
    for item in os.listdir('ingredients'):
        if item.lower().endswith('.json'):
            with open(item, 'r') as fp:
                ingredients += json.loads(fp.read())
 
    for ingredient in ingredients:
        ingredient = schemas.IngredientCreate(**ingredient)
        try:
            Unit.get_unit(ingredient.unit)
        except ValueError:
            print(f"Invalid unit: '{ingredient.unit}'")
            continue

        db_ingredient = crud.read_ingredient_by_name(db, name=ingredient.name)
        if db_ingredient:
            continue
        crud.create_ingredient(db=db, ingredient=ingredient)


    '''
    recipes = [
        "instant pot pinto beans"
    ]
    for recipe in recipes:
        recipe = schemas.RecipeCreate(name=recipe)

        db_recipe = crud.read_recipe_by_name(db, name=recipe.name)
        if not db_recipe:
            db_recipe = crud.create_recipe(db=db, recipe=recipe)

        tag_id = 8
        db_recipe_tag = db.query(models.RecipeTag).filter(models.RecipeTag.recipe_id == db_recipe.id, models.RecipeTag.tag_id == 8).first()
        if db_recipe_tag:
            continue
        db_recipe_tag = crud.add_tag_to_recipe(db, recipe_id=db_recipe.id, tag_id=tag_id)
    '''


setup()


@app.get("/", status_code=200)
def root() -> dict:
    return {"msg": "Hello, World!"}



# TAG

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

@app.delete("/tags/{tag_id}", status_code=200)
def delete_tag(tag_id: int, db: Session = Depends(get_db)):
    is_deleted = crud.delete_tag(db, tag_id)
    if not is_deleted:
        raise HTTPException(status_code=400, detail=f"could not delete tag with id: {tag_id}; (it probably does not exist anymore)")
    return {"details": f"deleted tag with id: {tag_id}"}


# INGREDIENT 

@app.post("/ingredients/", response_model=schemas.Ingredient)
def create_ingredient(ingredient: schemas.IngredientCreate, db: Session = Depends(get_db)):
    db_ingredient = crud.read_ingredient_by_name(db, name=ingredient.name)
    if db_ingredient:
        raise HTTPException(status_code=400, detail="Ingredient already exists.")
    try:
        Unit.get_unit(ingredient.unit)
    except ValueError:
        raise HTTPException(status_code=400, detail=f"Invalid unit: '{ingredient.unit}'")

    return crud.create_ingredient(db=db, ingredient=ingredient)


@app.put("/ingredients/{ingredient_id}", response_model=schemas.Ingredient)
def update_ingredient(ingredient_id: int, ingredient: schemas.IngredientCreate, db: Session = Depends(get_db)):
    if ingredient.unit is not None:
        try:
            Unit.get_unit(ingredient.unit)
        except ValueError:
            raise HTTPException(status_code=400, detail=f"Invalid unit: '{ingredient.unit}'")

    return crud.update_ingredient(db=db, ingredient=ingredient, ingredient_id=ingredient_id)


@app.get("/ingredients", status_code=200, response_model=list[schemas.Ingredient])
def read_ingredients(skip: int = 0, limit: int = 100, db: Session = Depends(get_db)):
    ingredients = crud.read_ingredients(db, skip=skip, limit=limit)
    return ingredients
 

@app.delete("/ingredients/{ingredient_id}", status_code=200)
def delete_ingredient(ingredient_id: int, db: Session = Depends(get_db)):
    is_deleted = crud.delete_ingredient(db, ingredient_id)
    if not is_deleted:
        raise HTTPException(status_code=400, detail=f"could not delete ingredient with id: {ingredient_id}; (it probably does not exist anymore)")
    return {"details": f"deleted ingredient with id: {ingredient_id}"}


# RECIPE
'''
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


    MOCK_RECIPES = [
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
'''