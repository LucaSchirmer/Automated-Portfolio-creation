
// Aquire data
async function getData(){

    // Url Json file
    let dataJsonUrl = "../data.json";

    const request =  await fetch(dataJsonUrl,);
    const response = await request.json();
    
    addMetaDescription(response[0].metaDescription);

    console.log(response)
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
 * 
 * @param {title => means name of the brand as String} title 
 * @param {content regarding the contact connection e.g. link to website} content 
 */
function addContact(title, content){
    const articleContact = document.querySelector(".articleContact");
    
    const contactDiv = document.createElement("div");
    const contentP = document.createElement("p");

    contactDiv.classList.add("contact");
    contentP.classList.add("contactContent");+

    contactDiv.setAttribute("id", title);

    contentP.innerText = content; 

    articleContact.appendChild(contactDiv);
    contactDiv.appendChild(contentP);
}

