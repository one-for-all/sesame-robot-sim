import * as monaco from "monaco-editor";
import demo_ino from "./assets/sesame.ino";
import movement_sequences from "./assets/movement-sequences.h";
import dummy_ino_hex from "./assets/dummy.ino.hex";
import readme from "./assets/README.md";
import { getSimulator } from ".";
import AnsiToHtml from "ansi-to-html";

type FileEntry = {
  content: string;
  language: string;
};

let files: Record<string, FileEntry> = {
  "sesame.ino": {
    content: demo_ino,
    language: "cpp",
  },
  "movement-sequences.h": {
    content: movement_sequences,
    language: "cpp",
  },
  "README.md": {
    content: readme,
    language: "markdown",
  },
};
let currentFile: string = null;

const editor = monaco.editor.create(document.getElementById("editor")!, {
  language: "cpp",
  theme: "vs-dark",
  automaticLayout: true,
  fontSize: 14,
  minimap: { enabled: false },
});

editor.onDidChangeModelContent(() => {
  if (currentFile) {
    files[currentFile].content = editor.getValue();
  }
});

renderFileBar();
openFile(Object.keys(files)[0]);

function renderFileBar() {
  const fileBar = document.getElementById("fileBar");
  const runButton = document.getElementById("runButton");
  const stopButton = document.getElementById("stopButton");
  fileBar.innerHTML = "";

  Object.keys(files).forEach((filename) => {
    const tab = document.createElement("div");
    tab.className = "file-tab";
    if (filename === currentFile) {
      tab.classList.add("active");
    }

    const icon = getFileIcon(filename);
    tab.innerHTML = `<span class="file-icon">${icon}</span><span>${filename}</span>`;
    tab.onclick = () => openFile(filename);

    fileBar.appendChild(tab);
  });

  fileBar.appendChild(runButton);
  fileBar.appendChild(stopButton);
}

function openFile(filename: string) {
  if (currentFile && files[currentFile]) {
    files[currentFile].content = editor.getValue();
  }

  currentFile = filename;
  const file = files[filename];
  monaco.editor.setModelLanguage(editor.getModel(), file.language);
  editor.setValue(file.content);

  renderFileBar();
}

function getFileIcon(filename: string) {
  const ext = filename.split(".").pop();
  const icons: Record<string, string> = {
    ino: "📝",
    h: "📝",
    md: "📋",
  };
  return icons[ext] || "📄";
}

// Build and Run the code
const url = "https://arduino-compiler-spl73nlieq-uc.a.run.app";

export interface HexiResult {
  stdout: string;
  hex: string;
  error: string;
  details: string;
  returncode: number;
  success: boolean;
  size: number;
}

async function buildHex(source: string) {
  const resp = await fetch(url + "/compile", {
    method: "POST",
    mode: "cors",
    cache: "no-cache",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ sketch: source }),
  });
  return (await resp.json()) as HexiResult;
}

document.getElementById("runButton").addEventListener("click", async () => {
  await runCode();
});

async function runCode() {
  const runButton = document.getElementById("runButton") as HTMLButtonElement;
  const stopButton = document.getElementById("stopButton") as HTMLButtonElement;
  const outputDiv = document.getElementById("buildOutput");
  const outputContent = document.getElementById("outputContent");

  // Save current file content
  if (currentFile && files[currentFile]) {
    files[currentFile].content = editor.getValue();
  }

  const source = files["sesame.ino"].content; // get sesame.ino file content

  // Disable button and show loading
  runButton.disabled = true;
  runButton.innerHTML = "<span>⏳</span><span>Compiling...</span>";
  stopButton.disabled = true;

  outputDiv.classList.add("show");
  outputContent.innerHTML = "<div>Compiling esp32 project...</div>";

  try {
    const result = await buildHex(source);

    let output = "";
    if (result.stdout) {
      const ansi = new AnsiToHtml();
      const html = ansi.toHtml(result.stdout);
      output += `<div class="stdout">STDOUT:<br>${html}</div>`;
    }

    if (result.error) {
      const ansi = new AnsiToHtml();
      const html = ansi.toHtml(result.details);
      output += `<div class="stderr">STDERR:<br>${html}</div>`;
    }

    if (result.hex) {
      output +=
        '<div class="success">✓ Compile successful!\nbin file generated (' +
        result.hex.length +
        " bytes)</div>";
      console.log("Hex output:", result.hex);
    }

    outputContent.innerHTML =
      output ||
      '<div class="success">Compilation completed successfully!</div>';

    if (!result.error) {
      let simulator = getSimulator();
      simulator.hybrid.reset();
      simulator.hybrid.reboot_code_controller(0, result.hex);
      // simulator.pendulum_raised = false;
    }
  } catch (error) {
    outputContent.innerHTML =
      '<div class="stderr">Compile error:\n' + error.message + "</div>";
    console.error("Compile error:", error);
  } finally {
    runButton.disabled = false;
    runButton.innerHTML = "<span>🔨</span><span>Run</span>";
    stopButton.disabled = false;
  }
}

document.getElementById("closeOutput").addEventListener("click", async () => {
  document.getElementById("buildOutput").classList.remove("show");
});

document.getElementById("stopButton").addEventListener("click", async () => {
  let simulator = getSimulator();
  simulator.hybrid.reboot_code_controller(0, dummy_ino_hex);
  simulator.hybrid.reset();
});
