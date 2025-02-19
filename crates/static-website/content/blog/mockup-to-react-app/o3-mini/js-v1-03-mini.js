document.addEventListener("DOMContentLoaded", initApp);

let dataStore = {
  aircraft: [
    { id: 1, name: "Boeing 747", airline: "Airline A" },
    { id: 2, name: "Airbus A320", airline: "Airline B" }
  ],
  airlines: [
    { id: 1, name: "Airline A" },
    { id: 2, name: "Airline B" }
  ],
  users: [
    { id: 1, name: "John Doe", email: "john@example.com" },
    { id: 2, name: "Jane Smith", email: "jane@example.com" }
  ]
};

function initApp() {
  // Attach click events to side menu buttons
  let entityButtons = document.querySelectorAll(".entity-btn");
  entityButtons.forEach(btn => {
    btn.addEventListener("click", function () {
      let entity = btn.getAttribute("data-entity");
      renderTable(entity);
    });
  });
  
  // Default view: display the "aircraft" entity
  renderTable("aircraft");
}

function renderTable(entity) {
  let contentDiv = document.getElementById("content");
  contentDiv.innerHTML = "";

  // Title for the table view
  let title = document.createElement("h2");
  title.className = "text-2xl font-bold mb-4";
  title.textContent = entity.charAt(0).toUpperCase() + entity.slice(1) + " List";
  contentDiv.appendChild(title);

  // Build table element
  let table = document.createElement("table");
  table.className = "min-w-full bg-white";
  
  let thead = document.createElement("thead");
  thead.className = "bg-gray-200";
  let headerRow = document.createElement("tr");
  let columns = [];
  
  if (entity === "aircraft") {
    columns = ["ID", "Name", "Airline", "Actions"];
  } else if (entity === "airlines") {
    columns = ["ID", "Name", "Actions"];
  } else if (entity === "users") {
    columns = ["ID", "Name", "Email", "Actions"];
  }
  
  columns.forEach(col => {
    let th = document.createElement("th");
    th.className = "py-2 px-4 border";
    th.textContent = col;
    headerRow.appendChild(th);
  });
  thead.appendChild(headerRow);
  table.appendChild(thead);
  
  let tbody = document.createElement("tbody");
  let records = dataStore[entity];
  records.forEach(record => {
    let row = document.createElement("tr");
    row.className = "border-b";
    
    // ID column
    let tdId = document.createElement("td");
    tdId.className = "py-2 px-4";
    tdId.textContent = record.id;
    row.appendChild(tdId);
    
    // Name column
    let tdName = document.createElement("td");
    tdName.className = "py-2 px-4";
    tdName.textContent = record.name;
    row.appendChild(tdName);
    
    if (entity === "aircraft") {
      let tdAirline = document.createElement("td");
      tdAirline.className = "py-2 px-4";
      tdAirline.textContent = record.airline;
      row.appendChild(tdAirline);
    }
    
    if (entity === "users") {
      let tdEmail = document.createElement("td");
      tdEmail.className = "py-2 px-4";
      tdEmail.textContent = record.email;
      row.appendChild(tdEmail);
    }
    
    // Actions column
    let tdActions = document.createElement("td");
    tdActions.className = "py-2 px-4";
    
    // Edit button
    let editBtn = document.createElement("button");
    editBtn.className = "bg-blue-500 text-white px-2 py-1 mr-2 rounded";
    editBtn.textContent = "Edit";
    editBtn.addEventListener("click", function () {
      renderForm(entity, record);
    });
    tdActions.appendChild(editBtn);
    
    // Delete button
    let deleteBtn = document.createElement("button");
    deleteBtn.className = "bg-red-500 text-white px-2 py-1 rounded";
    deleteBtn.textContent = "Delete";
    deleteBtn.addEventListener("click", function () {
      if (confirm("Are you sure you want to delete this record?")) {
        dataStore[entity] = dataStore[entity].filter(r => r.id !== record.id);
        renderTable(entity);
      }
    });
    tdActions.appendChild(deleteBtn);
    
    row.appendChild(tdActions);
    tbody.appendChild(row);
  });
  
  table.appendChild(tbody);
  contentDiv.appendChild(table);
  
  // Add new record button
  let addBtn = document.createElement("button");
  addBtn.className = "mt-4 bg-green-500 text-white px-4 py-2 rounded";
  addBtn.textContent = "Add New " + entity.charAt(0).toUpperCase() + entity.slice(1);
  addBtn.addEventListener("click", function () {
    renderForm(entity);
  });
  contentDiv.appendChild(addBtn);
}

