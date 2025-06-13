document.addEventListener('DOMContentLoaded', () => {
  const form = document.querySelector('form');

  form.addEventListener('submit', async (event) => {
    event.preventDefault();

    const title = document.querySelector('input[name="title"]').value;
    const author = document.querySelector('input[name="author"]').value;
    const isbn = document.querySelector('input[name="isbn"]').value;
    const pub_year = document.querySelector('input[name="year"]').value;
    const genre = document.querySelector('input[name="genre"]').value;
    const num_of_copies = parseInt(document.querySelector('input[name="copies"]').value);

    try {
      const response = await fetch("/api/books", {
        method: "POST",
        headers: {
          "Content-type": "application/json"
        },
        body: JSON.stringify({
          title: title,
          author: author,
          isbn: isbn,
          publication_year: pub_year,
          genre: genre,
          number_of_copies: num_of_copies,
        }),
      });

      const data = await response.json();

      if (response.ok) {

        window.location.href = 'manage_books.html';
      }
      else {
        console.error('Add Book failed:', data);
        alert('Add Book failed: ' + response.status);
      }
    }
    catch (error) {
      console.error('Add book failed: ', error);
      alert('Add book failed: ', data.error);
    }
  });
});