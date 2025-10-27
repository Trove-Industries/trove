const API_BASE_URL = "http://localhost:8000";

const form = document.getElementById("login-form");
const emailInput = document.getElementById("email");
const passwordInput = document.getElementById("password");
const messageDiv = document.getElementById("message");

form.addEventListener("submit", async (e) => {
    e.preventDefault();

    const email = emailInput.value.trim();
    const password = passwordInput.value.trim();

    if (!email || !password) {
        showMessage("Please fill in all fields.", "error");
        return;
    }

    const payload = { email, password };

    try {
        const res = await fetch(`${API_BASE_URL}/auth/login`, {
            method: "POST",
            credentials: "include",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload),
        });

        if (res.ok) {
            showMessage("Login successful! Redirecting...", "success");
            setTimeout(() => {
                window.location.href = "dashboard.html";
            }, 1000);
        } else {
            const errData = await res.json();
            showMessage(errData.error || "Invalid credentials", "error");
        }
    } catch (err) {
        console.error("Login error:", err);
        showMessage("Network error. Try again.", "error");
    }
});

// Helper to display messages
function showMessage(text, type) {
    messageDiv.textContent = text;
    messageDiv.className = `message ${type}`;
    messageDiv.style.display = "block";

    if (type === "success") setTimeout(() => (messageDiv.style.display = "none"), 3000);
}
