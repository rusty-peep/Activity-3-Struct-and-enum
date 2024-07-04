# Activity-3

# Library Management System in Rust

This project is a simple library management system written in Rust. The system tracks books and their statuses, allowing users to issue, reserve, and return books. Each book has an ID, author ID, ISBN number, and a status that indicates whether it is available, issued, or reserved.

## Overview

The library management system is designed using Rust's powerful enums and structs. Each book's status is managed through the `BookStatus` enum, which can be `Available`, `Issued`, or `Reserved`. The `Book` struct holds the book's details, including its status.

## Enums

### BookStatus

The `BookStatus` enum represents the current status of a book:

- **Available**: The book is available to be issued or reserved.
- **Issued(u8, i32)**: The book is currently issued to a user for a certain number of days at a certain timestamp.
- **Reserved**: The book is reserved by a user.

## Structs

### Book

The `Book` struct represents a book in the library. It includes the following fields:

- **id**: A unique identifier for the book, generated randomly using the `get_book_id` function.
- **author**: The ID of the author of the book.
- **isbn**: The ISBN number of the book.
- **status**: The current status of the book, which is an instance of the `BookStatus` enum.

## Functions

### Book Methods

- **new**: Creates a new book with the status `Available`. The book ID is generated randomly.
- **issue_book**: Issues the book to a user for a specified number of days. The book's status is updated to `Issued`.
- **reserve_book**: Reserves the book for a user. The book's status is updated to `Reserved`.
- **return_book**: Returns the book by the user. The book's status is updated to `Available`.
- **print_book_info**: Prints the details of the book, including its status.

### Utility Functions

- **get_book_id**: Generates a random book ID.
- **get_current_timestamp**: Gets the current timestamp.