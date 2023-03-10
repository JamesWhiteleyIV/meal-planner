import React, { useState } from 'react';
import TextField from '@mui/material/TextField';
import Autocomplete from '@mui/material/Autocomplete';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ListItemText from '@mui/material/ListItemText';
import Grid from '@mui/material/Grid';
import RecipeDetail from './RecipeDetail';


const recipes = [
  { id: 1, name: 'Pancakes', tags: ['test']},
  { id: 2, name: 'Spaghetti Bolognese', tags: ['dinner', 'italian'] },
  { id: 3, name: 'Chicken Curry', tags: ['dinner', 'indian'] },
  { id: 4, name: 'Avocado Toast', tags: ['breakfast', 'vegetarian'] },
];

const recipe = { id: 1, name: 'Pancakes', tags: ['breakfast', 'brunch'], ingredients: ['ingredient1', 'ingredient2'], instructions: ["step 1", "step 2"]};

const tags = [
  { id: 1, name: 'breakfast'}, 
  {id: 2, name: 'brunch'}, 
  {id: 3, name: 'dinner'}]


function RecipeList() {
  const [filter, setFilter] = useState('');
  const [tagFilter, setTagFilter] = useState('');
  const [selectedRecipe, setSelectedRecipe] = useState(null);

  const handleFilterChange = (event) => {
    setFilter(event.target.value);
  };

  const handleTagFilterChange = (event, value) => {
    if (value === null) {
      setTagFilter('');
    } else {
      setTagFilter(value.name);
    }
  };

  const handleRecipeClick = (recipe) => {
    setSelectedRecipe(recipe);
  };

  const handleBackClick = () => {
    setSelectedRecipe(null);
  };

  const filteredRecipes = recipes.filter(recipe => {
    if (tagFilter && !recipe.tags.includes(tagFilter)) {
      return false;
    }
    if (filter && !recipe.name.toLowerCase().includes(filter.toLowerCase())) {
      return false;
    }
    return true;
  });

  return (
    <div>
      {selectedRecipe ?
        <RecipeDetail recipe={recipe} onBackClick={handleBackClick} /> :
        <div>
      <Grid container spacing={2} alignItems="center">
        <Grid item xs={6}>
 
          <TextField
            label="Filter by recipe name"
            value={filter}
            onChange={handleFilterChange}
            fullWidth
          />
        </Grid>

        <Grid item xs={6}>
          <Autocomplete
            options={tags}
            getOptionLabel={(option) => option.name}
            onChange={handleTagFilterChange}
            renderInput={(params) => (
              <TextField {...params} label="Filter by tag" variant="outlined" />
            )}
          />
        </Grid>

      </Grid>
 
          <List>
            {filteredRecipes.map((recipe) => (
              <ListItem sx={{
                '&:hover': {
                  backgroundColor: '#D3D3D3',
                },
              }} key={recipe.id} onClick={() => handleRecipeClick(recipe)}>
                <ListItemText primary={recipe.name} secondary={recipe.tags.join(', ')} />
              </ListItem>
            ))}
          </List>
        </div>
      }
    </div>
  );
}

export default RecipeList;



