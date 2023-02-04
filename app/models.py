from sqlalchemy import Boolean, Column, ForeignKey, Integer, String, Float, UniqueConstraint
from sqlalchemy.orm import relationship

from database import Base


class Ingredient(Base):
    __tablename__ = "ingredients"

    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, unique=True, index=True)
    amount = Column(Float, index=True)
    unit = Column(String, index=True)
    calories_kcal = Column(Float, index=True)
    protein_g = Column(Float, index=True)
    carbohydrates_g = Column(Float, index=True)
    fat_g = Column(Float, index=True)
    saturated_fat_g = Column(Float, index=True)
    potassium_mg = Column(Float, index=True)
    fiber_g = Column(Float, index=True)
    sodium_mg = Column(Float, index=True)
    sugar_g = Column(Float, index=True)
    cholesterol_mg = Column(Float, index=True)


class Tag(Base):
    __tablename__ = "tags"

    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, unique=True, index=True)


class Recipe(Base):
    __tablename__ = "recipes"

    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, unique=True, index=True)


class RecipeTag(Base):
    __tablename__ = "recipes_tags"

    id = Column(Integer, primary_key=True, index=True)
    recipe_id = Column(Integer, ForeignKey("recipes.id"))
    tag_id = Column(Integer, ForeignKey("tags.id"))
    UniqueConstraint('recipe_id', 'tag_id', name='recipe_id_tag_id')

    recipe = relationship("Recipe", foreign_keys=[recipe_id])
    tag = relationship("Tag", foreign_keys=[tag_id])



class RecipeIngredient(Base):
    __tablename__ = "recipes_ingredients"

    id = Column(Integer, primary_key=True, index=True)
    recipe_id = Column(Integer, ForeignKey("recipes.id"))
    ingredient_id = Column(Integer, ForeignKey("ingredients.id"))
    UniqueConstraint('recipe_id', 'ingredient_id', name='recipe_id_ingredient_id')

    recipe = relationship("Recipe", foreign_keys=[recipe_id])
    ingredient = relationship("Ingredient", foreign_keys=[ingredient_id])



