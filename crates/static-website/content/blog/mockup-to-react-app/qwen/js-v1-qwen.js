// Data storage
let data = {
    aircraft: [
        { id: 1, name: 'Boeing 747', airline: 'Delta' },
        { id: 2, name: 'Airbus A380', airline: 'British Airways' }
    ],
    airlines: [
        { id: 1, name: 'Delta' },
        { id: 2, name: 'British Airways' }
    ],
    users: [
        { id: 1, name: 'John Doe', email: 'john.doe@example.com' },
        { id: 2, name: 'Jane Smith', email: 'jane.smith@example.com' }
    ]
};

// Function to show table for a specific entity
function showTable(entity) {
    const contentDiv = document.getElementById('content');
    contentDiv.innerHTML = `
        <h2 class="text-2xl font-bold mb-4">${entity.charAt(0).toUpperCase() + entity.slice(1)}</h2>
        <button class="bg-blue-500 text-white px-4 py-2 rounded mb-4" onclick="showForm('${entity}', 'add')">Add ${entity.charAt(0).toUpperCase() + entity.slice(1)}</button>
        <table class="w-full border-collapse bg-white text-left text-sm">
            <thead>
                <tr>
                    ${Object.keys(data[entity][0]).map(key => `<th class="border-b font-medium p-4 pl-8">${key.charAt(0).toUpperCase() + key.slice(1)}</th>`).join('')}
                    <th class="border-b font-medium p-4 pl-8">Actions</th>
                </tr>
            </thead>
            <tbody>
                ${data[entity].map(item => `
                    <tr>
                        ${Object.values(item).map(value => `<td class="border-b border-slate-100 p-4 pl-8">${value}</td>`).join('')}
                        <td class="border-b border-slate-100 p-4 pl-8">
                            <button class="bg-green-500 text-white px-2 py-1 rounded mr-2" onclick="showForm('${entity}', 'edit', ${item.id})">Edit</button>
                            <button class="bg-red-500 text-white px-2 py-1 rounded" onclick="deleteItem('${entity}', ${item.id})">Delete</button>
                        </td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
    `;
}

// Function to show form for adding or editing an entity
function showForm(entity, action, id) {
    const contentDiv = document.getElementById('content');
    let formTitle = action === 'add' ? `Add ${entity.charAt(0).toUpperCase() + entity.slice(1)}` : `Edit ${entity.charAt(0).toUpperCase() + entity.slice(1)}`;
    let formData = action === 'add' ? {} : data[entity].find(item => item.id === id);
    let formFields = Object.keys(data[entity][0]).map(key => `
        <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="${key}">${key.charAt(0).toUpperCase() + key.slice(1)}</label>
            <input class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="${key}" type="text" value="${formData[key] || ''}">
        </div>
    `).join('');

    contentDiv.innerHTML = `
        <h2 class="text-2xl font-bold mb-4">${formTitle}</h2>
        <form onsubmit="submitForm('${entity}', '${action}', ${id || 'null'}); return false;">
            ${formFields}
            <button class="bg-blue-500 text-white px-4 py-2 rounded" type="submit">Submit</button>
        </form>
    `;
}

// Function to submit form data
function submitForm(entity, action, id) {
    const formData = {};
    Object.keys(data[entity][0]).forEach(key => {
        formData[key] = document.getElementById(key).value;
    });

    if (action === 'add') {
        formData.id = data[entity].length ? data[entity][data[entity].length - 1].id + 1 : 1;
        data[entity].push(formData);
    } else if (action === 'edit') {
        const index = data[entity].findIndex(item => item.id === id);
        data[entity][index] = formData;
    }

    showTable(entity);
}

// Function to delete an item
function deleteItem(entity, id) {
    data[entity] = data[entity].filter(item => item.id !== id);
    showTable(entity);
}

// Initial table view for aircraft
showTable('aircraft');