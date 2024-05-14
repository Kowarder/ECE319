import os
import sys
import time
import mraa
import math
import M1_L5_LCD_Fun as LCD
from bluepy import sensortag
import mraa_beaglbone_pinmap as pm
import Adafruit_BBIO.PWM as PWM

adc_channel = 7

# Load the buttons
buttons = []
button_pins = [pm.B1, pm.B2, pm.B3, pm.B4]
for button in button_pins:
    buttons.append(mraa.Gpio(button))

# Set the relay and relay0
relay0_state = 0
relay_pins = [pm.GP2, pm.GP3]
relays = []

# Assign the relays and buttons
for r in relay_pins:
    relays.append(mraa.Gpio(r))
for button in buttons:
    button.dir(mraa.DIR_IN)
for relay in relays:
    time.sleep(1)
    r = relay.dir(mraa.DIR_OUT)
    relay.write(0)

# Initial the LCD, LED and relay values
LCD.LCD_init()
relay0_state = 0
relay1_state = 0
led_state = 0
LCD.LCD_clear()

# Print the prompt to open the SensorTag
print("\nPreparing to connect...")
print("\nYou might need to press the side button on Sensor Tag within 2 seconds...")
print("\nYou may press Button2 to exit\n")
time.sleep(2.0)

# Initial the SensorTag information
# SensorTag address
tag = sensortag.SensorTag('54:6C:0E:B7:88:03')
# SensorTag HumiditySensor
tag.humidity.enable()
# SensorTag lightmeter
tag.lightmeter.enable()
# SensorTag BarometerSensor
tag.barometer.enable()
# SensorTag AccelerometerSensor
tag.accelerometer.enable()

# List to store the temperature, humidity, pressure and illumination values
temp = []
humi = []
pres = []
illu = []

# Pompt users how to stop the program
LCD.LCD_print("Press B2 to stop")
LCD.LCD_clear()

# Open the LED
PWM.start("P9_16", 100)