function renderForm(entity, record = {}) {
  let contentDiv = document.getElementById("content");
  contentDiv.innerHTML = "";
  
  let isEdit = record && record.id !== undefined;
  let formTitle = document.createElement("h2");
  formTitle.className = "text-2xl font-bold mb-4";
  formTitle.textContent = (isEdit ? "Edit " : "Add New ") + entity.charAt(0).toUpperCase() + entity.slice(1);
  contentDiv.appendChild(formTitle);
  
  let form = document.createElement("form");
  form.className = "space-y-4";
  
  // Name field
  let nameDiv = document.createElement("div");
  let nameLabel = document.createElement("label");
  nameLabel.className = "block text-gray-700";
  nameLabel.textContent = "Name:";
  nameDiv.appendChild(nameLabel);
  let nameInput = document.createElement("input");
  nameInput.type = "text";
  nameInput.name = "name";
  nameInput.className = "mt-1 block w-full border-gray-300 rounded";
  nameInput.required = true;
  nameInput.value = record.name || "";
  nameDiv.appendChild(nameInput);
  form.appendChild(nameDiv);
  
  // Additional fields based on entity type
  if (entity === "aircraft") {
    let airlineDiv = document.createElement("div");
    let airlineLabel = document.createElement("label");
    airlineLabel.className = "block text-gray-700";
    airlineLabel.textContent = "Airline:";
    airlineDiv.appendChild(airlineLabel);
    let airlineInput = document.createElement("input");
    airlineInput.type = "text";
    airlineInput.name = "airline";
    airlineInput.className = "mt-1 block w-full border-gray-300 rounded";
    airlineInput.required = true;
    airlineInput.value = record.airline || "";
    airlineDiv.appendChild(airlineInput);
    form.appendChild(airlineDiv);
  }
  
  if (entity === "users") {
    let emailDiv = document.createElement("div");
    let emailLabel = document.createElement("label");
    emailLabel.className = "block text-gray-700";
    emailLabel.textContent = "Email:";
    emailDiv.appendChild(emailLabel);
    let emailInput = document.createElement("input");
    emailInput.type = "email";
    emailInput.name = "email";
    emailInput.className = "mt-1 block w-full border-gray-300 rounded";
    emailInput.required = true;
    emailInput.value = record.email || "";
    emailDiv.appendChild(emailInput);
    form.appendChild(emailDiv);
  }
  
  // Submit button
  let submitBtn = document.createElement("button");
  submitBtn.type = "submit";
  submitBtn.className = "bg-blue-500 text-white px-4 py-2 rounded";
  submitBtn.textContent = isEdit ? "Update" : "Add";
  form.appendChild(submitBtn);
  
  form.addEventListener("submit", function (e) {
    e.preventDefault();
    let formData = new FormData(form);
    if (isEdit) {
      // Update existing record
      dataStore[entity] = dataStore[entity].map(r => {
        if (r.id === record.id) {
          let updated = { id: r.id, name: formData.get("name") };
          if (entity === "aircraft") {
            updated.airline = formData.get("airline");
          }
          if (entity === "users") {
            updated.email = formData.get("email");
          }
          return updated;
        }
        return r;
      });
    } else {
      // Create new record
      let newId = dataStore[entity].reduce((max, r) => Math.max(max, r.id), 0) + 1;
      let newRecord = { id: newId, name: formData.get("name") };
      if (entity === "aircraft") {
        newRecord.airline = formData.get("airline");
      }
      if (entity === "users") {
        newRecord.email = formData.get("email");
      }
      dataStore[entity].push(newRecord);
    }
    // Return to table view after submission
    renderTable(entity);
  });
  
  contentDiv.appendChild(form);
}