import readme from "./assets/README.md";
import demo_ino from "./assets/sesame.ino";
import movement_sequences from "./assets/movement-sequences.h";

type FileEntry = {
  content: string;
  language: string;
  scrollTop?: number; // Added for scroll position
  scrollLeft?: number; // Added for scroll position
};

export function resetFiles() {
  files = {
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
  currentFile = ""; // set to empty current file so editor will reset on open
}

export function renameFile(oldName: string, newName: string) {
  if (files[newName]) {
    alert("A file with that name already exists.");
    return false;
  }
  files[newName] = files[oldName];
  delete files[oldName];
  if (currentFile === oldName) {
    currentFile = newName;
  }
  return true;
}

export let files: Record<string, FileEntry> = {
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
export let currentFile: string = null;
export function setCurrentFile(filename: string) {
  currentFile = filename;
}

export function inoFileName() {
  let ino_file_names = Object.keys(files).filter((key) => key.endsWith(".ino"));
  if (ino_file_names.length != 1) {
    alert("Need to have one and only one .ino file");
  }
  return ino_file_names[0];
}
