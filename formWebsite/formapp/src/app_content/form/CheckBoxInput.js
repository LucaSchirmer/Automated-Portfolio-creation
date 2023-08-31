import React, { useState } from "react";

const CheckBoxInput = ({ name, labelContent = name}) => {
  const [bool, setBool] = useState(false);
  const [urlValue, setUrlValue] = useState("");

  return (
    <div className="checkBoxInputContainer" style={{ gridTemplateColumns: `${bool ? "1fr 1fr" : "auto"} 1fr` }}>
      <label className="inputTextAreaLabel" htmlFor={name}>
        {labelContent}
      </label>
      {bool && (
        <input
          name={`${name}URL`}
          type="text"
          placeholder="https://example.com"
          onChange={(event) => setUrlValue(event.target.value)}
          value={urlValue}
          className="checkBoxInput"
        />
      )}
      <input
        type="checkbox"
        name={name}
        onChange={(event) => {
            setBool(event.target.checked);
        }}
        className="checkBoxInput"
      />
    </div>
  );
};

export default CheckBoxInput;
