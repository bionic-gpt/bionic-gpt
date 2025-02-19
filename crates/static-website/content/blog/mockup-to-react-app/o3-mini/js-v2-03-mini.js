document.addEventListener('DOMContentLoaded', function () {
  const loginForm = document.getElementById('login-form');
  const registerForm = document.getElementById('register-form');
  const authContainer = document.getElementById('auth-container');
  const appContainer = document.getElementById('app-container');

  if (loginForm) {
    loginForm.addEventListener('submit', function (event) {
      event.preventDefault();
      const username = document.getElementById('login-username').value;
      const password = document.getElementById('login-password').value;
      console.log('Logging in with:', username, password);
      if (username && password) {
        if (authContainer) authContainer.style.display = 'none';
        if (appContainer) appContainer.style.display = 'block';
        alert('Logged in as ' + username);
      } else {
        alert('Please enter valid credentials.');
      }
    });
  }

  if (registerForm) {
    registerForm.addEventListener('submit', function(event) {
      event.preventDefault();
      const username = document.getElementById('register-username').value;
      const email = document.getElementById('register-email').value;
      const password = document.getElementById('register-password').value;
      console.log('Registering user:', username, email, password);
      alert('Registration successful for ' + username);
    });
  }

  const navLinks = document.querySelectorAll('header nav ul li a');
  navLinks.forEach(link => {
    link.addEventListener('click', function (event) {
      event.preventDefault();
      const targetId = this.getAttribute('href').substring(1);
      const targetElement = document.getElementById(targetId);
      if (targetElement) {
        targetElement.scrollIntoView({ behavior: 'smooth' });
      }
    });
  });

  function createAircraft(data) {
    console.log('Creating aircraft:', data);
  }

  function updateAircraft(id, data) {
    console.log('Updating aircraft:', id, data);
  }

  function deleteAircraft(id) {
    console.log('Deleting aircraft:', id);
  }

  function createPilot(data) {
    console.log('Creating pilot:', data);
  }

  function updatePilot(id, data) {
    console.log('Updating pilot:', id, data);
  }

  function deletePilot(id) {
    console.log('Deleting pilot:', id);
  }

  const aircraftCrud = document.getElementById('aircraft-crud');
  const pilotsCrud = document.getElementById('pilots-crud');

  if (aircraftCrud) {
    const createAircraftButton = document.createElement('button');
    createAircraftButton.textContent = 'Add Aircraft';
    createAircraftButton.addEventListener('click', function () {
      createAircraft({ model: 'Boeing 747', status: 'active' });
    });
    aircraftCrud.appendChild(createAircraftButton);
  }

  if (pilotsCrud) {
    const createPilotButton = document.createElement('button');
    createPilotButton.textContent = 'Add Pilot';
    createPilotButton.addEventListener('click', function () {
      createPilot({ name: 'John Doe', license: 'ATPL' });
    });
    pilotsCrud.appendChild(createPilotButton);
  }
});