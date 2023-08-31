

function Contact() {
  return (
    <div className="Impressum" style={{padding: "2rem"}}>
        <h1 style={{textAlign: "center", fontSize: "2rem", marginBottom: "10px"}}>Contact</h1>
        <ul>
            <li style={{marginBottom: "5px"}}><a href="https://github.com/LucaSchirmer">Github</a></li>
            <li style={{marginBottom: "5px"}}><a href="https://www.fiverr.com/lucaschirmer">Fiverr</a></li>
            <li style={{marginBottom: "5px"}}>E-Mail: <a href="mailto:lucaschi@t-online.de">lucaschi@t-online.de</a></li>
            <li style={{marginBottom: "5px"}}>Telefon: +49 176 543386</li>
        </ul>

    
        <p style={{marginTop: "50px"}} ><a href="/">Back to the Homepage</a></p>
    </div>
  );
}

export default Contact;
