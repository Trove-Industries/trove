// Homepage specific JavaScript
document.addEventListener('DOMContentLoaded', async () => {
    const featuredContainer = document.getElementById('featured-items');

    showLoading(featuredContainer);

    // Fetch featured items - adjust endpoint as needed
    const data = await fetchData('/api/featured');

    if (data && data.items) {
        renderFeaturedItems(data.items);
    } else {
        // Demo data for development
        renderFeaturedItems([
            {
                id: 1,
                name: 'House Special',
                description: 'Our signature dish',
                price: 24.99,
                image: 'https://via.placeholder.com/300x200'
            },
            {
                id: 2,
                name: 'Chef\'s Choice',
                description: 'Daily selection',
                price: 29.99,
                image: 'https://via.placeholder.com/300x200'
            },
            {
                id: 3,
                name: 'Seasonal Delight',
                description: 'Fresh ingredients',
                price: 19.99,
                image: 'https://via.placeholder.com/300x200'
            }
        ]);
    }
});

function renderFeaturedItems(items) {
    const container = document.getElementById('featured-items');
    container.innerHTML = items.map(item => `
    <div class="featured-item">
      <img src="${item.image}" alt="${item.name}">
      <div class="featured-item-content">
        <h3>${item.name}</h3>
        <p>${item.description}</p>
        <div class="price">${formatPrice(item.price)}</div>
      </div>
    </div>
  `).join('');
}