
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
    let responsive_fjs_file = fs::File::create(script_locations.to_owned() + "/responsive.js").expect("creation failed");


    // html, css files
    let mut index_html_file = fs::File::create(directory_name.to_owned() + "/index.html").expect("creation failed");
    let mut portfolio_html_file = fs::File::create(directory_name.to_owned() + "/portfolio.html").expect("creation failed");
    let mut main_css_file = fs::File::create(directory_name.to_owned() + "/main.css").expect("creation failed");
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

    portfolio_html_file.write(b"<!DOCTYPE html>
    <html lang=\"en\">
    <head>
        <meta charset=\"UTF-8\">
        <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    
        <meta name=\"description\" content=\"\">
    
        <title>{Portfolio}</title>
    
        <link rel=\"stylesheet\" href=\"main.css\">
    </head>
    <body>
        <header class=\"topHeader\">
            <nav class=\"topNav\">
                <div class=\"LogoContainer\">
                    <img src=\"Logo.png\" alt=\"Logo\">
                </div>
                <ul class=\"navList\">
                    <li class=\"ListItem\">
                        <a href=\"index.html\">Home</a>     
                    </li>
                    <li class=\"ListItem\">
                        <a href=\"index.html#Portfolio\">Portfolio</a>
                    </li>
                    <li class=\"ListItem\">
                        <a href=\"index.html#Services\">Services</a>
                    </li>
                    <li class=\"ListItem\">
                        <a href=\"index.html#Contact\">Contact</a>
                    </li>
                    <li class=\"ListItem\">
                        <a href=\"index.html#Contact\">Clients</a>
                    </li>
                </ul>
            </nav>
    
            <nav class=\"topNavMobile\">
                <ul class=\"indexUlMobile navList\">
                    <div class=\"LogoContainer\">
                        <img src=\"Logo.png\" alt=\"Logo\">
                    </div>
                    <li class=\"ListItem\">
                        <a href=\"index.html\">Home</a>     
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
        <main class=\"portfolioMain\">
            <article class=\"subPortfolio\" id=\"subPortfolio\" style=\"padding-top: 8vh;\">
                <div class=\"buttonLeft\">
                    <img src=\"img/Pfeil-links.png\" alt=\"arrow left\" class=\"imgLeftArrow\" onclick=\"
                    let n = window.location.search.replace('?project=', ''); 
                    let output = parseInt(n) - 1;
                    console.log(output)
                    getData(output, true);
                    \">
                </div>
                <div class=\"buttonRight\">
                    <img src=\"img/Pfeil-rechts.png\" alt=\"arrow right\" class=\"imgRightArrow\"onclick=\"
                    let n = window.location.search.replace('?project=', ''); 
    
                    let output = parseInt(n) + 1;
                    console.log(output)
                    getData(output, true);
                    \">
                </div>
            </article>
        </main>
        <footer class=\"articleContact\" id=\"Contact\">
            
        </footer>
    </body>
    <script src=\"scripts/portfolio.js\"></script>
    <script src=\"scripts/responsive.js\"></script>
    </html>"); 

    main_css_file.write(b":root{
        font-size: 16px;
        font-weight: 400;
        line-height: 1.5;
    
        /* --mainColor: rgba(20, 20, 20, 1);
        --secondaryColor: rgb(33, 110, 211);
        --accentColor: rgb(28, 28, 155);
        --backgroundColor: rgba(255, 255, 255,1); */
    
        /* Has to be edited induvidually each time */
        --complementaryColor: rgba(249, 248, 113, 1);
    
        --fontColor: rgba(255, 255, 255,1);
    
        --fontFamily: 'Lato', sans-serif;
    }
    
    *{
        padding: 0;
        margin: 0;
    
        scroll-behavior: smooth;
    
        font-family: var(--fontFamily);
    } 
    
    
    /* toggle classes */
    
    .toggleFixed{
        position: fixed;
        top: 0;
        width: 100%;
    }
    
    .toggleRotate{
        transform: rotate(180deg);
        align-self: flex-start;
    }
    
    .nav-active{
        transform: translateX(0%) !important;
    }
    
    /* Navbar-Style */
    
    a{
        text-decoration: none;
        color: var(--fontColor)
    }
    
    ul{
        text-decoration: none;
        list-style-type: none;
    }
    
    .topHeader{
        width: 100%;
        height: 8vh;
    
        background-color: var(--secondaryColor);
        color: var(--fontColor);
    
        position: fixed;
        z-index: 1000;
    }
    
    .topNav, .topNavMobile{
        display: flex;
        flex-direction: row;
    }
    
    .LogoContainer{
        width: 30%;
        height: 8vh;
    }
    
    .navList{
        display: flex;
        flex-direction: row;
        justify-content: space-around;
        align-items: center;
    
        width: 70%;
        height: 8vh;
    }
    
    .ListItem{
        text-transform: uppercase;
        font-weight: 600;
        font-size: max(16px, 2vh);
    }
    
    /* responsive nav */
    
    .burger{
        display: none;
    }
    
    .burger img{
        height: 3.5vh;    
    }
    
    .burger div{
        width: 25px;
        height: 2px;
    
        background-color: white;
        transition: all 0.3s ease;
    }
    
    
    .toggleBurger div{
        display: none;
    }
    
    .toggleImageBurger{
        display: none; 
    }
    
    .indexUlMobile{
        display: flex; 
        flex-direction: row;
        width: 100%;
    }
    
    .indexUlMobile, .sideUL{
        display: none;
    }
    
    
    /* About article */
    
    .articleAbout{
        min-height: 92vh;
        padding-top: 8vh;
    
        display: grid;
        grid-template-areas: 
        \"a b\"
        \"a c\";
    
        text-align: center;
    }
    
    .aboutFoto{
        grid-area: a;
        width: 45vw;
    }
    
    .aboutName{
        grid-area: b;
        height: 15vh;
        width: 100%;
    
        padding-top: 2vh;
    
        color: var(--secondaryColor);
        font-size: 5vh;
        font-weight: 600;
    }
    
    .aboutText{
        height: 75vh;
        width: 50vw;
        grid-area: c;
    
        line-height: 2;
    }
    
    /* Portfolio Article */
    
    .articlePortfolio{
       display: grid; 
    
       grid-template-columns:  1fr 1fr 1fr;
       grid-template-rows: auto;
    
       justify-content: space-around;
    
       background-color: var(--secondaryColor);    
    
       column-gap: 5%;
       padding: 10vh 5vw 10vh 5vw;
    }
    
    .bannerPortfolio{
        height: 65vh;
    
        background-color: var(--backgroundColor);
    
        display: grid; 
        grid-template-rows: minmax(30vh, 3fr) 1fr;
        justify-content: space-evenly;
        border-radius: 3%;
    
        padding: 2%;
        margin-bottom: 10vh;
    }
    
    .subBannerPortfolio{
        height: 100vh;
    
        background-color: var(--backgroundColor);
        padding: 20vh;
    }
    
    .headPicture{
        aspect-ratio: 4/3;
        object-fit: contain;
    
        display: grid;
        justify-content: center;
        align-content: center;
    }
    
    .bannerArticle{
        padding-inline: 2%;
        line-height: 1.25;
    
        text-align: justify;
    }
    
    .bannerHeadline{
        color: var(--accentColor);
        filter: invert(1);
        font-weight: 800;
    
        padding: 3%;
    }
    
    .bannerText{
        color: var(--mainColor);
        padding: 3%;
    }
    
    .imgLeftArrow, .imgRightArrow{
        position: fixed;
        padding-top: 40vh;
        padding-left: 10vh;
        padding-right: 10vh;
    
        width: 5vw;
    
        z-index: 100;
    }
    
    .imgRightArrow{
        right: 0;
    }
    
    .headPicture img{
        width: 100%;
        object-fit: cover;
    
        max-height: 25vh;
    }
    
    .subBannerPortfolio .headPicture{
        max-height: 25vh;
        width: 100%;
    }
    
    
    /* Services Article */
    
    .articleServices{
        min-height: 50vh;
    }
    
    .service{
        padding: 20vh;
        min-height: 30vh;
        text-align: center;
    
        display: grid; 
        grid-template-columns: 1fr 2fr;
    }
    
    .articleServices .service:nth-child(2){
        background-color: var(--secondaryColor);
        color: #fff;
    }
    
    .serviceHeadline{
        font-size: 3rem;
    }
    
    .serviceText{
        font-size: 1.5rem;
    }
    
    /* Clients Article*/
    
    .articleClients{
        display: flex;
        flex-wrap: wrap;
        flex-wrap: wrap;
        flex-direction: row;
        justify-content: space-evenly;
    }
    
    .client{
        width: 21.8%;
        padding-top: 10vh;
        padding-bottom: 10vh;
        text-align: center;
    }
    
    /* Contact Article*/
    
    .articleContact{
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
    
        padding: 10vh;
        background-color: rgb(25, 22, 22);
    }
    
    .contactContainer{
        display: grid; 
    
        width: 100%;
    
        place-content: center;
    }
    
    .contact{
        height: 14vh; 
    
        display: flex; 
        flex-direction: column;
    }
    
    .contactSpan{
        color: white;
        margin: 1%;
        margin-top: 2vh;
    
        text-align: center;
    }
    
    .contact a{
        height: 7.5vh;
        text-align: center;
    }
    
    .contactIMG{
        height: 100%;
        padding: 1%;
    
        object-fit: contain;
        aspect-ratio: 4/3;
    }
    
    
    .mailNphone{
        display: flex; 
        flex-flow: row wrap;
        width: 100%;
        padding-left: 5vh;
        padding-right: 5vh;
        padding-top: 0vh;
        padding-bottom: 10vh;
        
        justify-content: space-evenly;
    }
    
    
    .contactEmail{
        padding-bottom: 2vh;
    }
    
    
    
    
        /*  Animations   */
    
    .fadeIn{
        opacity: 0;
    }
    
    .left{
        transform: translateX(-50%);    
    }
    
    .right{
        transform: translateX(50%);
    }
    
    .appear{
        transition: all 500ms ease-in;
        opacity: 1 !important;
        transform: translateX(0) !important;
    }
    
    
    
    /* Responsive */
    
    @media (max-width: 1000px){
    
        .client{
            width: 30%;
        }
    }
    
    @media (max-width: 800px){
    
        .topNav{
            display: none;
        }
    
        .indexUlMobile{
            display: flex;
        }
    
        .burger{
            display: flex;
            height: 60%;
            justify-content: space-evenly;
            flex-direction: column;
            cursor: pointer;                
        }
    
        .sideUL{
            top: 8vh;
            right: 0px;
    
            height: 92vh;
            width: 60vw;
            
            display: block;
            position: fixed;
            z-index: 2000;
    
            background-color: var(--secondaryColor);
            transform: translateX(100%);
            transition: transform 0.66s ease-in;
    
            display: flex;
            flex-direction: column;
            align-items: center;
        }
    
        .sideUL li{
            font-size: 16px;
            padding-top: 8vh;
            padding-bottom: 8vh;
        }
    
        #firstChildSideUl{
            padding-top: 12vh;
        }
    
        .articleAbout{
            display: flex; 
            flex-direction: column;
        }
    
        .aboutFoto{
            width: 100vw;
            margin-top: 5vh; 
        }
    
        .aboutFoto img{
            height: 30vh;
    
            margin: 2.5%
        }
    
        .aboutText{
            width: 90vw; 
            padding-left: 5vw;
            padding-right: 5vw;
        }
    
        .articlePortfolio{
            grid-template-columns: 1fr 1fr;
        }
    
        .service{
            display: flex;
            flex-direction: column;
        }
        
        .servicePicture{
            padding-bottom: 10vh;
        }
    
        .subPortfolio{
            background-color: var(--backgroundColor);
        }
    
        .subBannerPortfolio{
            margin-top: 30vh
        }
    
        .buttonLeft, .buttonRight{
            width: 50%; 
        }
    
        .buttonLeft img, .buttonRight img{
            left: 0;  
            position: fixed; 
            width: 25%;
            padding: 0;
            padding-left: 12.5%; 
            padding-top: 2.5%; 
        }
        .buttonRight img{
            left: 50%; 
        }
    
        .serviceHeadline{
            font-size: 1.75rem;
        }
        
        .serviceText{
            font-size: 1rem;
        }
        
        .service{
            padding: 10vh;
        }
    
        .subBannerPortfolio{
            padding: 7vh;
        }
        
    }
    
    
    @media (max-width: 600px){
        .articlePortfolio{
            grid-template-columns: 1fr;
        }
    }")

    // json file 
    let _data_json = fs::File::create("data.json").expect("creation failed");


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