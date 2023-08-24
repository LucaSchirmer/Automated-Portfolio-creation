import React, { useState } from 'react';
import ColorInput from "./ColorInput";

const Input = ({name,labelContent=name, required=false, type="text"}) => {
    const [value, setValue] = useState(''); // State for the textarea value

    const handleInputChange = (event) => {
        setValue(event.target.value);

        console.log(event.target.value);
    }; 

    if(required){
        labelContent = labelContent + '<span class="StartYellowSpan"> <abbr title="Your response here is required.">*</abbr></span>';
    }


    // logic to extend the content when the user wants to choose the colors
    const [showInjectedContent, setShowInjectedContent] = useState(false);
    const [isHighlightedYes, setIsHighlightedYes] = useState(false);
    const [isHighlightedNo, setIsHighlightedNo] = useState(false);
        
    const handleColorSettingsInjectionClick = (event) => {
        console.log(event.target);
        if(event.target.classList[0] === "inputBoolFocused" && event.target.name === "Yes"){
            setShowInjectedContent(false);
            setIsHighlightedYes(!isHighlightedYes);
            return;

        }else{

            if(isHighlightedYes){
                setIsHighlightedYes(!isHighlightedYes);
            }
            if(isHighlightedNo){
                setIsHighlightedNo(!isHighlightedNo);
            }

            

            console.log(event.target.name)
            if(event.target.name === "Yes"){
                setShowInjectedContent(true);
                setIsHighlightedYes(!isHighlightedYes);
            }else{
                setShowInjectedContent(false);
                setIsHighlightedNo(!isHighlightedNo);
            }
        }
    };

    const parseCamelCase = (el)=>{
        //  parses camelCase into first word  ( from firstWord )
        el = el.split("C");
        el = el[0];
        el += " color";

        return el
    }

    const highlightedClassYes =  isHighlightedYes  ? 'inputBoolFocused': '';
    const highlightedClassNo =  isHighlightedNo  ? 'inputBoolFocused': '';

    const colorVarOptions = ["mainColor", "secondaryColor", "accentColor", "backgroundColor"];
    
    if(type === "bool"){
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

                
                {/* bug => ColorPicker can't find its parent */}
                {showInjectedContent && (
                    <div className="colorInputParent">
                        {[...Array(colorVarOptions.length)].map((_, index) => (
                            <div className="inputElementColor" key={index}>
                                <label
                                    htmlFor={colorVarOptions[index]}
                                    dangerouslySetInnerHTML={{ __html: `Choose a ${parseCamelCase(colorVarOptions[index])}` }}
                                />
                                <div className={`pickr-${colorVarOptions[index]}`}>
                                    <ColorInput colorTag={colorVarOptions[index]}/>
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


