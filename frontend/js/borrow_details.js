document.addEventListener('DOMContentLoaded', async () => {
  const container = document.getElementById("books-container");
  const user_id = localStorage.getItem('userId');

  if (!user_id) {
    alert("User not logged in.");
    window.location.href = "login.html";
    return;
  }

  try {
    const response = await fetch(`/api/borrow/${user_id}`);
    if (!response.ok) {
      throw new Error("Failed to fetch books");
    }

    const borrowed_books = await response.json();

    if (borrowed_books.length === 0) {
      container.textContent = "No books borrowed";
      return;
    }

    for (const book of borrowed_books) {

      const dueDate = new Date(book.due_date);
      const now = new Date();
      const isOverdue = now > dueDate;
      const statusText = isOverdue ? "Overdue" : "Within Return Period";

      const bookDiv = document.createElement("div");
      bookDiv.style.border = "1px solid #ccc";
      bookDiv.style.padding = "10px";
      bookDiv.style.marginBottom = "10px";

      bookDiv.innerHTML = `
        <span style="font-size:20px;"><strong>${book.book.title}</strong><br></span>
        Author: ${book.book.author}<br><br>
        Year: ${book.book.publication_year}&nbsp;&nbsp;
        Genre: ${book.book.genre}&nbsp;&nbsp;
        ISBN: ${book.book.isbn}<br><br>
        Due Date: ${book.due_date}<br>
        <span style="color: ${isOverdue ? 'red' : 'green'}; font-weight: bold;">
          ${statusText}
        </span><br><br>
        <button class="return-btn" data-borrowed-id="${book.borrowed_id}" data-book-id="${book.book.id}">Return</button>
      `;

      container.appendChild(bookDiv);
    }

    // Optional: Add event listeners for return buttons
    document.querySelectorAll('.return-btn').forEach(button => {
      button.addEventListener('click', async () => {
        const borrowed_id = button.getAttribute('data-borrowed-id');
        const book_id = button.getAttribute('data-book-id');
        try {
          const res = await fetch(`/api/borrow/${borrowed_id}/${book_id}`, { method: 'DELETE' });
          if (res.ok) {
            button.parentElement.remove(); // Remove book card on success
          } else {
            alert("Failed to return book");
          }
        } catch (e) {
          console.error("Error returning book:", e);
        }
      });
    });

  } catch (error) {
    container.textContent = "Error loading books.";
    console.error(error);
  }
});
