// Minimalist Homepage JS
document.addEventListener('DOMContentLoaded', () => {
    const cards = document.querySelectorAll('.feature-card');
    cards.forEach((card, index) => {
        card.style.opacity = 0;
        setTimeout(() => {
            card.style.transition = "opacity 0.5s ease-in";
            card.style.opacity = 1;
        }, 150 * index);
    });
});
