use std::{fs::File, io::BufRead};

use crate::error::{Error, PrintResult};

use super::parse::parse_line;

/// Validates the full file gathering all present errors in the gcode file. Returns said set of errors if any
/// Returns error if fails to handle any line of the file
pub fn validate_file(file: &File) -> PrintResult<Vec<Error>> {
    let reader = std::io::BufReader::new(file);
    let mut error_list = vec![];

    for (line_number, line_result) in reader.lines().enumerate() {
        let line = line_result.map_err(Error::InputOutputError)?;
        //  Send line + 1 because the enumerate method starts at 0
        if let Err(error) = parse_line(&line, line_number + 1) {
            error_list.push(error);
        }
    }

    Ok(error_list)
}

#[cfg(test)]
mod test {
    use crate::error::Error;

    use super::validate_file;

    #[test]
    fn load_small_file_ok() {
        let file = std::fs::File::open("small_example.gcode").unwrap();
        let errors = validate_file(&file).unwrap();

        assert!(errors.is_empty())
    }

    #[test]
    fn load_full_example_file() {
        let file = std::fs::File::open("benchy_example.gcode").unwrap();
        let errors = validate_file(&file).unwrap();

        assert!(errors.is_empty())
    }

    #[test]
    fn load_exapmle_file_error() {
        let file = std::fs::File::open("small_example_error.gcode").unwrap();
        let result = validate_file(&file).unwrap();

        //  Validate there was some error in line 22 of the example file
        assert!(matches!(
            result[0],
            Error::InvalidCommandInLine(_, Some(22))
        ))
    }
}
