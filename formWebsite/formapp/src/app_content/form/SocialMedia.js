import React from "react";
import CheckBoxInput from "./CheckBoxInput";

const SocialMedia = () => {

  return (
    <div className="socialMediaContainer">
      <p className="socialMediaP">Which social media platforms would you like to incorporate into your website?</p>
      
      <CheckBoxInput
        name="linkedIn"
        labelContent="LinkedIn: "

      />

      <CheckBoxInput
        name="instagram"
        labelContent="Instagram: "
      />

      <CheckBoxInput
        name="facebook"
        labelContent="Facebook: "
      />

      <CheckBoxInput
        name="youtube"
        labelContent="Youtube: "
      />

      <CheckBoxInput
        name="pinterest"
        labelContent="Pinterest: "
      />

      <CheckBoxInput
        name="github"
        labelContent="Github: "
      />

      <CheckBoxInput
        name="twitter"
        labelContent="Twitter: "    
      />
    </div>
  );
}

export default SocialMedia;
