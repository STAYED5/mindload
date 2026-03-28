const { invoke } = window.__TAURI__.core;
const { path } = window.__TAURI__;

const btnBuscar = document.getElementById("btn-buscar");
const btnDescargar = document.getElementById("btn-descargar");
const btnVolver = document.getElementById("btn-volver");
const urlInput = document.getElementById("url-input");
const pantallaInicio = document.getElementById("pantalla-inicio");
const pantallaDetalles = document.getElementById("pantalla-detalles");
const tituloElement = document.getElementById("titulo");
const duracionElement = document.getElementById("duracion");
const mp3Container = document.getElementById("mp3-container");
const mp4Container = document.getElementById("mp4-container");
const mp3Section = document.getElementById("mp3-section");
const mp4Section = document.getElementById("mp4-section");
const sinFormatosDiv = document.getElementById("sin-formatos");
const btnDescargarElement = document.getElementById("btn-descargar");
const estadoDiv = document.getElementById("estado");
const spinner = document.getElementById("spinner");

let videoInfo = null;
let formatoSeleccionado = null;
let carpetaDestino = null;

function mostrarEstado(mensaje, tipo) {
  if (!estadoDiv) return;
  estadoDiv.textContent = mensaje;
  estadoDiv.className = `estado ${tipo}`;
  estadoDiv.style.display = "block";
}

function ocultarEstado() {
  if (!estadoDiv) return;
  estadoDiv.style.display = "none";
  estadoDiv.textContent = "";
}

function mostrarSpinner() {
  if (spinner) spinner.style.display = "flex";
}

function ocultarSpinner() {
  if (spinner) spinner.style.display = "none";
}

function limpiarFormatos() {
  if (mp3Container) mp3Container.innerHTML = "";
  if (mp4Container) mp4Container.innerHTML = "";
  if (mp3Section) mp3Section.style.display = "none";
  if (mp4Section) mp4Section.style.display = "none";
  if (sinFormatosDiv) sinFormatosDiv.style.display = "none";
  if (btnDescargarElement) btnDescargarElement.style.display = "none";
  formatoSeleccionado = null;
}

function resetearApp() {
  limpiarFormatos();
  ocultarEstado();
  if (urlInput) urlInput.value = "";
  formatoSeleccionado = null;
  if (pantallaDetalles) pantallaDetalles.style.display = "none";
  if (pantallaInicio) pantallaInicio.style.display = "block";
  if (tituloElement) tituloElement.textContent = "Cargando...";
  if (duracionElement) duracionElement.textContent = "";
}

function crearBotonFormato(fmt) {
  const btn = document.createElement("button");
  btn.className = "format-btn";
  btn.textContent = `${fmt.quality} (${fmt.size || "?"} MB)`;
  btn.dataset.formatId = fmt.id;
  btn.addEventListener("click", () => {
    document.querySelectorAll(".format-btn").forEach((b) => b.classList.remove("seleccionado"));
    btn.classList.add("seleccionado");
    formatoSeleccionado = fmt.id;
  });
  return btn;
}

async function obtenerCarpetaMindload() {
  try {
    carpetaDestino = await path.downloadDir();
    console.log("Carpeta:", carpetaDestino);
  } catch (error) {
    carpetaDestino = "C:\\Users\\LUZY\\Downloads";
    console.log("Carpeta por defecto:", carpetaDestino);
  }
}

btnBuscar.addEventListener("click", async () => {
  const url = urlInput.value.trim();
  if (!url) {
    mostrarEstado("Ingresa una URL", "error");
    return;
  }

  mostrarEstado("Obteniendo información...", "cargando");
  btnBuscar.disabled = true;

  try {
    const info = await invoke("get_video_info", { url });

    console.log("INFO:", info);

    if (tituloElement) {
      tituloElement.innerText = info.title || "Sin título";
      console.log("Título asignado:", tituloElement.innerText);
    }

    if (duracionElement) {
      duracionElement.innerText = `Duración: ${info.duration || "0:00"}`;
      console.log("Duración asignada:", duracionElement.innerText);
    }

    limpiarFormatos();

    let tieneMp3 = false;
    let tieneMp4 = false;

    info.formats.forEach((fmt) => {
      if (fmt.format_type === "audio") {
        tieneMp3 = true;
        mp3Container?.appendChild(crearBotonFormato(fmt));
      } else if (fmt.format_type === "video") {
        tieneMp4 = true;
        mp4Container?.appendChild(crearBotonFormato(fmt));
      }
    });

    if (mp3Section) mp3Section.style.display = tieneMp3 ? "block" : "none";
    if (mp4Section) mp4Section.style.display = tieneMp4 ? "block" : "none";

    if (!tieneMp3 && !tieneMp4) {
      if (sinFormatosDiv) sinFormatosDiv.style.display = "block";
    } else {
      if (btnDescargarElement) btnDescargarElement.style.display = "block";
    }

    if (pantallaInicio) pantallaInicio.style.display = "none";
    if (pantallaDetalles) pantallaDetalles.style.display = "block";
    ocultarEstado();

  } catch (error) {
    console.error("Error:", error);
    mostrarEstado("Error: " + error, "error");
  } finally {
    btnBuscar.disabled = false;
  }
});

btnDescargar.addEventListener("click", async () => {
  if (!formatoSeleccionado) {
    mostrarEstado("Selecciona un formato", "error");
    return;
  }

  mostrarSpinner();
  btnDescargar.disabled = true;

  try {
    await invoke("descargar_video", {
      url: urlInput.value,
      formatId: formatoSeleccionado,
      destino: carpetaDestino,
    });

    ocultarSpinner();
    mostrarEstado(`Descarga completada en: ${carpetaDestino}`, "exito");

    setTimeout(() => {
      resetearApp();
      btnDescargar.disabled = false;
    }, 3000);
  } catch (error) {
    ocultarSpinner();
    mostrarEstado("Error: " + error, "error");
    btnDescargar.disabled = false;
  }
});

btnVolver.addEventListener("click", resetearApp);

obtenerCarpetaMindload();