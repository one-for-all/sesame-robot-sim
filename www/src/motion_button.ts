import { getSimulator } from "./sim";

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
  { payload: "rn tl", label: "Turn Left" },
  { payload: "rn tr", label: "Turn Right" },
  { payload: "rn rs", label: "Rest" },
  { payload: "rn st", label: "Stand" },
  { payload: "rn wv", label: "Wave" },
  { payload: "rn dn", label: "Dance" },
  { payload: "rn sw", label: "Swim" },
  { payload: "rn pt", label: "Point" },
  { payload: "rn pu", label: "Pushup" },
  { payload: "rn bw", label: "Bow" },
  { payload: "rn ct", label: "Cute" },
  { payload: "rn fk", label: "Freaky" },
  { payload: "rn wm", label: "Worm" },
  { payload: "rn sk", label: "Shake" },
  { payload: "rn sg", label: "Shrug" },
  { payload: "rn dd", label: "Dead" },
  { payload: "rn cb", label: "Crab" },
];

for (const { payload, label } of motions) {
  createMotionButton(payload, label);
}
