import gpiod
import time
LED_PIN = 4
chip = gpiod.Chip('gpiochip0')
led_line = chip.get_line(LED_PIN)
led_line.request(consumer="LED", type=gpiod.LINE_REQ_DIR_OUT)
try:
   while True:
       led_line.set_value(1)
       time.sleep(1)
       led_line.set_value(0)
       time.sleep(1)
finally:
   led_line.release()
