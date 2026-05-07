const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

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
const pantallaProgreso = document.getElementById("pantalla-progreso");
const progresoTitulo = document.getElementById("progreso-titulo");
const progresoDuracion = document.getElementById("progreso-duracion");
const progressFill = document.getElementById("progress-fill");
const progressText = document.getElementById("progress-text");
const downloadStatus = document.getElementById("download-status");
const downloadSpeed = document.getElementById("download-speed");
const downloadSize = document.getElementById("download-size");
const btnCancelar = document.getElementById("btn-cancelar");
const btnVolverProgreso = document.getElementById("btn-volver-progreso");
const thumbnailImg = document.getElementById("thumbnail-img");
const spotifyBadge = document.getElementById("spotify-badge");
const playlistBadge = document.getElementById("playlist-badge");

let currentCancelToken = null;
let videoInfo = null;
let formatoSeleccionado = null;

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
  if (spotifyBadge) spotifyBadge.style.display = "none";
  if (playlistBadge) playlistBadge.style.display = "none";
  formatoSeleccionado = null;
}

function resetearApp() {
  limpiarFormatos();
  ocultarEstado();
  if (urlInput) urlInput.value = "";
  formatoSeleccionado = null;
  if (pantallaDetalles) pantallaDetalles.style.display = "none";
  if (pantallaProgreso) pantallaProgreso.style.display = "none";
  if (pantallaInicio) pantallaInicio.style.display = "block";
  if (tituloElement) tituloElement.textContent = "Cargando...";
  if (duracionElement) duracionElement.textContent = "";
  if (progressFill) progressFill.style.width = "0%";
  if (progressText) progressText.textContent = "0%";
  if (downloadStatus) downloadStatus.textContent = "Preparando descarga...";
  if (downloadStatus) downloadStatus.className = "download-status";
  if (downloadSpeed) downloadSpeed.textContent = "Velocidad: --";
  if (downloadSize) downloadSize.textContent = "Tamano: --";
}

function crearBotonFormato(fmt) {
  const btn = document.createElement("button");
  btn.className = "format-btn";
  btn.textContent = `${fmt.quality} ${fmt.size ? `(${fmt.size} MB)` : ""}`;
  btn.dataset.formatId = fmt.id;
  btn.addEventListener("click", () => {
    document
      .querySelectorAll(".format-btn")
      .forEach((b) => b.classList.remove("seleccionado"));
    btn.classList.add("seleccionado");
    formatoSeleccionado = fmt.id;
  });
  return btn;
}

// ============================================
// BUSCADOR PRINCIPAL (SIN MODAL)
// ============================================
btnBuscar.addEventListener("click", async () => {
  const url = urlInput.value.trim();
  if (!url) {
    mostrarEstado("Ingresa una URL", "error");
    return;
  }

  mostrarEstado("Obteniendo informacion...", "cargando");
  btnBuscar.disabled = true;

  try {
    const info = await invoke("get_video_info", { url });

    console.log("INFO:", info);

    if (tituloElement) {
      tituloElement.innerText = info.title || "Sin titulo";
    }

    if (duracionElement) {
      duracionElement.innerText = `Duracion: ${info.duration || "0:00"}`;
    }

    if (info.thumbnail && thumbnailImg) {
      thumbnailImg.src = info.thumbnail;
      thumbnailImg.style.display = "block";
    } else if (thumbnailImg) {
      thumbnailImg.style.display = "none";
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

    if (info.is_spotify && spotifyBadge) {
      spotifyBadge.style.display = "block";
    }

    if (info.is_playlist && playlistBadge) {
      playlistBadge.style.display = "block";
    }

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
    if (error === "SPOTIFY_PLAYLIST_NOT_SUPPORTED") {
      mostrarEstado("Playlists y albumes de Spotify no soportados. Usa una cancion individual.", "error");
    } else {
      mostrarEstado("Error: " + error, "error");
    }
  } finally {
    btnBuscar.disabled = false;
  }
});

// ============================================
// DESCARGA
// ============================================
btnDescargar.addEventListener("click", async () => {
  if (!formatoSeleccionado) {
    mostrarEstado("Selecciona un formato", "error");
    return;
  }

  const url = urlInput.value.trim();

  pantallaDetalles.style.display = "none";
  pantallaProgreso.style.display = "block";

  if (progresoTitulo) progresoTitulo.innerText = tituloElement.innerText;
  if (progresoDuracion) progresoDuracion.innerText = duracionElement.innerText;

  const progresoThumbnail = document.getElementById("progreso-thumbnail");
  if (progresoThumbnail && thumbnailImg) {
    progresoThumbnail.src = thumbnailImg.src;
    progresoThumbnail.style.display = thumbnailImg.style.display;
  }

  if (progressFill) progressFill.style.width = "0%";
  if (progressText) progressText.textContent = "0%";
  if (downloadStatus) {
    downloadStatus.textContent = "Iniciando descarga...";
    downloadStatus.className = "download-status";
  }
  if (downloadSpeed) downloadSpeed.textContent = "Velocidad: --";
  if (downloadSize) downloadSize.textContent = "Tamano: --";

  btnDescargar.disabled = true;

  try {
    await invoke("descargar_video", {
      url: url,
      formatId: formatoSeleccionado,
      spotifyQuery: null
    });

    if (downloadStatus) {
      downloadStatus.textContent = "Descarga completada!";
      downloadStatus.className = "download-status complete-status";
    }
    if (progressFill) progressFill.style.width = "100%";
    if (progressText) progressText.textContent = "100%";

    setTimeout(() => {
      resetearApp();
      btnDescargar.disabled = false;
    }, 2000);
  } catch (error) {
    if (downloadStatus) {
      downloadStatus.textContent = "Error: " + error;
      downloadStatus.className = "download-status error-status";
    }
    btnDescargar.disabled = false;
  }
});

btnVolverProgreso.addEventListener("click", () => {
  resetearApp();
});

btnCancelar.addEventListener("click", () => {
  if (downloadStatus) {
    downloadStatus.textContent = "Descarga cancelada";
    downloadStatus.className = "download-status error-status";
  }
  setTimeout(() => {
    resetearApp();
    btnDescargar.disabled = false;
  }, 1000);
});

btnVolver.addEventListener("click", resetearApp);