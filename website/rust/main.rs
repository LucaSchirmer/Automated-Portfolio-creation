
use std::fs;
use std::io::Write;

fn main(){
    println!("init Programm");

    create_files("project");
}

fn create_files(directoryName: &str){
        
    let project_folder = fs::create_dir(directoryName);
    let script_folder = fs::create_dir(directoryName.to_owned() + "/script");
    
    // script files
    let main_js_file = fs::File::create("/script/".to_owned() + "main.js");

    let mut index_html_file = fs::File::create("index.html").expect("creation failed");
    let portfolio_html_file = fs::File::create("portfolio.html").expect("creation failed");
    let main_css_file = fs::File::create("main.css").expect("creation failed");
    index_html_file.write(b"das ist ein Test").expect("write failed");

}

