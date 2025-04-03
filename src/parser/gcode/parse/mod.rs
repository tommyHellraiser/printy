#[cfg(test)]
mod tests;

use super::commands::{G1Move, GcodeExecutable};
use crate::error::Error;
use crate::error::PrintResult;
use crate::types::LineNumberType;

/// Reads the contents of a line and returns the command in generic format
pub(super) fn parse_line(
    line: &str,
    line_number: LineNumberType,
) -> PrintResult<Option<impl GcodeExecutable>> {
    //  Extract the instructions from a line
    let instructions = divide_into_instructions(line);

    //  Match the first instruction with the command, subsequent instructions are parameters to the first one
    parse_command(instructions, line_number)
}

/// Takes the contents of a single line and divides it into a set of instructions per line
fn divide_into_instructions(line: &str) -> Vec<&str> {
    if line.is_empty() {
        return vec![];
    }

    //  Fisrt look for a semicolon which indicates a comment. If no comment is present, assign the length of the full line
    let comment_index = line.find(';').unwrap_or(line.len());

    //  Check if comment starts at first position of the line, and return an empty vector without further processing
    if comment_index == 0 {
        return vec![];
    }

    //  Then parse only the instructions ignoring the comment at any point of the line
    line[0..comment_index]
        .split_whitespace()
        .collect::<Vec<&str>>()
}

/// Takes a set of instructions and builds the full command with its parameters
/// Ex: G1 command with the target locations and extrusion amount
fn parse_command(
    instructions: Vec<&str>,
    line_number: LineNumberType,
) -> PrintResult<Option<impl GcodeExecutable>> {
    //  If instructions is empty, it means the line was either a comment or empty
    if instructions.is_empty() {
        return Ok(None);
    }

    //  First we need to extract the raw command, for example, there are subcommands for the M862, noted M862.1, M862.2, etc
    //  We match the base command and the subcommand will be parsed by the command handler
    let subcommand_index = instructions[0].find('.').unwrap_or(instructions[0].len());
    let base_command = &instructions[0][0..subcommand_index];

    //  Match by the first element of the instructions set, it determines the command
    match base_command {
        //  G Commands
        "G0" => Ok(Some(G1Move::default())),
        "G1" => Ok(Some(G1Move::default())),
        "G2" => Ok(Some(G1Move::default())),
        "G3" => Ok(Some(G1Move::default())),
        "G4" => Ok(Some(G1Move::default())),
        "G10" => Ok(Some(G1Move::default())),
        "G11" => Ok(Some(G1Move::default())),
        "G21" => Ok(Some(G1Move::default())),
        "G28" => Ok(Some(G1Move::default())),
        "G29" => Ok(Some(G1Move::default())),
        "G80" => Ok(Some(G1Move::default())),
        "G90" => Ok(Some(G1Move::default())),
        "G91" => Ok(Some(G1Move::default())),
        "G92" => Ok(Some(G1Move::default())),

        // M Commands
        "M73" => Ok(Some(G1Move::default())),
        "M82" => Ok(Some(G1Move::default())),
        "M83" => Ok(Some(G1Move::default())),
        "M84" => Ok(Some(G1Move::default())),
        "M104" => Ok(Some(G1Move::default())),
        "M106" => Ok(Some(G1Move::default())),
        "M107" => Ok(Some(G1Move::default())),
        "M109" => Ok(Some(G1Move::default())),
        "M112" => Ok(Some(G1Move::default())),
        "M115" => Ok(Some(G1Move::default())),
        "M140" => Ok(Some(G1Move::default())),
        "M190" => Ok(Some(G1Move::default())),
        "M201" => Ok(Some(G1Move::default())),
        "M203" => Ok(Some(G1Move::default())),
        "M204" => Ok(Some(G1Move::default())),
        "M205" => Ok(Some(G1Move::default())),
        "M221" => Ok(Some(G1Move::default())),
        "M500" => Ok(Some(G1Move::default())),
        "M501" => Ok(Some(G1Move::default())),
        "M502" => Ok(Some(G1Move::default())),
        "M600" => Ok(Some(G1Move::default())),
        "M701" => Ok(Some(G1Move::default())),
        "M702" => Ok(Some(G1Move::default())),
        "M862" => Ok(Some(G1Move::default())),
        "M900" => Ok(Some(G1Move::default())),

        //  Any other command might be either unsupported or wrong
        _ => {
            //  If the error is that a command is invalid, add the line information and return. Otherwise just reroute
            let error = check_unsupported_commands(base_command);
            if let Error::InvalidCommandInLine(command, _) = error {
                Err(Error::InvalidCommandInLine(command, Some(line_number)))
            } else {
                Err(error)
            }
        }
    }
}

