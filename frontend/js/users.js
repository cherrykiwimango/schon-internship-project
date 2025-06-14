document.addEventListener('DOMContentLoaded', async () => {
  const container = document.getElementById("users-list");

  try {
    const response = await fetch("/api/users");
    if (!response.ok) {
      throw new Error("Failed to fetch user details");
    }

    const users = await response.json();

    if (users.length === 0) {
      container.textContent = "No logged in.";
      return;
    }

    for (const user of users) {
      const userDiv = document.createElement("div");
      userDiv.style.border = "1px solid #ddd";
      userDiv.style.padding = "8px";
      userDiv.style.marginBottom = "6px";

      userDiv.innerHTML = `
        <strong>User:</strong> ${user.username}
      `;

      container.appendChild(userDiv);
    }
  } catch (error) {
    container.textContent = "Error loading users.";
    console.error(error);
  }
})