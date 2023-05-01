
// Aquire data
async function getData(){

    // Url Json file
    let dataJsonUrl = "../data.json";

    const request =  await fetch(dataJsonUrl,);
    const response = await request.json();
    
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

