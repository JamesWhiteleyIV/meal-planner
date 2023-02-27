import React, { useState } from 'react';
import Grid from '@mui/material/Grid';
import Button from '@mui/material/Button';
import Typography from '@mui/material/Typography';
import TagsDialog from './TagsDialog';

const tags = [
    { id: 1, name: 'breakfast'}, 
    {id: 2, name: 'brunch'}, 
    {id: 3, name: 'dinner'}]
  
  

function RecipeDetail(props) {
    const { recipe, onBackClick } = props;
    const [tagsDialogOpen, setTagsDialogOpen] = useState(false);

    /*
    const handleAddIngredientClick = () => {
        recipe.ingredients.push(newIngredient);
        setNewIngredient('');
    };
    */

    const handleAddTagClick = () => {
        setTagsDialogOpen(true);
    };

    const handleAddTags = (tags) => {
        console.log(tags);
    };

    return (
        <div>
            <TagsDialog open={tagsDialogOpen} tags={tags} onCancel={()=>setTagsDialogOpen(false)} onAdd={handleAddTags}/>
            <Typography variant="h4" gutterBottom>
                {recipe.name}
            </Typography>

            <Typography variant="h5" gutterBottom>
                Ingredients
            </Typography>
            <Typography variant="body1" gutterBottom>
                <div>
                    {recipe.ingredients.map((ingredient, index) => (
                        <Typography key={index}>{ingredient}</Typography>
                    ))}
                </div>
            </Typography>

            <Typography variant="h5" gutterBottom>
                Instructions
            </Typography>
            <Typography variant="body1" gutterBottom>
                <div>
                    {recipe.instructions.map((instruction, index) => (
                        <Typography key={index}>{`${index + 1}. ${instruction}`}</Typography>
                    ))}
                </div>
            </Typography>

            <Typography variant="h5" gutterBottom>
                Tags
            </Typography>
            <Typography variant="body1" gutterBottom>
                <div>
                    {recipe.tags.map((tag, index) => (
                        <Typography key={index}>{tag}</Typography>
                    ))}
                </div>
            </Typography>
            <Grid container justify="space-between">
                <Grid item>
                    <Button variant="contained" color="secondary" onClick={onBackClick}>
                        Back
                    </Button>
                </Grid>
 
                <Grid item>
                    <Button variant="contained" color="primary" onClick={console.log("1")}>
                        Add Ingredient
                    </Button>
                </Grid>
                <Grid item>
                    <Button variant="contained" color="primary" onClick={handleAddTagClick}>
                        Add Tag
                    </Button>
                </Grid>
            </Grid>
 
        </div>
    );
}


export default RecipeDetail;