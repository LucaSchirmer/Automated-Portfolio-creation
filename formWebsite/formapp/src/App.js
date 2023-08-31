import React from 'react';
import { BrowserRouter as Router, Route,  Routes} from 'react-router-dom';
import Impressum from './app_content/footer_content/Impressum.js'
import Datenschutz from './app_content/footer_content/Datenschutz.js'
import Contact from './app_content/footer_content/Contact.js'
import Home from "./Home.js"

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Home />} /> 
        <Route path="/Impressum" element={<Impressum/>} /> 
        <Route path="/Datenschutz" element={<Datenschutz/>} /> 
        <Route path="/Contact" element={<Contact/>} /> 
      </Routes>
    </Router>
  );
}

export default App;
