import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { FoundationShell } from "./presentation/FoundationShell";
import "./styles/global.css";

const root = document.getElementById("root");

if (root === null) {
  throw new Error("Desktop root element was not found.");
}

createRoot(root).render(
  <StrictMode>
    <FoundationShell />
  </StrictMode>,
);
