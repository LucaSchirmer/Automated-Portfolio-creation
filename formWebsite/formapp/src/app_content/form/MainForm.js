import React, { Component } from 'react';
import Textarea from './Textarea.js';
import Input from './Input.js';



class Form extends Component {

  constructor(props) {
    super(props);
    this.state = {
      title: '',
      aboutText: '',
      aboutFoto: '',
      email: '',
      phoneNumber: '',
      colorBool: '',
      mainColor: '#fff',
      secondaryColor: '#222',
      accentColor: '#aa2211',
      backgroundColor: '#f00',
      metaDescription: 'Buy Number Stickers which i produce for u in person'
    };
  }


  handleSubmit = (event) => {
    event.preventDefault();
    console.log('Form submitted:', this.state);
  };

  render() {
    return (
      <form className="mainForm" onSubmit={this.handleSubmit}>
        <div className='formContainer'>
          <Textarea name="title" labelContent="Name / Firmenname:" required="true"/>
          <Textarea name="aboutText" labelContent="Introduction Text about:" required="true"/>
          <Input name="email" labelContent="Provide your Email:" type="email"/>
          <Input name="phoneNumber" labelContent="Provide your phone number:" type="number"/>

          <Input name="colorBool" labelContent="Would you like to define colors for your Website?" type="number"/>   



          <button type="submit">Submit</button>
        </div>
      </form>
    );
  }
}

export default Form;

