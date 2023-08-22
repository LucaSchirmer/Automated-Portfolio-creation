import React, { useState } from 'react';

const Textarea = ({name,labelContent=name, required=false, type="text"}) => {
    const [value, setValue] = useState(''); 

    const handleInputChange = (event) => {
        setValue(event.target.value);

        console.log(event.target.value);
    }; 

    if(required){
        labelContent = labelContent + '<span class="StartYellowSpan"> *</span>';
    }

    return (
    <div className="formElement">
        <label 
            htmlFor={name} 
            dangerouslySetInnerHTML={{ __html: labelContent }}
        />


        <textarea
            type={type}
            name={name}
            value={value} 
            onChange={handleInputChange}
        />

    </div>
    );
}


export default Textarea;
