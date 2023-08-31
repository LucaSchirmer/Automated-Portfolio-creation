import Input from './Input.js';
import DragnDrop from './DragnDrop.js';

const Customer = ({className}) => {

    return (
        <div className={`customerContainter ${className}`}>

            <Input name="customerTitle" labelContent="Name of your customer:" className={className}/>
            <DragnDrop 
                name="of your customer (e.g. Logo)" 
                className={className} 
            />   
            
        </div>
    );
}


export default Customer;