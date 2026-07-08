import { createSesame } from "sesame";
import { Simulator } from "gorilla-physics-ui";

let _simulator: Simulator | null = null;

export function initSimulator() {
  let interfaceSimulator = null;
  let simulator = new Simulator(interfaceSimulator);

  createSesame().then((state) => {
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
}

function setSimulator(sim: Simulator) {
  _simulator = sim;
}

export function getSimulator(): Simulator | null {
  return _simulator;
}

/// Reset the simulator and controller
export function reset_simulator(ino_bin: Uint8Array, symbols: string) {
  let simulator = getSimulator();
  simulator.hybrid.reset();

  // Initial pose
  let targets = [135, 45, 45, 135, 0, 180, 0, 180];
  for (let i = 0; i < targets.length; i++) {
    simulator.hybrid.set_joint_q(i + 1, targets[i] * (Math.PI / 180)); // skip first floating joint
  }
  simulator.hybrid.reboot_esp32_controller(0, ino_bin, symbols);
}
