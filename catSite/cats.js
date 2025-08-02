// ECE-3600 Specific Note: This file is out of scope for the course, but is included for completeness.
// This file was originally created by Kenneth (Alex) Jenkins, https://github.com/rockenman1234/CatTemplateWebsite.


const desiredCount = 5; // Set the number of cat images to display
const url = `https://api.thecatapi.com/v1/images/search?limit=10`; // Fetch a larger number to account for potential API issues

fetch(url)
    .then((response) => response.json())
    .then((data) => {
        let imagesData = data.slice(0, desiredCount); // Limit to the desired count
        
        imagesData.forEach(function(imageData) {
            let image = document.createElement('img');
            image.src = `${imageData.url}`;
            
            let infoBox = document.createElement('div');
            infoBox.classList.add('info-box');
            
            let header = document.createElement('h3');
            header.innerText = 'Cat Info';
            
            let paragraph = document.createElement('p');
            paragraph.innerText = 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.';
            
            infoBox.appendChild(header);
            infoBox.appendChild(paragraph);

            let gridCell = document.createElement('div');
            gridCell.classList.add('gallery-item'); // Updated to match CSS class
            gridCell.appendChild(image);
            gridCell.appendChild(infoBox);
            
            document.getElementById('grid').appendChild(gridCell);
        });
    })
    .catch(function(error) {
        console.log(error);
    });