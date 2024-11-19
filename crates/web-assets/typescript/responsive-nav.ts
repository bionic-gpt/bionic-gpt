// Call this function to set the sidebar state when the page loads
export const initializeSidebar = () => {
  // Attach the toggleSidebar function to the button click
  document.getElementById('toggleButton')?.addEventListener('click', toggleSidebar);
};

// Function to toggle sidebar and save state to localStorage
function toggleSidebar() {

  // Add or remove a class from the sidebar based on the new state
  const sidebar = document.getElementById('sidebar');
  if (sidebar) {
    // On mobile screens
    if (window.innerWidth < 1024) { // Tailwind's lg breakpoint is 1024px
      sidebar.classList.toggle('-translate-x-full');
    } else {
      sidebar.classList.toggle('hidden');
    }
  }
}
