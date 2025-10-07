// Events page specific JavaScript
document.addEventListener('DOMContentLoaded', async () => {
    const eventsContainer = document.getElementById('events-list');

    showLoading(eventsContainer);

    // Fetch events data - adjust endpoint as needed
    const data = await fetchData('/api/events');

    if (data && data.events) {
        renderEvents(data.events);
    } else {
        // Demo data for development
        renderEvents([
            {
                title: 'Wine Tasting Night',
                date: '2024-12-15',
                time: '7:00 PM - 10:00 PM',
                description: 'Join us for an evening of fine wines paired with appetizers.'
            },
            {
                title: 'Live Jazz Performance',
                date: '2024-12-20',
                time: '8:00 PM - 11:00 PM',
                description: 'Enjoy live music while dining with us.'
            },
            {
                title: 'Chef\'s Table Experience',
                date: '2024-12-28',
                time: '6:00 PM - 9:00 PM',
                description: 'Exclusive multi-course dining experience with our head chef.'
            }
        ]);
    }
});

function renderEvents(events) {
    const container = document.getElementById('events-list');
    container.innerHTML = events.map(event => {
        const date = new Date(event.date);
        const day = date.getDate();
        const month = date.toLocaleString('en-US', { month: 'short' });

        return `
      <div class="event-card">
        <div class="event-date">
          <div class="day">${day}</div>
          <div class="month">${month}</div>
        </div>
        <div class="event-content">
          <h3>${event.title}</h3>
          <div class="time">${event.time}</div>
          <p>${event.description}</p>
        </div>
      </div>
    `;
    }).join('');
}