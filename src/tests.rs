
#[cfg(test)]
mod tests {
    use crate::*;

    fn get_test_ingredients() -> Vec<Ingredient> {
        vec![
            Ingredient {
                food_item: FoodItem { id: 23, name: "carrot".to_string() },
                amount: 2.0,
                unit: Unit::TSP
            },
            Ingredient {
                food_item: FoodItem { id: 24, name: "potato".to_string() },
                amount: 5.5,
                unit: Unit::ML
            },
        ]
    }


    #[test]
    fn test_grocerylist_add() {
        let mut grocery_list = GroceryList {
            ingredients: vec![]
        };

        let ingredient = Ingredient {
            food_item: FoodItem {
                id: 123,
                name: "Onion Powder".to_string()
            },
            amount: 2.0,
            unit: Unit::TSP    // e.g. ml, fl oz, lbs
        };

        grocery_list.add(ingredient);

        assert_eq!(grocery_list.ingredients[0].food_item.name, "Onion Powder".to_string());
        assert_eq!(grocery_list.ingredients[0].amount, 2.0);
        assert_eq!(grocery_list.ingredients[0].unit.value(), Unit::TSP.value());
    }


    #[test]
    fn test_recipe_multiply() {
        let mut recipe = Recipe {
            id: 1,
            name: "Really Gud Recipe".to_string(),
            ingredients: get_test_ingredients(),
            instructions: vec![],
            notes: "".to_string(),
            tags: vec![]
        };

        let recipe = recipe.multiply(2.0);

        assert_eq!(recipe.ingredients[0].amount, 4.0);
        assert_eq!(recipe.ingredients[1].amount, 11.0);
    }

    #[test]
    fn test_recipe_multiply_float() {
        let mut recipe = Recipe {
            id: 1,
            name: "Really Gud Recipe".to_string(),
            ingredients: get_test_ingredients(),
            instructions: vec![],
            notes: "".to_string(),
            tags: vec![]
        };

        let recipe = recipe.multiply(2.5);

        assert_eq!(recipe.ingredients[0].amount, 5.0);
        assert_eq!(recipe.ingredients[1].amount, 13.75);
    }



}

