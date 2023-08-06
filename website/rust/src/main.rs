
use std::fs;
use std::io::Write;
use std::io::copy;
use std::path::Path;
use reqwest::blocking::Client;
use zip::read::ZipArchive;

// for google forms interaction
use google_drive3::{
    GoogleForms, DefaultError, DefaultTokenResponse, Result,
    api::Form,
    auth::GoogleAuthenticator,
    types::{Date, DateTime, TimeOfDay},
};
use yup_oauth2::ServiceAccountAuthenticator;



// main fn => calls programm
fn main(){
    println!("init Programm");


    //init files => old method 
    // create_files("project");

    // get "create" new project 
    create_files_github();
}


// struct FotoFile{
//     name: String,
//     url: String,
// }

// impl FotoFile {
//     fn new(name: &str, url: &str) -> FotoFile {
//         FotoFile { name: name.to_string(), url: url.to_string() }
//     }

//     fn name(&self) -> &String{
//         &self.name
//     }
    
//     fn url(&self) -> &String{
//         &self.url
//     }
// }


fn create_files_github(){

    let project_url = "https://github.com/LucaSchirmer/Automated-Portfolio-creation/archive/master.zip"; 

    // current directory (./) where the .exe is executed + file name 
    let destination_path = "./project.zip";

    // Responsive of the zip.file
    let mut response = reqwest::blocking::get(project_url).expect("Request failed!");

    let mut zip_file = fs::File::create(Path::new(destination_path)).expect("File creation failed!");

    copy(&mut response, &mut zip_file).expect("Failed file write!");


    unzip_github_project(destination_path, "./");

    // Delete the .zip file after extraction
    if let Err(err) = fs::remove_file(destination_path) {
        eprintln!("Error deleting .zip file: {}", err);
    }
}



fn unzip_github_project(zip_file_path: &str, destination_folder_path: &str){

    // opening zip
    let zip_content = fs::File::open(zip_file_path).expect("File Opening failed!");    
    let mut archive = ZipArchive::new(zip_content).expect("ZIP archiving failed!");


    for i in 0..archive.len(){
        let mut file = archive.by_index(i).expect("Failed to get file from archive");
        let file_name = file.name();
        let file_path = format!("{}/{}", destination_folder_path, Path::new(file_name).display());

        // test if given index is a file or a directory
        if file.is_dir() {
            std::fs::create_dir_all(&file_path).expect("Folder creation failed!");
        } else {
            let mut output_file = fs::File::create(&file_path).expect("Failed to create file");
            std::io::copy(&mut file, &mut output_file).expect("Failed to extract file");
        }
    }

    // Close the ZIP archive file
    drop(archive);
}



fn read_poll(){

    const ACCESS_TOKEN: &str  = "";

}

// function to create the project files 
// taking {directory_name as String} as an input
// fn create_files(directory_name: &str){
//     let script_locations: String = directory_name.to_owned() + "/script";
//     let img_locations: String = directory_name.to_owned() + "/img";


//     // init folders
//     let project_folder = fs::create_dir(directory_name.to_owned()).expect("creation of main dir failed");
//     let script_folder = fs::create_dir(script_locations.to_owned()).expect("creation of script dir failed");
//     let img_folder = fs::create_dir(img_locations.to_owned()).expect("creation of img dir failed");
    


//     // script files
//     let mut main_js_file = fs::File::create(script_locations.to_owned() + "/main.js").expect("creation failed");
//     let mut intersection_observer_js_file = fs::File::create(script_locations.to_owned() + "/intersectionObserver.js").expect("creation failed");
//     let mut portfolio_js_file = fs::File::create(script_locations.to_owned() + "/portfolio.js").expect("creation failed");
//     let mut responsive_fjs_file = fs::File::create(script_locations.to_owned() + "/responsive.js").expect("creation failed");

//     main_js_file.write(b"// Aquire data
//         async function getData(){
        
//             // Url Json file
//             let dataJsonUrl = \"../data.json\";
        
//             const request =  await fetch(dataJsonUrl);
//             const response = await request.json();
            
//             addMetaDescription(response[0].metaDescription);
//             document.title = response[0].title;
//             document.querySelector(\".aboutName\").innerHTML = response[0].title;
//             document.querySelector(\".aboutText\").innerHTML = response[0].aboutText;
//             document.querySelector(\".aboutFoto\").firstElementChild.src = response[0].aboutFoto;
//             document.querySelector(\".aboutFoto\").firstElementChild.alt = response[0].aboutFoto;
        
//             document.querySelector(\":root\").style.setProperty(\"--mainColor\", response[0].mainColor);
//             document.querySelector(\":root\").style.setProperty(\"--secondaryColor\", response[0].secondaryColor);
//             document.querySelector(\":root\").style.setProperty(\"--accentColor\", response[0].accentColor);
//             document.querySelector(\":root\").style.setProperty(\"--backgroundColor\", response[0].backgroundColor);
            
        
        
//             // Adding Projects
//             for(let i in response[1]){
        
//                 let projectArr = [];
        
//                 for (const value of Object.values(response[1][i])) {
//                     projectArr.push(value)
//                 }
        
//                 addProject(projectArr[0], projectArr[1], projectArr[2], i);
//             } 
            
//             // Adding Services
//             for(let i in response[2]){
        
//                 let serviceArr = [];
        
