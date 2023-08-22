import React, { Component } from 'react';



class Form extends Component {

  constructor(props) {
    super(props);
    this.state = {
      name: '',
      email: '',
    };
  }

  handleInputChange = (event) => {
    const { name, value } = event.target;
    this.setState({
      [name]: value,
    });
  };

  handleSubmit = (event) => {
    event.preventDefault();
    console.log('Form submitted:', this.state);
    // You can add logic here to send the form data to a server or perform other actions.
  };

  render() {
    return (
      <form onSubmit={this.handleSubmit}>
        <div>
          <label>
            Name:
            <input
              type="text"
              name="name"
              value={this.state.name}
              onChange={this.handleInputChange}
            />
          </label>
        </div>
        <div>
          <label>
            Email:
            <input
              type="email"
              name="email"
              value={this.state.email}
              onChange={this.handleInputChange}
            />
          </label>
        </div>
        <button type="submit">Submit</button>
      </form>
    );
  }
}

export default Form;

