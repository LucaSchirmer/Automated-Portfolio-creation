import React, { Component } from 'react';
import Textarea from './Textarea.js';
import Input from './Input.js';
import NumberInput from './NumberInput.js';
import Services from './Services.js';
import Projects from './Projects.js';



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
      metaDescription: 'Buy Number Stickers which i produce for u in person',
      amoutOfProjects: 0,
      amoutOfServices: 0,
    };
  }

  changeStateAmoutOfProjects = (newValue) => {
    this.setState({ amoutOfProjects: newValue })
  }

  changeStateAmoutOfServices = (newValue) => {
    this.setState({ amoutOfServices: newValue })
  }

  handleSubmit = (event) => {
    event.preventDefault();
    console.log('Form submitted:', this.state);
  };

  renderProjects = () => {
    const projects = [];
    if(this.state.amoutOfProjects > 20){
      this.setState({ amoutOfProjects: 20 })
    }

    for (let i = 0; i < parseInt(this.state.amoutOfProjects); i++) {
      projects.push(<Projects key={i} index={i} />);
    }
    return projects;
  }

  renderServices = () => {
    const services = [];
    if(this.state.amoutOfServices > 20){
      this.setState({ amoutOfServices: 20 })
    }

    for (let i = 0; i < parseInt(this.state.amoutOfServices); i++) {
      services.push(<Services key={i} index={i} />);
    }
    return services;
  }

  render() {
    return (
      <form className="mainForm" onSubmit={this.handleSubmit}>
        <div className='formContainer'>
          
          <h2 className="subFormHeading" id="ColorsID">
            General
          </h2>
          <Textarea name="title" labelContent="Name / Firmenname:" required="true"/>
          <Textarea name="aboutText" labelContent="Introduction Text about:" required="true"/>
          <Input name="email" labelContent="Provide your Email:" type="email"/>
          <Input name="phoneNumber" labelContent="Provide your phone number:" type="number"/>

          <h2 className="subFormHeading" id="ColorsID">
            Colors
          </h2>
          <Input name="colorBool" labelContent="Would you like to define colors for your Website?" type="bool"/>   

          <h2 className="subFormHeading" id="ProjectID">
            Projects
          </h2>
          <NumberInput name="amoutOfProjects" labelContent='How many projects do you want to have on your Website? (20 max)' changeState={this.changeStateAmoutOfProjects}/>
 
          {/* Projects render by Number of prior input */}
          {(this.renderProjects())}
          
          <h2 className="subFormHeading" id="ProjectID">
            Services
          </h2>      
          <NumberInput name="amoutOfServices" labelContent='How many services do you want to have on your Website? (20 max)' changeState={this.changeStateAmoutOfServices}/>

          {/* Services render by Number of prior input */}
          {(this.renderServices())}

          <button type="submit">Submit</button>
        </div>
      </form>
    );
  }
}

export default Form;

