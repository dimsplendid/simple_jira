use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    // use the truncate_ellipse function from the ellipse crate
    // to truncate the text to the given width
    let len = text.len();
    if width <= 3 {
        ".".repeat(width).to_owned()
    } else {
        if len > width {
            text.truncate_ellipse(width - 3).into()
        } else {
            let mut result = text.to_owned();
            result.push_str(&" ".repeat(width - len));
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    } 
}