function editBook(bookDataJson) {
  localStorage.setItem("editBookData", bookDataJson);
  window.location.href = "edit_book.html";
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
      bookDiv.style.padding = "10px";
      bookDiv.style.marginBottom = "10px";

      bookDiv.innerHTML = `
        <span style="font-size:20px;"><strong>${book.title}</strong><br></span>
        Author: ${book.author}<br><br>
        Year: ${book.publication_year}&nbsp;&nbsp;
        Genre: ${book.genre}&nbsp;&nbsp;
        ISBN: ${book.isbn}<br><br>
        Copies: ${book.number_of_copies}<br>
        <span style="color: ${book.available ? 'green' : 'red'};">
        ${book.available ? 'Available' : 'Checked Out'}
        </span><br><br>
        <button onclick="editBook('${encodeURIComponent(JSON.stringify(book))}')">Edit</button>
      `;

      container.appendChild(bookDiv);
    }
  } catch (error) {
    container.textContent = "Error loading books.";
    console.error(error);
  }
});
