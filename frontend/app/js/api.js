const BASE_URL = "https://api.troveindustries.dev";

export async function apiRequest(endpoint, method = "GET", body = null) {
    const options = { method, headers: { "Content-Type": "application/json" } };
    if (body) options.body = JSON.stringify(body);

    const res = await fetch(`${BASE_URL}${endpoint}`, options);
    if (!res.ok) throw new Error(`API error: ${res.status}`);
    return res.json();
}
