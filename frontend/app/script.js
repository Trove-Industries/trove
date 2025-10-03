const API_BASE = "https://api.troveindustries.dev";


// Extract query params (e.g. ?restaurant=MyPizza)
function getRestaurant() {
    const params = new URLSearchParams(window.location.search);
    return params.get("restaurant");
}

// Handle login
const loginForm = document.getElementById("loginForm");
if (loginForm) {
    loginForm.addEventListener("submit", async (e) => {
        e.preventDefault();
        const name = document.getElementById("restaurantName").value.trim();
        try {
            const res = await fetch(`${API_BASE}/menu-items/validate/${name}`);
            if (res.ok) {
                const data = await res.json();
                if (data.length > 0) {
                    window.location.href = `dashboard.html?restaurant=${encodeURIComponent(name)}`;
                } else {
                    document.getElementById("errorMsg").innerText = "Restaurant not found.";
                }
            } else {
                document.getElementById("errorMsg").innerText = "Server error.";
            }
        } catch (err) {
            document.getElementById("errorMsg").innerText = "Network error.";
        }
    });
}

// Fill sidebar with restaurant name
const restaurant = getRestaurant();
if (restaurant) {
    const title = document.getElementById("restaurantTitle");
    if (title) title.innerText = restaurant;
    document.querySelectorAll(".sidebar a").forEach(a => {
        const url = new URL(a.getAttribute("href"), window.location.origin);
        url.searchParams.set("restaurant", restaurant);
        a.href = url.toString();
    });
}

// Handle add menu form
const menuForm = document.getElementById("menuForm");
if (menuForm) {
    menuForm.addEventListener("submit", async (e) => {
        e.preventDefault();
        const item = {
            restaurant_name: restaurant,
            food: document.getElementById("food").value,
            description: document.getElementById("description").value,
            price: parseFloat(document.getElementById("price").value),
            image: document.getElementById("image").value
        };
        try {
            const res = await fetch(`${API_BASE}/menu-items`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(item)
            });
            if (res.ok) {
                document.getElementById("msg").innerText = "Item added successfully!";
                menuForm.reset();
            } else {
                document.getElementById("msg").innerText = "Error adding item.";
            }
        } catch {
            document.getElementById("msg").innerText = "Network error.";
        }
    });
}

// Handle view menu
const menuList = document.getElementById("menuList");
if (menuList && restaurant) {
    fetch(`${API_BASE}/menu-items/${restaurant}`)
        .then(res => res.json())
        .then(data => {
            if (data.length === 0) {
                menuList.innerHTML = "<p>No menu items found.</p>";
            } else {
                menuList.innerHTML = data.map(item => `
          <div class="menu-item">
            <h3>${item.food} - $${item.price}</h3>
            <p>${item.description}</p>
            ${item.image ? `<img src="${item.image}" alt="${item.food}">` : ""}
          </div>
        `).join("");
            }
        })
        .catch(() => {
            menuList.innerHTML = "<p>Error loading menu.</p>";
        });
}
