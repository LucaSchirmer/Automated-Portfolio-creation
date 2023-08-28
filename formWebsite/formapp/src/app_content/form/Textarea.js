import React, { useState } from 'react';

const Textarea = ({name,labelContent=name, required=false}) => {
    const [value, setValue] = useState(''); 

    const handleInputChange = (event) => {
        setValue(event.target.value);

        console.log(event.target.value);
    }; 

    if(required){
         labelContent = labelContent + '<span class="StartYellowSpan"> <abbr title="Your response here is required.">*</abbr></span>';
    }

    return (
    <div className="formElement">
        <label 
            className="inputTextAreaLabel"
            htmlFor={name} 
            dangerouslySetInnerHTML={{ __html: labelContent }}
        />


        <textarea
            name={name}
            value={value} 
            onChange={handleInputChange}
        />

    </div>
    );
}


export default Textarea;
