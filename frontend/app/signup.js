const API_BASE_URL = "http://localhost:8000";
const form = document.getElementById("signup-form");
const messageDiv = document.getElementById("message");

form.addEventListener("submit", async (e) => {
    e.preventDefault();

    const email = document.getElementById("email").value.trim();
    const password = document.getElementById("password").value.trim();

    if (!email || !password) {
        return showMessage("Please enter both email and password.", "error");
    }

    try {
        const res = await fetch(`${API_BASE_URL}/auth/signup`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ email, password }),
            credentials: "include",
        });

        if (res.ok) {
            showMessage("Account created successfully! Redirecting...", "success");
            setTimeout(() => (window.location.href = "dashboard.html"), 1500);
        } else {
            const errData = await res.json();
            showMessage(errData.error || "Signup failed. Try again.", "error");
        }
    } catch (err) {
        console.error("Signup error:", err);
        showMessage("Network error. Please try again.", "error");
    }
});

function showMessage(text, type) {
    messageDiv.textContent = text;
    messageDiv.className = `message ${type}`;
}