/// Check a list of unsupported commands to this moment. These will be later implemented and added to the function `parse_command()`
fn check_unsupported_commands(base_command: &str) -> Error {
    match base_command {
        //  List of unsupported commands to this moment
        "G5" => Error::UnsupportedCommand(base_command.to_string()),
        "G6" => Error::UnsupportedCommand(base_command.to_string()),
        "G12" => Error::UnsupportedCommand(base_command.to_string()),
        "G17" => Error::UnsupportedCommand(base_command.to_string()),
        "G18" => Error::UnsupportedCommand(base_command.to_string()),
        "G19" => Error::UnsupportedCommand(base_command.to_string()),
        "G20" => Error::UnsupportedCommand(base_command.to_string()),
        "G26" => Error::UnsupportedCommand(base_command.to_string()),
        "G27" => Error::UnsupportedCommand(base_command.to_string()),
        "G30" => Error::UnsupportedCommand(base_command.to_string()),
        "G31" => Error::UnsupportedCommand(base_command.to_string()),
        "G32" => Error::UnsupportedCommand(base_command.to_string()),
        "G33" => Error::UnsupportedCommand(base_command.to_string()),
        "G34" => Error::UnsupportedCommand(base_command.to_string()),
        "G35" => Error::UnsupportedCommand(base_command.to_string()),
        "G38" => Error::UnsupportedCommand(base_command.to_string()),
        "G42" => Error::UnsupportedCommand(base_command.to_string()),
        "G53" => Error::UnsupportedCommand(base_command.to_string()),
        "G54" => Error::UnsupportedCommand(base_command.to_string()),
        "G55" => Error::UnsupportedCommand(base_command.to_string()),
        "G56" => Error::UnsupportedCommand(base_command.to_string()),
        "G57" => Error::UnsupportedCommand(base_command.to_string()),
        "G58" => Error::UnsupportedCommand(base_command.to_string()),
        "G59" => Error::UnsupportedCommand(base_command.to_string()),
        "G60" => Error::UnsupportedCommand(base_command.to_string()),
        "G61" => Error::UnsupportedCommand(base_command.to_string()),
        "G76" => Error::UnsupportedCommand(base_command.to_string()),
        "G425" => Error::UnsupportedCommand(base_command.to_string()),
        "M0" => Error::UnsupportedCommand(base_command.to_string()),
        "M1" => Error::UnsupportedCommand(base_command.to_string()),
        "M3" => Error::UnsupportedCommand(base_command.to_string()),
        "M4" => Error::UnsupportedCommand(base_command.to_string()),
        "M5" => Error::UnsupportedCommand(base_command.to_string()),
        "M7" => Error::UnsupportedCommand(base_command.to_string()),
        "M8" => Error::UnsupportedCommand(base_command.to_string()),
        "M9" => Error::UnsupportedCommand(base_command.to_string()),
        "M10" => Error::UnsupportedCommand(base_command.to_string()),
        "M11" => Error::UnsupportedCommand(base_command.to_string()),
        "M16" => Error::UnsupportedCommand(base_command.to_string()),
        "M17" => Error::UnsupportedCommand(base_command.to_string()),
        "M18" => Error::UnsupportedCommand(base_command.to_string()),
        "M20" => Error::UnsupportedCommand(base_command.to_string()),
        "M21" => Error::UnsupportedCommand(base_command.to_string()),
        "M22" => Error::UnsupportedCommand(base_command.to_string()),
        "M23" => Error::UnsupportedCommand(base_command.to_string()),
        "M24" => Error::UnsupportedCommand(base_command.to_string()),
        "M25" => Error::UnsupportedCommand(base_command.to_string()),
        "M26" => Error::UnsupportedCommand(base_command.to_string()),
        "M27" => Error::UnsupportedCommand(base_command.to_string()),
        "M28" => Error::UnsupportedCommand(base_command.to_string()),
        "M29" => Error::UnsupportedCommand(base_command.to_string()),
        "M30" => Error::UnsupportedCommand(base_command.to_string()),
        "M31" => Error::UnsupportedCommand(base_command.to_string()),
        "M32" => Error::UnsupportedCommand(base_command.to_string()),
        "M33" => Error::UnsupportedCommand(base_command.to_string()),
        "M34" => Error::UnsupportedCommand(base_command.to_string()),
        "M42" => Error::UnsupportedCommand(base_command.to_string()),
        "M43" => Error::UnsupportedCommand(base_command.to_string()),
        "M48" => Error::UnsupportedCommand(base_command.to_string()),
        "M75" => Error::UnsupportedCommand(base_command.to_string()),
        "M76" => Error::UnsupportedCommand(base_command.to_string()),
        "M77" => Error::UnsupportedCommand(base_command.to_string()),
        "M78" => Error::UnsupportedCommand(base_command.to_string()),
        "M85" => Error::UnsupportedCommand(base_command.to_string()),
        "M92" => Error::UnsupportedCommand(base_command.to_string()),
        "M100" => Error::UnsupportedCommand(base_command.to_string()),
        "M110" => Error::UnsupportedCommand(base_command.to_string()),
        "M111" => Error::UnsupportedCommand(base_command.to_string()),
        "M114" => Error::UnsupportedCommand(base_command.to_string()),
        "M117" => Error::UnsupportedCommand(base_command.to_string()),
        "M118" => Error::UnsupportedCommand(base_command.to_string()),
        "M119" => Error::UnsupportedCommand(base_command.to_string()),
        "M120" => Error::UnsupportedCommand(base_command.to_string()),
        "M121" => Error::UnsupportedCommand(base_command.to_string()),
        "M122" => Error::UnsupportedCommand(base_command.to_string()),
        "M125" => Error::UnsupportedCommand(base_command.to_string()),
        "M126" => Error::UnsupportedCommand(base_command.to_string()),
        "M127" => Error::UnsupportedCommand(base_command.to_string()),
        "M128" => Error::UnsupportedCommand(base_command.to_string()),
        "M129" => Error::UnsupportedCommand(base_command.to_string()),
        "M141" => Error::UnsupportedCommand(base_command.to_string()),
        "M143" => Error::UnsupportedCommand(base_command.to_string()),
        "M145" => Error::UnsupportedCommand(base_command.to_string()),
        "M149" => Error::UnsupportedCommand(base_command.to_string()),
        "M150" => Error::UnsupportedCommand(base_command.to_string()),
        "M154" => Error::UnsupportedCommand(base_command.to_string()),
        "M155" => Error::UnsupportedCommand(base_command.to_string()),
        "M163" => Error::UnsupportedCommand(base_command.to_string()),
        "M164" => Error::UnsupportedCommand(base_command.to_string()),
        "M165" => Error::UnsupportedCommand(base_command.to_string()),
        "M166" => Error::UnsupportedCommand(base_command.to_string()),
        "M192" => Error::UnsupportedCommand(base_command.to_string()),
        "M193" => Error::UnsupportedCommand(base_command.to_string()),
        "M200" => Error::UnsupportedCommand(base_command.to_string()),
        "M202" => Error::UnsupportedCommand(base_command.to_string()),
        "M206" => Error::UnsupportedCommand(base_command.to_string()),
        "M207" => Error::UnsupportedCommand(base_command.to_string()),
        "M208" => Error::UnsupportedCommand(base_command.to_string()),
        "M209" => Error::UnsupportedCommand(base_command.to_string()),
        "M210" => Error::UnsupportedCommand(base_command.to_string()),
        "M211" => Error::UnsupportedCommand(base_command.to_string()),
        "M212" => Error::UnsupportedCommand(base_command.to_string()),
        "M218" => Error::UnsupportedCommand(base_command.to_string()),
        "M220" => Error::UnsupportedCommand(base_command.to_string()),
        "M226" => Error::UnsupportedCommand(base_command.to_string()),
        "M240" => Error::UnsupportedCommand(base_command.to_string()),
        "M241" => Error::UnsupportedCommand(base_command.to_string()),
        "M245" => Error::UnsupportedCommand(base_command.to_string()),
        "M246" => Error::UnsupportedCommand(base_command.to_string()),
        "M300" => Error::UnsupportedCommand(base_command.to_string()),
        "M301" => Error::UnsupportedCommand(base_command.to_string()),
        "M302" => Error::UnsupportedCommand(base_command.to_string()),
        "M303" => Error::UnsupportedCommand(base_command.to_string()),
        "M304" => Error::UnsupportedCommand(base_command.to_string()),
        "M305" => Error::UnsupportedCommand(base_command.to_string()),
        "M306" => Error::UnsupportedCommand(base_command.to_string()),
        "M307" => Error::UnsupportedCommand(base_command.to_string()),
        "M310" => Error::UnsupportedCommand(base_command.to_string()),
        "M320" => Error::UnsupportedCommand(base_command.to_string()),
        "M321" => Error::UnsupportedCommand(base_command.to_string()),
        "M322" => Error::UnsupportedCommand(base_command.to_string()),
        "M323" => Error::UnsupportedCommand(base_command.to_string()),
        "M340" => Error::UnsupportedCommand(base_command.to_string()),
        "M350" => Error::UnsupportedCommand(base_command.to_string()),
        "M351" => Error::UnsupportedCommand(base_command.to_string()),
        "M355" => Error::UnsupportedCommand(base_command.to_string()),
        "M360" => Error::UnsupportedCommand(base_command.to_string()),
        "M361" => Error::UnsupportedCommand(base_command.to_string()),
        "M362" => Error::UnsupportedCommand(base_command.to_string()),
        "M363" => Error::UnsupportedCommand(base_command.to_string()),
        "M364" => Error::UnsupportedCommand(base_command.to_string()),
        "M365" => Error::UnsupportedCommand(base_command.to_string()),
        "M380" => Error::UnsupportedCommand(base_command.to_string()),
        "M381" => Error::UnsupportedCommand(base_command.to_string()),
        "M400" => Error::UnsupportedCommand(base_command.to_string()),
        "M401" => Error::UnsupportedCommand(base_command.to_string()),
        "M402" => Error::UnsupportedCommand(base_command.to_string()),
        "M404" => Error::UnsupportedCommand(base_command.to_string()),
        "M405" => Error::UnsupportedCommand(base_command.to_string()),
        "M406" => Error::UnsupportedCommand(base_command.to_string()),
        "M407" => Error::UnsupportedCommand(base_command.to_string()),
        "M410" => Error::UnsupportedCommand(base_command.to_string()),
        "M412" => Error::UnsupportedCommand(base_command.to_string()),
        "M420" => Error::UnsupportedCommand(base_command.to_string()),
        "M421" => Error::UnsupportedCommand(base_command.to_string()),
        "M422" => Error::UnsupportedCommand(base_command.to_string()),
        "M425" => Error::UnsupportedCommand(base_command.to_string()),
        "M428" => Error::UnsupportedCommand(base_command.to_string()),
        "M503" => Error::UnsupportedCommand(base_command.to_string()),
        "M540" => Error::UnsupportedCommand(base_command.to_string()),
        "M601" => Error::UnsupportedCommand(base_command.to_string()),
        "M602" => Error::UnsupportedCommand(base_command.to_string()),
        "M603" => Error::UnsupportedCommand(base_command.to_string()),
        "M604" => Error::UnsupportedCommand(base_command.to_string()),
        "M605" => Error::UnsupportedCommand(base_command.to_string()),
        "M665" => Error::UnsupportedCommand(base_command.to_string()),
        "M666" => Error::UnsupportedCommand(base_command.to_string()),
        "M851" => Error::UnsupportedCommand(base_command.to_string()),
        "M852" => Error::UnsupportedCommand(base_command.to_string()),
        "M860" => Error::UnsupportedCommand(base_command.to_string()),
        "M861" => Error::UnsupportedCommand(base_command.to_string()),
        "M863" => Error::UnsupportedCommand(base_command.to_string()),
        "M864" => Error::UnsupportedCommand(base_command.to_string()),
        "M865" => Error::UnsupportedCommand(base_command.to_string()),
        "M866" => Error::UnsupportedCommand(base_command.to_string()),
        "M867" => Error::UnsupportedCommand(base_command.to_string()),
        "M868" => Error::UnsupportedCommand(base_command.to_string()),
        "M869" => Error::UnsupportedCommand(base_command.to_string()),
        "M871" => Error::UnsupportedCommand(base_command.to_string()),
        "M906" => Error::UnsupportedCommand(base_command.to_string()),
        "M907" => Error::UnsupportedCommand(base_command.to_string()),
        "M908" => Error::UnsupportedCommand(base_command.to_string()),
        "M909" => Error::UnsupportedCommand(base_command.to_string()),
        "M910" => Error::UnsupportedCommand(base_command.to_string()),
        "M911" => Error::UnsupportedCommand(base_command.to_string()),
        "M912" => Error::UnsupportedCommand(base_command.to_string()),
        "M913" => Error::UnsupportedCommand(base_command.to_string()),
        "M914" => Error::UnsupportedCommand(base_command.to_string()),
        "M915" => Error::UnsupportedCommand(base_command.to_string()),
        "M916" => Error::UnsupportedCommand(base_command.to_string()),
        "M917" => Error::UnsupportedCommand(base_command.to_string()),
        "M918" => Error::UnsupportedCommand(base_command.to_string()),
        "M920" => Error::UnsupportedCommand(base_command.to_string()),
        "M921" => Error::UnsupportedCommand(base_command.to_string()),
        "M922" => Error::UnsupportedCommand(base_command.to_string()),
        "M923" => Error::UnsupportedCommand(base_command.to_string()),
        "M924" => Error::UnsupportedCommand(base_command.to_string()),
        "M925" => Error::UnsupportedCommand(base_command.to_string()),
        "M926" => Error::UnsupportedCommand(base_command.to_string()),
        "M927" => Error::UnsupportedCommand(base_command.to_string()),
        "M928" => Error::UnsupportedCommand(base_command.to_string()),
        "M929" => Error::UnsupportedCommand(base_command.to_string()),
        "M930" => Error::UnsupportedCommand(base_command.to_string()),
        "M931" => Error::UnsupportedCommand(base_command.to_string()),
        "M932" => Error::UnsupportedCommand(base_command.to_string()),
        "M933" => Error::UnsupportedCommand(base_command.to_string()),
        "M934" => Error::UnsupportedCommand(base_command.to_string()),
        "M935" => Error::UnsupportedCommand(base_command.to_string()),
        "M936" => Error::UnsupportedCommand(base_command.to_string()),
        "M937" => Error::UnsupportedCommand(base_command.to_string()),
        "M938" => Error::UnsupportedCommand(base_command.to_string()),
        "M939" => Error::UnsupportedCommand(base_command.to_string()),
        "M940" => Error::UnsupportedCommand(base_command.to_string()),
        "M941" => Error::UnsupportedCommand(base_command.to_string()),
        "M942" => Error::UnsupportedCommand(base_command.to_string()),
        "M943" => Error::UnsupportedCommand(base_command.to_string()),
        "M944" => Error::UnsupportedCommand(base_command.to_string()),
        "M945" => Error::UnsupportedCommand(base_command.to_string()),
        "M946" => Error::UnsupportedCommand(base_command.to_string()),
        "M947" => Error::UnsupportedCommand(base_command.to_string()),
        "M948" => Error::UnsupportedCommand(base_command.to_string()),
        "M949" => Error::UnsupportedCommand(base_command.to_string()),
        "M950" => Error::UnsupportedCommand(base_command.to_string()),
        "M951" => Error::UnsupportedCommand(base_command.to_string()),
        "M952" => Error::UnsupportedCommand(base_command.to_string()),
        "M953" => Error::UnsupportedCommand(base_command.to_string()),
        "M954" => Error::UnsupportedCommand(base_command.to_string()),
        "M955" => Error::UnsupportedCommand(base_command.to_string()),
        "M956" => Error::UnsupportedCommand(base_command.to_string()),
        "M957" => Error::UnsupportedCommand(base_command.to_string()),
        "M958" => Error::UnsupportedCommand(base_command.to_string()),
        "M959" => Error::UnsupportedCommand(base_command.to_string()),
        "M960" => Error::UnsupportedCommand(base_command.to_string()),
        "M961" => Error::UnsupportedCommand(base_command.to_string()),
        "M962" => Error::UnsupportedCommand(base_command.to_string()),
        "M963" => Error::UnsupportedCommand(base_command.to_string()),
        "M964" => Error::UnsupportedCommand(base_command.to_string()),
        "M965" => Error::UnsupportedCommand(base_command.to_string()),
        "M966" => Error::UnsupportedCommand(base_command.to_string()),
        "M967" => Error::UnsupportedCommand(base_command.to_string()),
        "M968" => Error::UnsupportedCommand(base_command.to_string()),
        "M969" => Error::UnsupportedCommand(base_command.to_string()),
        "M970" => Error::UnsupportedCommand(base_command.to_string()),
        "M971" => Error::UnsupportedCommand(base_command.to_string()),
        "M972" => Error::UnsupportedCommand(base_command.to_string()),
        "M973" => Error::UnsupportedCommand(base_command.to_string()),
        "M974" => Error::UnsupportedCommand(base_command.to_string()),
        "M975" => Error::UnsupportedCommand(base_command.to_string()),
        "M976" => Error::UnsupportedCommand(base_command.to_string()),
        "M977" => Error::UnsupportedCommand(base_command.to_string()),
        "M978" => Error::UnsupportedCommand(base_command.to_string()),
        "M979" => Error::UnsupportedCommand(base_command.to_string()),
        "M980" => Error::UnsupportedCommand(base_command.to_string()),
        "M981" => Error::UnsupportedCommand(base_command.to_string()),
        "M982" => Error::UnsupportedCommand(base_command.to_string()),
        "M983" => Error::UnsupportedCommand(base_command.to_string()),
        "M984" => Error::UnsupportedCommand(base_command.to_string()),
        "M985" => Error::UnsupportedCommand(base_command.to_string()),
        "M986" => Error::UnsupportedCommand(base_command.to_string()),
        "M987" => Error::UnsupportedCommand(base_command.to_string()),
        "M988" => Error::UnsupportedCommand(base_command.to_string()),
        "M989" => Error::UnsupportedCommand(base_command.to_string()),
        "M990" => Error::UnsupportedCommand(base_command.to_string()),
        "M991" => Error::UnsupportedCommand(base_command.to_string()),
        "M992" => Error::UnsupportedCommand(base_command.to_string()),
        "M993" => Error::UnsupportedCommand(base_command.to_string()),
        "M994" => Error::UnsupportedCommand(base_command.to_string()),
        "M995" => Error::UnsupportedCommand(base_command.to_string()),
        "M996" => Error::UnsupportedCommand(base_command.to_string()),
        "M997" => Error::UnsupportedCommand(base_command.to_string()),
        "M998" => Error::UnsupportedCommand(base_command.to_string()),
        "M999" => Error::UnsupportedCommand(base_command.to_string()),
        _ => Error::InvalidCommandInLine(Some(base_command.to_string()), None),
    }
}
