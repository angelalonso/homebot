import gpiod
import time
from time import sleep

# GPIO Pins (BCM numbering)
LED_PIN = 4
MOTOR_A_ENABLE = 22
MOTOR_A_IN1 = 17
MOTOR_A_IN2 = 27
MOTOR_B_ENABLE = 25
MOTOR_B_IN1 = 23
MOTOR_B_IN2 = 24

STOP_DISTANCE_CM = 20

class MotorController:
    def __init__(self, enable_pin, in1_pin, in2_pin):
        self.chip = gpiod.Chip('gpiochip0')

        # Setup GPIO lines
        self.enable = self.chip.get_line(enable_pin)
        self.in1 = self.chip.get_line(in1_pin)
        self.in2 = self.chip.get_line(in2_pin)

        # Request lines with output direction
        self.enable.request(consumer="MOTOR_EN", type=gpiod.LINE_REQ_DIR_OUT)
        self.in1.request(consumer="MOTOR_IN1", type=gpiod.LINE_REQ_DIR_OUT)
        self.in2.request(consumer="MOTOR_IN2", type=gpiod.LINE_REQ_DIR_OUT)

        # Initialize to stopped state
        self.set_speed(0)

    def set_speed(self, speed):
        if speed > 0:  # Forward
            self.in1.set_value(1)
            self.in2.set_value(0)
            # Simulate PWM by toggling (crude implementation)
            self.enable.set_value(1)
        elif speed < 0:  # Reverse
            self.in1.set_value(0)
            self.in2.set_value(1)
            # Simulate PWM by toggling
            self.enable.set_value(1)
        else:  # Stop
            self.in1.set_value(0)
            self.in2.set_value(0)
            self.enable.set_value(0)


    def release(self):
        self.enable.release()
        self.in1.release()
        self.in2.release()
        self.chip.close()

def main():
    try:
        # Initialize LED
        led_chip = gpiod.Chip('gpiochip0')
        led_line = led_chip.get_line(LED_PIN)
        led_line.request(consumer="LED", type=gpiod.LINE_REQ_DIR_OUT)
        
        # Initialize Motors
        motor_a = MotorController(MOTOR_A_ENABLE, MOTOR_A_IN1, MOTOR_A_IN2)
        motor_b = MotorController(MOTOR_B_ENABLE, MOTOR_B_IN1, MOTOR_B_IN2)
        
        # Start motors forward at 50% (simulated)
        motor_a.set_speed(50)
        motor_b.set_speed(50)
        
        try:
            while True:
                # Your distance sensor logic would go here
                # For now, just toggle LED for testing
                led_line.set_value(1)
                sleep(0.5)
                led_line.set_value(0)
                sleep(0.5)
                
        except KeyboardInterrupt:
            print("Stopping...")
            
    finally:
        # Cleanup
        motor_a.set_speed(0)
        motor_b.set_speed(0)
        motor_a.release()
        motor_b.release()
        led_line.set_value(0)
        led_line.release()
        led_chip.close()

if __name__ == "__main__":
    main()
