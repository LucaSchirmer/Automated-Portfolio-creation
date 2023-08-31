import Input from './Input.js';
import Textarea from './Textarea.js';
import DragnDrop from './DragnDrop.js';

const Services = () => {


    return (
    <div className="serviceContainter">

        <Input name="serviceTitle" labelContent="Title of your Service:"/>
        <Textarea name="serviceText" labelContent="Write a text about your Service:"/>
        <DragnDrop name="for your service"/>   
        
    </div>
    );
}


export default Services;