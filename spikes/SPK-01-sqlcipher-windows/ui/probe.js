window.addEventListener("DOMContentLoaded", async () => {
  const status = document.getElementById("status");
  try {
    const result = await window.__TAURI__.core.invoke("db_probe");
    status.textContent = result.ok
      ? `Aprovado: banco cifrado acessado pelo núcleo (${result.cipher_family}).`
      : "Falha controlada no probe.";
    document.title = result.ok ? "SPK01_PASS" : "SPK01_FAIL";
  } catch (_error) {
    status.textContent = "Falha controlada na comunicação com o núcleo.";
    document.title = "SPK01_FAIL";
  }
});

