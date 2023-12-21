use crate::file::SourceCursor;

pub const TAB_WIDTH: usize = 4;

pub fn count_spaces(cursor: &mut SourceCursor) -> usize {
    let mut count = 0;
    loop {
        if cursor.current() == ' ' {
            count += 1;
            cursor.advance();
        } else if cursor.current() == '\t' {
            count += TAB_WIDTH;
            cursor.advance();
        } else {
            break;
        }
    }
    count
}

pub fn read_newline(cursor: &mut SourceCursor) -> bool {
    if cursor.current() == '\n' {
        cursor.advance();
        true
    } else if cursor.current() == '\r' && cursor.next() == '\n' {
        cursor.advance();
        cursor.advance();
        true
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use crate::{ file::SourceCursor, tokens::spacing::read_newline };
    use super::count_spaces;
    #[test]
    fn test_regular_spaces() {
        let mut cursor = SourceCursor::new("    ");
        let count = count_spaces(&mut cursor);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_tab() {
        let mut cursor = SourceCursor::new("\t");
        let count = count_spaces(&mut cursor);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_mixed() {
        let mut cursor = SourceCursor::new(" \t");
        let count = count_spaces(&mut cursor);
        assert_eq!(count, 5);
    }

    #[test]
    fn test_newlines() {
        let mut cursor = SourceCursor::new("\n\r\nHello World!");
        assert_eq!(read_newline(&mut cursor), true);
        cursor.advance();
        assert_eq!(read_newline(&mut cursor), true);
        cursor.advance();
        assert_eq!(read_newline(&mut cursor), false);
    }
}
