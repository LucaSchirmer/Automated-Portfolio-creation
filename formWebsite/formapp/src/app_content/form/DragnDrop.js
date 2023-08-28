import React, { useState } from 'react';

const DragNDrop = ({ maxAmount = 50 }) => {
    const [isDraggingOver, setIsDraggingOver] = useState(false);
    const [images, setImages] = useState([]);

    const handleDrop = (event) => {
        event.preventDefault();
        setIsDraggingOver(false);

        const droppedFiles = Array.from(event.dataTransfer.files);

        const imageFiles = droppedFiles.filter(file => file.type.startsWith('image/'));

        const imageUrls = imageFiles.map(file => URL.createObjectURL(file));

        if (images.length + imageUrls.length <= maxAmount) {
            setImages(prevImages => [...prevImages, ...imageUrls]);
        }
    };

    const handlechangeAfterOnclick = async (event) => {
        const fileInput = event.target;

        if (fileInput.files && fileInput.files.length > 0) {
            const droppedFiles = Array.from(fileInput.files);

            const imageFiles = droppedFiles.filter(file => file.type.startsWith('image/'));

            const imageUrls = await Promise.all(imageFiles.map(file => URL.createObjectURL(file)));

            if (images.length + imageUrls.length <= maxAmount) {
                setImages(prevImages => [...prevImages, ...imageUrls]);
            }
        }
    };

    const handleRemoveImage = (index) => {
        const updatedImages = images.filter((_, i) => i !== index);
        setImages(updatedImages);
    };

    const handleDragOver = (event) => {
        event.preventDefault();

        setIsDraggingOver(true);
    };

    const handleDragLeave = () => {
        setIsDraggingOver(false);
    };

    return (
        <div className="dragNdrop">
            <div
                className={`dropArea ${isDraggingOver ? 'draggingOver' : ''}`}
                onDrop={handleDrop}
                onDragOver={handleDragOver}
                onDragLeave={handleDragLeave}
            >
                <input
                    onChange={handlechangeAfterOnclick}
                    type="file"
                    className='FileInputDragNDrop'
                    multiple
                />
            </div>
            <div className="imageList">
                {images.map((imageUrl, index) => (
                    <div key={index} className="imageContainer">
                        <div className="removeIMG" onClick={() => handleRemoveImage(index)}>X</div>
                        <img src={imageUrl} alt={`${index}`} />
                    </div>
                ))}
            </div>
        </div>
    );
}

export default DragNDrop;
