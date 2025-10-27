const API_BASE_URL = "http://localhost:8000";

// DOM elements
const form = document.getElementById("ingredient-form");
const mealSelect = document.getElementById("meal-select");
const ingredientNameInput = document.getElementById("ingredient-name");
const ingredientImageInput = document.getElementById("ingredient-image");
const messageDiv = document.getElementById("message");
const ingredientsList = document.getElementById("ingredients-list");
const restaurantNameEl = document.getElementById("restaurant-name");
const finishButton = document.getElementById("finish-button");

// Data holders
let meals = [];
let ingredients = [];

// ✅ On load
window.addEventListener("DOMContentLoaded", async () => {
    await checkSession();
    await loadMeals();
    await loadIngredients();
});

// ✅ Verify restaurant session
async function checkSession() {
    try {
        const res = await fetch(`${API_BASE_URL}/restaurant/restore-restaurant-session`, {
            method: "GET",
            credentials: "include",
        });

        if (res.ok) {
            const data = await res.json();
            restaurantNameEl.textContent = `🍴 ${data.restaurant_name}`;
        } else {
            showMessage("No restaurant session found. Redirecting...", "error");
            setTimeout(() => (window.location.href = "index.html"), 2000);
        }
    } catch (err) {
        console.error("Session check failed:", err);
        showMessage("Failed to verify session.", "error");
    }
}

// ✅ Load meals
async function loadMeals() {
    try {
        const res = await fetch(`${API_BASE_URL}/menu/restore-meal-session`, {
            method: "GET",
            credentials: "include",
        });

        if (res.ok) {
            meals = await res.json();
            populateMealDropdown();
        } else {
            mealSelect.innerHTML = `<option value="">No meals found</option>`;
        }
    } catch (err) {
        console.error("Error loading meals:", err);
    }
}

// ✅ Populate dropdown
function populateMealDropdown() {
    if (!meals.length) {
        mealSelect.innerHTML = `<option value="">No meals available</option>`;
        return;
    }

    mealSelect.innerHTML = `
    <option value="" disabled selected>Select a meal</option>
    ${meals
        .map((m) => `<option value="${m.id}">${m.meal_name}</option>`)
        .join("")}
  `;
}

// ✅ Load existing ingredients (optional)
async function loadIngredients() {
    try {
        const res = await fetch(`${API_BASE_URL}/menu/restore-ingredient-session`, {
            method: "GET",
            credentials: "include",
        });

        if (res.ok) {
            ingredients = await res.json();
            renderIngredients();
        } else {
            ingredients = [];
            renderIngredients();
        }
    } catch (err) {
        console.error("Error loading ingredients:", err);
    }
}

// ✅ Submit form
form.addEventListener("submit", async (e) => {
    e.preventDefault();

    const mealId = parseInt(mealSelect.value);
    const ingredientName = ingredientNameInput.value.trim();
    const ingredientImage = ingredientImageInput.value.trim();

    if (!mealId || isNaN(mealId)) return showMessage("Select a meal.", "error");
    if (!ingredientName) return showMessage("Enter ingredient name.", "error");

    const payload = { meal_id: mealId, ingredient_name: ingredientName, ingredient_image: ingredientImage };

    try {
        const res = await fetch(`${API_BASE_URL}/menu/create-ingredient`, {
            method: "POST",
            credentials: "include",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload),
        });

        if (res.ok) {
            const newIngredient = await res.json();
            ingredients.push(newIngredient);
            renderIngredients();
            showMessage("Ingredient added successfully!", "success");
            form.reset();
        } else {
            const errData = await res.json();
            showMessage(errData.error || "Failed to add ingredient.", "error");
        }
    } catch (err) {
        console.error("Error adding ingredient:", err);
        showMessage("Network error.", "error");
    }
});

// ✅ Render ingredients
function renderIngredients() {
    if (!ingredients.length) {
        ingredientsList.innerHTML = `<p class="empty-state">No ingredients added yet.</p>`;
        finishButton.classList.add("hidden");
        return;
    }

    ingredientsList.innerHTML = ingredients
        .map(
            (i) => `
      <div class="ingredient-card">
        <div>🥕 <strong>${i.ingredient_name}</strong></div>
      </div>`
        )
        .join("");

    finishButton.classList.remove("hidden");
}

// ✅ Message helper
function showMessage(text, type) {
    messageDiv.textContent = text;
    messageDiv.className = `message ${type}`;
    messageDiv.style.display = "block";
    if (type === "success") setTimeout(() => (messageDiv.style.display = "none"), 3000);
}

// ✅ Finish setup button → go to signup
finishButton.addEventListener("click", () => {
    alert("🎉 Setup complete! Let's create your account next.");
    window.location.href = "signup.html";
});

