
use std::fs;
use std::io::Write;
use image;
use reqwest::blocking::Client;


// main fn => calls programm
fn main(){
    println!("init Programm");


    //init files
    create_files("project");
}


struct FotoFile{
    name: String,
    url: String,
}

impl FotoFile {
    fn new(name: &str, url: &str) -> FotoFile {
        FotoFile { name: name.to_string(), url: url.to_string() }
    }

    fn name(&self) -> &String{
        &self.name
    }
    
    fn url(&self) -> &String{
        &self.url
    }
}

// function to create the project files 
// taking {directory_name as String} as an input
fn create_files(directory_name: &str){
    let script_locations: String = directory_name.to_owned() + "/script";
    let img_locations: String = directory_name.to_owned() + "/img";


    // init folders
    let project_folder = fs::create_dir(directory_name.to_owned()).expect("creation of main dir failed");
    let script_folder = fs::create_dir(script_locations.to_owned()).expect("creation of script dir failed");
    let img_folder = fs::create_dir(img_locations.to_owned()).expect("creation of img dir failed");
    


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


    let file_map = [
        FotoFile::new("cancelXpng.png", "https://www.drive.google.com/uc?export=download&id=1qPgOzD3nuOtLHjSH1mn0OIP87-RCrZEP"),
        FotoFile::new("Pfeil-links.png", "https://www.drive.google.com/uc?export=download&id=1nl2uPA5YB_MKzBH24w0a2Fc_5TSSjhAP"),
        FotoFile::new("Pfeil-rechts.png", "https://www.drive.google.com/uc?export=download&id=1jtrav1Dek-x9TSPGUZsVr4QO_rfB_4Yl"),
        FotoFile::new("instagram.png", "https://www.drive.google.com/uc?export=download&id=1-O98p9e5vLn55BreOVmfG6QbJp_6HKG-"),
        FotoFile::new("whatsapp.png", "https://www.drive.google.com/uc?export=download&id=1-SsFO1o3DaPZYhnRjGiV21prd_VK4CWu"),
        FotoFile::new("facebook.png", "https://www.drive.google.com/uc?export=download&id=1LLiZjgFZHQt21hdmf-TLpKdrF9rzAuO0"),
        FotoFile::new("pinterest.png", "https://www.drive.google.com/uc?export=download&id=1NLWay3PrahDrylPhMn_2f3CLEf56dph-"),
        FotoFile::new("youtube.png", "https://www.drive.google.com/uc?export=download&id=1Q-HmSVBTF1lTFqc-wllXufn-Hm6Wfehw"),
        FotoFile::new("linkedin.png", "https://www.drive.google.com/uc?export=download&id=1THqwA4OaztIt1yKZFYeWccIg3Sy-fxil"),
        FotoFile::new("github.png", "https://www.drive.google.com/uc?export=download&id=1UL-nPbpu2Ph6wLC2OIw1XqeZnUrVP2bu"),
        FotoFile::new("twitter.png", "https://www.drive.google.com/uc?export=download&id=1Z112yuzn14VrHaMvTwarIRY_gjiOPmBT"),
    ];   


    for file in file_map.iter() {
        // let mut out = fs::File::create(img_locations.to_owned() + file.name()).expect("failed to create img-file");
        // let mut img = image::io::Reader::open(img_locations.to_owned() + file.name()).expect("failed to read img");

        // let decode_img = img.decode().expect("cant decode");
        // let raw_pixel_img = decode_img.as_bytes();

        // // google drive link => of img content
        // let request = reqwest::blocking::get(file.url()).expect("request failed");

        // raw_pixel_img = request.text().expect("Converting Error").as_bytes();

        // Download the image from the Google Drive link
        let client = Client::new();
        let response = client.get(file.url()).send().expect("Request failed");

        let bytes = response.bytes().expect("Failed to retrieve image bytes");
        let raw_pixel_img: Vec<u8> = bytes.to_vec();

        // Save the image
        let img_path = format!("{}/{}", img_locations, file.name());
        let mut out = fs::File::create(&img_path).expect("Failed to create image file");
        out.write_all(&raw_pixel_img).expect("Failed to write image file");
    }  

}

// fn get_image_data(hash_map: HashMap::<String, String>, directory_name, img_locations){
//      // todo adding autocreated fotos => using the github link


//     // img.save("rust_test.png");
//     // let response = request.bytes().expect("body invalid");

//     // let img = image::load_from_memory(&response).expect("loading memory failed");


//     // io::copy(&mut img, &mut out).expect("failed to copy content");

//     // data_json.write(&mut response.as_bytes());

// }
