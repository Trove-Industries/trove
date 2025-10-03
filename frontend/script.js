// script.js

import { createClient } from '@supabase/supabase-js'

// ----- Supabase Setup -----
const SUPABASE_URL = 'https://szpoiclwmhdtcsnbwewq.supabase.co'
const SUPABASE_KEY = 'YOUR_ANON_OR_SERVICE_KEY' // replace with your anon key
const supabase = createClient(SUPABASE_URL, SUPABASE_KEY)

// ----- Add Menu Item -----
document.getElementById("menuForm").addEventListener("submit", async (e) => {
    e.preventDefault()

    const formData = {
        restaurant_id: parseInt(document.getElementById("restaurant_id").value, 10),
        food: document.getElementById("food").value.trim(),
        description: document.getElementById("description").value.trim(),
        price: parseFloat(document.getElementById("price").value),
        image: document.getElementById("image").value.trim() || null
    }

    try {
        const { data, error } = await supabase
            .from('menu-items')
            .insert([formData])
            .select() // optional: returns inserted row(s)

        if (error) throw error

        alert(`✅ Menu item "${data[0].food}" added successfully!`)
        e.target.reset() // reset form

    } catch (err) {
        console.error("Error adding menu item:", err)
        alert("❌ Failed to add menu item. Check console for details.")
    }
})

// ----- Fetch Menu Items -----
document.getElementById("fetchForm").addEventListener("submit", async (e) => {
    e.preventDefault()

    const restaurantId = parseInt(document.getElementById("fetch_restaurant_id").value, 10)
    const menuList = document.getElementById("menuList")
    menuList.innerHTML = "<p>Loading...</p>"

    try {
        const { data: menuItems, error } = await supabase
            .from('menu-items')
            .select('*')
            .eq('restaurant_id', restaurantId)

        if (error) throw error

        if (!menuItems || menuItems.length === 0) {
            menuList.innerHTML = "<p>No menu items found for this restaurant.</p>"
            return
        }

        // render menu items
        menuList.innerHTML = ""
        menuItems.forEach(item => {
            const div = document.createElement("div")
            div.classList.add("menu-item")
            div.innerHTML = `
                <h3>${item.food} - $${item.price}</h3>
                <p>${item.description || "No description available."}</p>
                ${item.image ? `<img src="${item.image}" alt="${item.food}">` : ""}
            `
            menuList.appendChild(div)
        })

    } catch (err) {
        console.error("Error fetching menu:", err)
        menuList.innerHTML = "<p>❌ Failed to fetch menu items. Check console for details.</p>"
    }
})
