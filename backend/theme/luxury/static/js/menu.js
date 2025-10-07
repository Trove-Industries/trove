// Menu Page Interactivity

const dishSection = document.getElementById('dish-section');
const categories = document.querySelector('.categories');

function showDishes(category) {
    categories.style.display = 'none';
    dishSection.style.display = 'flex';
    // TODO: Load dishes dynamically based on category if needed
}

function showCategories() {
    dishSection.style.display = 'none';
    categories.style.display = 'grid';
}

function openModal(modalId) {
    document.getElementById(modalId).style.display = 'flex';
}

function closeModal(modalId) {
    document.getElementById(modalId).style.display = 'none';
}
