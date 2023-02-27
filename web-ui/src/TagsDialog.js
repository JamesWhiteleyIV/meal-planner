import React, { useState } from 'react';
import {
  Chip,
  Button,
  Box,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
} from '@mui/material';

function TagsDialog({ open, tags, onCancel, onAdd }) {
  const [selectedTags, setSelectedTags] = useState([]);

  const handleToggleTag = (tag) => {
    if (selectedTags.includes(tag)) {
      setSelectedTags(selectedTags.filter((t) => t !== tag));
    } else {
      setSelectedTags([...selectedTags, tag]);
    }
  };

  const handleCancel = () => {
    setSelectedTags([]);
    onCancel();
  };

  const handleAdd = () => {
    onAdd(selectedTags);
    setSelectedTags([]);
  };


  return (
    <Dialog open={open} onClose={handleCancel}>
      <DialogTitle>Choose tags</DialogTitle>
      <DialogContent>
        <Box display="flex" flexWrap="wrap" justifyContent="flex-start">
          {tags.map((tag) => (
            <Chip
              key={tag.id}
              label={tag.name}
              color={selectedTags.includes(tag) ? 'primary' : 'default'}
              onClick={() => handleToggleTag(tag)}
              style={{ margin: 4 }}
            />
          ))}
        </Box>
      </DialogContent>
      <DialogActions>
        <Button onClick={handleCancel} color="secondary">
          Cancel
        </Button>
        <Button onClick={handleAdd} color="primary" disabled={!selectedTags.length}>
          Add
        </Button>
      </DialogActions>
    </Dialog>
  );
}

export default TagsDialog;