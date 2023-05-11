
// Aquire data
async function getData(){

    // Url Json file
    let dataJsonUrl = "../data.json";

    const request =  await fetch(dataJsonUrl,);
    const response = await request.json();
    
    addMetaDescription(response[0].metaDescription);
    document.title = response[0].title;
    document.querySelector(".aboutName").innerHTML = response[0].title;
    document.querySelector(".aboutText").innerHTML = response[0].aboutText;
    document.querySelector(".aboutFoto").firstChild.src = response[0].aboutFoto;
    document.querySelector(".aboutFoto").firstChild.alt = response[0].aboutFoto;
       
   

    // Adding Projects
    for(let i in response[1]){

        let projectArr = [];

        for (const value of Object.values(response[1][i])) {
            projectArr.push(value)
        }

        addProject(projectArr[0], projectArr[1], projectArr[2], i);
    } 
    
    // Adding Services
    for(let i in response[2]){

        let serviceArr = [];

        for (const value of Object.values(response[2][i])) {
            serviceArr.push(value)
        }

        addServices(serviceArr[0], serviceArr[1], serviceArr[2]);
    }
    
    // Adding Clients
    for(let i in response[2]){

        let clienteArr = [];

        for (const value of Object.values(response[3][i])) {
            clienteArr.push(value)
        }

        addClients(clienteArr[0], clienteArr[1], clienteArr[2]);
    }

     // Adding Contacts
    for (const [key, value] of Object.entries(response[4])) {
        addContact(key, value);
    }

    console.log(response);
}

getData();

// // create Banner href
// const bannerPortfolios = document.querySelectorAll(".bannerPortfolio");

// bannerPortfolios.forEach(portfolio => {
//     portfolio.addEventListener("click", () =>{

//         // Searchparameter has to be added
//         var url = window.location.href;
//         let newUrl = url.split("index.html");
//         window.location.href =  `${newUrl[0]}portfolio.html`; 
//     });
// });



function addMetaDescription(desc){
    document.querySelector('meta[name="description"]').setAttribute("content", desc);
}


/**
 * @param {title => means name of the brand as String} title 
 * @param {link regarding the contact connection} link 
 */

function addContact(title, link){
    const articleContact = document.querySelector(".articleContact");
    
    const htmlString = 
    `
        <a class="contactAnker" href="${link}">
            <img class="contactIMG" src="img/ContactLogos/${title}.png" alt="${title}"> 
        </a>
        <span class="contactSpan"> ${title} </span>
    `;

    const contactDiv = document.createElement("div");

    contactDiv.classList.add("contact");
    contactDiv.setAttribute("id", title);
    contactDiv.innerHTML = htmlString;

    articleContact.appendChild(contactDiv);
}


/**
 * @param {title of the Portfolio} title 
 * @param {text of the Portfolio} text 
 * @param {fotoUrl URL if the image} fotoUrl 
 * @param {num of the Projects} num 
 */

function addProject(title, text, fotoUrl, num){
    const articlePortfolio = document.querySelector(".articlePortfolio");
    
    const htmlString = 
    `
        <div class="headPicture">
            <img src="${fotoUrl}" alt="PortfolioPicture">
        </div>
        <div class="bannerArticle">
            <h1 class="bannerHeadline">
                ${title}
            </h1>

            <!-- Text has to be limited to a certain amout of characters -->
            <p class="bannerText">
                ${text}
            </p>
        </div>
    `;

    const bannerPortfolio = document.createElement("div");

    bannerPortfolio.classList.add("bannerPortfolio");
    bannerPortfolio.setAttribute("id", title);
    bannerPortfolio.innerHTML = htmlString;

    bannerPortfolio.addEventListener("click", () =>{

        // Searchparameter has to be added
        var url = window.location.href;
        let arrayUrl = url.split("/");

        arrayUrl[arrayUrl.length-1] = "";

        let newURL = arrayUrl.join("/")

        let param = new URLSearchParams();
        param.append("project", num);


        window.location.href =  `${newURL}portfolio.html?${param}`; 

    });

    articlePortfolio.appendChild(bannerPortfolio);
}


/**
 * @param {title of the Service} title 
 * @param {text of the Service} text 
 */

// Perhaps adding a Photo?  
function addServices(title, text){
    const articleServices = document.querySelector(".articleServices");
    
    const htmlString = 
    `
        <h1 class="serviceHeadline">
            ${title}
        </h1>

        <!-- Text has to be limited to a certain amout of characters -->
        <p class="serviceText">
            ${text}
        </p>
    `;

    const service = document.createElement("div");

    service.classList.add("service");
    service.setAttribute("id", title);
    service.innerHTML = htmlString;

    articleServices.appendChild(service);
}


/**
 * @param {name of the client} name 
 * @param {fotoUrl of the client} fotoUrl 
 */

// Perhaps adding a Photo?  
function addClients(name, fotoUrl){
    const articleClients = document.querySelector(".articleClients");
    
    const htmlString = 
    `
        <div class="clientPicture">
            <img src="${fotoUrl}" alt="${name} Picture">
        </div>
        <div class="clientName">
             <h1 class="clientHeadline">
            ${name}
            </h1>
        </div>
    `;

    const client = document.createElement("div");

    client.classList.add("client");
    client.setAttribute("id", name);
    client.innerHTML = htmlString;

    articleClients.appendChild(client);
}