a) Allow the user to preset a temperature value in a variable on the Keysight board. Use the B1 and B2 buttons to increase or decrease the value, respectively, by 1°C for every button press
(minimum temperature is 17°C, maximum temperature is 37°C). After setting the temperature, display the current temperature and preset temperature on the LCD of the board.
b) When the temperature in the sensor node exceeds the preset temperature, blink an LED at 2 Hz (use PWM). Allow the user to disarm the alarm using the B3 button when the user is aware of the LED blinking. 
c) When the alarm is not disarmed within 1 minute, set the LED to blink at 10 Hz
d) When the alarm is not disarmed after 2 minutes, turn on a DC fan to cool down the machine and keep blinking the LED at 10 Hz. You can simulate the operation of the DC fan using a relay module. Pressing B3 at any time will disable the relay operation and turn off the LED and restart the process of monitoring the temperature.
