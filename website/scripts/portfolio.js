// Aquire data
async function getData(projectNum, bool = false){

    // Url Json file
    let dataJsonUrl = "../data.json";

    const request =  await fetch(dataJsonUrl,);
    const response = await request.json();

    var projectNumMax = 0;

    addMetaDescription(response[0].metaDescription);
    

    document.querySelector(":root").style.setProperty("--mainColor", response[0].mainColor);
    document.querySelector(":root").style.setProperty("--secondaryColor", response[0].secondaryColor);
    document.querySelector(":root").style.setProperty("--accentColor", response[0].accentColor);
    document.querySelector(":root").style.setProperty("--backgroundColor", response[0].backgroundColor);


    let setLength = 10000;

    for(let i = 0; i <= setLength; i++){
         if(!response[1][i]){
            projectNumMax = i-1; 
            break;
         }
    }



    let projectArr = [];
    if(projectNum > projectNumMax){
        projectNum = 0;
    }
    
    if(projectNum < 0){
        projectNum = projectNumMax;
    }
    var url = window.location.href;
    let arrayUrl = url.split("/");

    arrayUrl[arrayUrl.length-1] = "";

    let newURL = arrayUrl.join("/")

    let param = new URLSearchParams();
    param.append("project", projectNum);

    if(bool == true){
        window.location.href =  `${newURL}portfolio.html?${param}`; 
    }
    



    for (const value of Object.values(response[1][projectNum])) {
        projectArr.push(value)
    }

    addProject(projectArr[0], projectArr[1], projectArr[2]);
    document.title = projectArr[0];

    // Adding Contacts
    addMailNumberContacts(response[0].email, response[0].phoneNumber);

    for (const [key, value] of Object.entries(response[4])) {
        addContact(key, value);
    }

    console.log(response);
}

var projectNum = window.location.search.replace("?project=", ""); 

getData(projectNum);

/**
 * @param {title of the Portfolio} title 
 * @param {text of the Portfolio} text 
 * @param {fotoUrl URL if the image} fotoUrl 
 * @param {num of the Projects} num 
 */

function addProject(title, text, fotoUrl){
    const articlePortfolio = document.querySelector(".subPortfolio");
    
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

    bannerPortfolio.classList.add("subBannerPortfolio");
    bannerPortfolio.setAttribute("id", title);
    bannerPortfolio.innerHTML = htmlString;


    articlePortfolio.appendChild(bannerPortfolio);
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
    articleContact.style.backgroundColor = "rgb(25, 22, 22)";


    articleContact.appendChild(contactDiv);
}


/**
 * @param {email of the client} email 
 * @param {number of the client} number 
 */

function addMailNumberContacts(email, number){
    const articleContact = document.querySelector(".articleContact");
    
    const htmlString = 
    `
        <a class="contactEmail" href="mailto: ${email}">
            email: ${email}
        </a>
        <a class="contactPhone" href="tel: ${number}">
            phonenumber: ${number}
        </a>
    `;

    const contactDiv = document.createElement("div");

    contactDiv.classList.add("mailNphone");
    contactDiv.innerHTML = htmlString;

    articleContact.appendChild(contactDiv);
}


function addMetaDescription(desc){
    document.querySelector('meta[name="description"]').setAttribute("content", desc);
}
