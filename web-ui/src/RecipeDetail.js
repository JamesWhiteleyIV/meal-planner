import React, { useState } from 'react';
import TextField from '@mui/material/TextField';
import Autocomplete from '@mui/material/Autocomplete';
import Grid from '@mui/material/Grid';
import Button from '@mui/material/Button';
import Typography from '@mui/material/Typography';


const TEST_RECIPE = { id: 1, name: 'Pancakes', tags: ['breakfast', 'brunch'], ingredients: ['beef'], instructions: ["step 1", "step 2"]};


function RecipeDetail(props) {
    const { recipe, onBackClick } = props;
    const [editingIngredients, setEditingIngredients] = useState(false);
    const [editingInstructions, setEditingInstructions] = useState(false);
    const [newIngredient, setNewIngredient] = useState('');
    const [newTag, setNewTag] = useState('');

    const handleIngredientEditClick = () => {
        setEditingIngredients(true);
    };

    const handleInstructionsEditClick = () => {
        setEditingInstructions(true);
    };

    const handleIngredientSaveClick = () => {
        setEditingIngredients(false);
    };

    const handleInstructionsSaveClick = () => {
        setEditingInstructions(false);
    };

    const handleIngredientCancelClick = () => {
        setEditingIngredients(false);
    };

    const handleInstructionsCancelClick = () => {
        setEditingInstructions(false);
    };

    const handleNewIngredientChange = (event) => {
        setNewIngredient(event.target.value);
    };

    const handleNewTagChange = (event) => {
        setNewTag(event.target.value);
    };

    const handleAddIngredientClick = () => {
        recipe.ingredients.push(newIngredient);
        setNewIngredient('');
    };

    const handleAddTagClick = () => {
        recipe.tags.push(newTag);
        setNewTag('');
    };


    return (
        <div>
            <Typography variant="h4" gutterBottom>
                {recipe.name}
            </Typography>
            <Typography variant="h5" gutterBottom>
                Ingredients
            </Typography>
            <Typography variant="body1" gutterBottom>
                {editingIngredients ?
                    <div>
                        {recipe.ingredients.map((ingredient, index) => (
                            <TextField
                                key={index}
                                value={ingredient}
                                onChange={(e) => /*handleIngredientChange(index, e.target.value)*/ console.log(e)}
                            />
                        ))}
                        <div>
                            <Button variant="contained" color="primary">
                                Save
                            </Button>
                            <Button variant="contained">
                                Cancel
                            </Button>
                        </div>
                    </div>
                    :
                    <div>
                        {recipe.ingredients.map((ingredient, index) => (
                            <Typography key={index}>{ingredient}</Typography>
                        ))}
                        <Button variant="contained">
                            Edit
                        </Button>
                    </div>
                }
            </Typography>
            <Typography variant="h5" gutterBottom>
                Instructions
            </Typography>
            <Typography variant="body1" gutterBottom>
                {editingInstructions ?
                    <div>
                        {recipe.instructions.map((instruction, index) => (
                            <TextField
                                key={index}
                                value={instruction}
                                onChange={(e) => /* handleInstructionChange(index, e.target.value)*/ console.log(e)}
                            />
                        ))}
                        <div>
                            <Button variant="contained" color="primary" onClick={handleInstructionsSaveClick}>
                                Save
                            </Button>
                            <Button variant="contained" onClick={handleInstructionsCancelClick}>
                                Cancel
                            </Button>
                        </div>
                    </div>
                    :
                    <div>
                        {recipe.instructions.map((instruction, index) => (
                            <Typography key={index}>{`${index + 1}. ${instruction}`}</Typography>
                        ))}
                        <Button variant="contained" onClick={handleInstructionsEditClick}>
                            Edit
                        </Button>
                    </div>
                }
            </Typography>
            <Grid container justify="space-between">
                <Grid item>
                    <Button variant="contained" onClick={onBackClick}>
                        Back
                    </Button>
                    <Button variant="contained" color="secondary">
                        Edit
                    </Button>
                </Grid>
                <Grid item>
                    <div>
                        <TextField label="New Ingredient" value={newIngredient} onChange={handleNewIngredientChange} />
                        <Button variant="contained" color="primary" onClick={handleAddIngredientClick}>
                            Add Ingredient
                        </Button>
                    </div>
                    <div>
                        <TextField label="New Tag" value={newTag} onChange={handleNewTagChange} />
                        <Autocomplete
                            multiple
                            options={['Dinner', 'Lunch', 'Breakfast']}
                            freeSolo
                            renderInput={(params) => (
                                <TextField
                                    {...params}
                                    variant="outlined"
                                    label="Add Tag"
                                    placeholder="Add a tag"
                                />
                            )}
                        />
                        <Button variant="contained" color="primary" onClick={handleAddTagClick}>
                            Add Tag
                        </Button>
                    </div>
                </Grid>
            </Grid>
        </div>
    );
}


export default RecipeDetail;