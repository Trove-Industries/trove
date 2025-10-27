document.addEventListener("DOMContentLoaded", async () => {
    const table = document.getElementById("users-table");
    const tbody = document.getElementById("users-body");
    const loader = document.getElementById("loader");
    const errorMsg = document.getElementById("error-msg");

    try {
        const response = await fetch("http://0.0.0.0:8000/admin/get-all-users");
        if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`);

        const users = await response.json();

        loader.classList.add("hidden");
        table.classList.remove("hidden");

        users.forEach(user => {
            const row = document.createElement("tr");

            row.innerHTML = `
                <td>${user.id}</td>
                <td>${user.username}</td>
                <td>${user.email ?? "—"}</td>
                <td>${user.is_verified ? "✅" : "❌"}</td>
                <td>${new Date(user.created_at).toLocaleString()}</td>
                <td>${new Date(user.last_seen_at).toLocaleString()}</td>
            `;

            tbody.appendChild(row);
        });

    } catch (err) {
        loader.classList.add("hidden");
        errorMsg.classList.remove("hidden");
        errorMsg.textContent = `Failed to load users: ${err.message}`;
        console.error(err);
    }
});
