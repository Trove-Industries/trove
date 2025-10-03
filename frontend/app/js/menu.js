import { apiRequest } from "./api.js";

// Add Menu
async function addMenu(event) {
    event.preventDefault();
    const data = {
        restaurant_name: localStorage.getItem("restaurantName"),
        food: document.getElementById("food").value,
        description: document.getElementById("description").value,
        price: parseFloat(document.getElementById("price").value),
        image: document.getElementById("image").value,
    };
    try {
        await apiRequest("/create-menu", "POST", data);
        alert("Menu item added!");
        document.getElementById("addMenuForm").reset();
    } catch (err) {
        alert("Error: " + err.message);
    }
}

// View Menus
async function loadMenus() {
    const restaurantName = localStorage.getItem("restaurantName");
    try {
        const menus = await apiRequest(`/get-menu/${restaurantName}`, "GET");
        const container = document.getElementById("menuList");
        if (!container) return;

        container.innerHTML = menus.map(m => `
      <div class="menu-item">
        <h3>${m.food} - $${m.price}</h3>
        <p>${m.description}</p>
        ${m.image ? `<img src="${m.image}" alt="${m.food}" width="100"/>` : ""}
      </div>
    `).join("");
    } catch (err) {
        alert("Error: " + err.message);
    }
}

// Attach event listeners
document.getElementById("addMenuForm")?.addEventListener("submit", addMenu);
if (document.getElementById("menuList")) loadMenus();
