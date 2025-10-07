// Minimalist Theme About Page JS
document.addEventListener('DOMContentLoaded', () => {
    const paragraphs = document.querySelectorAll('.about-content .text p');
    paragraphs.forEach((p, index) => {
        p.style.opacity = 0;
        setTimeout(() => {
            p.style.transition = "opacity 0.5s ease-in";
            p.style.opacity = 1;
        }, 200 * index);
    });
});
