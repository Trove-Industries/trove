// About page specific JavaScript
document.addEventListener('DOMContentLoaded', async () => {
    const teamContainer = document.getElementById('team-content');

    // Fetch team data - adjust endpoint as needed
    const data = await fetchData('/api/team');

    if (data && data.team) {
        renderTeam(data.team);
    } else {
        // Demo data for development
        renderTeam([
            {
                name: 'John Doe',
                role: 'Head Chef',
                bio: 'Passionate about culinary arts',
                image: 'https://via.placeholder.com/150'
            },
            {
                name: 'Jane Smith',
                role: 'Manager',
                bio: 'Ensuring great experiences',
                image: 'https://via.placeholder.com/150'
            },
            {
                name: 'Mike Wilson',
                role: 'Sous Chef',
                bio: 'Creative food preparation',
                image: 'https://via.placeholder.com/150'
            }
        ]);
    }
});

function renderTeam(team) {
    const container = document.getElementById('team-content');
    container.innerHTML = team.map(member => `
    <div class="team-member">
      <img src="${member.image}" alt="${member.name}">
      <h3>${member.name}</h3>
      <div class="role">${member.role}</div>
      <p>${member.bio}</p>
    </div>
  `).join('');
}