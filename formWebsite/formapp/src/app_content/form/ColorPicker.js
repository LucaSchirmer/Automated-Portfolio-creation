// USED Pickr a npm package -- npm install @simonwep/pickr'
// https://github.com/Simonwep/pickr 

import { useEffect } from 'react';
import Pickr from '@simonwep/pickr';
import '@simonwep/pickr/dist/themes/classic.min.css';

const ColorPicker = ({name}) => {

  console.log(name)
  useEffect(() => {
    // Create an instance of Pickr
    console.log(document.querySelector(`.pickr-${name}`));

    const pickr = Pickr.create({
        el: `.pickr-${name}`,
        theme: 'classic', // Choose a theme (classic, monolith, nano)
        default: '#ffffff', // Initial color
        swatches: [
        'rgba(244, 67, 54, 1)',
        'rgba(233, 30, 99, 1)',
        'rgba(156, 39, 176, 1)',
        'rgba(103, 58, 183, 1)',
        'rgba(63, 81, 181, 1)',
        'rgba(33, 150, 243, 1)',
        'rgba(3, 169, 244, 1)',
        'rgba(0, 188, 212, 1)',
        'rgba(0, 150, 136, 1)',
        'rgba(76, 175, 80, 1)',
        'rgba(139, 195, 74, 1)',
        'rgba(205, 220, 57, 1)',
        'rgba(255, 235, 59, 1)',
        'rgba(255, 193, 7, 1)'
    ],
      components: {
        preview: true,
        opacity: true,
        hue: true,
        interaction: {
            hex: true,
            rgba: true,
            hsla: true,
            cmyk: true,
            input: true,
            clear: true,
            save: true,
        },
        },
    });

    // Listen to color change event
    pickr.on('change', (color) => {
      console.log(color.toHEXA().toString()); // Print HEX value
    });

    return () => {
      pickr.destroy(); // Clean up on component unmount
    };
  }, [name]);


};

export default ColorPicker;