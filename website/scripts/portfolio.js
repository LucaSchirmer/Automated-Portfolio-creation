// Aquire data
async function getData(){

    // Url Json file
    let dataJsonUrl = "../data.json";

    const request =  await fetch(dataJsonUrl,);
    const response = await request.json();

    let projectNum = window.location.search.replace("?project=", ""); 


    let projectArr = [];
    for (const value of Object.values(response[1][projectNum])) {
        projectArr.push(value)
    }

    addProject(projectArr[0], projectArr[1], projectArr[2]);

     // Adding Contacts
    for (const [key, value] of Object.entries(response[4])) {
        addContact(key, value);
    }

    console.log(response);
}

getData();

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

    articleContact.appendChild(contactDiv);
}