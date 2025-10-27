const API_BASE_URL = "http://localhost:8000";
const restaurantNameEl = document.getElementById("restaurant-name");
const logoutBtn = document.getElementById("logout-btn");

// Stats
const totalCategories = document.getElementById("total-categories");
const totalMealGroups = document.getElementById("total-meal-groups");
const totalMeals = document.getElementById("total-meals");
const totalIngredients = document.getElementById("total-ingredients");

// On load
window.addEventListener("DOMContentLoaded", async () => {
    await loadRestaurant();
    await loadStats();
});

// ✅ Load restaurant session
async function loadRestaurant() {
    try {
        const res = await fetch(`${API_BASE_URL}/restaurant/restore-restaurant-session`, {
            method: "GET",
            credentials: "include",
        });

        if (res.ok) {
            const data = await res.json();
            restaurantNameEl.textContent = `🍴 ${data.restaurant_name}`;
        } else {
            restaurantNameEl.textContent = "🍴 Trove Restaurant";
        }
    } catch (err) {
        console.error("Error loading restaurant:", err);
        restaurantNameEl.textContent = "🍴 Trove Restaurant";
    }
}

// ✅ Load counts/statistics
async function loadStats() {
    try {
        const [catRes, groupRes, mealRes, ingRes] = await Promise.all([
            fetch(`${API_BASE_URL}/menu/restore-category-session`, { credentials: "include" }),
            fetch(`${API_BASE_URL}/menu/restore-meal-group-session`, { credentials: "include" }),
            fetch(`${API_BASE_URL}/menu/restore-meal-session`, { credentials: "include" }),
            fetch(`${API_BASE_URL}/menu/restore-ingredient-session`, { credentials: "include" }),
        ]);

        totalCategories.textContent = catRes.ok ? (await catRes.json()).length : 0;
        totalMealGroups.textContent = groupRes.ok ? (await groupRes.json()).length : 0;
        totalMeals.textContent = mealRes.ok ? (await mealRes.json()).length : 0;
        totalIngredients.textContent = ingRes.ok ? (await ingRes.json()).length : 0;
    } catch (err) {
        console.error("Failed to load stats:", err);
    }
}

// ✅ Logout
logoutBtn.addEventListener("click", async () => {
    try {
        await fetch(`${API_BASE_URL}/restaurant/logout`, {
            method: "POST",
            credentials: "include",
        });
    } catch (e) {
        console.error("Logout error:", e);
    } finally {
        window.location.href = "index.html";
    }
});