//                 for (const value of Object.values(response[2][i])) {
//                     serviceArr.push(value)
//                 }
        
//                 addServices(serviceArr[0], serviceArr[1], serviceArr[2]);
//         }
        
//             // Adding Clients
//             for(let i in response[2]){
        
//                 let clienteArr = [];
//                 if(response[3][0]){
//                     for (const value of Object.values(response[3][i])) {
//                         clienteArr.push(value)
//                     }
//                 }
        
//                 addClients(clienteArr[0], clienteArr[1], clienteArr[2]);
//             }
        
                
//             // Adding Contacts
//             addMailNumberContacts(response[0].email, response[0].phoneNumber);
        
//             // Adding Parent div for contacts
//             addContantDiv();
        
//             let i = 0;
//             for (const [key, value] of Object.entries(response[4])) {
//                 i += 1;
//                 addContact(key, value);
//             }
//             console.log(response);
//             let contactContainer = document.querySelector(\".contactContainer\");
//             if(i % 3 == 0 || i > 7) {
//                 contactContainer.style.gridTemplateColumns = \"1fr 1fr 1fr\";
//             }else{
//                 contactContainer.style.gridTemplateColumns = \"1fr 1fr\";
//             }
        
        
        
        
//             const faders = document.querySelectorAll(\".fadeIn\");
//             startObserver(faders);
//         }
        
//         getData();
        
        
        
        
//         function addMetaDescription(desc){
//             document.querySelector('meta[name=\"description\"]').setAttribute(\"content\", desc);
//         }
        
        
//         /**
//          * @param {title => means name of the brand as String} title 
//          * @param {link regarding the contact connection} link 
//          */
        
//         function addContact(title, link){
//             const contactContainer = document.querySelector(\".contactContainer\");
            
//             //html prefab
//             const htmlString = 
//             `
//                 <a class=\"contactAnker\" href=\"${link}\">
//                     <img class=\"contactIMG\" src=\"img/${title}.png\" alt=\"${title}\"> 
//                 </a>
//                 <span class=\"contactSpan\"> ${title} </span>
//             `;
        
//             const contactDiv = document.createElement(\"div\");
        
//             contactDiv.classList.add(\"contact\");
//             contactDiv.classList.add(\"fadeIn\");
//             contactDiv.setAttribute(\"id\", title);
//             contactDiv.innerHTML = htmlString;
        
//             contactContainer.appendChild(contactDiv);
//         }
        
        
//         function addContantDiv(){
//             let contactDiv = document.createElement(\"div\");
//             contactDiv.classList.add(\"contactContainer\");
        
//             document.querySelector(\".articleContact\").appendChild(contactDiv)
//         }
        
//         /**
//          * @param {title of the Portfolio} title 
//          * @param {text of the Portfolio} text 
//          * @param {fotoUrl URL if the image} fotoUrl 
//          * @param {num of the Projects} num 
//          */
        
//         function addProject(title, text, fotoUrl, num){
//             const articlePortfolio = document.querySelector(\".articlePortfolio\");
            
//             if(text.length > 150){
//             text = text.slice(0,147);
//             text += \"...\";
//             console.log(text)
//         }
        
//             //html prefab
//             const htmlString = 
//             `
//                 <div class=\"headPicture\">
//                     <img src=\"${fotoUrl}\" alt=\"${title}\" id=\"${title.replace(\" \", \"_\").replace(\"=\", \"_\")}\">
//                 </div>
//                 <div class=\"bannerArticle\">
//                     <h1 class=\"bannerHeadline\">
//                         ${title}
//                     </h1>
        
//                     <!-- Text has to be limited to a certain amout of characters -->
//                     <p class=\"bannerText\">
//                         ${text}
//                     </p>
//                 </div>
//             `;
        
//             const bannerPortfolio = document.createElement(\"div\");
        
//             bannerPortfolio.classList.add(\"bannerPortfolio\");
//             bannerPortfolio.classList.add(\"fadeIn\");
//             bannerPortfolio.classList.add(\"left\");
//             bannerPortfolio.setAttribute(\"id\", title);
//             bannerPortfolio.innerHTML = htmlString;
        
//             // adding click event
//             // logic to open the project in a new .html - file
//             bannerPortfolio.addEventListener(\"click\", () =>{
        
//                 var url = window.location.href;
//                 let arrayUrl = url.split(\"/\");
        
//                 // \"deleting\" last element
//                 arrayUrl[arrayUrl.length-1] = \"\";
        
//                 let newURL = arrayUrl.join(\"/\")
        
//                 let param = new URLSearchParams();
//                 param.append(\"project\", num);
        
//                 // redirecting to portfolio.html with the choosen content 
//                 window.location.href =  `${newURL}portfolio.html?${param}`; 
        
//             });
        
//             articlePortfolio.appendChild(bannerPortfolio);
//         }
        
        
//         /**
//          * @param {title of the Service} title 
//          * @param {text of the Service} text 
//          */
        
//         function addServices(title, text, fotoUrl){
//             const articleServices = document.querySelector(\".articleServices\");
            
        
//             // html prefab
//             const htmlString = 
//             `
//                 <div class=\"servicePicture fadeIn left\">
//                     <img src=\"${fotoUrl}\" alt=\"${title}\" id=\"${title}\">
//                 </div>
//                 <div class=\"serviceArticle fadeIn right\">
//                     <h1 class=\"serviceHeadline\">
//                         ${title}
//                     </h1>
        
