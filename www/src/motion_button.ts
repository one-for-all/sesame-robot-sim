import { getSimulator } from "./index";

const container = document.getElementById("movementButtonContainer");

function createMotionButton(payload: string, label: string) {
  if (!container) return;

  const button = document.createElement("button");
  button.className = "movement-button";
  button.textContent = label;

  button.addEventListener("click", () => {
    console.log(`Sending ${label} command:`, JSON.stringify(payload));
    const simulator = getSimulator();
    if (simulator?.hybrid) {
      simulator.hybrid.send_uart(payload + "\n");
    }
  });

  container.appendChild(button);
}

const motions = [
  { payload: "rn wf", label: "Walk" },
  { payload: "rn wb", label: "Backward" },
];

for (const { payload, label } of motions) {
  createMotionButton(payload, label);
}
