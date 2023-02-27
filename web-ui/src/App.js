import './App.css';
import Box from '@mui/material/Box';
import RecipeList  from './RecipeList';

/*
function test() {
  fetch('/recipes')
  .then(response => response.json())
  .then(data => {
    console.log(data);
    // Handle the response data here
  })
  .catch(error => console.error(error));
}
*/

function App() {
  return (
    <Box sx={{ padding: 2 }}>
        <RecipeList/>
  </Box>
  );
}

export default App;
