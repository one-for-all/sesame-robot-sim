#include <Arduino.h>
#include <ESP32Servo.h>
#include "movement-sequences.h"

// Global state for Animation
String currentCommand = "";
int frameDelay = 100;
int walkCycles = 10;

// ======================================================================
// --- CONFIGURATION ---
// ======================================================================
Servo servos[8];
// Sesame Distro Board V2 Pinout
// const int servoPins[8] = {4, 5, 6, 7, 15, 16, 17, 18};

// Sesame Distro Board Pinout
const int servoPins[8] = {15, 2, 23, 19, 4, 16, 17, 18};

// Subtrim values for each servo (offset in degrees)
int8_t servoSubtrim[8] = {0, 0, 0, 0, 0, 0, 0, 0};

// Animation constants
int motorCurrentDelay = 20; // ms delay between motor movements to prevent over-current

// Prototypes
void setServoAngle(uint8_t channel, int angle);
void enterIdle();
void setFaceWithMode(const String& faceName, FaceAnimMode mode);
void delayWithFace(unsigned long ms);
bool pressingCheck(String cmd, int ms);

// ======================================================================
// --- SETUP ---
// ======================================================================

void setup() {
  Serial.begin(115200);
  while (!Serial);

  Serial.println("-----------------------------------");
  Serial.println("   Sesame Robot Simulation   ");
  Serial.println("-----------------------------------");

  // PWM Init
  ESP32PWM::allocateTimer(0);
  ESP32PWM::allocateTimer(1);
  ESP32PWM::allocateTimer(2);
  ESP32PWM::allocateTimer(3);


  for (int i = 0; i < 8; i++) {
    servos[i].setPeriodHertz(50);
    // Map 0-180 to approx 732-2929us
    servos[i].attach(servoPins[i], 732, 2929);
  }
  delay(10);

  Serial.println("----------- Setup Done -----------");
}

void loop() {
  // runStandPose(1);
  runWavePose();
  // runWalkPose();
  // runTurnLeft();
  // runTurnRight();
  // runPushupPose();
  // runWalkBackward();
  // runCrabPose();
  // runDancePose(); // TODO: need to add more collision points for it
}


// ====== HELPERS ======
void setServoAngle(uint8_t channel, int angle) {
  if (channel < 8) {
    int adjustedAngle = constrain(angle + servoSubtrim[channel], 0, 180);
    servos[channel].write(adjustedAngle);
    delayWithFace(motorCurrentDelay);
  }
}

void delayWithFace(unsigned long ms) {
  unsigned long start = millis();
  while (millis() - start < ms) {
    // updateAnimatedFace();
    // server.handleClient();
    // dnsServer.processNextRequest();
    delay(5);
  }
}


void setFaceWithMode(const String& faceName, FaceAnimMode mode) {
  // setFaceMode(mode);
  // setFace(faceName);
}

void enterIdle() {
  // idleActive = true;
  // idleBlinkActive = false;
  // idleBlinkRepeatsLeft = 0;
  // setFaceWithMode("idle", FACE_ANIM_BOOMERANG);
  // scheduleNextIdleBlink(3000, 7000);
}

bool pressingCheck(String cmd, int ms) {
  unsigned long start = millis();
  while (millis() - start < ms) {
    // server.handleClient();
    // dnsServer.processNextRequest();
    // updateAnimatedFace();
    // if (currentCommand != cmd) {
    //   runStandPose(1);
    //   return false;
    // }
    yield();
  }
  return true;
}
