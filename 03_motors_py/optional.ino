// In setup():
Serial.begin(115200);  // Faster baud rate

// In loop():
if (millis() - last_sample >= SAMPLE_INTERVAL_MS) {
  last_sample = millis();
  
  float distance = getDistance();
  digitalWrite(LED_BUILTIN, distance < 20);
  
  // Send binary data
  Serial.write('D');
  Serial.write((byte*)&distance, sizeof(distance));
}