//                     <!-- Text has to be limited to a certain amout of characters -->
//                     <p class=\"serviceText\">
//                         ${text}
//                     </p>
//                 </div>
//             `;
        
//             const service = document.createElement(\"div\");
        
//             service.classList.add(\"service\");
//             service.setAttribute(\"id\", title);
//             service.innerHTML = htmlString;
        
//             articleServices.appendChild(service);
//         }
        
        
//         /**
//          * @param {name of the client} name 
//          * @param {fotoUrl of the client} fotoUrl 
//          */
        
//         // Perhaps adding a Photo?  
//         function addClients(name, fotoUrl){
//             const articleClients = document.querySelector(\".articleClients\");
            
//             // html prefab
//             const htmlString = 
//             `
//                 <div class=\"clientPicture\">
//                     <img src=\"${fotoUrl}\" alt=\"${name} Picture\">
//                 </div>
//                 <div class=\"clientName\">
//                     <h1 class=\"clientHeadline\">
//                     ${name}
//                     </h1>
//                 </div>
//             `;
        
//             const client = document.createElement(\"div\");
        
//             client.classList.add(\"client\");
//             client.classList.add(\"fadeIn\");
//             client.classList.add(\"right\");
            
//             client.setAttribute(\"id\", name);
//             client.innerHTML = htmlString;
        
//             articleClients.appendChild(client);
//         }
        
        
//         /**
//          * @param {email of the client} email 
//          * @param {number of the client} number 
//          */
        
//         function addMailNumberContacts(email, number){
//             const articleContact = document.querySelector(\".articleContact\");
            
//             // html prefab
//             const htmlString = 
//             `
//                 <a class=\"contactEmail\" href=\"mailto: ${email}\">
//                     email: ${email}
//                 </a>
//                 <a class=\"contactPhone\" href=\"tel: ${number}\">
//                     phonenumber: ${number}
//                 </a>
//             `;
        
//             const contactDiv = document.createElement(\"div\");
        
//             contactDiv.classList.add(\"mailNphone\");
//             contactDiv.classList.add(\"fadeIn\");
//             contactDiv.innerHTML = htmlString;
        
//             articleContact.appendChild(contactDiv);
//         }");

//     intersection_observer_js_file.write(b"function startObserver(faders){
//         const options = {
//             threshold: 0, 
//             rootMargin: \"0px 0px -100px 0px\"
//         }
    
    
//         const onScroll = new IntersectionObserver(
//             function(entries, onScroll){
//                 entries.forEach(entry => {
//                     if(!entry.isIntersecting){
//                         return;
//                     }else{
//                         entry.target.classList.add(\"appear\");
//                         onScroll.unobserve(entry.target);
//                     }
//                 });
//             }, options  
//         );
    
    
//         faders.forEach(fader => {
//             console.log(fader)
//             onScroll.observe(fader)
//         });
    
//     }");       

//     portfolio_js_file.write(b"// Aquire data
//         async function getData(projectNum, bool = false){
        
//             // Url Json file
//             let dataJsonUrl = \"../data.json\";
        
//             const request =  await fetch(dataJsonUrl,);
//             const response = await request.json();
        
//             var projectNumMax = 0;
        
//             addMetaDescription(response[0].metaDescription);
            
        
//             document.querySelector(\":root\").style.setProperty(\"--mainColor\", response[0].mainColor);
//             document.querySelector(\":root\").style.setProperty(\"--secondaryColor\", response[0].secondaryColor);
//             document.querySelector(\":root\").style.setProperty(\"--accentColor\", response[0].accentColor);
//             document.querySelector(\":root\").style.setProperty(\"--backgroundColor\", response[0].backgroundColor);
        
        
//             let setLength = 10000;
        
//             for(let i = 0; i <= setLength; i++){
//                 if(!response[1][i]){
//                     projectNumMax = i-1; 
//                     break;
//                 }
//             }
        
        
        
//             let projectArr = [];
//             if(projectNum > projectNumMax){
//                 projectNum = 0;
//             }
            
//             if(projectNum < 0){
//                 projectNum = projectNumMax;
//             }
//             var url = window.location.href;
//             let arrayUrl = url.split(\"/\");
        
//             arrayUrl[arrayUrl.length-1] = \"\";
        
//             let newURL = arrayUrl.join(\"/\")
        
//             let param = new URLSearchParams();
//             param.append(\"project\", projectNum);
        
//             if(bool == true){
//                 window.location.href =  `${newURL}portfolio.html?${param}`; 
//             }
            
        
        
        
//             for (const value of Object.values(response[1][projectNum])) {
//                 projectArr.push(value)
//             }
        
//             addProject(projectArr[0], projectArr[1], projectArr[2]);
//             document.title = projectArr[0];
        
//             // Adding Contacts
//             addMailNumberContacts(response[0].email, response[0].phoneNumber);
        
//             // Adding Parent div for contacts
//             addContantDiv();
        
//             let i = 0;
//             for (const [key, value] of Object.entries(response[4])) {
//                 i += 1;
//                 addContact(key, value);
//             }
        
