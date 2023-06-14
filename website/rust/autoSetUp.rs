
use std::fs;
use std::io::Write;

fn main(){
    println("init Programm");

    create_files("project");
}

fn create_files(directoryName: String){
        
    let mut project_folder = fs::create_dir(directoryName);

    let mut index_html_file = fs::create("index.html").expect("creation failed");
    index_html_file.write("das ist ein Test").expect("write failed");

}

