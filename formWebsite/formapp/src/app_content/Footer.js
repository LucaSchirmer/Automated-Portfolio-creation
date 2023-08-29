import Logo from "../app_img/LsoulutionsIT_LOGO.png";


function Footer() {
  return (
    <footer className="footer"> 
        <div className="footerContent">
            <div className="footerLogo">
                <img src={Logo} alt="Logo LS" />
            </div>
            <div className="footerLinks">
                <ul className="footerNav">
                    <li><a href="/">Impressum</a></li>
                    <li><a href="Datenschutzerklärung">Datenschutzerklärung</a></li>
                    <li><a href="/contact">Contact</a></li>
                </ul>
            </div>
        </div>
        <div className="footerBottom">
            <p>&copy; {new Date().getFullYear()} Luca Schirmer. All rights reserved.</p>
        </div>
    </footer>
  );
}

export default Footer;
