#[cfg(test)]
mod test {
    use crate::parser::gcode::parse::{divide_into_instructions, parse_line};

    #[test]
    fn line_into_instructions_ok_with_comment() {
        let move_instruction = "G1 X116.259 Y130.177 E0.04011 ; skirt";
        let instructions = divide_into_instructions(move_instruction);

        let expected = vec!["G1", "X116.259", "Y130.177", "E0.04011"];
        assert_eq!(instructions, expected)
    }

    #[test]
    fn line_into_instrutcitons_ok_no_comment() {
        let move_instruction = "G1 F1200.000";
        let instructions = divide_into_instructions(move_instruction);

        let expected = vec!["G1", "F1200.000"];
        assert_eq!(instructions, expected)
    }

    #[test]
    fn line_into_instructions_ok_all_comment() {
        let all_comment = "; This is a commented line, no instructions should be processed here";
        let instructions = divide_into_instructions(all_comment);

        let expected: Vec<&str> = vec![];
        assert_eq!(instructions, expected)
    }

    #[test]
    fn line_into_instructions_ok_empty_line() {
        let empty_line = "";
        let instructions = divide_into_instructions(empty_line);

        let expected: Vec<&str> = vec![];
        assert_eq!(instructions, expected)
    }

    #[test]
    fn line_into_instructions_ok_space() {
        let space_line = " ";
        let instructions = divide_into_instructions(space_line);

        let expected: Vec<&str> = vec![];
        assert_eq!(instructions, expected)
    }

    #[test]
    fn parse_full_line_ok() {
        let line = "G1 X109.383 Y119.062 E0.00431 ; perimeter";
        let result = parse_line(line, 69);

        assert!(result.is_ok())
    }

    #[test]
    fn parse_full_line_err() {
        let line = "GA1 X109.383 Y119.062 E0.00431 ; perimeter";
        let result = parse_line(line, 420);

        assert!(result.is_err())
    }
}
