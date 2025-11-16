void setup() {
  pinMode(LED_BUILTIN, OUTPUT);  // Set the built-in LED pin as output
  Serial.begin(9600);           // Start serial communication at 9600 baud
}

void loop() {
  // Turn LED ON and send first message
  digitalWrite(LED_BUILTIN, HIGH);  
  Serial.println("LED is ON");     
  delay(500);  // Wait 0.5 seconds (half of the total cycle)

  // Turn LED OFF and send second message
  digitalWrite(LED_BUILTIN, LOW);   
  Serial.println("LED is OFF");    
  delay(500);  // Wait another 0.5 seconds (total cycle = 1 second)
}
