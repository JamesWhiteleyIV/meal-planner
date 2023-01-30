from pydantic import BaseModel, ValidationError
from typing import Optional, Sequence


class TagBase(BaseModel):
    name: str


class TagCreate(TagBase):
    pass


class Tag(TagBase):
    id: int

    class Config:
        orm_mode = True


class IngredientBase(BaseModel):
    name: str
    amount: float | None
    unit: str | None
    calories_kcal: float | None
    protein_g: float | None
    carbohydrates_g: float | None
    fat_g: float | None 
    saturated_fat_g: float | None
    potassium_mg: float | None
    fiber_g: float | None
    sodium_mg: float | None
    sugar_g: float | None
    cholesterol_mg: float | None

class IngredientCreate(IngredientBase):
    pass

class Ingredient(IngredientBase):
    id: int

    class Config:
        orm_mode = True


class RecipeBase(BaseModel):
    name: str

class RecipeCreate(RecipeBase):
    pass


class Recipe(RecipeBase):
    id: int
    instructions: Sequence[str] 
    notes: Sequence[str]
    ingredient: Sequence[Ingredient]
    tags: Sequence[Tag]

    class Config:
        orm_mode = True

