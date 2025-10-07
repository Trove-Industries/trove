// Minimalist Theme Menu Page JS

function showCategory(categoryId) {
    const grids = document.querySelectorAll('.dish-grid');
    grids.forEach(grid => grid.style.display = 'none');
    document.getElementById(categoryId).style.display = 'grid';

    const buttons = document.querySelectorAll('.category-btn');
    buttons.forEach(btn => btn.classList.remove('active'));
    document.querySelector(`[data-cat='${categoryId}']`).classList.add('active');
}

function openDishModal(modalId) {
    document.getElementById(modalId).style.display = 'flex';
}

function closeDishModal(modalId) {
    document.getElementById(modalId).style.display = 'none';
}
