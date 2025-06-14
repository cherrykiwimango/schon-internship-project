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

      // Create the borrow button
      const borrowButton = document.createElement("button");
      borrowButton.textContent = "Borrow Book";
      borrowButton.style.marginTop = "10px";
      borrowButton.disabled = book.number_of_copies < 1;

      // Borrow click handler
      borrowButton.addEventListener("click", async () => {
        const userId = localStorage.getItem("userId");

        if (!userId) {
          alert("User not logged in.");
          return;
        }

        try {
          const res = await fetch("/api/borrow", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({
              user_id: parseInt(userId),
              book_id: book.id,
            }),
          });

          const result = await res.json();

          if (res.ok) {
            alert(result.message || "Book borrowed successfully.");
            location.reload();
          } else {
            alert(result.message || "Could not borrow book.");
          }
        } catch (err) {
          console.error("Error borrowing book:", err);
          alert("Something went wrong.");
        }
      });


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
      `;

      bookDiv.appendChild(borrowButton);
      container.appendChild(bookDiv);
    }
  } catch (error) {
    container.textContent = "Error loading books.";
    console.error(error);
  }
});