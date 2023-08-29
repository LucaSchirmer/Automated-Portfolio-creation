import Input from './Input.js';
import DragnDrop from './DragnDrop.js';

const Customer = () => {


    return (
    <div className="customerContainter">

        <Input name="customerTitle" labelContent="Name of your customer:"/>
        <DragnDrop name="customer (e.g. Logo)"/>   
        
    </div>
    );
}


export default Customer;