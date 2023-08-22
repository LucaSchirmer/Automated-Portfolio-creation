import githubIcon from '../app_img/github.png';
import fiverrIcon from '../app_img/fiverr.png';


function Header() {
  return (
    <header className="appHeader">
        <nav className="mainNav">
            <h1 className="mainHeadline">
              Poll for your Portfolio-Website
            </h1>
            <ul>
              <li className="listItem">
                  <a className="ankerFiverr" href="https://www.fiverr.com/lucaschirmer">
                    <img className='navIMG' alt="Fivver Icon" src={fiverrIcon}>
                    </img>
                  </a>
              </li>
              <li className="listItem">
                  <a className="ankerGithub" href="https://github.com/LucaSchirmer">
                    <img className='navIMG' alt="Github Icon" src={githubIcon}>
                    </img>
                    </a>
              </li>
            </ul>
        </nav> 
    </header>
  );
}

export default Header;
