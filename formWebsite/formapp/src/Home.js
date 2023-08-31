import './App.css';
import React from 'react';
import Header from './app_content/Header.js';
import Form from './app_content/form/MainForm.js';
import Sidebar from './app_content/Sidebar.js';
import Footer from './app_content/Footer.js';


function Home() {
  return (
    <div className="App">
      <Header />
      <Sidebar />
      <main 
        className='main'
      >

        <div className='requiredDisclaimer'>
          Questions marked with a (<span className="StartYellowSpan"> * </span>) necessitate a response.
        </div>


        <Form />
      </main>
      <Footer />
    </div>
  );
}

export default Home;
