
// Aquire data
async function getData(){

    // Url Json file
    let dataJsonUrl = "../data.json";

    const request =  await fetch(dataJsonUrl,);
    const response = await request.json();
    
    addMetaDescription(response[0].metaDescription);

        
    for (const [key, value] of Object.entries(response[4])) {
        addContact(key, value);
    }
    
    console.log(response);
}

getData();

// create Banner href
const bannerPortfolios = document.querySelectorAll(".bannerPortfolio");

bannerPortfolios.forEach(portfolio => {
    portfolio.addEventListener("click", () =>{

        // Searchparameter has to be added
        var url = window.location.href;
        let newUrl = url.split("index.html");
        window.location.href =  `${newUrl[0]}portfolio.html`; 
    });
});



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
            <img class="contactIMG" src="img/ContactLogos/${title}.jpg" alt="${title}"> 
        </a>
        <span class="contactSpan"> ${title} </span>
    `;

    const contactDiv = document.createElement("div");

    contactDiv.classList.add("contact");
    contactDiv.setAttribute("id", title);
    contactDiv.innerHTML = htmlString;

    articleContact.appendChild(contactDiv);
}

