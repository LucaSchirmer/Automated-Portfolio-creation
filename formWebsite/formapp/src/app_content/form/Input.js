import React, { useState } from 'react';
import { HexColorPicker } from 'react-colorful';

const Input = ({ name, labelContent = name, required = false, type = "text" }) => {
  const [value, setValue] = useState(''); // State for the textarea value

  const handleInputChange = (event) => {
    setValue(event.target.value);
    console.log(event.target.value);
  }; 

  if (required) {
    labelContent = labelContent + '<span class="StartYellowSpan"> <abbr title="Your response here is required.">*</abbr></span>';
  }

  const [showInjectedContent, setShowInjectedContent] = useState(false);
  const [isHighlightedYes, setIsHighlightedYes] = useState(false);
  const [isHighlightedNo, setIsHighlightedNo] = useState(false);
        
  const handleColorSettingsInjectionClick = (event) => {
    if (event.target.classList[0] === "inputBoolFocused" && event.target.name === "Yes") {
      setShowInjectedContent(false);
      setIsHighlightedYes(!isHighlightedYes);
      return;
    } else {
      if (isHighlightedYes) {
        setIsHighlightedYes(!isHighlightedYes);
      }
      if (isHighlightedNo) {
        setIsHighlightedNo(!isHighlightedNo);
      }
      if (event.target.name === "Yes") {
        setShowInjectedContent(true);
        setIsHighlightedYes(!isHighlightedYes);
      } else {
        setShowInjectedContent(false);
        setIsHighlightedNo(!isHighlightedNo);
      }
    }
  };

  const parseCamelCase = (el) => {
    el = el.split("C");
    el = el[0];
    el += " color";
    return el;
  }


//   maybe do color Templates => where you can choose from? like green red white and grey, instead of white white white white
  const initialColors = Array.from({ length: 4 }, () => '#ffffff');
  const [colors, setColors] = useState(initialColors);

  const handleColorChange = (index, color) => {
    console.log(colors)
    const updatedColors = [...colors];
    updatedColors[index] = color;
    setColors(updatedColors);
    console.log(colors)
  };

  const highlightedClassYes =  isHighlightedYes ? 'inputBoolFocused' : '';
  const highlightedClassNo =  isHighlightedNo ? 'inputBoolFocused' : '';

  const colorVarOptions = ["mainColor", "secondaryColor", "accentColor", "backgroundColor"];
    
  if (type === "bool") {
    return (
      <div className='ColorSetterrFormater'>
        <div className="inputElement">
          <label 
            htmlFor={name} 
            dangerouslySetInnerHTML={{ __html: labelContent }}
          />
          <div className='yesNoButtonContainer'>
            <input
              className={highlightedClassYes}
              type="button"
              name="Yes"
              value="Yes"
              onClick={handleColorSettingsInjectionClick}
            />
            <input
              className={highlightedClassNo}
              type="button"
              name="No"
              value="No"
              onClick={handleColorSettingsInjectionClick}
            />
          </div>
        </div>

        {showInjectedContent && (
          <div className="colorInputParent">
            {colors.map((color, index) => (
              <div className="inputElementColor" key={index}>
                <label
                  htmlFor={colorVarOptions[index]}
                  dangerouslySetInnerHTML={{ __html: `Choose a ${parseCamelCase(colorVarOptions[index])}:` }}
                />
                <HexColorPicker
                  key={index}
                  color={color}
                  onChange={(newColor) => handleColorChange(index, newColor)}
                />
                <div className={`color-value color-value${index}`}>
                    Color: <span style={{ color: color, textShadow: '2px 2px 2px black', letterSpacing: '3px' }}>{color}</span>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    );
  }

  return (
    <div className="inputElement">
      <label 
        htmlFor={name} 
        dangerouslySetInnerHTML={{ __html: labelContent }}
      />
      <input
        type={type}
        name={name}
        value={value}
        onChange={handleInputChange}
      />
    </div>
  );
}

export default Input;
