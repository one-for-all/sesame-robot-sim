import { createSesame } from "sesame";
import { Simulator } from "gorilla-physics-ui";

import("sesame").then((furuta) => {
  createSesame().then((state) => {
    let interfaceSimulator = null;
    let simulator = new Simulator(interfaceSimulator);

    simulator.addHybrid(state);
    // simulator.updateHybrid();

    let cameraPosition = {
      eye: { x: 0.3, y: 0.0, z: 0.3 },
      target: { x: 0.0, y: 0, z: 0 },
    };
    simulator.graphics.lookAt(cameraPosition);

    simulator.run(10, 0); // 10

    setSimulator(simulator);
  });
});

let _simulator: Simulator | null = null;

function setSimulator(sim: Simulator) {
  _simulator = sim;
}

export function getSimulator(): Simulator | null {
  return _simulator;
}
