import gpiod
import time
import serial
import serial.tools.list_ports

# needs install python3-serial

# GPIO setup
LED_PIN = 4
chip = gpiod.Chip('gpiochip0')
led_line = chip.get_line(LED_PIN)
led_line.request(consumer="LED", type=gpiod.LINE_REQ_DIR_OUT)

def find_arduino_port():
    """Identify Arduino by checking common ports and querying the device"""
    arduino_ports = [
        p.device
        for p in serial.tools.list_ports.comports()
        if 'arduino' in p.description.lower() or 'ACM' in p.device or 'USB' in p.device
    ]
    
    if not arduino_ports:
        raise IOError("No Arduino found - check your connection")
    if len(arduino_ports) > 1:
        print("Multiple devices found - using first one")
    
    return arduino_ports[0]

try:
    # Auto-detect Arduino port
    arduino_port = find_arduino_port()
    print(f"Found Arduino at {arduino_port}")
    
    ser = serial.Serial(arduino_port, 9600, timeout=1)
    ser.flush()

    while True:
        if ser.in_waiting > 0:
            line = ser.readline().decode('utf-8').rstrip()
            
            if line.startswith("Distance: "):
                try:
                    distance = float(line.split()[1])
                    print(f"Current distance: {distance} cm")
                    
                    # Control LED
                    led_line.set_value(1 if distance < 20 else 0)
                    
                except (IndexError, ValueError) as e:
                    print(f"Error parsing distance: {e}")
                    
        time.sleep(0.1)

except Exception as e:
    print(f"Error: {e}")

finally:
    led_line.release()
    if 'ser' in locals():
        ser.close()
