import {useState} from "react";

const NumberInput = ({ name, labelContent = name, type = "number", required="true", changeState}) => {

    const [num, setNum] = useState(0);

    if (required) {
        labelContent = labelContent + '<span class="StartYellowSpan"> <abbr title="Your response here is required.">*</abbr></span>';
    }

    return (
        <div className="inputElement">
            <label 
                className="inputTextAreaLabel"
                htmlFor={name} 
                dangerouslySetInnerHTML={{ __html: labelContent}}
            />
            <input
                name="amoutOfProjects" 
                type={type}
                min={0} 
                max={20}
                value={num}
                onChange={(event) =>{
                    changeState(event.target.value);
                    setNum(event.target.value);
                }}

                // prevent scroll when clicking on the Input
                onWheel={(e) => e.target.blur()}
            />
        </div>
    );
}


export default NumberInput;


