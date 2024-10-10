// Function to fetch and display users
async function fetchUsers() {
    try {
        const response = await fetch('/users');
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const users = await response.json();
        renderUsers(users);
    } catch (error) {
        console.error('Error fetching users:', error);
        document.getElementById('userList').innerHTML = '<p>Error loading users.</p>';
    }
}

// Function to render users in the DOM
function renderUsers(users) {
    const userList = document.getElementById('userList');
    userList.innerHTML = '';
    users.forEach(user => {
        const userElement = document.createElement('div');
        userElement.className = 'user-item';
        userElement.innerHTML = `
            <span class="user-name">${user.name}</span>
            <span class="user-email">${user.email}</span>
            <button class="edit-btn" onclick="editUser('${user.id}', this)">Edit</button>
            <button class="delete-btn" onclick="confirmDelete('${user.id}')">Delete</button>
        `;
        userList.appendChild(userElement);
    });
}

// Function to create a new user
async function createUser(event) {
    event.preventDefault();
    const name = document.getElementById('newUserName').value;
    const email = document.getElementById('newUserEmail').value;
    try {
        const response = await fetch('/users', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ name, email }),
        });
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        fetchUsers(); // Refresh the user list
        document.getElementById('createUserForm').reset();
    } catch (error) {
        console.error('Error creating user:', error);
    }
}

// Function to edit a user
function editUser(userId, editButton) {
    const userElement = editButton.parentElement;
    const nameSpan = userElement.querySelector('.user-name');
    const emailSpan = userElement.querySelector('.user-email');

    if (editButton.textContent === 'Edit') {
        // Switch to edit mode
        nameSpan.contentEditable = true;
        emailSpan.contentEditable = true;
        nameSpan.focus();
        editButton.textContent = 'Save';
    } else {
        // Save changes
        const newName = nameSpan.textContent;
        const newEmail = emailSpan.textContent;
        updateUser(userId, newName, newEmail);
        nameSpan.contentEditable = false;
        emailSpan.contentEditable = false;
        editButton.textContent = 'Edit';
    }
}

// Function to update a user
async function updateUser(userId, newName, newEmail) {
    try {
        const response = await fetch(`/users/${userId}`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ name: newName, email: newEmail }),
        });
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        // Optionally, you can fetch users again to refresh the list
        // fetchUsers();
    } catch (error) {
        console.error('Error updating user:', error);
    }
}

// Function to confirm deletion
function confirmDelete(userId) {
    if (confirm('Are you sure you want to delete this user?')) {
        deleteUser(userId);
    }
}

// Function to delete a user
async function deleteUser(userId) {
    try {
        const response = await fetch(`/users/${userId}`, {
            method: 'DELETE',
        });
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        fetchUsers(); // Refresh the user list
    } catch (error) {
        console.error('Error deleting user:', error);
    }
}

// Initial fetch of users
fetchUsers();

// Event listener for create user form
document.getElementById('createUserForm').addEventListener('submit', createUser);