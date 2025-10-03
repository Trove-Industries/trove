import { apiRequest } from "./api.js";

// Register
async function registerRestaurant(event) {
    event.preventDefault();
    const data = {
        restaurant_name: document.getElementById("restaurantName").value,
        country: document.getElementById("country").value,
        city: document.getElementById("city").value,
    };
    try {
        await apiRequest("/register-restaurant", "POST", data);
        alert("Registered successfully!");
        window.location.href = "login.html";
    } catch (err) {
        alert("Error: " + err.message);
    }
}

// Login
async function loginRestaurant(event) {
    event.preventDefault();
    const restaurantName = document.getElementById("restaurantName").value;
    try {
        const res = await apiRequest(`/validate/${restaurantName}`, "GET");
        if (res.length > 0) {
            localStorage.setItem("restaurantName", restaurantName);
            window.location.href = "dashboard.html";
        } else {
            alert("Restaurant not found.");
        }
    } catch (err) {
        alert("Error: " + err.message);
    }
}

// Attach event listeners
document.getElementById("registerForm")?.addEventListener("submit", registerRestaurant);
document.getElementById("loginForm")?.addEventListener("submit", loginRestaurant);