//             let contactContainer = document.querySelector(\".contactContainer\");
//             if(i % 3 == 0 || i > 7) {
//                 contactContainer.style.gridTemplateColumns = \"1fr 1fr 1fr\";
//             }else{
//                 contactContainer.style.gridTemplateColumns = \"1fr 1fr\";
//             }
        
        
//             console.log(response);
//         }
        
//         var projectNum = window.location.search.replace(\"?project=\", \"\"); 
        
//         getData(projectNum);
        
//         /**
//          * @param {title of the Portfolio} title 
//          * @param {text of the Portfolio} text 
//          * @param {fotoUrl URL if the image} fotoUrl 
//          * @param {num of the Projects} num 
//          */
        
//         function addProject(title, text, fotoUrl){
//             const articlePortfolio = document.querySelector(\".subPortfolio\");
            
//             const htmlString = 
//             `
//                 <div class=\"headPicture\">
//                     <img src=\"${fotoUrl}\"alt=\"${title}\" id=\"${title.replace(\" \", \"_\").replace(\"=\", \"_\")}\">
//                 </div>
//                 <div class=\"bannerArticle\">
//                     <h1 class=\"bannerHeadline\">
//                         ${title}
//                     </h1>
        
//                     <!-- Text has to be limited to a certain amout of characters -->
//                     <p class=\"bannerText\">
//                         ${text}
//                     </p>
//                 </div>
//             `;
        
//             const bannerPortfolio = document.createElement(\"div\");
        
//             bannerPortfolio.classList.add(\"subBannerPortfolio\");
//             bannerPortfolio.setAttribute(\"id\", title);
//             bannerPortfolio.innerHTML = htmlString;
        
        
//             articlePortfolio.appendChild(bannerPortfolio);
//         }
        
        
//         /**
//          * @param {title => means name of the brand as String} title 
//          * @param {link regarding the contact connection} link 
//          */
        
//         function addContact(title, link){
//             const contactContainer = document.querySelector(\".contactContainer\");
            
//             const htmlString = 
//             `
//                 <a class=\"contactAnker\" href=\"${link}\">
//                     <img class=\"contactIMG\" src=\"img/${title}.png\" alt=\"${title}\"> 
//                 </a>
//                 <span class=\"contactSpan\"> ${title} </span>
//             `;
        
//             const contactDiv = document.createElement(\"div\");
        
//             contactDiv.classList.add(\"contact\");
//             contactDiv.setAttribute(\"id\", title);
//             contactDiv.innerHTML = htmlString;
        
//             contactContainer.appendChild(contactDiv);
//         }
        
//         function addContantDiv(){
//             let contactDiv = document.createElement(\"div\");
//             contactDiv.classList.add(\"contactContainer\");
        
//             document.querySelector(\".articleContact\").appendChild(contactDiv)
//         }
        
//         /**
//          * @param {email of the client} email 
//          * @param {number of the client} number 
//          */
        
//         function addMailNumberContacts(email, number){
//             const articleContact = document.querySelector(\".articleContact\");
            
//             const htmlString = 
//             `
//                 <a class=\"contactEmail\" href=\"mailto: ${email}\">
//                     email: ${email}
//                 </a>
//                 <a class=\"contactPhone\" href=\"tel: ${number}\">
//                     phonenumber: ${number}
//                 </a>
//             `;
        
//             const contactDiv = document.createElement(\"div\");
        
//             contactDiv.classList.add(\"mailNphone\");
//             contactDiv.innerHTML = htmlString;
        
//             articleContact.appendChild(contactDiv);
//         }
        
        
//         function addMetaDescription(desc){
//             document.querySelector('meta[name=\"description\"]').setAttribute(\"content\", desc);
//         }");                                                      

//     responsive_fjs_file.write(b"const burger = document.querySelector('.burger');
//     const imageBurger = burger.querySelector('img');
//     const nav = document.querySelector('.sideUL');
//     const navLI = document.querySelectorAll('.sideUL li');
    
//     const NavSilde = () => {
        
//         burger.addEventListener('click', () =>{
//             //toggle NAV
//             nav.classList.toggle('nav-active');
    
    
//             //toggle Burger
//             imageBurger.classList.toggle('toggleImageBurger');
//             burger.classList.toggle('toggleBurger')       
            
          
//             //Animate Links
//             let filteredNavlist = Array.prototype.filter.call(navLI, f => !f.classList.contains('toggleDisplay'));  
    
//             filteredNavlist.forEach((links, index) => {
//                 if(links.style.animation){
//                     links.style.animation = '';
//                 }else{
//                     links.style.animation = `navLinksFade 500ms ease forwards ${index / 9 + 0.5}s`;
//                 }
//             });
//         });
    
//     }
    
//     NavSilde();
    
//     function slideIn(){
//         nav.classList.toggle('nav-active');
    
//             //toggle Burger
//             burger.classList.toggle('toggleBurger');
    
//              //Animate Links
//             let filteredNavlist = Array.prototype.filter.call(navLI, f => !f.classList.contains('toggleDisplay')); 
    
             
    
//             filteredNavlist.forEach((links, index) => {
//                 if(links.style.animation){
//                     links.style.animation = '';
//                 }else{
//                     links.style.animation = `navLinksFade 500ms ease forwards ${index / 9 + 0.5}s`;
//                 }
//             }
//         );
//     }");    

