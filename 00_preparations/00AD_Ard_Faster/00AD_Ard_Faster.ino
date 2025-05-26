#define TRIG_PIN 9
#define ECHO_PIN 10
#define SAMPLE_INTERVAL_MS 50  // ~20Hz sampling rate (HC-SR04 min ~16ms)

void setup() {
  pinMode(LED_BUILTIN, OUTPUT);
  pinMode(TRIG_PIN, OUTPUT);
  pinMode(ECHO_PIN, INPUT);
  Serial.begin(115200);  // Faster baud rate
}

void loop() {
  static uint32_t last_sample = 0;
  uint32_t now = millis();
  
  if (now - last_sample >= SAMPLE_INTERVAL_MS) {
    last_sample = now;
    
    float distance = getDistance();
    digitalWrite(LED_BUILTIN, distance < 20);
    
    // Compact binary protocol: 'D' header + 4-byte float
    // Serial.write('D');
    // Serial.write((byte*)&distance, sizeof(distance));
    Serial.print(" [Debug: ");
    Serial.print(distance);
    Serial.println(" cm]");
  }
  
  // Add future sensor reads here with similar timing control
}

float getDistance() {
  digitalWrite(TRIG_PIN, LOW);
  delayMicroseconds(2);
  digitalWrite(TRIG_PIN, HIGH);
  delayMicroseconds(10);
  digitalWrite(TRIG_PIN, LOW);
  
  long duration = pulseIn(ECHO_PIN, HIGH, 30000); // 30ms timeout (~5m max range)
  return duration * 0.0343 / 2;
}
