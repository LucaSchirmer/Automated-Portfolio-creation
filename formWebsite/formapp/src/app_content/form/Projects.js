import Input from './Input.js';
import Textarea from './Textarea.js';
import DragnDrop from './DragnDrop.js';

const Projects = ({title, labelContent=title}) => {


    return (
    <div className="projectContainter">

        <Input name={title} labelContent={labelContent}/>
        <Textarea name="projectText" labelContent="Write a text about you Project"/>
        <DragnDrop/>   
        
    </div>
    );
}


export default Projects;