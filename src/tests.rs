#[cfg(test)]
mod tests {
    use crate::{Book, BookStatus};

    use super::*;

    #[test]
    fn test_new_book() {
        let book = Book::new(1, 1234567890);
        assert_eq!(book.author, 1);
        assert_eq!(book.isbn, 1234567890);
        matches!(book.status, BookStatus::Available);
    }

    #[test]
    fn test_issue_book() {
        let book = Book::new(1, 1234567890).issue_book(7);
        matches!(book.status, BookStatus::Issued(days, _) if days == 7);
    }

    #[test]
    fn test_reserve_book() {
        let book = Book::new(1, 1234567890).reserve_book();
        matches!(book.status, BookStatus::Reserved);
    }

    #[test]
    fn test_return_book() {
        let book = Book::new(1, 1234567890).issue_book(7).return_book();
        matches!(book.status, BookStatus::Available);
    }

    #[test]
    fn test_book_status_transitions() {
        let mut book = Book::new(1, 1234567890);
        assert!(matches!(book.status, BookStatus::Available));

        book = book.issue_book(7);
        assert!(matches!(book.status, BookStatus::Issued(_, _)));

        book = book.return_book();
        assert!(matches!(book.status, BookStatus::Available));

        book = book.reserve_book();
        assert!(matches!(book.status, BookStatus::Reserved));
    }
}