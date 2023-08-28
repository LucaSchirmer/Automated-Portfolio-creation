import Input from './Input.js';
import Textarea from './Textarea.js';
import DragnDrop from './DragnDrop.js';

const Projects = () => {


    return (
    <div className="projectContainter">

        <Input name="titleProject" labelContent="Title of your Project:"/>
        <Textarea name="projectText" labelContent="Write a text about you Project:"/>
        <DragnDrop/>   
        
    </div>
    );
}


export default Projects;