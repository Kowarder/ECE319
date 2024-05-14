Topic 2. Application of Bluetooth SensorTag to Smart Homes:
Assume the SensorTag is integrated into the collar of a pet and it is used to sense the light, temperature,
and humidity of the environment. It also has an IMU sensor to estimate the motion of the pet. This topic
will design an IoT solution for a smart home for taking care of the pet when the owner is away from
the home. The desired IoT solution should achieve three of the following tasks:
2.a. Use the SensorTag to keep track of the temperature, illuminance, humidity, and pressure of the
room every five minutes (seconds) and average them hourly (per minute). Send the data to the owner’s
cell phone (see Item 2.d below) then average them in the app, or upload the averaged data to the cloud.
2.b. Detect that the room is too dark when the illuminance is lower than 250 lux. Then automatically
turn on the light by either controlling a relay or generating a PWM output to dim or brighten the light.
Meanwhile, notify the user when the light is being turned on or off and the user must have a choice to
overwrite the decision by pressing a button on the IoT board or sending a command remotely;
2.c. Use the accelerometer in the SensorTag to detect the motion of the pet. If the motion of the pet
exceeds a threshold, turn a relay on to simulate opening a gate or a food box.
2.d. Install the Senortag App on your cell phone and play with the SensorTag and the app:
https://www.ti.com/lit/ml/swru410a/swru410a.pdf?ts=1637556285634&ref_url=https%253A%252F
%252Fwww.ti.com%252Ftool%252FCC2650STK.
2.e. Read some of the technical articles or application notes about the SensorTag and describe what you
learn from these articles: https://www.ti.com/tool/CC2650STK.
