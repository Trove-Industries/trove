// Fun Theme About Page JS
document.addEventListener('DOMContentLoaded', () => {
    // Example: animations or small interactions
    const textElements = document.querySelectorAll('.about-content .text p');
    textElements.forEach((p, index) => {
        p.style.opacity = 0;
        setTimeout(() => {
            p.style.transition = "opacity 0.6s ease-in";
            p.style.opacity = 1;
        }, 200 * index);
    });
});
