// Call this function to set the sidebar state when the page loads
export const initializeSidebar = () => {
  const sidebarState = localStorage.getItem('sidebarState') || 'open';
  const sidebar = document.getElementById('sidebar');

  if (sidebar) {
    sidebar.classList.toggle('w-0', sidebarState === 'closed');
    sidebar.classList.toggle('md:w-64', sidebarState === 'open');
  }

  // Attach the toggleSidebar function to the button click
  document.getElementById('toggleButton')?.addEventListener('click', toggleSidebar);
};

// Function to toggle sidebar and save state to localStorage
function toggleSidebar() {
  // Get the current state from localStorage or default to 'closed'
  const sidebarState = localStorage.getItem('sidebarState') || 'closed';

  // Determine the new state
  const newState = sidebarState === 'open' ? 'closed' : 'open';

  // Update the state in localStorage
  localStorage.setItem('sidebarState', newState);

  // Add or remove a class from the sidebar based on the new state
  const sidebar = document.getElementById('sidebar');
  if (sidebar) {
    sidebar.classList.toggle('w-0', newState === 'closed');
    sidebar.classList.toggle('md:w-64', newState === 'open');
  }
}
