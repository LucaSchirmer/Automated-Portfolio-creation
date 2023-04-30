const bannerPortfolios = document.querySelectorAll(".bannerPortfolio");

bannerPortfolios.forEach(portfolio => {
    portfolio.addEventListener("click", () =>{
        var url = window.location.href;
        let newUrl = url.split("index.html");
        window.location.href =  `${newUrl[0]}portfolio.html`; 
    });
});