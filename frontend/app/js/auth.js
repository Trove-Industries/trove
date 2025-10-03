import { registerRestaurant, validateRestaurant } from "./api.js";

document.addEventListener("DOMContentLoaded", () => {
    // Tab switching
    window.showTab = (tab) => {
        document.querySelectorAll(".tab-content").forEach(el => el.classList.remove("active"));
        document.querySelectorAll(".tab-btn").forEach(el => el.classList.remove("active"));
        document.getElementById(tab).classList.add("active");
        document.querySelector(`.tab-btn[onclick="showTab('${tab}')"]`).classList.add("active");
    };

    // Register form
    document.getElementById("registerForm").addEventListener("submit", async (e) => {
        e.preventDefault();
        const data = {
            restaurant_name: document.getElementById("restaurant_name").value,
            country: document.getElementById("country").value,
            city: document.getElementById("city").value,
        };
        const res = await registerRestaurant(data);
        alert("Registered successfully! Now login.");
        showTab("login");
    });

    // Login form
    document.getElementById("loginForm").addEventListener("submit", async (e) => {
        e.preventDefault();
        const name = document.getElementById("login_restaurant").value;
        const res = await validateRestaurant(name);
        if (res.length > 0) {
            localStorage.setItem("restaurantName", name); // save session
            window.location.href = "dashboard.html";
        } else {
            alert("Restaurant not found. Please register.");
        }
    });
});
