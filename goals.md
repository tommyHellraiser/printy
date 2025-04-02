# Goals
## This crate
- This crate should be an interface to abstract the reading of `gcode` for the printer firmware
- Receive a `.gcode` file and read it from start to finish
- Parse each line of the example `gcode` file, ignoring the comments and all that good stuff
- Consider units are set in mms or inches, according to the G20 or G21 codes.
- Bed dimensions should be set in printer setup, and should not allow any printing without configuring them
- Allow the construction and usage of an instance of a printer reader, or something like that. Maybe call it GcodeReader
- Work as a helper to drive the motors, using some kind of transformation between the raw parameters and the output to the motor driver, considering that the configuration and calibration of the motors will be executed by the printer firmware. Maybe this crate could use another one (or a module for that matter) that actually drives the motors and converts the needed input into the calibrated and transformed output

## Printer (scope outside this crate)
- Set up printer with the touchscreen
- Calibrate servos speed, and available print area using the sensors (this logic should go on the printer side, but this crate should receive the config)
- Touch screen controls should allow the setting of an absolute origin point, and limits for the printing area, being able to adapt to any bed configuration and any motor
- To achieve the previous point, user will be able to move the extruder to an origin point, the printer should use the limit sensors to determine its relative location, and then user should set the horizontal and vertical limits, and again the printer should usa the limit sensors to determine its relative position to the end of the print area. This relative positioning will be useful in case the motors move out of place when the printer is off or not reading inputs. This way, we can move the stepper motor to the origin's closest limit and to the edge's closest limit to determine its new start and and points.
- This previous calibration should be performed every time the printer turns on, to ensure consistent positioning across all prints and on cycles