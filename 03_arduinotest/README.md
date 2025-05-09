1. Hardware Setup (HC-SR04 + Arduino)
Wiring Guide
HC-SR04 Pin	Arduino Pin
VCC	        5V
Trig	      Digital 9
Echo	      Digital 10
GND	        GND

Note: If your sensor has a different pinout (e.g., 4-pin vs. 3-pin), check its datasheet.




2. Arduino Code (With Distance Measurement)

02_serial.ino reads the distance and sends it over Serial every second while blinking the built-in LED.
cpp


3. How It Works

    getDistance() function:

        Sends a 10µs ultrasonic pulse from the Trig pin.

        Measures the time taken for the echo to return (pulseIn()).

        Converts time to distance (in cm) using the speed of sound.

    Main Loop:

        Blinks the built-in LED every 1 second (500ms ON, 500ms OFF).

        Prints the latest distance reading over Serial when the LED turns ON.

4. Expected Serial Output

Distance: 15.34 cm  
Distance: 20.12 cm  
Distance: 12.50 cm  
... (updates every 1 second)

5. Troubleshooting

🔹 No readings?

    Check wiring (especially Trig and Echo pins).

    Ensure the sensor has a clear path (no obstacles too close).

🔹 Incorrect readings?

    Avoid placing the sensor too close to objects (<2cm can be unreliable).

    Add a small delay (e.g., delay(100)) between readings if needed.

6. Extra Improvements (Optional)

    Smoothing readings: Use an average of multiple measurements.

    Threshold alerts: Trigger an action if distance is below a certain value.

