import ColorPicker from './ColorPicker';


const ColorInput = ({colorTag}) => {

    
    const parseCamelCase = (el)=>{
        //  parses camelCase into first word  ( from firstWord )
        el = el.split("C");
        el = el[0];
        el += " color";

        return el; 
    }

    return(
        <div className="colorInputParent">
            <div className="inputElementColor">
                <label
                    htmlFor={colorTag}
                    dangerouslySetInnerHTML={{ __html: `Choose a ${parseCamelCase(colorTag)}` }}
                />
                <div className={`pickr-${colorTag}`}>
                    <ColorPicker name={colorTag}/>
                </div>
            </div>
        </div>
    );

};

export default ColorInput;