// Fun Theme Menu Page JS
function openModal(modalId) {
    document.getElementById(modalId).style.display = 'flex';
}

function closeModal(modalId) {
    document.getElementById(modalId).style.display = 'none';
}

// Category filtering (optional)
document.querySelectorAll('.menu-categories button').forEach(btn => {
    btn.addEventListener('click', () => {
        document.querySelectorAll('.menu-categories button').forEach(b => b.classList.remove('active'));
        btn.classList.add('active');
        const category = btn.dataset.category;
        document.querySelectorAll('.menu-card').forEach(card => {
            card.style.display = card.dataset.category === category || category === 'all' ? 'block' : 'none';
        });
    });
});
