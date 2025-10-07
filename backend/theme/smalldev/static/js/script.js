document.addEventListener("DOMContentLoaded", () => {
    console.log("Menu page loaded.");

    // Example of caching strategy (for later use with Cloudflare)
    if ('caches' in window) {
        caches.open('restaurant-menu-cache').then(cache => {
            cache.addAll([
                '/index.html',
                '/style.css',
                '/script.js'
            ]);
            console.log('Static assets cached!');
        });
    }
});
