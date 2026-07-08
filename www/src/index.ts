import { createSesame } from "sesame";
import { Simulator } from "gorilla-physics-ui";
import "./motion_button";
import { isPhoneUA } from "chimpanzee-ui";

if (isPhoneUA()) {
  document.getElementById("editorContainer").classList.add("hidden");
}

import("sesame").then((furuta) => {
  createSesame().then((state) => {
    let interfaceSimulator = null;
    let simulator = new Simulator(interfaceSimulator);

    simulator.addHybrid(state);
    simulator.updateHybrid();

    let cameraPosition = {
      eye: { x: -0.5, y: 0.0, z: 0.1 },
      target: { x: 0.0, y: 0, z: 0 },
    };
    simulator.graphics.lookAt(cameraPosition);

    simulator.run(70, 0); // 10

    setSimulator(simulator);

    setInterval(() => {
      const realtimeRatio = document.getElementById("realtimeRatio");
      realtimeRatio.innerHTML =
        "realtime rate: " + simulator.realtimeRatio.toFixed(2);
    }, 500);

    const loadingUI = document.getElementById("loading");
    if (loadingUI) {
      loadingUI.remove();
    }
  });
});

let _simulator: Simulator | null = null;

function setSimulator(sim: Simulator) {
  _simulator = sim;
}

export function getSimulator(): Simulator | null {
  return _simulator;
}
