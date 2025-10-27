const API_BASE_URL = 'http://localhost:8000';

// DOM Elements
const form = document.getElementById('category-form');
const categoryNameInput = document.getElementById('category-name');
const categoryIconInput = document.getElementById('category-icon');
const messageDiv = document.getElementById('message');
const categoriesList = document.getElementById('categories-list');
const restaurantNameEl = document.getElementById('restaurant-name');
const continueButton = document.getElementById('continue-button');
const submitButton = form.querySelector('button[type="submit"]');

// Store categories in memory
let categories = [];

// Check session and load existing categories on page load
window.addEventListener('DOMContentLoaded', async () => {
    await checkSession();
    await loadCategories();
});

// Check if user has a valid session
async function checkSession() {
    try {
        const response = await fetch(`${API_BASE_URL}/restaurant/restore-restaurant-session`, {
            method: 'GET',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
            }
        });

        if (response.ok) {
            const restaurantData = await response.json();
            restaurantNameEl.textContent = `Restaurant: ${restaurantData.restaurant_name}`;
        } else {
            // No session - redirect back to index
            showMessage('No restaurant session found. Please create a restaurant first.', 'error');
            setTimeout(() => {
                window.location.href = 'index.html';
            }, 2000);
        }
    } catch (error) {
        console.error('Failed to check session:', error);
        showMessage('Failed to verify session. Redirecting...', 'error');
        setTimeout(() => {
            window.location.href = 'index.html';
        }, 2000);
    }
}

// Load existing categories
async function loadCategories() {
    try {
        const response = await fetch(`${API_BASE_URL}/menu/restore-category-session`, {
            method: 'GET',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
            }
        });

        if (response.ok) {
            categories = await response.json();
            renderCategories();
        } else if (response.status === 404) {
            // No categories yet - that's fine
            categories = [];
            renderCategories();
        }
    } catch (error) {
        console.error('Failed to load categories:', error);
        // Don't show error - just start with empty list
    }
}

// Handle form submission
form.addEventListener('submit', async (e) => {
    e.preventDefault();

    const categoryData = {
        category_name: categoryNameInput.value.trim(),
        category_icon: categoryIconInput.value.trim()
    };

    // Validate input
    if (!categoryData.category_name) {
        showMessage('Please enter a category name', 'error');
        return;
    }

    // Disable button and show loading state
    submitButton.disabled = true;
    const originalText = submitButton.textContent;
    submitButton.innerHTML = 'Adding... <span class="loading"></span>';

    try {
        const response = await fetch(`${API_BASE_URL}/menu/create-category`, {
            method: 'POST',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(categoryData)
        });

        if (response.ok) {
            const newCategory = await response.json();
            categories.push(newCategory);
            renderCategories();

            // Clear form
            categoryNameInput.value = '';
            categoryIconInput.value = '';

            showMessage('Category added successfully!', 'success');

            // Show continue button if at least one category exists
            if (categories.length > 0) {
                continueButton.style.display = 'block';
            }
        } else {
            const errorData = await response.json();
            showMessage(errorData.error || 'Failed to create category', 'error');
        }
    } catch (error) {
        console.error('Error creating category:', error);
        showMessage('Network error. Please try again.', 'error');
    } finally {
        submitButton.disabled = false;
        submitButton.textContent = originalText;
    }
});

// Render categories list
function renderCategories() {
    if (categories.length === 0) {
        categoriesList.innerHTML = '<p class="empty-state">No categories added yet. Add your first category above!</p>';
        continueButton.style.display = 'none';
        return;
    }

    categoriesList.innerHTML = categories.map(category => `
        <div class="category-card">
            <div class="category-icon">${category.category_icon || '📁'}</div>
            <div class="category-info">
                <h3>${category.category_name}</h3>
                <p class="category-id">ID: ${category.id}</p>
            </div>
        </div>
    `).join('');

    continueButton.style.display = 'block';
}

// Show message to user
function showMessage(text, type) {
    messageDiv.textContent = text;
    messageDiv.className = type;
    messageDiv.style.display = 'block';

    // Auto-hide success messages after 3 seconds
    if (type === 'success') {
        setTimeout(() => {
            messageDiv.style.display = 'none';
        }, 3000);
    }
}

// Handle Continue button click
document.getElementById('continue-button').addEventListener('click', () => {
    window.location.href = 'menu_add_meal_group.html';
});
