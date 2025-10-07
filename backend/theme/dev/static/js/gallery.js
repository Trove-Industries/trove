// Gallery page specific JavaScript
document.addEventListener('DOMContentLoaded', async () => {
    const galleryContainer = document.getElementById('gallery-grid');

    showLoading(galleryContainer);

    // Fetch gallery data from R2 - adjust endpoint as needed
    const data = await fetchData('/api/gallery');

    if (data && data.images) {
        renderGallery(data.images);
    } else {
        // Demo data for development
        renderGallery([
            { id: 1, title: 'Signature Dish', caption: 'Our most popular item', url: 'https://via.placeholder.com/400' },
            { id: 2, title: 'Interior View', caption: 'Cozy atmosphere', url: 'https://via.placeholder.com/400' },
            { id: 3, title: 'Fresh Ingredients', caption: 'Quality produce', url: 'https://via.placeholder.com/400' },
            { id: 4, title: 'Dessert Selection', caption: 'Sweet treats', url: 'https://via.placeholder.com/400' },
            { id: 5, title: 'Bar Area', caption: 'Craft cocktails', url: 'https://via.placeholder.com/400' },
            { id: 6, title: 'Outdoor Seating', caption: 'Al fresco dining', url: 'https://via.placeholder.com/400' }
        ]);
    }
});

function renderGallery(images) {
    const container = document.getElementById('gallery-grid');
    container.innerHTML = images.map(image => `
    <div class="gallery-item">
      <img src="${image.url}" alt="${image.title}">
      <div class="gallery-item-overlay">
        <h3>${image.title}</h3>
        <p>${image.caption}</p>
      </div>
    </div>
  `).join('');
}