//     // html, css files
//     let mut index_html_file = fs::File::create(directory_name.to_owned() + "/index.html").expect("creation failed");
//     let mut portfolio_html_file = fs::File::create(directory_name.to_owned() + "/portfolio.html").expect("creation failed");
//     let mut main_css_file = fs::File::create(directory_name.to_owned() + "/main.css").expect("creation failed");
//     index_html_file.write(b"<!DOCTYPE html>
//         <html lang=\"en\">
        
//         <!-- Made by Luca Schirmer -->
//         <head>
//             <meta charset=\"UTF-8\">
//             <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
//             <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
            
//             <meta name=\"description\" content=\"\">
            
//             <link rel=\"stylesheet\" href=\"main.css\">
//             <title>{title}</title>
//         </head>
//         <body style=\"overflow-x: hidden;\">
//             <header class=\"topHeader\">
//                 <nav class=\"topNav\">
//                     <div class=\"LogoContainer\">
//                         <img src=\"Logo.png\" alt=\"Logo\">
//                     </div>
//                     <ul class=\"navList\">
//                         <li class=\"ListItem\">
//                             <a href=\"#\">Home</a>     
//                         </li>
//                         <li class=\"ListItem\">
//                             <a href=\"#Portfolio\">Portfolio</a>
//                         </li>
//                         <li class=\"ListItem\">
//                             <a href=\"#Services\">Services</a>
//                         </li>
//                         <li class=\"ListItem\">
//                             <a href=\"#Contact\">Contact</a>
//                         </li>
//                         <li class=\"ListItem\">
//                             <a href=\"#Clients\">Clients</a>
//                         </li>
//                     </ul>
//                 </nav>
        
//                 <nav class=\"topNavMobile\">
//                     <ul class=\"indexUlMobile navList\">
//                         <div class=\"LogoContainer\">
//                             <img src=\"Logo.png\" alt=\"Logo\">
//                         </div>
//                         <li class=\"ListItem\">
//                             <a href=\"#\">Home</a>     
//                         </li>
                        
            
//                         <div class=\"burger\">
//                             <img class=\"toggleImageBurger\" src=\"img/cancelXpng.png\" alt=\"close Sidebar\">
//                             <div class=\"line1\"></div>
//                             <div class=\"line2\"></div>
//                             <div class=\"line3\"></div>
//                         </div>
//                     </ul>
//                     <ul class=\"sideUL\">
//                         <li class=\"ListItem\" id=\"firstChildSideUl\">
//                             <a href=\"#Portfolio\">Portfolio</a>
//                         </li>
//                         <li class=\"ListItem\">
//                             <a href=\"#Services\">Services</a>
//                         </li>
//                         <li class=\"ListItem\">
//                             <a href=\"#Contact\">Contact</a>
//                         </li>
//                         <li class=\"ListItem\">
//                             <a href=\"#Clients\">Clients</a>
//                         </li>
                    
//                     </ul>  
//                 </nav>
                
        
//             </header>
//             <main class=\"indexMain\">
//                 <article class=\"articleAbout\" id=\"articleAbout\">
//                     <div class=\"aboutFoto fadeIn\">
//                         <img src=\"\" alt=\"aboutFoto\">
//                     </div>
//                     <div class=\"aboutName fadeIn\">
//                         {aboutName}
//                     </div>
//                     <div class=\"aboutText fadeIn\">
//                         {aboutText}
//                     </div>      
//                 </article>
//                 <article class=\"articlePortfolio\" id=\"Portfolio\">
//                     <!-- dynamic -->
        
//                 </article>   
//                 <article class=\"articleServices\" id=\"Services\">
        
//                 </article>     
//                 <article class=\"articleClients\" id=\"Clients\">
        
//                 </article>     
//             </main>
//             <footer class=\"articleContact\" id=\"Contact\">
                
//             </footer>
//         </body>
        
//         <script src=\"scripts/main.js\"></script>
//         <script src=\"scripts/responsive.js\"></script>
//         <script defer src=\"scripts/intersectionObserver.js\"></script>
        
//         </html>").expect("write failed");

//     portfolio_html_file.write(b"<!DOCTYPE html>
//     <html lang=\"en\">
//     <head>
//         <meta charset=\"UTF-8\">
//         <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
//         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    
//         <meta name=\"description\" content=\"\">
    
//         <title>{Portfolio}</title>
    
//         <link rel=\"stylesheet\" href=\"main.css\">
//     </head>
//     <body>
//         <header class=\"topHeader\">
//             <nav class=\"topNav\">
//                 <div class=\"LogoContainer\">
//                     <img src=\"Logo.png\" alt=\"Logo\">
//                 </div>
//                 <ul class=\"navList\">
//                     <li class=\"ListItem\">
//                         <a href=\"index.html\">Home</a>     
//                     </li>
//                     <li class=\"ListItem\">
//                         <a href=\"index.html#Portfolio\">Portfolio</a>
//                     </li>
//                     <li class=\"ListItem\">
//                         <a href=\"index.html#Services\">Services</a>
//                     </li>
//                     <li class=\"ListItem\">
//                         <a href=\"index.html#Contact\">Contact</a>
//                     </li>
//                     <li class=\"ListItem\">
//                         <a href=\"index.html#Contact\">Clients</a>
//                     </li>
//                 </ul>
//             </nav>
    
