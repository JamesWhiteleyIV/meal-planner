import logo from './logo.svg';
import './App.css';

function test() {
  fetch('/recipes')
  .then(response => response.json())
  .then(data => {
    console.log(data);
    // Handle the response data here
  })
  .catch(error => console.error(error));
}

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <button onClick={test}></button>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
