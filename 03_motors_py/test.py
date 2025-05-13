## required python3-serial python3-rpi.gpio
import serial
import serial.tools.list_ports
import RPi.GPIO as GPIO
import time

# GPIO Pins (BCM numbering)
LED_PIN = 4
MOTOR_A_ENABLE = 17
MOTOR_A_IN1 = 27
MOTOR_A_IN2 = 22
MOTOR_B_ENABLE = 23
MOTOR_B_IN1 = 24
MOTOR_B_IN2 = 25

STOP_DISTANCE_CM = 20
SERIAL_TIMEOUT = 0.1  # seconds

class MotorController:
    def __init__(self, enable_pin, in1_pin, in2_pin):
        self.enable_pin = enable_pin
        self.in1_pin = in1_pin
        self.in2_pin = in2_pin
        
        GPIO.setup(enable_pin, GPIO.OUT)
        GPIO.setup(in1_pin, GPIO.OUT)
        GPIO.setup(in2_pin, GPIO.OUT)
        
        self.pwm = GPIO.PWM(enable_pin, 1000)  # 1kHz PWM frequency
        self.pwm.start(0)
    
    def set_speed(self, speed):
        if speed > 0:  # Forward
            GPIO.output(self.in1_pin, GPIO.HIGH)
            GPIO.output(self.in2_pin, GPIO.LOW)
            self.pwm.ChangeDutyCycle(speed)
        elif speed < 0:  # Reverse
            GPIO.output(self.in1_pin, GPIO.LOW)
            GPIO.output(self.in2_pin, GPIO.HIGH)
            self.pwm.ChangeDutyCycle(abs(speed))
        else:  # Stop
            self.pwm.ChangeDutyCycle(0)

def find_arduino_port():
    ports = serial.tools.list_ports.comports()
    for port in ports:
        if 'arduino' in port.description.lower() or 'ACM' in port.device or 'USB' in port.device:
            return port.device
    raise IOError("No Arduino found")

def main():
    # GPIO Setup
    GPIO.setmode(GPIO.BCM)
    GPIO.setup(LED_PIN, GPIO.OUT)
    
    # Motor Setup
    motor_a = MotorController(MOTOR_A_ENABLE, MOTOR_A_IN1, MOTOR_A_IN2)
    motor_b = MotorController(MOTOR_B_ENABLE, MOTOR_B_IN1, MOTOR_B_IN2)
    
    # Start motors at 50% speed forward
    motor_a.set_speed(50)
    motor_b.set_speed(50)
    
    try:
        # Serial Setup
        arduino_port = find_arduino_port()
        print(f"Found Arduino at {arduino_port}")
        ser = serial.Serial(arduino_port, 115200, timeout=SERIAL_TIMEOUT)
        
        while True:
            if ser.in_waiting >= 5:  # 'D' + 4-byte float
                header = ser.read(1)
                if header == b'D':
                    distance_bytes = ser.read(4)
                    distance = float.from_bytes(distance_bytes, byteorder='little')
                    
                    # Control LED and motors
                    if distance < STOP_DISTANCE_CM:
                        GPIO.output(LED_PIN, GPIO.HIGH)
                        motor_a.set_speed(0)
                        motor_b.set_speed(0)
                        print(f"STOPPED - Distance: {distance:.1f}cm")
                    else:
                        GPIO.output(LED_PIN, GPIO.LOW)
                        motor_a.set_speed(50)
                        motor_b.set_speed(50)
                        print(f"MOVING - Distance: {distance:.1f}cm")
                
            time.sleep(0.01)  # Small delay to prevent CPU overload
            
    except KeyboardInterrupt:
        print("Stopping...")
    finally:
        # Cleanup
        motor_a.set_speed(0)
        motor_b.set_speed(0)
        GPIO.cleanup()
        if 'ser' in locals():
            ser.close()

if __name__ == "__main__":
    main()
