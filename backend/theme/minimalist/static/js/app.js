/* ===== app.js =====
   Main application logic for menu interactions
   Data is provided by server-side rendering via window.restaurantData, window.mealGroupsData, window.mealsData
*/

// Currency configuration
const currencyConfig = {
    current: "KSH",
    options: [
        { code: "KSH", symbol: "KSh", rate: 1 },
        { code: "USD", symbol: "$", rate: 0.0077 }
    ]
};

// Format price based on current currency
function formatPrice(priceInKsh) {
    const currency = currencyConfig.options.find(c => c.code === currencyConfig.current);
    const convertedPrice = priceInKsh * currency.rate;
    return `${currency.symbol}${convertedPrice.toFixed(2)}`;
}

// Change currency (can be called from UI if needed)
function setCurrency(currencyCode) {
    currencyConfig.current = currencyCode;
    if (typeof window.refreshPrices === 'function') {
        window.refreshPrices();
    }
}

// Make currency functions globally available
window.formatPrice = formatPrice;
window.setCurrency = setCurrency;

document.addEventListener("DOMContentLoaded", () => {
    // Get data from window (server-rendered)
    const restaurantData = window.restaurantData || {};
    const mealGroups = window.mealGroupsData || {};
    const meals = window.mealsData || {};

    // DOM refs
    const restaurantNameEl = document.getElementById("restaurant-name");
    const categoriesContainer = document.getElementById("categories");
    const searchBtnEl = document.getElementById("search-btn");

    const mealGroupsRow = document.getElementById("meal-categories");
    const mealListEl = document.getElementById("meal-list");

    // Selected meal details (left panel)
    const selectedImage = document.getElementById("selected-meal-image");
    const selectedName = document.getElementById("selected-meal-name");
    const selectedDesc = document.getElementById("selected-meal-desc");
    const selectedSizeButtons = document.getElementById("selected-size-buttons");
    const selectedPairings = document.getElementById("selected-pairings");
    const leftQtyNumber = document.getElementById("left-qty-number");
    const leftQtyDec = document.getElementById("left-qty-dec");
    const leftQtyInc = document.getElementById("left-qty-inc");
    const leftPriceValue = document.getElementById("left-price-value");
    const ingredientsGrid = document.getElementById("ingredients-grid");

    // State management: mealId -> {selectedSizeIndex, qty, selectedPairingIndices, selectedIngredientIndices}
    const mealState = {};

    // Currently selected meal id
    let selectedMealId = null;

    // Restaurant name is already rendered by server, but we can update if needed
    if (restaurantNameEl && restaurantData.name) {
        restaurantNameEl.textContent = restaurantData.name;
    }

    // Add click event listeners to pre-rendered category items
    function initializeCategoryListeners() {
        const categoryItems = categoriesContainer.querySelectorAll(".category-item");
        categoryItems.forEach((el, idx) => {
            el.addEventListener("click", () => {
                categoryItems.forEach(x => x.classList.remove("active"));
                el.classList.add("active");
                const categoryName = el.dataset.categoryName;
                renderMealGroups(categoryName);
            });
        });

        // Initialize with first category
        if (categoryItems.length > 0) {
            const firstCategoryName = categoryItems[0].dataset.categoryName;
            if (firstCategoryName) {
                renderMealGroups(firstCategoryName);
            }
        }
    }

    // Render meal-group buttons in left container
    function renderMealGroups(categoryName) {
        mealGroupsRow.innerHTML = "";
        const groups = mealGroups[categoryName] || [];

        groups.forEach((groupName, idx) => {
            const btn = document.createElement("button");
            btn.className = "meal-group-button";
            btn.textContent = groupName;
            if (idx === 0) btn.classList.add("active");

            btn.addEventListener("click", () => {
                mealGroupsRow.querySelectorAll(".meal-group-button").forEach(b => b.classList.remove("active"));
                btn.classList.add("active");
                renderMealsForGroup(groupName);
            });

            mealGroupsRow.appendChild(btn);
        });

        if (groups.length > 0) {
            renderMealsForGroup(groups[0]);
        } else {
            mealListEl.innerHTML = `<div style="color:#666">No meals for "${categoryName}"</div>`;
            clearSelectedView();
        }
    }

    function clearSelectedView() {
        selectedImage.src = "";
        selectedName.textContent = "";
        selectedDesc.textContent = "";
        selectedSizeButtons.innerHTML = "";
        selectedPairings.innerHTML = "";
        ingredientsGrid.innerHTML = "";
        leftQtyNumber.textContent = "1";
        leftPriceValue.textContent = "0";
        selectedMealId = null;
    }

    function findMealById(mid) {
        for (const group in meals) {
            const found = meals[group].find(m => m.id === mid);
            if (found) return found;
        }
        return null;
    }

    // Initialize state for a meal (without pre-selecting ingredients)
    function getMealState(meal) {
        if (!meal) return null;
        if (!mealState[meal.id]) {
            let selectedSizeIndex = 0;
            const finalSizes = meal.sizes || [];
            const smallIdx = finalSizes.findIndex(s => s.name.toLowerCase() === "small");
            if (smallIdx !== -1) selectedSizeIndex = smallIdx;

            mealState[meal.id] = {
                selectedSizeIndex,
                qty: 1,
                selectedPairingIndices: new Set(),
                selectedIngredientIndices: new Set()
            };
        }
        return mealState[meal.id];
    }

    // Render meals for a group
    function renderMealsForGroup(groupName) {
        mealListEl.innerHTML = "";
        const items = meals[groupName] || [];

        if (!items.length) {
            mealListEl.innerHTML = `<div style="color:#666">No meals available in "${groupName}"</div>`;
            clearSelectedView();
            return;
        }

        items.forEach((meal, idx) => {
            const card = document.createElement("div");
            card.className = "meal-card";
            card.dataset.mealid = meal.id;

            card.innerHTML = `
                <img class="meal-img" src="${meal.image}" alt="${meal.food}" onerror="this.style.opacity=.45; this.style.filter='grayscale(60%)'">
                <div class="meal-details">
                    <div>
                        <div class="meal-name">${meal.food}</div>
                        <div class="meal-desc">${meal.description}</div>
                    </div>
                    <div class="size-buttons" role="tablist"></div>
                    <div class="price-row">
                        <div class="price-block">
                            <div class="price-value">0</div>
                        </div>
                        <div style="display:flex; gap:0.8rem; align-items:center;">
                            <div class="qty-controls">
                                <button class="qty-dec">−</button>
                                <div class="qty-number">1</div>
                                <button class="qty-inc">+</button>
                            </div>
                        </div>
                    </div>
                </div>
            `;

            card.addEventListener("click", () => {
                selectMeal(meal.id);
                document.querySelectorAll(".meal-card").forEach(c => c.classList.remove("selected"));
                card.classList.add("selected");
            });

            mealListEl.appendChild(card);

            const sizeButtonsRow = card.querySelector(".size-buttons");
            const priceValueEl = card.querySelector(".price-value");
            const qtyNumberEl = card.querySelector(".qty-number");
            const decBtn = card.querySelector(".qty-dec");
            const incBtn = card.querySelector(".qty-inc");

            const sizes = Array.isArray(meal.sizes) ? meal.sizes : [];
            const finalSizes = reorderSizes(sizes);

            const state = getMealState(meal);
            if (state.selectedSizeIndex >= finalSizes.length) state.selectedSizeIndex = 0;

            finalSizes.forEach((s, i) => {
                const b = document.createElement("button");
                b.className = "size-button";
                b.textContent = s.name;
                if (i === state.selectedSizeIndex) b.classList.add("active");

                b.addEventListener("click", ev => {
                    ev.stopPropagation();
                    state.selectedSizeIndex = i;
                    sizeButtonsRow.querySelectorAll(".size-button").forEach(x => x.classList.remove("active"));
                    b.classList.add("active");
                    updateCardPrice(meal, state, priceValueEl);
                    if (selectedMealId === meal.id) renderSelectedMeal(meal);
                });

                sizeButtonsRow.appendChild(b);
            });

            decBtn.addEventListener("click", ev => {
                ev.stopPropagation();
                if (state.qty > 1) state.qty -= 1;
                qtyNumberEl.textContent = state.qty;
                updateCardPrice(meal, state, priceValueEl);
                if (selectedMealId === meal.id) syncLeftFromState(meal, state);
            });

            incBtn.addEventListener("click", ev => {
                ev.stopPropagation();
                state.qty += 1;
                qtyNumberEl.textContent = state.qty;
                updateCardPrice(meal, state, priceValueEl);
                if (selectedMealId === meal.id) syncLeftFromState(meal, state);
            });

            qtyNumberEl.textContent = state.qty;
            updateCardPrice(meal, state, priceValueEl);

            if (idx === 0 && selectedMealId === null) {
                card.classList.add("selected");
                selectMeal(meal.id);
            }
        });
    }

    // Update card price (includes size, qty, pairings, ingredients)
    function updateCardPrice(meal, state, priceValueEl) {
        const finalSizes = reorderSizes(meal.sizes || []);
        const size = finalSizes[state.selectedSizeIndex] || { price: 0 };
        let total = (size.price || 0) * (state.qty || 1);

        // Add pairings price
        if (meal.pairings) {
            state.selectedPairingIndices.forEach(i => {
                const p = meal.pairings[i];
                if (p) total += (p.price || 0);
            });
        }

        // Add ingredients price
        if (meal.ingredients) {
            state.selectedIngredientIndices.forEach(i => {
                const ingredient = meal.ingredients[i];
                if (ingredient && ingredient.price) {
                    total += ingredient.price;
                }
            });
        }

        priceValueEl.textContent = formatPrice(total);
    }

    function reorderSizes(sizes) {
        const order = ["Small", "Medium", "Large"];
        const sorted = [];
        order.forEach(o => {
            const s = sizes.find(x => x.name.toLowerCase() === o.toLowerCase());
            if (s) sorted.push(s);
        });
        sizes.forEach(s => {
            if (!["small","medium","large"].includes(s.name.toLowerCase())) sorted.push(s);
        });
        return sorted.length ? sorted : sizes.slice();
    }

    function selectMeal(mealId) {
        const meal = findMealById(mealId);
        if (!meal) return;
        selectedMealId = mealId;
        document.querySelectorAll(".meal-card").forEach(c => {
            c.dataset.mealid === mealId ? c.classList.add("selected") : c.classList.remove("selected");
        });
        renderSelectedMeal(meal);
    }

    // Render selected meal in left panel
    function renderSelectedMeal(meal) {
        const state = getMealState(meal);
        const finalSizes = reorderSizes(meal.sizes || []);

        // Image & texts
        selectedImage.src = meal.image || "";
        selectedImage.onerror = null;
        selectedName.textContent = meal.food;
        selectedDesc.textContent = meal.description;

        // Sizes
        selectedSizeButtons.innerHTML = "";
        finalSizes.forEach((s, i) => {
            const b = document.createElement("button");
            b.className = "size-button";
            b.textContent = s.name;
            if (i === state.selectedSizeIndex) b.classList.add("active");

            b.addEventListener("click", () => {
                state.selectedSizeIndex = i;
                selectedSizeButtons.querySelectorAll(".size-button").forEach(x => x.classList.remove("active"));
                b.classList.add("active");
                updateAllCardsForMeal(meal);
                updateLeftPrice(meal);
            });

            selectedSizeButtons.appendChild(b);
        });

        // Pairings
        selectedPairings.innerHTML = "";
        if (meal.pairings && meal.pairings.length) {
            meal.pairings.forEach((p, i) => {
                const pc = document.createElement("div");
                pc.className = "pairing-card";
                if (state.selectedPairingIndices.has(i)) pc.classList.add("active");

                pc.innerHTML = `
                    <img class="pairing-img" src="${p.image}" alt="${p.name}" onerror="this.style.opacity=.6;">
                    <div class="pairing-content">
                        <div class="pairing-name">${p.name}</div>
                        <div class="pairing-bottom-row">
                            <div class="pairing-price">${formatPrice(p.price)}</div>
                            <button class="pairing-add-btn">${state.selectedPairingIndices.has(i) ? '✓' : '+'}</button>
                        </div>
                    </div>
                `;

                pc.addEventListener("click", ev => {
                    ev.stopPropagation();
                    state.selectedPairingIndices.has(i) ? state.selectedPairingIndices.delete(i) : state.selectedPairingIndices.add(i);
                    pc.classList.toggle("active");
                    pc.querySelector(".pairing-add-btn").textContent = state.selectedPairingIndices.has(i) ? '✓' : '+';
                    updateLeftPrice(meal);
                    updateAllCardsForMeal(meal);
                });

                selectedPairings.appendChild(pc);
            });
        } else {
            selectedPairings.innerHTML = `<div style="color:#999; font-size:0.9rem">No recommended pairings</div>`;
        }

        // Build Your Meal section
        renderBuildYourMeal(meal, state);

        // Qty controls
        leftQtyNumber.textContent = state.qty;
        leftQtyDec.onclick = ev => {
            ev.stopPropagation();
            if (state.qty > 1) state.qty -= 1;
            leftQtyNumber.textContent = state.qty;
            updateLeftPrice(meal);
            updateAllCardsForMeal(meal);
        };
        leftQtyInc.onclick = ev => {
            ev.stopPropagation();
            state.qty += 1;
            leftQtyNumber.textContent = state.qty;
            updateLeftPrice(meal);
            updateAllCardsForMeal(meal);
        };

        updateLeftPrice(meal);
    }

    // Render Build Your Meal ingredients (not active on load)
    function renderBuildYourMeal(meal, state) {
        if (!ingredientsGrid) return;
        ingredientsGrid.innerHTML = "";

        if (!Array.isArray(meal.ingredients) || meal.ingredients.length === 0) {
            ingredientsGrid.innerHTML = `<div style="color:#999; font-size:0.9rem">No customizable ingredients</div>`;
            return;
        }

        meal.ingredients.forEach((ingredient, index) => {
            const item = document.createElement("div");
            item.className = "ingredient-item";
            if (state.selectedIngredientIndices.has(index)) {
                item.classList.add("active");
            }

            item.innerHTML = `
                <img class="ingredient-img" src="${ingredient.image}" alt="${ingredient.name}" onerror="this.style.opacity=.6;">
                <div class="ingredient-name">${ingredient.name}</div>
                <div class="checkmark">✓</div>
            `;

            item.addEventListener("click", ev => {
                ev.stopPropagation();
                if (state.selectedIngredientIndices.has(index)) {
                    state.selectedIngredientIndices.delete(index);
                    item.classList.remove("active");
                } else {
                    state.selectedIngredientIndices.add(index);
                    item.classList.add("active");
                }
                updateLeftPrice(meal);
                updateAllCardsForMeal(meal);
            });

            ingredientsGrid.appendChild(item);
        });
    }

    // Update left panel price
    function updateLeftPrice(meal) {
        const state = getMealState(meal);
        const finalSizes = reorderSizes(meal.sizes || []);
        const size = finalSizes[state.selectedSizeIndex] || { price: 0 };
        let total = (size.price || 0) * (state.qty || 1);

        // Add pairings
        if (meal.pairings && meal.pairings.length) {
            state.selectedPairingIndices.forEach(i => {
                const p = meal.pairings[i];
                if (p) total += (p.price || 0);
            });
        }

        // Add ingredients price
        if (meal.ingredients) {
            state.selectedIngredientIndices.forEach(i => {
                const ingredient = meal.ingredients[i];
                if (ingredient && ingredient.price) {
                    total += ingredient.price;
                }
            });
        }

        leftPriceValue.textContent = formatPrice(total);
    }

    function updateAllCardsForMeal(meal) {
        document.querySelectorAll(`.meal-card`).forEach(card => {
            const cardMealId = card.dataset.mealid;
            if (cardMealId !== meal.id) return;

            const priceValueEl = card.querySelector(".price-value");
            const qtyNumberEl = card.querySelector(".qty-number");
            const state = getMealState(meal);

            qtyNumberEl.textContent = state.qty;

            const sizeBtnEls = card.querySelectorAll(".size-button");
            sizeBtnEls.forEach((b, i) => b.classList.toggle("active", i === state.selectedSizeIndex));

            updateCardPrice(meal, state, priceValueEl);
        });
    }

    function syncLeftFromState(meal, state) {
        if (selectedMealId !== meal.id) return;
        leftQtyNumber.textContent = state.qty;
        selectedSizeButtons.querySelectorAll(".size-button").forEach((b, i) => b.classList.toggle("active", i === state.selectedSizeIndex));
        selectedPairings.querySelectorAll(".pairing-card").forEach((pc, i) => {
            pc.classList.toggle("active", state.selectedPairingIndices.has(i));
            const btn = pc.querySelector(".pairing-add-btn");
            if (btn) btn.textContent = state.selectedPairingIndices.has(i) ? '✓' : '+';
        });

        ingredientsGrid.querySelectorAll(".ingredient-item").forEach((item, i) => {
            item.classList.toggle("active", state.selectedIngredientIndices.has(i));
        });

        updateLeftPrice(meal);
    }

    // Initialize
    initializeCategoryListeners();
});