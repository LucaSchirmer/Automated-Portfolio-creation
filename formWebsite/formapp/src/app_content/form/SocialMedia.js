import React from "react";
import CheckBoxInput from "./CheckBoxInput";

const SocialMedia = () => {

  return (
    <div className="socialMediaContainer">
      <p className="socialMediaP">Which social media platforms would you like to incorporate into your website?</p>
      
      <CheckBoxInput
        name="LinkedIn"
        labelContent="LinkedIn: "

      />

      <CheckBoxInput
        name="Instagram"
        labelContent="Instagram: "
      />

      <CheckBoxInput
        name="Facebook"
        labelContent="Facebook: "
      />

      <CheckBoxInput
        name="Youtube"
        labelContent="Youtube: "
      />

      <CheckBoxInput
        name="Pinterest"
        labelContent="Pinterest: "
      />

      <CheckBoxInput
        name="Github"
        labelContent="Github: "
      />

      <CheckBoxInput
        name="Twitter"
        labelContent="Twitter: "    
      />
    </div>
  );
}

export default SocialMedia;
