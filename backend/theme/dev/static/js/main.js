// Main JavaScript - Common utilities and functions
console.log('Theme loaded - Development mode');

// Utility function to fetch data from R2 or API
async function fetchData(endpoint) {
    try {
        const response = await fetch(endpoint);
        if (!response.ok) throw new Error('Network response was not ok');
        return await response.json();
    } catch (error) {
        console.error('Error fetching data:', error);
        return null;
    }
}

// Format price
function formatPrice(price) {
    return `$${parseFloat(price).toFixed(2)}`;
}

// Format date
function formatDate(dateString) {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'long',
        day: 'numeric'
    });
}

// Show loading state
function showLoading(element) {
    element.innerHTML = '<p style="text-align: center; color: #999;">Loading...</p>';
}

// Show error state
function showError(element, message = 'Failed to load data') {
    element.innerHTML = `<p style="text-align: center; color: #e74c3c;">${message}</p>`;
}