# Set a start_time, when the total_time >= 60 upload the temp, humi, pres and illu values to the Google Sheet 
start_time = time.time()
# Start the while loop
while(buttons[1].read() == 1):
    tag.waitForNotifications(1.0)
    
    # Read the data from SensorTag HumiditySensor
    data = tag.humidity.read()
    # Assign the data to temperature and humidity
    temperature = abs(data[0])
    humidity = abs(data[1])
    # Read the data from SensorTag lightmeter
    lux = tag.lightmeter.read()
    # Assign the data to illumination
    illumination = abs(lux)
    # Read the data from SensorTag BarometerSensor
    pressure = tag.barometer.read()
    # Assign the data to pressure
    pressure = abs(pressure[1])
    # Read the current data from SensorTag AccelerometerSensor
    cur_x_accel, cur_y_accel, cur_z_accel = tag.accelerometer.read()
    
    
    
    # Append the values to the list
    temp.append(temperature)
    humi.append(humidity)
    pres.append(pressure)
    illu.append(illumination)
    
    # Print the values to check
    print(temperature, humidity, illumination, pressure, tag.accelerometer.read())
    # Display the values on the LCD
    disp = "{:.1f}".format(temperature) + "C, " + "{:.1f}".format(humidity) + "%RH"
    disp2 = "{:.1f}".format(illumination) + "lux" + "{:.1f}".format(pressure) + "Pa"
    LCD.LCD_print(disp)
    LCD.LCD_print2(disp2)
    time.sleep(5.0)
    
    # When the illuminance in lower than 250 lux, turn on the light (relay to represent light)
    while( illumination < 250):
        LCD.LCD_clear()
        
        # Set the led to 1  
        led_state = 1
        # If led=0, means the light is off
        if (led_state == 0):
            PWM.set_duty_cycle("P9_16", 100)
            LCD.LCD_print("LED Off ")
        # If led=1, means the light is on
        if (led_state == 1):
            PWM.set_duty_cycle("P9_16", 0)
            LCD.LCD_print("LED On  ")
            # If the user want to close the light, press the button button1
            LCD.LCD_print2("Press B1 Turn off")
        # Press button 1 to turn off the light(relay) and break from this while loop
        if (buttons[0].read() == 0):
            led_state = 0
            PWM.set_duty_cycle("P9_16", 100)
            LCD.LCD_clear()
            LCD.LCD_print2("LED Turned off")
            break
        # Press button 2 to exit
        if (buttons[1].read() == 0):
            PWM.set_duty_cycle("P9_16", 100)
            break
        # Record the end time and calculate the total time
        end_time = time.time()
        total_time = end_time - start_time
        # If the total time >= 60, show the avg values on the LCD and upload the data to the Google Sheet
        if (total_time >= 60):
            LCD.LCD_clear()
            # Calculate the average values of temp, humi, illu, pres
            temp_avg = sum(temp)/len(temp)
            humi_avg = sum(humi)/len(humi)
            illu_avg = sum(illu)/len(illu)
            pres_avg = sum(pres)/len(pres)
            # Display on the LCD
            disp = "{:.1f}".format(temp_avg) + "C, " + "{:.1f}".format(humi_avg) + "%RH"
            disp2 = "{:.1f}".format(illu_avg) + "lux" + "{:.1f}".format(pres_avg) + "Pa"
            # Command
            curl_command = "curl -L \"https://script.google.com/macros/s/AKfycbwJ-F4rj52rhcXiI4kElrazpycW7kCmoHpMlJazO56bOMmMw56SawNHu0C5NovPWO1S/exec?Temp={}&Pressure={}&Illumination={}&Humid={}\"".format(temp_avg, pres_avg, illu_avg, humi_avg)
                
            LCD.LCD_print(disp)
            LCD.LCD_print2(disp2)
            print("Average values displayed on the LCD\n")
            # Reset the start time
            start_time = time.time()
            # System command to upload the values
            os.system(curl_command)
        x_accel, y_accel, z_accel = tag.accelerometer.read()
        if (abs(x_accel-cur_x_accel)>5 or  abs(y_accel-cur_y_accel)>5 or abs(z_accel-cur_z_accel)>5):
            print("Your pet has moved!\n")
            relays[0].write(0)
        else:
            relays[0].write(1)
	lux = tag.lightmeter.read()
    	# Assign the data to illumination
    	illumination = abs(lux)

    PWM.set_duty_cycle("P9_16", 100)
    # Same as above
    end_time = time.time()
    total_time = end_time - start_time
    if (total_time >= 60):
        start_time = time.time()
        LCD.LCD_clear()
        temp_avg = sum(temp)/len(temp)
        humi_avg = sum(humi)/len(humi)
        illu_avg = sum(illu)/len(illu)
        pres_avg = sum(pres)/len(pres)
        disp = "{:.1f}".format(temp_avg) + "C, " + "{:.1f}".format(humi_avg) + "%RH"
        disp2 = "{:.1f}".format(illu_avg) + "lux" + "{:.1f}".format(pres_avg) + "Pa"
        curl_command = "curl -L \"https://script.google.com/macros/s/AKfycbwJ-F4rj52rhcXiI4kElrazpycW7kCmoHpMlJazO56bOMmMw56SawNHu0C5NovPWO1S/exec?Temp={}&Pressure={}&Illumination={}&Humid={}\"".format(temp_avg, pres_avg, illu_avg, humi_avg)
        LCD.LCD_print(disp)
        LCD.LCD_print2(disp2)
        print("Average values displayed on the LCD\n")
        os.system(curl_command) 
    # If the accelerometer value has been changed, open the relay to represent the door is open
    x_accel, y_accel, z_accel = tag.accelerometer.read()
    if (abs(x_accel-cur_x_accel) > 5 or abs(y_accel-cur_y_accel) > 5 or abs(z_accel-cur_z_accel) > 5):
        print("Your pet has moved!\n")
        relays[0].write(0)
    else:
        relays[0].write(1)

PWM.stop("P9_16")
PWM.cleanup()
tag.disconnect()
del tag
