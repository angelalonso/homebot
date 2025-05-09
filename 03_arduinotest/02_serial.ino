// Define HC-SR04 pins
const int trigPin = 9;
const int echoPin = 10;

void setup() {
  pinMode(LED_BUILTIN, OUTPUT);  // Built-in LED
  pinMode(trigPin, OUTPUT);      // Trig pin = OUTPUT
  pinMode(echoPin, INPUT);       // Echo pin = INPUT
  Serial.begin(9600);            // Start serial
}

void loop() {
  // Turn LED ON + measure distance
  digitalWrite(LED_BUILTIN, HIGH);
  float distance = getDistance();
  Serial.print("Distance: ");
  Serial.print(distance);
  Serial.println(" cm");
  delay(500);  // Wait 0.5s

  // Turn LED OFF
  digitalWrite(LED_BUILTIN, LOW);
  delay(500);  // Wait another 0.5s (total = 1s)
}

// Helper function to measure distance (in cm)
float getDistance() {
  // Send ultrasonic pulse
  digitalWrite(trigPin, LOW);
  delayMicroseconds(2);
  digitalWrite(trigPin, HIGH);
  delayMicroseconds(10);
  digitalWrite(trigPin, LOW);

  // Measure echo duration (microseconds)
  long duration = pulseIn(echoPin, HIGH);
  // Convert to distance (cm) using speed of sound (343 m/s ≈ 0.0343 cm/µs)
  float distance = duration * 0.0343 / 2;
  return distance;
}

