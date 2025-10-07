// Luxury Theme - Gallery Page JS

document.querySelectorAll('.gallery-item').forEach(item => {
    item.addEventListener('click', (e) => {
        e.preventDefault();
        const lightboxId = item.getAttribute('href').substring(1);
        const lightbox = document.getElementById(lightboxId);
        if(lightbox) lightbox.style.display = 'flex';
    });
});

document.querySelectorAll('.lightbox').forEach(box => {
    box.addEventListener('click', () => box.style.display = 'none');
});
