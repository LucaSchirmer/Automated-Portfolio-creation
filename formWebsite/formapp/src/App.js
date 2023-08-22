import './App.css';
import React from 'react';
import Header from './app_content/Header.js';
import Form from './app_content/form/MainForm.js';



function App() {
  return (
    <div className="App">
      <Header />
      <main className='main'>

        <div className='requiredDisclaimer'>
          Questions marked with a (<span className="StartYellowSpan"> * </span>) necessitate a response.
        </div>


        <Form />
      </main>
    </div>
  );
}

export default App;
