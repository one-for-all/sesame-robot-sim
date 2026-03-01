#include <AccelStepper.h>

#define dirPin 2  // Direction
#define stepPin 3 // Step

AccelStepper stepper(AccelStepper::DRIVER, stepPin, dirPin, 0, 0, false);

#define stepsPerRevolution (200 * 4) // 200 steps per revolution * 4 microsteps

void setup() {
  stepper.setMaxSpeed(2000);
  stepper.setAcceleration(500);

  stepper.setEnablePin(5);
  stepper.setPinsInverted(false, false, true);
  stepper.enableOutputs();

  stepper.setSpeed(1000.0);

  // To read current angle in radians
  float theta = (float)stepper.currentPosition() / stepsPerRevolution * TWO_PI;

  // To read current speed in radians/second
  float theta_dot = (float)stepper.speed() / stepsPerRevolution * TWO_PI;
}

void loop() {
  stepper.runSpeed();
}
