document.addEventListener('DOMContentLoaded', function () {
  const form = document.querySelector('form');

  form.addEventListener('submit', async function (event) {
    event.preventDefault();

    const username = document.querySelector('input[name="username"]').value;
    const password = document.querySelector('input[name="password"]').value;

    try {
      const response = await fetch('/api/signup', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          username: username,
          password: password
        })
      });

      const data = await response.json();
      console.log(data);
      if (response.ok) {
        // Success - redirect to login page
        window.location.href = 'login.html';
      } else {
        // Handle error
        console.error('Signup failed:', data);
        alert('Signup failed: ' + data.error);
      }

    } catch (error) {
      console.error('Error:', error);
      alert('Network error occurred');
    }
  });
});