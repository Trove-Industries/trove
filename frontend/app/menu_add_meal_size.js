const API_BASE_URL = 'http://localhost:8000';

const form = document.getElementById('meal-size-form');
const mealSelect = document.getElementById('meal-select');
const sizeNameInput = document.getElementById('size-name');
const sizePriceInput = document.getElementById('size-price');
const messageDiv = document.getElementById('message');
const mealSizesList = document.getElementById('meal-sizes-list');
const restaurantNameEl = document.getElementById('restaurant-name');
const nextButton = document.getElementById('next-button');
const submitButton = form.querySelector('button[type="submit"]');

let meals = [];
let mealSizes = [];

window.addEventListener('DOMContentLoaded', async () => {
    console.log('🚀 Add Meal Size Page Loaded');
    await restoreRestaurantSession();
    await loadMeals();
    await loadMealSizes();
});

// ✅ Restore Restaurant Session
async function restoreRestaurantSession() {
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

// ✅ Load all Meals
async function loadMeals() {
    try {
        const res = await fetch(`${API_BASE_URL}/menu/restore-meal-session`, {
            method: 'GET',
            credentials: 'include',
            headers: { 'Content-Type': 'application/json' },
        });

        if (res.ok) {
            meals = await res.json();
            populateMealDropdown();
        } else {
            mealSelect.innerHTML = `<option value="">No meals found</option>`;
        }
    } catch (err) {
        console.error('❌ Failed to load meals:', err);
        mealSelect.innerHTML = `<option value="">Error loading meals</option>`;
    }
}

function populateMealDropdown() {
    if (!meals || meals.length === 0) {
        mealSelect.innerHTML = `<option value="">No meals available</option>`;
        return;
    }

    mealSelect.innerHTML = `
    <option value="" disabled selected>Select a meal</option>
    ${meals.map(m => `<option value="${m.id}">${m.meal_name}</option>`).join('')}
  `;
}

// ✅ Load Meal Sizes
async function loadMealSizes() {
    try {
        const res = await fetch(`${API_BASE_URL}/menu/restore-meal-size-session`, {
            method: 'GET',
            credentials: 'include',
            headers: { 'Content-Type': 'application/json' },
        });

        if (res.ok) {
            mealSizes = await res.json();
            renderMealSizes();
        } else if (res.status === 404) {
            mealSizes = [];
            renderMealSizes();
        }
    } catch (err) {
        console.error('❌ Error loading meal sizes:', err);
    }
}

// ✅ Form submission
form.addEventListener('submit', async (e) => {
    e.preventDefault();

    const mealId = parseInt(mealSelect.value);
    const sizeName = sizeNameInput.value.trim();
    const sizePrice = parseFloat(sizePriceInput.value);

    if (!mealId || isNaN(mealId)) {
        showMessage('Please select a meal.', 'error');
        return;
    }

    if (!sizeName || isNaN(sizePrice) || sizePrice <= 0) {
        showMessage('Please enter valid size details.', 'error');
        return;
    }

    submitButton.disabled = true;
    const originalText = submitButton.textContent;
    submitButton.textContent = 'Adding...';

    try {
        const payload = { meal_id: mealId, size_name: sizeName, size_price: sizePrice };

        const res = await fetch(`${API_BASE_URL}/menu/create-meal-size`, {
            method: 'POST',
            credentials: 'include',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload),
        });

        if (res.ok) {
            const newSize = await res.json();
            mealSizes.push(newSize);
            renderMealSizes();
            form.reset();
            showMessage('Meal size added successfully!', 'success');
        } else {
            const errData = await res.json();
            showMessage(errData.error || 'Failed to add meal size.', 'error');
        }
    } catch (err) {
        console.error('❌ Network error:', err);
        showMessage('Network error. Try again.', 'error');
    } finally {
        submitButton.disabled = false;
        submitButton.textContent = originalText;
    }
});

// ✅ Render meal sizes
function renderMealSizes() {
    if (mealSizes.length === 0) {
        mealSizesList.innerHTML = `<p class="placeholder">No meal sizes added yet.</p>`;
        nextButton.classList.add('hidden');
        return;
    }

    mealSizesList.innerHTML = mealSizes
        .map(size => {
            const meal = meals.find(m => m.id === size.meal_id);
            const mealName = meal ? meal.meal_name : 'Unknown Meal';
            return `
        <div class="meal-card">
          <strong>${size.size_name}</strong> — ${size.size_price} Ksh
          <p>Meal: ${mealName}</p>
        </div>
      `;
        })
        .join('');

    nextButton.classList.remove('hidden');
}

// ✅ Next button → Create Pairing
nextButton.addEventListener('click', () => {
    window.location.href = 'menu_add_pairing.html';
});

// ✅ Helper
function showMessage(text, type) {
    messageDiv.textContent = text;
    messageDiv.className = `message ${type}`;
    messageDiv.style.display = 'block';
    if (type === 'success') {
        setTimeout(() => (messageDiv.style.display = 'none'), 3000);
    }
}
