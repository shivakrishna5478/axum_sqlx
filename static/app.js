const userForm = document.getElementById("userForm");
const userList = document.getElementById("userList");

// Fetch and display users on load
document.addEventListener("DOMContentLoaded", fetchUsers);

// Form submission to add a new user
userForm.addEventListener("submit", async (event) => {
    event.preventDefault();

    const name = document.getElementById("name").value;
    const email = document.getElementById("email").value;

    try {
        const response = await fetch("/api/users", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ name, email }),
        });

        if (response.ok) {
            const user = await response.json();
            addUserToUI(user);
        } else {
            alert("Failed to add user.");
        }
    } catch (error) {
        console.error("Error:", error);
    }

    userForm.reset();
});

// Fetch all users
async function fetchUsers() {
    try {
        const response = await fetch("/api/users");

        if (response.ok) {
            const users = await response.json();
            users.forEach(addUserToUI);
        }
    } catch (error) {
        console.error("Error:", error);
    }
}

// Add a user to the UI
function addUserToUI(user) {
    const li = document.createElement("li");
    li.textContent = `ID: ${user.id} | Name: ${user.name} | Email: ${user.email}`;
    userList.appendChild(li);
}
