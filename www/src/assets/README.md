# Can You Swing Up and Balance the Pendulum?

(scroll to the bottom to find the link to a working code)

## Components
* Arduino Nano
* 17HS4023 Stepper Motor
* DRV8825 Stepper Motor Driver
* AS5600 Angle Encoder

## Pins
Motor Driver 
  * Dir Pin -> 2
  * Step Pin -> 3
  * Enable Pin -> 5

## Getting Started
### Stepper Motor Usage
```
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
```

### Angle Encoder Usage
Note: it gives reading 0 at the top, and -PI or PI at the bottom.

```
#include <AS5600.h>
#include <Wire.h>

AMS_5600 ams5600;

void setup() {
  while (!ams5600.detectMagnet())
  {
      delay(1000); // Wait for the magnet to be detected
  }
}

void loop() {
  // To read the angle of the pendulum in radians. Upright is zero.
  float alpha = (float)ams5600.getRawAngle() / 4096 * TWO_PI;
}
```

## Physical Parameters
* pendulum mass = 0.0184946 (kilogram)
* length from pendulum center of mass to pendulum rotation axis = 0.0441602 (meter)
* moment of inertia about center of mass in pendulum rotation axis direction = 1.50201e-05 (kg m^2)
* distance from motor axis to pendulum center of mass in pendulum rotation axis direction = 0.059 (meter)

## Working example
[Energy-shaping Swingup + LQR Balancing](https://github.com/one-for-all/furuta-sim/blob/main/arduino/furuta/furuta.ino)
