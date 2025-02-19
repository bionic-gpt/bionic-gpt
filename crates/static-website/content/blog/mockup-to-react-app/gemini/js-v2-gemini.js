document.addEventListener('DOMContentLoaded', function () {
    const entityLinks = document.querySelectorAll('.entity-link');
    const entityTableContainer = document.getElementById('entity-table-container');
    const entityFormContainer = document.getElementById('entity-form-container');
    const landingPage = document.getElementById('landing-page');
    const entityList = document.getElementById('entity-list');

    let data = {
        aircraft: [
            { id: 1, name: 'Boeing 747', airline: 'United' },
            { id: 2, name: 'Airbus A380', airline: 'Emirates' }
        ],
        airlines: [
            { id: 1, name: 'United Airlines' },
            { id: 2, name: 'Emirates' }
        ],
        users: [
            { id: 1, name: 'John Doe', email: 'john.doe@example.com' },
            { id: 2, name: 'Jane Smith', email: 'jane.smith@example.com' }
        ],
        pilots: [
            { id: 1, name: 'Tom Cruise', license: 'P123' },
            { id: 2, name: 'Val Kilmer', license: 'P456' }
        ]
    };

    entityLinks.forEach(link => {
        link.addEventListener('click', function (event) {
            event.preventDefault();
            const entity = this.dataset.entity;
            showTable(entity);
        });
    });

    // Prepopulate login form
    document.getElementById('login-email').value = 'test@example.com';
    document.getElementById('login-password').value = 'password';

    // Handle login form submission
    document.getElementById('login-form').addEventListener('submit', function (event) {
        event.preventDefault();
        alert('Login successful!'); // Mock login
    });

    // Show landing page initially and hide entity list
    landingPage.classList.remove('hidden');
    entityList.classList.add('hidden');

    // Handle show entities button click
    document.getElementById('show-entities').addEventListener('click', function () {
        landingPage.classList.add('hidden');
        entityList.classList.remove('hidden');
    });

    function showTable(entity) {
        let tableHtml = `
            <h3 class="text-xl font-bold mb-2">${entity.charAt(0).toUpperCase() + entity.slice(1)}</h3>
            <button class="add-entity bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded mb-4">Add ${entity.charAt(0).toUpperCase() + entity.slice(1)}</button>
            <table class="table-auto w-full">
                <thead>
                    <tr>
                        ${Object.keys(data[entity][0]).map(key => `<th class="px-4 py-2">${key}</th>`).join('')}
                        <th class="px-4 py-2">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    ${data[entity].map(item => `
                        <tr>
                            ${Object.values(item).map(value => `<td class="border px-4 py-2">${value}</td>`).join('')}
                            <td class="border px-4 py-2">
                                <button class="edit-entity bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mr-2" data-id="${item.id}">Edit</button>
                                <button class="delete-entity bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded" data-id="${item.id}">Delete</button>
                            </td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
        entityTableContainer.innerHTML = tableHtml;

        // Add event listeners for edit and delete buttons
        const editButtons = document.querySelectorAll('.edit-entity');
        editButtons.forEach(button => {
            button.addEventListener('click', function () {
                const id = this.dataset.id;
                showEditForm(entity, id);
            });
        });

        const deleteButtons = document.querySelectorAll('.delete-entity');
        deleteButtons.forEach(button => {
            button.addEventListener('click', function () {
                const id = this.dataset.id;
                deleteEntity(entity, id);
            });
        });

        // Add event listener for add button
        const addButton = document.querySelector('.add-entity');
        addButton.addEventListener('click', function () {
            showAddForm(entity);
        });
    }

    function showAddForm(entity) {
        let formHtml = `
            <h3 class="text-xl font-bold mb-2">Add ${entity.charAt(0).toUpperCase() + entity.slice(1)}</h3>
            <form id="add-form">
                ${Object.keys(data[entity][0]).map(key => `
                    <div class="mb-4">
                        <label for="${key}" class="block text-gray-700 text-sm font-bold mb-2">${key.charAt(0).toUpperCase() + key.slice(1)}:</label>
                        <input type="text" id="${key}" name="${key}" class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline">
                    </div>
                `).join('')}
                <button class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="submit">Add</button>
                <button class="cancel-form bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="button">Cancel</button>
            </form>
        `;
        entityFormContainer.innerHTML = formHtml;

        const addForm = document.getElementById('add-form');
        addForm.addEventListener('submit', function (event) {
            event.preventDefault();
            const newEntity = {};
            Object.keys(data[entity][0]).forEach(key => {
                newEntity[key] = document.getElementById(key).value;
            });
            newEntity.id = data[entity].length + 1; // Simple ID generation
            data[entity].push(newEntity);
            showTable(entity);
            entityFormContainer.innerHTML = '';
        });

        const cancelButton = document.querySelector('.cancel-form');
        cancelButton.addEventListener('click', function () {
            entityFormContainer.innerHTML = '';
            showTable(entity);
        });
    }

    function showEditForm(entity, id) {
        const item = data[entity].find(item => item.id == id);
        let formHtml = `
            <h3 class="text-xl font-bold mb-2">Edit ${entity.charAt(0).toUpperCase() + entity.slice(1)}</h3>
            <form id="edit-form">
                ${Object.keys(item).map(key => `
                    <div class="mb-4">
                        <label for="${key}" class="block text-gray-700 text-sm font-bold mb-2">${key.charAt(0).toUpperCase() + key.slice(1)}:</label>
                        <input type="text" id="${key}" name="${key}" value="${item[key]}" class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline">
                    </div>
                `).join('')}
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="submit">Update</button>
                <button class="cancel-form bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="button">Cancel</button>
            </form>
        `;
        entityFormContainer.innerHTML = formHtml;

        const editForm = document.getElementById('edit-form');
        editForm.addEventListener('submit', function (event) {
            event.preventDefault();
            Object.keys(item).forEach(key => {
                item[key] = document.getElementById(key).value;
            });
            showTable(entity);
            entityFormContainer.innerHTML = '';
        });

        const cancelButton = document.querySelector('.cancel-form');
        cancelButton.addEventListener('click', function () {
            entityFormContainer.innerHTML = '';
            showTable(entity);
        });
    }

    function deleteEntity(entity, id) {
        data[entity] = data[entity].filter(item => item.id != id);
        showTable(entity);
    }
});