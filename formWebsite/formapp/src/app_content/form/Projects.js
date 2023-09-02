import Input from './Input.js';
import Textarea from './Textarea.js';
import DragnDrop from './DragnDrop.js';

const Projects = ({className}) => {


    return (
    <div className="projectContainter">

        <Input name="titleProject" labelContent="Title of your Project:" className={className}/>
        <Textarea name="projectText" labelContent="Write a text about you Project:" className={className}/>
        <DragnDrop name="for your project" className={className} key={className}/>   
        
    </div>
    );
}


export default Projects;