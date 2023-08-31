import React, { Component } from 'react';
import Textarea from './Textarea.js';
import Input from './Input.js';
import NumberInput from './NumberInput.js';
import Services from './Services.js';
import Projects from './Projects.js';
import Customer from './Customer.js';
import DragNDrop from './DragnDrop.js';
import SocialMedia from './SocialMedia.js';



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
      amoutOfCustomers: 0,
    };
  }

  changeStateAmoutOfProjects = (newValue) => {
    this.setState({ amoutOfProjects: newValue })
  }

  changeStateAmoutOfServices = (newValue) => {
    this.setState({ amoutOfServices: newValue })
  }

  changeStateAmoutOfCustomers = (newValue) => {
    this.setState({ amoutOfCustomers: newValue })
  }

  handleSubmit = (event) => {
    event.preventDefault();
    console.log('Form submitted:', this.state);

    let inputs = document.querySelectorAll("input");
    let textareas = document.querySelectorAll("textarea");

    

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

  renderCustomers = () => {
    const customers = [];
    if(this.state.amoutOfCustomers > 20){
      this.setState({ amoutOfCustomers: 20 })
    }

    for (let i = 0; i < parseInt(this.state.amoutOfCustomers); i++) {
      customers.push(<Customer key={i} index={i} />);
    }
    return customers;
  }

  render() {
    return (
      <form className="mainForm" onSubmit={this.handleSubmit}>
        <div className='formContainer'>
          
          <h2 className="subFormHeading" id="GeneralID">
            General
          </h2>
          <Input name="title" labelContent="Name / Company name:" required="true"/>
          <Textarea name="aboutText" labelContent="Introduction Text about:" required="true"/>
          <DragNDrop name="of you or your team"/>
          <DragNDrop name="to represent your Logo"/>

          <Input name="email" labelContent="Provide your Email:" type="email"/>
          <Input name="phoneNumber" labelContent="Provide your phone number:" type="number"/>

          <h2 className="subFormHeading" id="ColorsID">
            Colors
          </h2>
          <Input name="colorBool" labelContent="Would you like to define colors for your website?" type="bool"/>   

          <h2 className="subFormHeading" id="ProjectID">
            Projects
          </h2>
          <NumberInput name="amoutOfProjects" labelContent='How many projects do you want to have on your website? (20 max)' changeState={this.changeStateAmoutOfProjects}/>
 
          {/* Projects render by Number of prior input */}
          {(this.renderProjects())}
          
          <h2 className="subFormHeading" id="ServiceID">
            Services
          </h2>      
          <NumberInput name="amoutOfServices" labelContent='How many services do you want to have on your website? (20 max)' changeState={this.changeStateAmoutOfServices}/>

          {/* Services render by Number of prior input */}
          {(this.renderServices())}

          <h2 className="subFormHeading" id="CustomerID">
            Customer
          </h2>      
          <NumberInput name="amoutOfCustomers" labelContent='Do you want to add a list of customers to your website? (20 max)' changeState={this.changeStateAmoutOfCustomers}/>

          {/* Services render by Number of prior input */}
          {(this.renderCustomers())}

          <h2 className="subFormHeading" id="SocialMediaID">
            Social Media
          </h2> 

          <SocialMedia changeSocialMedia={this.changeSocialMedia}/>


          <button className="submitButtonMainForm" type="submit">Submit</button>
        </div>
      </form>
    );
  }
}

export default Form;

