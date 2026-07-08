import { reset_simulator, getSimulator } from "./sim";
import { toggleMode } from "chimpanzee-ui";
import default_ino_bin_buffer from "./assets/sesame.ino.bin";
import default_symbols from "./assets/symbols.txt";

const default_ino_bin = new Uint8Array(default_ino_bin_buffer);

document.getElementById("resetButton").addEventListener("click", () => {
  const simulator = getSimulator();
  if (simulator?.hybrid) {
    reset_simulator(default_ino_bin, default_symbols);
  }
});

document.getElementById("toggleModeButton").addEventListener("click", () => {
  toggleMode();
  document.getElementById("projectDialog")?.classList.add("hidden");
});
