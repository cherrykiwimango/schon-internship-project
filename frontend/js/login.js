document.addEventListener('DOMContentLoaded', function () {
  const form = document.querySelector('form');
  document.addEventListener('submit', async function (event) {
    event.preventDefault();

    const username = document.querySelector('input[name="username"]').value;
    const password = document.querySelector('input[name="password"]').value;

    try {
      const response = await fetch('/api/login', {
        method: "POST",
        headers: {
          "Content-type": "application/json"
        },
        body: JSON.stringify({
          username: username,
          password: password
        })
      });
      const data = await response.json();

      console.log(data);
      if (response.ok) {
        const userRole = data.role;
        const userName = data.username;
        const userId = data.userId;
        localStorage.setItem('userRole', userRole);
        localStorage.setItem('userName', userName);
        localStorage.setItem('userId', userId);

        if (userRole == "admin") {
          window.location.href = 'admin_dashboard.html';
        }
        else {
          window.location.href = 'dashboard.html';
        }

      }
      else {
        console.error("Login failed: ", data);
        alert('Login failed: ', data.error);
      }
    }
    catch (error) {
      console.error("Error: ", error);
      alert("Network error occured");
    }
  });
});