//             <nav class=\"topNavMobile\">
//                 <ul class=\"indexUlMobile navList\">
//                     <div class=\"LogoContainer\">
//                         <img src=\"Logo.png\" alt=\"Logo\">
//                     </div>
//                     <li class=\"ListItem\">
//                         <a href=\"index.html\">Home</a>     
//                     </li>
                    
        
//                     <div class=\"burger\">
//                         <img class=\"toggleImageBurger\" src=\"img/cancelXpng.png\" alt=\"close Sidebar\">
//                         <div class=\"line1\"></div>
//                         <div class=\"line2\"></div>
//                         <div class=\"line3\"></div>
//                     </div>
//                 </ul>
//                 <ul class=\"sideUL\">
//                     <li class=\"ListItem\" id=\"firstChildSideUl\">
//                         <a href=\"#Portfolio\">Portfolio</a>
//                     </li>
//                     <li class=\"ListItem\">
//                         <a href=\"#Services\">Services</a>
//                     </li>
//                     <li class=\"ListItem\">
//                         <a href=\"#Contact\">Contact</a>
//                     </li>
//                     <li class=\"ListItem\">
//                         <a href=\"#Clients\">Clients</a>
//                     </li>
                
//                 </ul>  
//             </nav>
//         </header>
//         <main class=\"portfolioMain\">
//             <article class=\"subPortfolio\" id=\"subPortfolio\" style=\"padding-top: 8vh;\">
//                 <div class=\"buttonLeft\">
//                     <img src=\"img/Pfeil-links.png\" alt=\"arrow left\" class=\"imgLeftArrow\" onclick=\"
//                     let n = window.location.search.replace('?project=', ''); 
//                     let output = parseInt(n) - 1;
//                     console.log(output)
//                     getData(output, true);
//                     \">
//                 </div>
//                 <div class=\"buttonRight\">
//                     <img src=\"img/Pfeil-rechts.png\" alt=\"arrow right\" class=\"imgRightArrow\"onclick=\"
//                     let n = window.location.search.replace('?project=', ''); 
    
//                     let output = parseInt(n) + 1;
//                     console.log(output)
//                     getData(output, true);
//                     \">
//                 </div>
//             </article>
//         </main>
//         <footer class=\"articleContact\" id=\"Contact\">
            
//         </footer>
//     </body>
//     <script src=\"scripts/portfolio.js\"></script>
//     <script src=\"scripts/responsive.js\"></script>
//     </html>"); 

//     main_css_file.write(b":root{
//         font-size: 16px;
//         font-weight: 400;
//         line-height: 1.5;
    
//         /* --mainColor: rgba(20, 20, 20, 1);
//         --secondaryColor: rgb(33, 110, 211);
//         --accentColor: rgb(28, 28, 155);
//         --backgroundColor: rgba(255, 255, 255,1); */
    
//         /* Has to be edited induvidually each time */
//         --complementaryColor: rgba(249, 248, 113, 1);
    
//         --fontColor: rgba(255, 255, 255,1);
    
//         --fontFamily: 'Lato', sans-serif;
//     }
    
//     *{
//         padding: 0;
//         margin: 0;
    
//         scroll-behavior: smooth;
    
//         font-family: var(--fontFamily);
//     } 
    
    
//     /* toggle classes */
    
//     .toggleFixed{
//         position: fixed;
//         top: 0;
//         width: 100%;
//     }
    
//     .toggleRotate{
//         transform: rotate(180deg);
//         align-self: flex-start;
//     }
    
//     .nav-active{
//         transform: translateX(0%) !important;
//     }
    
//     /* Navbar-Style */
    
//     a{
//         text-decoration: none;
//         color: var(--fontColor)
//     }
    
//     ul{
//         text-decoration: none;
//         list-style-type: none;
//     }
    
//     .topHeader{
//         width: 100%;
//         height: 8vh;
    
//         background-color: var(--secondaryColor);
//         color: var(--fontColor);
    
//         position: fixed;
//         z-index: 1000;
//     }
    
//     .topNav, .topNavMobile{
//         display: flex;
//         flex-direction: row;
//     }
    
//     .LogoContainer{
//         width: 30%;
//         height: 8vh;
//     }
    
//     .navList{
//         display: flex;
//         flex-direction: row;
//         justify-content: space-around;
//         align-items: center;
    
//         width: 70%;
//         height: 8vh;
//     }
    
//     .ListItem{
//         text-transform: uppercase;
//         font-weight: 600;
//         font-size: max(16px, 2vh);
//     }
    
//     /* responsive nav */
    
//     .burger{
//         display: none;
//     }
    
//     .burger img{
//         height: 3.5vh;    
//     }
    
//     .burger div{
//         width: 25px;
//         height: 2px;
    
//         background-color: white;
//         transition: all 0.3s ease;
//     }
    
    
//     .toggleBurger div{
//         display: none;
//     }
    
//     .toggleImageBurger{
//         display: none; 
//     }
    
//     .indexUlMobile{
//         display: flex; 
//         flex-direction: row;
//         width: 100%;
//     }
    
//     .indexUlMobile, .sideUL{
//         display: none;
//     }
    
    
//     /* About article */
    
//     .articleAbout{
//         min-height: 92vh;
//         padding-top: 8vh;
    
//         display: grid;
//         grid-template-areas: 
//         \"a b\"
//         \"a c\";
    
