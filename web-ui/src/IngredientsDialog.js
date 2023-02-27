import React, { useState } from 'react';
import {
  Box,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogContentText,
  DialogActions,
  Button,
} from '@mui/material';


const units = ['ml', 'oz', 'l']

function IngredientsDialog({ ingredients, open, onCancel}) {
  const [filter, setFilter] = useState('');

  const handleFilterChange = (event) => {
    setFilter(event.target.value);
  };

  const filteredIngredients = ingredients.filter(
    (ingredient) =>
      ingredient.name.toLowerCase().indexOf(filter.toLowerCase()) !== -1
  );

  return (
    <Dialog open={open} onClose={onCancel}>
      <DialogTitle>Ingredients</DialogTitle>
      <DialogContent>
        <DialogContentText>
          Use the form below to specify the ingredients and their amounts for your recipe:
        </DialogContentText>
        <TextField
          label="Filter by name"
          variant="outlined"
          fullWidth
          value={filter}
          onChange={handleFilterChange}
          style={{ marginBottom: 16 }}
        />
        {filteredIngredients.map((ingredient) => (
          <Box
            key={ingredient.id}
            display="flex"
            alignItems="center"
            marginBottom={2}
          >
            <TextField
              label="Amount"
              variant="outlined"
              type="number"
              inputProps={{ step: '0.1', min: '0' }}
              value={ingredient.amount}
              style={{ marginRight: 16, width: 100 }}
            />
            <FormControl variant="outlined" style={{ marginRight: 16, width: 100 }}>
              <InputLabel>Unit</InputLabel>
              <Select value={ingredient.unit} /*onChange={handleUnitChange(ingredient.id)}*/>
                {units.map((unit, index) => (
                    <MenuItem value={unit}>{unit}</MenuItem>
                ))}
              </Select>
            </FormControl>
            <span>{ingredient.name}</span>
          </Box>
        ))}
      </DialogContent>
      <DialogActions>
        <Button onClick={onCancel} color="primary">
          Cancel
        </Button>
      </DialogActions>
    </Dialog>
  );
}


export default IngredientsDialog;