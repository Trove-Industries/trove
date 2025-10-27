const API_BASE_URL = "http://localhost:8000";

// DOM elements
const form = document.getElementById("pairing-form");
const mealSelect = document.getElementById("meal-select");
const pairingNameInput = document.getElementById("pairing-name");
const pairingImageInput = document.getElementById("pairing-image");
const pairingPriceInput = document.getElementById("pairing-price");
const messageDiv = document.getElementById("message");
const pairingsList = document.getElementById("pairings-list");
const restaurantNameEl = document.getElementById("restaurant-name");
const continueButton = document.getElementById("continue-button");

// Data holders
let meals = [];
let pairings = [];

// ✅ On load
window.addEventListener("DOMContentLoaded", async () => {
    await checkSession();
    await loadMeals();
    await loadPairings();
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

// ✅ Load existing pairings (optional)
async function loadPairings() {
    try {
        const res = await fetch(`${API_BASE_URL}/menu/restore-pairing-session`, {
            method: "GET",
            credentials: "include",
        });

        if (res.ok) {
            pairings = await res.json();
            renderPairings();
        } else {
            pairings = [];
            renderPairings();
        }
    } catch (err) {
        console.error("Error loading pairings:", err);
    }
}

// ✅ Submit form
form.addEventListener("submit", async (e) => {
    e.preventDefault();

    const mealId = parseInt(mealSelect.value);
    const pairingName = pairingNameInput.value.trim();
    const pairingImage = pairingImageInput.value.trim();
    const pairingPrice = parseFloat(pairingPriceInput.value);

    if (!mealId || isNaN(mealId)) return showMessage("Select a meal.", "error");
    if (!pairingName) return showMessage("Enter pairing name.", "error");

    const payload = { meal_id: mealId, pairing_name: pairingName, pairing_image: pairingImage, pairing_price: pairingPrice };

    try {
        const res = await fetch(`${API_BASE_URL}/menu/create-pairing`, {
            method: "POST",
            credentials: "include",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload),
        });

        if (res.ok) {
            const newPairing = await res.json();
            pairings.push(newPairing);
            renderPairings();
            showMessage("Pairing added successfully!", "success");
            form.reset();
        } else {
            const errData = await res.json();
            showMessage(errData.error || "Failed to add pairing.", "error");
        }
    } catch (err) {
        console.error("Error adding pairing:", err);
        showMessage("Network error.", "error");
    }
});

// ✅ Render pairings
function renderPairings() {
    if (!pairings.length) {
        pairingsList.innerHTML = `<p class="empty-state">No pairings added yet.</p>`;
        continueButton.classList.add("hidden");
        return;
    }

    pairingsList.innerHTML = pairings
        .map(
            (p) => `
      <div class="pairing-card">
        <div>🍲 <strong>${p.pairing_name}</strong> — Ksh ${p.pairing_price}</div>
      </div>`
        )
        .join("");

    continueButton.classList.remove("hidden");
}

// ✅ Message helper
function showMessage(text, type) {
    messageDiv.textContent = text;
    messageDiv.className = `message ${type}`;
    messageDiv.style.display = "block";
    if (type === "success") setTimeout(() => (messageDiv.style.display = "none"), 3000);
}

// ✅ Go to next screen
continueButton.addEventListener("click", () => {
    window.location.href = "create_ingredient.html";
});