//         text-align: center;
//     }
    
//     .aboutFoto{
//         grid-area: a;
//         width: 45vw;
//     }
    
//     .aboutName{
//         grid-area: b;
//         height: 15vh;
//         width: 100%;
    
//         padding-top: 2vh;
    
//         color: var(--secondaryColor);
//         font-size: 5vh;
//         font-weight: 600;
//     }
    
//     .aboutText{
//         height: 75vh;
//         width: 50vw;
//         grid-area: c;
    
//         line-height: 2;
//     }
    
//     /* Portfolio Article */
    
//     .articlePortfolio{
//        display: grid; 
    
//        grid-template-columns:  1fr 1fr 1fr;
//        grid-template-rows: auto;
    
//        justify-content: space-around;
    
//        background-color: var(--secondaryColor);    
    
//        column-gap: 5%;
//        padding: 10vh 5vw 10vh 5vw;
//     }
    
//     .bannerPortfolio{
//         height: 65vh;
    
//         background-color: var(--backgroundColor);
    
//         display: grid; 
//         grid-template-rows: minmax(30vh, 3fr) 1fr;
//         justify-content: space-evenly;
//         border-radius: 3%;
    
//         padding: 2%;
//         margin-bottom: 10vh;
//     }
    
//     .subBannerPortfolio{
//         height: 100vh;
    
//         background-color: var(--backgroundColor);
//         padding: 20vh;
//     }
    
//     .headPicture{
//         aspect-ratio: 4/3;
//         object-fit: contain;
    
//         display: grid;
//         justify-content: center;
//         align-content: center;
//     }
    
//     .bannerArticle{
//         padding-inline: 2%;
//         line-height: 1.25;
    
//         text-align: justify;
//     }
    
//     .bannerHeadline{
//         color: var(--accentColor);
//         filter: invert(1);
//         font-weight: 800;
    
//         padding: 3%;
//     }
    
//     .bannerText{
//         color: var(--mainColor);
//         padding: 3%;
//     }
    
//     .imgLeftArrow, .imgRightArrow{
//         position: fixed;
//         padding-top: 40vh;
//         padding-left: 10vh;
//         padding-right: 10vh;
    
//         width: 5vw;
    
//         z-index: 100;
//     }
    
//     .imgRightArrow{
//         right: 0;
//     }
    
//     .headPicture img{
//         width: 100%;
//         object-fit: cover;
    
//         max-height: 25vh;
//     }
    
//     .subBannerPortfolio .headPicture{
//         max-height: 25vh;
//         width: 100%;
//     }
    
    
//     /* Services Article */
    
//     .articleServices{
//         min-height: 50vh;
//     }
    
//     .service{
//         padding: 20vh;
//         min-height: 30vh;
//         text-align: center;
    
//         display: grid; 
//         grid-template-columns: 1fr 2fr;
//     }
    
//     .articleServices .service:nth-child(2){
//         background-color: var(--secondaryColor);
//         color: #fff;
//     }
    
//     .serviceHeadline{
//         font-size: 3rem;
//     }
    
//     .serviceText{
//         font-size: 1.5rem;
//     }
    
//     /* Clients Article*/
    
//     .articleClients{
//         display: flex;
//         flex-wrap: wrap;
//         flex-wrap: wrap;
//         flex-direction: row;
//         justify-content: space-evenly;
//     }
    
//     .client{
//         width: 21.8%;
//         padding-top: 10vh;
//         padding-bottom: 10vh;
//         text-align: center;
//     }
    
//     /* Contact Article*/
    
//     .articleContact{
//         display: flex;
//         flex-direction: row;
//         flex-wrap: wrap;
    
//         padding: 10vh;
//         background-color: rgb(25, 22, 22);
//     }
    
//     .contactContainer{
//         display: grid; 
    
//         width: 100%;
    
//         place-content: center;
//     }
    
//     .contact{
//         height: 14vh; 
    
//         display: flex; 
//         flex-direction: column;
//     }
    
//     .contactSpan{
//         color: white;
//         margin: 1%;
//         margin-top: 2vh;
    
//         text-align: center;
//     }
    
//     .contact a{
//         height: 7.5vh;
//         text-align: center;
//     }
    
//     .contactIMG{
//         height: 100%;
//         padding: 1%;
    
//         object-fit: contain;
//         aspect-ratio: 4/3;
//     }
    
    
//     .mailNphone{
//         display: flex; 
//         flex-flow: row wrap;
//         width: 100%;
//         padding-left: 5vh;
//         padding-right: 5vh;
//         padding-top: 0vh;
//         padding-bottom: 10vh;
        
//         justify-content: space-evenly;
//     }
    
    
//     .contactEmail{
//         padding-bottom: 2vh;
//     }
    
    
    
    
//         /*  Animations   */
    
//     .fadeIn{
//         opacity: 0;
//     }
    
//     .left{
//         transform: translateX(-50%);    
//     }
    
//     .right{
//         transform: translateX(50%);
//     }
    
//     .appear{
//         transition: all 500ms ease-in;
//         opacity: 1 !important;
//         transform: translateX(0) !important;
//     }
    
    
    
//     /* Responsive */
    
//     @media (max-width: 1000px){
    
//         .client{
//             width: 30%;
//         }
//     }
    
