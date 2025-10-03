// js/api.js
const API_BASE = "https://api.troveindustries.dev";

export async function registerRestaurant(data) {
    const res = await fetch(`${API_BASE}/register-restaurant`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(data)
    });
    return res.json();
}

export async function validateRestaurant(name) {
    const res = await fetch(`${API_BASE}/validate/${name}`);
    return res.json();
}

export async function addMenuItem(data) {
    const res = await fetch(`${API_BASE}/create-menu`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(data)
    });
    return res.json();
}

export async function getMenu(restaurantName) {
    const res = await fetch(`${API_BASE}/get-menu/${restaurantName}`);
    return res.json();
}
