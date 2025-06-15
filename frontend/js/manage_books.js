function editBook(bookDataJson) {
  localStorage.setItem("editBookData", bookDataJson);
  window.location.href = "edit_book.html";
}

async function deleteBook(bookId) {
  if (!confirm("Are you sure you want to delete this book?")) return;

  try {
    const response = await fetch(`/api/books/${bookId}`, {
      method: "DELETE",
    });

    if (!response.ok) throw new Error("Failed to delete book");

    // Refresh the page after deletion
    location.reload();
  } catch (error) {
    console.error("Delete failed:", error);
    alert("Failed to delete the book.");
  }
}

document.addEventListener("DOMContentLoaded", async () => {
  const container = document.getElementById("books-container");

  try {
    const response = await fetch("/api/books");
    if (!response.ok) {
      throw new Error("Failed to fetch books");
    }

    const books = await response.json();

    if (books.length === 0) {
      container.textContent = "No books available.";
      return;
    }

    for (const book of books) {
      const bookDiv = document.createElement("div");
      bookDiv.style.border = "1px solid #ccc";
      bookDiv.style.padding = "10px 30px";
      bookDiv.style.marginBottom = "10px";
      bookDiv.style.borderRadius = "5px";

      bookDiv.innerHTML = `
        <span style="font-size:20px;"><strong>${book.title}</strong><br></span>
        Author: ${book.author}<br><br>
        Year: ${book.publication_year}&nbsp;&nbsp;
        Genre: ${book.genre}&nbsp;&nbsp;
        ISBN: ${book.isbn}<br><br>
        Copies: ${book.number_of_copies}<br>
        <span style="color: ${book.number_of_copies < 1 ? 'red' : 'green'};">
        ${book.number_of_copies < 1 ? 'Checked Out' : 'Available'}
        </span><br><br>
        <div style="display: flex; gap: 10px;">
          <button onclick="editBook('${encodeURIComponent(JSON.stringify(book))}')">Edit</button>
          <button onclick="deleteBook(${book.id})" style="background-color: rgb(255, 182, 182);">Delete</button>
        </div>
      `;

      container.appendChild(bookDiv);
    }
  } catch (error) {
    container.textContent = "Error loading books.";
    console.error(error);
  }
});