//     @media (max-width: 800px){
    
//         .topNav{
//             display: none;
//         }
    
//         .indexUlMobile{
//             display: flex;
//         }
    
//         .burger{
//             display: flex;
//             height: 60%;
//             justify-content: space-evenly;
//             flex-direction: column;
//             cursor: pointer;                
//         }
    
//         .sideUL{
//             top: 8vh;
//             right: 0px;
    
//             height: 92vh;
//             width: 60vw;
            
//             display: block;
//             position: fixed;
//             z-index: 2000;
    
//             background-color: var(--secondaryColor);
//             transform: translateX(100%);
//             transition: transform 0.66s ease-in;
    
//             display: flex;
//             flex-direction: column;
//             align-items: center;
//         }
    
//         .sideUL li{
//             font-size: 16px;
//             padding-top: 8vh;
//             padding-bottom: 8vh;
//         }
    
//         #firstChildSideUl{
//             padding-top: 12vh;
//         }
    
//         .articleAbout{
//             display: flex; 
//             flex-direction: column;
//         }
    
//         .aboutFoto{
//             width: 100vw;
//             margin-top: 5vh; 
//         }
    
//         .aboutFoto img{
//             height: 30vh;
    
//             margin: 2.5%
//         }
    
//         .aboutText{
//             width: 90vw; 
//             padding-left: 5vw;
//             padding-right: 5vw;
//         }
    
//         .articlePortfolio{
//             grid-template-columns: 1fr 1fr;
//         }
    
//         .service{
//             display: flex;
//             flex-direction: column;
//         }
        
//         .servicePicture{
//             padding-bottom: 10vh;
//         }
    
//         .subPortfolio{
//             background-color: var(--backgroundColor);
//         }
    
//         .subBannerPortfolio{
//             margin-top: 30vh
//         }
    
//         .buttonLeft, .buttonRight{
//             width: 50%; 
//         }
    
//         .buttonLeft img, .buttonRight img{
//             left: 0;  
//             position: fixed; 
//             width: 25%;
//             padding: 0;
//             padding-left: 12.5%; 
//             padding-top: 2.5%; 
//         }
//         .buttonRight img{
//             left: 50%; 
//         }
    
//         .serviceHeadline{
//             font-size: 1.75rem;
//         }
        
//         .serviceText{
//             font-size: 1rem;
//         }
        
//         .service{
//             padding: 10vh;
//         }
    
//         .subBannerPortfolio{
//             padding: 7vh;
//         }
        
//     }
    
    
//     @media (max-width: 600px){
//         .articlePortfolio{
//             grid-template-columns: 1fr;
//         }
//     }");

//     // json file 
//     let _data_json = fs::File::create("data.json").expect("creation failed");


//     let file_map = [
//         FotoFile::new("cancelXpng.png", "https://www.drive.google.com/uc?export=download&id=1qPgOzD3nuOtLHjSH1mn0OIP87-RCrZEP"),
//         FotoFile::new("Pfeil-links.png", "https://www.drive.google.com/uc?export=download&id=1nl2uPA5YB_MKzBH24w0a2Fc_5TSSjhAP"),
//         FotoFile::new("Pfeil-rechts.png", "https://www.drive.google.com/uc?export=download&id=1jtrav1Dek-x9TSPGUZsVr4QO_rfB_4Yl"),
//         FotoFile::new("instagram.png", "https://www.drive.google.com/uc?export=download&id=1-O98p9e5vLn55BreOVmfG6QbJp_6HKG-"),
//         FotoFile::new("whatsapp.png", "https://www.drive.google.com/uc?export=download&id=1-SsFO1o3DaPZYhnRjGiV21prd_VK4CWu"),
//         FotoFile::new("facebook.png", "https://www.drive.google.com/uc?export=download&id=1LLiZjgFZHQt21hdmf-TLpKdrF9rzAuO0"),
//         FotoFile::new("pinterest.png", "https://www.drive.google.com/uc?export=download&id=1NLWay3PrahDrylPhMn_2f3CLEf56dph-"),
//         FotoFile::new("youtube.png", "https://www.drive.google.com/uc?export=download&id=1Q-HmSVBTF1lTFqc-wllXufn-Hm6Wfehw"),
//         FotoFile::new("linkedin.png", "https://www.drive.google.com/uc?export=download&id=1THqwA4OaztIt1yKZFYeWccIg3Sy-fxil"),
//         FotoFile::new("github.png", "https://www.drive.google.com/uc?export=download&id=1UL-nPbpu2Ph6wLC2OIw1XqeZnUrVP2bu"),
//         FotoFile::new("twitter.png", "https://www.drive.google.com/uc?export=download&id=1Z112yuzn14VrHaMvTwarIRY_gjiOPmBT"),
//     ];   


//     for file in file_map.iter() {

//         // Download the image from the Google Drive link
//         let client = Client::new();
//         let response = client.get(file.url()).send().expect("Request failed");

//         let bytes = response.bytes().expect("Failed to retrieve image bytes");
//         let raw_pixel_img: Vec<u8> = bytes.to_vec();

//         // Save the image
//         let img_path = format!("{}/{}", img_locations, file.name());
//         let mut out = fs::File::create(&img_path).expect("Failed to create image file");
//         out.write_all(&raw_pixel_img).expect("Failed to write image file");
//     }  

// }