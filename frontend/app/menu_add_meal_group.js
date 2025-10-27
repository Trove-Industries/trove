const API_BASE_URL = 'http://localhost:8000';

// DOM elements
const form = document.getElementById('meal-group-form');
const categorySelect = document.getElementById('category-select');
const mealGroupNameInput = document.getElementById('meal-group-name');
const messageDiv = document.getElementById('message');
const mealGroupsList = document.getElementById('meal-groups-list');
const restaurantNameEl = document.getElementById('restaurant-name');
const continueButton = document.getElementById('continue-button');
const submitButton = form.querySelector('button[type="submit"]');

// Data holders
let categories = [];
let mealGroups = [];

// Initialize
window.addEventListener('DOMContentLoaded', async () => {
    await checkSession();
    await loadCategories();
    await loadMealGroups();
});

// ✅ Check restaurant session
async function checkSession() {
    try {
        const res = await fetch(`${API_BASE_URL}/restaurant/restore-restaurant-session`, {
            method: 'GET',
            credentials: 'include',
            headers: { 'Content-Type': 'application/json' }
        });

        if (res.ok) {
            const data = await res.json();
            restaurantNameEl.textContent = `Restaurant: ${data.restaurant_name}`;
        } else {
            showMessage('No restaurant session found. Redirecting...', 'error');
            setTimeout(() => (window.location.href = 'index.html'), 2000);
        }
    } catch (err) {
        console.error('Session check failed:', err);
        showMessage('Failed to verify session.', 'error');
    }
}

// ✅ Load categories
async function loadCategories() {
    try {
        const res = await fetch(`${API_BASE_URL}/menu/restore-category-session`, {
            method: 'GET',
            credentials: 'include',
            headers: { 'Content-Type': 'application/json' }
        });

        if (res.ok) {
            categories = await res.json();
            populateCategoryDropdown();
        } else {
            categorySelect.innerHTML = `<option value="">No categories found</option>`;
        }
    } catch (err) {
        console.error('Error loading categories:', err);
        categorySelect.innerHTML = `<option value="">Failed to load categories</option>`;
    }
}

function populateCategoryDropdown() {
    if (!Array.isArray(categories) || categories.length === 0) {
        categorySelect.innerHTML = `<option value="">No categories available</option>`;
        return;
    }

    categorySelect.innerHTML = `
        <option value="">Select a category</option>
        ${categories.map(cat =>
        `<option value="${cat.id}">${cat.category_icon || '📁'} ${cat.category_name}</option>`
    ).join('')}
    `;
}

// ✅ Load meal groups
async function loadMealGroups() {
    try {
        const res = await fetch(`${API_BASE_URL}/menu/restore-meal-group-session`, {
            method: 'GET',
            credentials: 'include',
            headers: { 'Content-Type': 'application/json' }
        });

        if (res.ok) {
            mealGroups = await res.json();
            renderMealGroups();
        } else if (res.status === 404) {
            mealGroups = [];
            renderMealGroups();
        }
    } catch (err) {
        console.error('Error loading meal groups:', err);
    }
}

// ✅ Form submit handler
form.addEventListener('submit', async (e) => {
    e.preventDefault();

    const categoryId = parseInt(categorySelect.value, 10);
    const mealGroupName = mealGroupNameInput.value.trim();

    if (!categoryId) {
        showMessage('Please select a category.', 'error');
        return;
    }

    if (!mealGroupName) {
        showMessage('Please enter a meal group name.', 'error');
        return;
    }

    submitButton.disabled = true;
    const originalText = submitButton.textContent;
    submitButton.textContent = 'Adding...';

    try {
        const res = await fetch(`${API_BASE_URL}/menu/create-meal-group`, {
            method: 'POST',
            credentials: 'include',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ category_id: categoryId, meal_group_name: mealGroupName })
        });

        if (res.ok) {
            const newGroup = await res.json();
            mealGroups.push(newGroup);
            renderMealGroups();
            mealGroupNameInput.value = '';
            showMessage('Meal group added successfully!', 'success');
        } else {
            const errData = await res.json();
            showMessage(errData.error || 'Failed to add meal group.', 'error');
        }
    } catch (err) {
        console.error('Error creating meal group:', err);
        showMessage('Network error. Try again.', 'error');
    } finally {
        submitButton.disabled = false;
        submitButton.textContent = originalText;
    }
});

// ✅ Render meal groups
function renderMealGroups() {
    if (!mealGroups || mealGroups.length === 0) {
        mealGroupsList.innerHTML = `<p class="empty-state">No meal groups added yet.</p>`;
        continueButton.classList.add('hidden');
        return;
    }

    mealGroupsList.innerHTML = mealGroups.map(group => {
        const category = categories.find(c => c.id === group.category_id);
        const categoryName = category ? category.category_name : 'Unknown';
        return `
            <div class="category-card">
                <div class="category-icon">🍽️</div>
                <div class="category-info">
                    <h3>${group.meal_group_name}</h3>
                    <p>Category: ${categoryName} | ID: ${group.id}</p>
                </div>
            </div>
        `;
    }).join('');

    continueButton.classList.remove('hidden');
}

// ✅ Continue button → Go to next screen
continueButton.addEventListener('click', () => {
    window.location.href = 'menu_add_meal.html';
});

// ✅ Message handler
function showMessage(text, type) {
    messageDiv.textContent = text;
    messageDiv.className = `message ${type}`;
    messageDiv.style.display = 'block';
    if (type === 'success') {
        setTimeout(() => messageDiv.style.display = 'none', 3000);
    }
}
