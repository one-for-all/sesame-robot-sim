// Add html for keys overlay
const keysDiv = document.createElement("div");
keysDiv.innerHTML = `
  <!-- WASD Overlay -->
  <div class="wasd-overlay">
      <div class="wasd-row">
          <div class="wasd-key" id="key-w">W</div>
      </div>
      <div class="wasd-row">
          <div class="wasd-key" id="key-a">A</div>
          <div class="wasd-key" id="key-s">S</div>
          <div class="wasd-key" id="key-d">D</div>
      </div>
  </div>
  <div class="space-overlay">
      <div class="space-key" id="key-space">SPACE</div>
  </div>
`;
document.body.appendChild(keysDiv);

// Create and dispatch keyboard events
function simulateKeyEvent(key, type) {
  let eventKey = key;
  let eventCode = `Key${key.toUpperCase()}`;
  if (key === "space") {
    eventKey = " ";
    eventCode = "Space";
  }
  const event = new KeyboardEvent(type, {
    key: eventKey,
    code: eventCode,
    bubbles: true,
    cancelable: true,
    composed: true,
  });
  document.dispatchEvent(event);
}

// Add mouse/touch event handlers for each key
["w", "a", "s", "d", "space"].forEach((key) => {
  const element = document.getElementById(`key-${key}`);

  // Mouse events
  element.addEventListener("mousedown", () => {
    element.classList.add("active");
    simulateKeyEvent(key, "keydown");
  });

  element.addEventListener("mouseup", () => {
    element.classList.remove("active");
    simulateKeyEvent(key, "keyup");
  });

  element.addEventListener("mouseleave", () => {
    if (element.classList.contains("active")) {
      element.classList.remove("active");
      simulateKeyEvent(key, "keyup");
    }
  });

  // Touch events
  element.addEventListener("touchstart", (e) => {
    e.preventDefault();
    element.classList.add("active");
    simulateKeyEvent(key, "keydown");
  });

  element.addEventListener("touchend", (e) => {
    e.preventDefault();
    element.classList.remove("active");
    simulateKeyEvent(key, "keyup");
  });
});

// Add key press highlighting
document.addEventListener("keydown", (e) => {
  let key = e.key.toLowerCase();
  if (key === " ") {
    key = "space";
  }
  if (
    key === "w" ||
    key === "a" ||
    key === "s" ||
    key === "d" ||
    key === "space"
  ) {
    document.getElementById(`key-${key}`).classList.add("active");
  }
});

document.addEventListener("keyup", (e) => {
  let key = e.key.toLowerCase();
  if (key === " ") {
    key = "space";
  }
  if (
    key === "w" ||
    key === "a" ||
    key === "s" ||
    key === "d" ||
    key === "space"
  ) {
    document.getElementById(`key-${key}`).classList.remove("active");
  }
});
