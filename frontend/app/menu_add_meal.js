const API_BASE_URL = 'http://localhost:8000';

// DOM Elements
const form = document.getElementById('meal-form');
const mealGroupSelect = document.getElementById('meal-group-select');
const mealNameInput = document.getElementById('meal-name');
const mealDescriptionInput = document.getElementById('meal-description');
const mealImageInput = document.getElementById('meal-image');
const messageDiv = document.getElementById('message');
const mealsList = document.getElementById('meals-list');
const restaurantNameEl = document.getElementById('restaurant-name');
const nextButton = document.getElementById('next-button');
const submitButton = form.querySelector('button[type="submit"]');

let mealGroups = [];
let meals = [];

window.addEventListener('DOMContentLoaded', async () => {
    console.log('🚀 Add Meal Page Loaded');
    await checkSession();
    await loadMealGroups();
    await loadMeals();
});

// ✅ Check restaurant session
async function checkSession() {
    try {
        const res = await fetch(`${API_BASE_URL}/restaurant/restore-restaurant-session`, {
            method: 'GET',
            credentials: 'include',
            headers: { 'Content-Type': 'application/json' },
        });
        if (res.ok) {
            const data = await res.json();
            restaurantNameEl.textContent = `🍴 ${data.restaurant_name}`;
        } else {
            showMessage('No restaurant session found. Redirecting...', 'error');
            setTimeout(() => (window.location.href = 'index.html'), 2000);
        }
    } catch (err) {
        console.error('❌ Session check failed:', err);
        showMessage('Failed to verify session.', 'error');
    }
}

// ✅ Load meal groups
async function loadMealGroups() {
    try {
        const res = await fetch(`${API_BASE_URL}/menu/restore-meal-group-session`, {
            method: 'GET',
            credentials: 'include',
            headers: { 'Content-Type': 'application/json' },
        });

        if (res.ok) {
            mealGroups = await res.json();
            populateMealGroupDropdown();
        } else {
            mealGroupSelect.innerHTML = `<option value="">No meal groups found</option>`;
        }
    } catch (err) {
        console.error('❌ Error loading meal groups:', err);
        mealGroupSelect.innerHTML = `<option value="">Failed to load meal groups</option>`;
    }
}

function populateMealGroupDropdown() {
    if (!mealGroups || mealGroups.length === 0) {
        mealGroupSelect.innerHTML = `<option value="">No meal groups available</option>`;
        return;
    }

    const optionsHTML = mealGroups
        .map(group => `<option value="${group.id}">${group.meal_group_name}</option>`)
        .join('');

    mealGroupSelect.innerHTML = `
        <option value="" disabled selected>Select a meal group</option>
        ${optionsHTML}
    `;
}

// ✅ Load meals
async function loadMeals() {
    try {
        const res = await fetch(`${API_BASE_URL}/menu/restore-meal-session`, {
            method: 'GET',
            credentials: 'include',
            headers: { 'Content-Type': 'application/json' },
        });

        if (res.ok) {
            meals = await res.json();
            renderMeals();
        } else if (res.status === 404) {
            meals = [];
            renderMeals();
        }
    } catch (err) {
        console.error('❌ Error loading meals:', err);
    }
}

// ✅ Form submission
form.addEventListener('submit', async (e) => {
    e.preventDefault();

    const mealGroupId = parseInt(mealGroupSelect.value);
    const mealName = mealNameInput.value.trim();
    const mealDescription = mealDescriptionInput.value.trim();
    const mealImage = mealImageInput.value.trim();

    if (!mealGroupId || isNaN(mealGroupId)) {
        showMessage('Please select a meal group.', 'error');
        return;
    }
    if (!mealName || !mealDescription || !mealImage) {
        showMessage('Please fill in all fields.', 'error');
        return;
    }

    submitButton.disabled = true;
    const originalText = submitButton.textContent;
    submitButton.textContent = 'Adding...';

    try {
        const payload = { meal_group_id: mealGroupId, meal_name: mealName, meal_description: mealDescription, meal_image: mealImage };

        const res = await fetch(`${API_BASE_URL}/menu/create-meal`, {
            method: 'POST',
            credentials: 'include',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload),
        });

        if (res.ok) {
            const newMeal = await res.json();
            meals.push(newMeal);
            renderMeals();
            form.reset();
            showMessage('Meal added successfully!', 'success');
        } else {
            const errData = await res.json();
            showMessage(errData.error || 'Failed to add meal.', 'error');
        }
    } catch (err) {
        console.error('❌ Network error:', err);
        showMessage('Network error. Try again.', 'error');
    } finally {
        submitButton.disabled = false;
        submitButton.textContent = originalText;
    }
});

// ✅ Render meals
function renderMeals() {
    if (meals.length === 0) {
        mealsList.innerHTML = `<p class="empty-state">No meals added yet.</p>`;
        nextButton.classList.add('hidden');
        return;
    }

    mealsList.innerHTML = meals
        .map(meal => {
            const group = mealGroups.find(g => g.id === meal.meal_group_id);
            const groupName = group ? group.meal_group_name : 'Unknown Group';
            return `
            <div class="category-card">
                <div class="category-icon">🥗</div>
                <div class="category-info">
                    <h3>${meal.meal_name}</h3>
                    <p>${meal.meal_description}</p>
                    <p>Group: ${groupName} | ID: ${meal.id}</p>
                </div>
            </div>`;
        })
        .join('');

    nextButton.classList.remove('hidden');
}

// ✅ Go to next screen
nextButton.addEventListener('click', () => {
    window.location.href = 'menu_add_meal_size.html';
});

// ✅ Message helper
function showMessage(text, type) {
    messageDiv.textContent = text;
    messageDiv.className = `message ${type}`;
    messageDiv.style.display = 'block';
    if (type === 'success') {
        setTimeout(() => (messageDiv.style.display = 'none'), 3000);
    }
}
