document.addEventListener('DOMContentLoaded', async () => {
  const bookDataJson = localStorage.getItem("editBookData");

  if (!bookDataJson) {
    alert("No book data found.");
    return;
  }

  const book = JSON.parse(decodeURIComponent(bookDataJson));
  const form = document.querySelector('form');

  document.querySelector('input[name="title"]').value = book.title;
  document.querySelector('input[name="author"]').value = book.author;
  document.querySelector('input[name="isbn"]').value = book.isbn;
  document.querySelector('input[name="year"]').value = book.publication_year;
  document.querySelector('input[name="genre"]').value = book.genre;
  document.querySelector('input[name="copies"]').value = book.number_of_copies;

  form.addEventListener("submit", async (e) => {
    e.preventDefault();
    const updatedBook = {
      title: document.querySelector('input[name="title"]').value,
      author: document.querySelector('input[name="author"]').value,
      isbn: document.querySelector('input[name="isbn"]').value,
      publication_year: parseInt(document.querySelector('input[name="year"]').value),
      genre: document.querySelector('input[name="genre"]').value,
      number_of_copies: parseInt(document.querySelector('input[name="copies"]').value),
    };
    try {
      const response = await fetch(`/api/books/${book.id}`, {
        method: "PATCH",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(updatedBook),
      });

      if (!response.ok) throw new Error("Update failed");

      localStorage.removeItem("editBookData"); // cleanup
      window.location.href = "manage_books.html"; // redirect if needed
    } catch (error) {
      console.error("Update error:", error);
      alert("Failed to update the book.");
    }
  })
})  