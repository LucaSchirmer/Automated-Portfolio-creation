
use std::fs;
use std::io::Write;
use image;
use reqwest;


// main fn => calls programm
fn main(){
    println!("init Programm");


    //init files
    create_files("project");
}


// function to create the project files 
// taking {directory_name as String} as an input
fn create_files(directory_name: &str){
    let script_locations: String = directory_name.to_owned() + "/script";


    // init folders
    let project_folder = fs::create_dir(directory_name).expect("creation of dir failed");
    let script_folder = fs::create_dir(directory_name.to_owned() + "/script").expect("creation of dir failed");
    


    // script files
    let main_js_file = fs::File::create(script_locations.to_owned() + "/main.js").expect("creation failed");
    let intersection_observer_js_file = fs::File::create(script_locations.to_owned() + "/intersectionObserver.js").expect("creation failed");
    let portfolio_js_file = fs::File::create(script_locations.to_owned() + "/portfolio.js").expect("creation failed");
    let responsive_js_file = fs::File::create(script_locations.to_owned() + "/responsive.js").expect("creation failed");


    // html, css files
    let mut index_html_file = fs::File::create(directory_name.to_owned() + "/index.html").expect("creation failed");
    let portfolio_html_file = fs::File::create(directory_name.to_owned() + "/portfolio.html").expect("creation failed");
    let main_css_file = fs::File::create(directory_name.to_owned() + "/main.css").expect("creation failed");
    index_html_file.write(b"<!DOCTYPE html>
        <html lang=\"en\">
        
        <!-- Made by Luca Schirmer -->
        <head>
            <meta charset=\"UTF-8\">
            <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
            
            <meta name=\"description\" content=\"\">
            
            <link rel=\"stylesheet\" href=\"main.css\">
            <title>{title}</title>
        </head>
        <body style=\"overflow-x: hidden;\">
            <header class=\"topHeader\">
                <nav class=\"topNav\">
                    <div class=\"LogoContainer\">
                        <img src=\"Logo.png\" alt=\"Logo\">
                    </div>
                    <ul class=\"navList\">
                        <li class=\"ListItem\">
                            <a href=\"#\">Home</a>     
                        </li>
                        <li class=\"ListItem\">
                            <a href=\"#Portfolio\">Portfolio</a>
                        </li>
                        <li class=\"ListItem\">
                            <a href=\"#Services\">Services</a>
                        </li>
                        <li class=\"ListItem\">
                            <a href=\"#Contact\">Contact</a>
                        </li>
                        <li class=\"ListItem\">
                            <a href=\"#Clients\">Clients</a>
                        </li>
                    </ul>
                </nav>
        
                <nav class=\"topNavMobile\">
                    <ul class=\"indexUlMobile navList\">
                        <div class=\"LogoContainer\">
                            <img src=\"Logo.png\" alt=\"Logo\">
                        </div>
                        <li class=\"ListItem\">
                            <a href=\"#\">Home</a>     
                        </li>
                        
            
                        <div class=\"burger\">
                            <img class=\"toggleImageBurger\" src=\"img/cancelXpng.png\" alt=\"close Sidebar\">
                            <div class=\"line1\"></div>
                            <div class=\"line2\"></div>
                            <div class=\"line3\"></div>
                        </div>
                    </ul>
                    <ul class=\"sideUL\">
                        <li class=\"ListItem\" id=\"firstChildSideUl\">
                            <a href=\"#Portfolio\">Portfolio</a>
                        </li>
                        <li class=\"ListItem\">
                            <a href=\"#Services\">Services</a>
                        </li>
                        <li class=\"ListItem\">
                            <a href=\"#Contact\">Contact</a>
                        </li>
                        <li class=\"ListItem\">
                            <a href=\"#Clients\">Clients</a>
                        </li>
                    
                    </ul>  
                </nav>
                
        
            </header>
            <main class=\"indexMain\">
                <article class=\"articleAbout\" id=\"articleAbout\">
                    <div class=\"aboutFoto fadeIn\">
                        <img src=\"\" alt=\"aboutFoto\">
                    </div>
                    <div class=\"aboutName fadeIn\">
                        {aboutName}
                    </div>
                    <div class=\"aboutText fadeIn\">
                        {aboutText}
                    </div>      
                </article>
                <article class=\"articlePortfolio\" id=\"Portfolio\">
                    <!-- dynamic -->
        
                </article>   
                <article class=\"articleServices\" id=\"Services\">
        
                </article>     
                <article class=\"articleClients\" id=\"Clients\">
        
                </article>     
            </main>
            <footer class=\"articleContact\" id=\"Contact\">
                
            </footer>
        </body>
        
        <script src=\"scripts/main.js\"></script>
        <script src=\"scripts/responsive.js\"></script>
        <script defer src=\"scripts/intersectionObserver.js\"></script>
        
        </html>").expect("write failed");

    // json file 
    let data_json = fs::File::create("data.json").expect("creation failed");


    // todo adding autocreated fotos => using the github link

    let mut out = fs::File::create("rust_test.png").expect("failed to create file");
    let mut img = image::io::Reader::open("path/to/image.png")?
    .decode().raw_pixels();

    

    let request = reqwest::blocking::get("https://www.drive.google.com/uc?export=download&id=1qPgOzD3nuOtLHjSH1mn0OIP87-RCrZEP").expect("request failed");

    img = request.text().expect("REASON").as_bytes().to_vec();

    // img.save("rust_test.png");
    // let response = request.bytes().expect("body invalid");

    // let img = image::load_from_memory(&response).expect("loading memory failed");


    // io::copy(&mut img, &mut out).expect("failed to copy content");

    // data_json.write(&mut response.as_bytes());

}