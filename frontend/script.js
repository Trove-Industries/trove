// script.js

const API_BASE_URL = "http://127.0.0.1:3000"; // TODO: replace with your prod domain

// Handle Add Menu Item
document.getElementById("menuForm").addEventListener("submit", async (e) => {
    e.preventDefault();

    const formData = {
        restaurant_id: parseInt(document.getElementById("restaurant_id").value, 10),
        food: document.getElementById("food").value.trim(),
        description: document.getElementById("description").value.trim(),
        price: parseFloat(document.getElementById("price").value),
        image: document.getElementById("image").value.trim() || null
    };

    try {
        const response = await fetch(`${API_BASE_URL}/menu-items`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(formData)
        });

        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`Server error: ${response.status} ${errorText}`);
        }

        const newItem = await response.json();
        alert(`✅ Menu item "${newItem.food}" added successfully!`);

        // reset form
        e.target.reset();

    } catch (err) {
        console.error("Error adding menu item:", err);
        alert("❌ Failed to add menu item. Check console for details.");
    }
});

// Handle Fetch Menu Items
document.getElementById("fetchForm").addEventListener("submit", async (e) => {
    e.preventDefault();

    const restaurantId = parseInt(document.getElementById("fetch_restaurant_id").value, 10);
    const menuList = document.getElementById("menuList");
    menuList.innerHTML = "<p>Loading...</p>";

    try {
        const response = await fetch(`${API_BASE_URL}/menu-items/${restaurantId}`);
        if (!response.ok) {
            throw new Error(`Failed to fetch menu. Status: ${response.status}`);
        }

        const menuItems = await response.json();

        if (menuItems.length === 0) {
            menuList.innerHTML = "<p>No menu items found for this restaurant.</p>";
            return;
        }

        menuList.innerHTML = "";
        menuItems.forEach(item => {
            const div = document.createElement("div");
            div.classList.add("menu-item");
            div.innerHTML = `
                <h3>${item.food} - $${item.price}</h3>
                <p>${item.description || "No description available."}</p>
                ${item.image ? `<img src="${item.image}" alt="${item.food}">` : ""}
            `;
            menuList.appendChild(div);
        });

    } catch (err) {
        console.error("Error fetching menu:", err);
        menuList.innerHTML = "<p>❌ Failed to fetch menu items. Check console for details.</p>";
    }
});
