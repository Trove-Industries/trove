const API_BASE_URL = 'http://localhost:8000';

// DOM Elements
const form = document.getElementById('restaurant-form');
const nameInput = document.getElementById('name');
const countryInput = document.getElementById('country');
const cityInput = document.getElementById('city');
const subdomainInput = document.getElementById('subdomain');
const messageDiv = document.getElementById('message');
const submitButton = form.querySelector('button[type="submit"]');
const nextButton = document.getElementById('next-button');

// Next button click
nextButton.addEventListener('click', () => {
    if (!nameInput.disabled) {
        showMessage('Please create your restaurant first!', 'error');
        return;
    }
    window.location.href = 'menu.html';
});

// Check for existing session on load
window.addEventListener('DOMContentLoaded', () => {
    checkExistingSession();
});

// Restore session if exists
async function checkExistingSession() {
    try {
        const response = await fetch(`${API_BASE_URL}/restaurant/restore-restaurant-session`, {
            method: 'GET',
            credentials: 'include',
            headers: { 'Content-Type': 'application/json' }
        });

        if (response.ok) {
            const restaurantData = await response.json();

            nameInput.value = restaurantData.restaurant_name || '';
            countryInput.value = restaurantData.restaurant_country || '';
            cityInput.value = restaurantData.restaurant_city || '';
            subdomainInput.value = restaurantData.restaurant_subdomain || '';

            disableForm();
            showMessage('Session restored! Restaurant details loaded.', 'info');
        }
    } catch (error) {
        console.error('Failed to check session:', error);
    }
}

// Handle form submission
form.addEventListener('submit', async (e) => {
    e.preventDefault();

    const restaurantDetails = {
        restaurant_name: nameInput.value.trim(),
        restaurant_country: countryInput.value.trim(),
        restaurant_city: cityInput.value.trim(),
        restaurant_subdomain: subdomainInput.value.trim()
    };

    if (!restaurantDetails.restaurant_name || !restaurantDetails.restaurant_country ||
        !restaurantDetails.restaurant_city || !restaurantDetails.restaurant_subdomain) {
        showMessage('Please fill in all fields', 'error');
        return;
    }

    submitButton.disabled = true;
    const originalText = submitButton.textContent;
    submitButton.innerHTML = 'Creating... <span class="loading"></span>';

    try {
        const response = await fetch(`${API_BASE_URL}/restaurant/create-restaurant`, {
            method: 'POST',
            credentials: 'include',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(restaurantDetails)
        });

        const data = await response.json();

        if (response.ok) {
            showMessage('Restaurant created successfully!', 'success');
            disableForm();
        } else {
            showMessage(data.error || 'Failed to create restaurant', 'error');
            submitButton.disabled = false;
            submitButton.textContent = originalText;
        }
    } catch (error) {
        console.error('Error creating restaurant:', error);
        showMessage('Network error. Please try again.', 'error');
        submitButton.disabled = false;
        submitButton.textContent = originalText;
    }
});

// Disable form after creation
function disableForm() {
    nameInput.disabled = true;
    countryInput.disabled = true;
    cityInput.disabled = true;
    subdomainInput.disabled = true;
    submitButton.disabled = true;
    submitButton.textContent = 'Restaurant Already Created';
}

// Show messages
function showMessage(text, type) {
    messageDiv.textContent = text;
    messageDiv.className = type;
    messageDiv.style.display = 'block';

    if (type === 'success') {
        setTimeout(() => { messageDiv.style.display = 'none'; }, 5000);
    }
}
