document.addEventListener("DOMContentLoaded", () => {
    console.log("Menu page loaded successfully!");

    // Example: simple client-side filter by name
    const searchInput = document.getElementById("search");
    if (searchInput) {
        searchInput.addEventListener("input", (e) => {
            const query = e.target.value.toLowerCase();
            document.querySelectorAll(".menu-card").forEach(card => {
                const title = card.querySelector(".menu-title").textContent.toLowerCase();
                card.style.display = title.includes(query) ? "block" : "none";
            });
        });
    }
});
