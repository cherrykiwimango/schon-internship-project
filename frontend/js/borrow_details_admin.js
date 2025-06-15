document.addEventListener('DOMContentLoaded', async () => {
  const container = document.getElementById("books-container");

  try {
    const response = await fetch("/api/borrow");
    if (!response.ok) {
      throw new Error("Failed to fetch borrowed book details");
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
      bookDiv.style.padding = "10px 30px";
      bookDiv.style.marginBottom = "10px";
      bookDiv.style.borderRadius = "5px";

      bookDiv.innerHTML = `
        <span style="font-size:20px;"><strong>${book.book.title}</strong><br></span>
        Borrowed by: ${book.username}<br><br>
        Author: ${book.book.author}<br><br>
        Year: ${book.book.publication_year}&nbsp;&nbsp;
        Genre: ${book.book.genre}&nbsp;&nbsp;
        ISBN: ${book.book.isbn}<br><br>
        Copies: ${book.book.number_of_copies}<br>
        <span style="color: ${book.book.number_of_copies < 1 ? 'red' : 'green'};">
        ${book.book.number_of_copies < 1 ? 'Checked Out' : 'Available'}
        </span><br><br>
        Due Date: ${book.due_date}<br>
        <span style="color: ${isOverdue ? 'red' : 'green'}; font-weight: bold;">
          ${statusText}
        </span><br><br>
      `;

      container.appendChild(bookDiv);
    }
  } catch (error) {
    container.textContent = "Error loading books.";
    console.error(error);
  }
})