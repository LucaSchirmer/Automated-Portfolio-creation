import Input from './Input.js';
import Textarea from './Textarea.js';
import DragnDrop from './DragnDrop.js';

const Services = ({className}) => {


    return (
    <div className="serviceContainter">

        <Input name="serviceTitle" labelContent="Title of your Service:" className={className}/>
        <Textarea name="serviceText" labelContent="Write a text about your Service:" className={className}/>
        <DragnDrop name="for your service" className={className} key={className}/>   
        
    </div>
    );
}


export default Services;