# 🎵 Mindload

<div align="center">

![Version](https://img.shields.io/badge/version-2.0.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Tauri](https://img.shields.io/badge/Tauri-2.0-purple)
![Rust](https://img.shields.io/badge/Rust-1.75-orange)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)

**Mindload** es una aplicación de escritorio para descargar música y videos de YouTube con metadatos completos.

</div>

---

## ✨ Características

| Característica | Descripción |
|----------------|-------------|
| 🎵 **Descarga de audio** | Extrae audio en formato MP3 con calidad óptima |
| 🎬 **Descarga de video** | Soporte para resoluciones desde 360p hasta 4K (60 FPS) |
| 📁 **Organización automática** | Crea carpetas por playlist o artista |
| 🏷️ **Metadatos completos** | Incrusta título, artista, carátula y número de pista |
| 📋 **Playlists** | Descarga listas de reproducción completas |
| 🎯 **Interfaz simple** | Diseño minimalista y fácil de usar |
| 🚀 **Multiplataforma** | Compatible con Windows, Linux y macOS |

---

## 📸 Capturas de pantalla

| Pantalla principal | Detalles de descarga |
|--------------------|----------------------|
| *[Pendiente]* | *[Pendiente]* |

---

## 🚀 Instalación

### Requisitos previos

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) instalado y en PATH
- [ffmpeg](https://ffmpeg.org/) instalado y en PATH

### Descargar ejecutable

1. Ve a la sección [Releases](https://github.com/STAYED5/mindload/releases)
2. Descarga el archivo correspondiente a tu sistema operativo:
   - `mindload_2.0.0_x64_en-US.msi` (Windows)
   - `mindload_2.0.0_amd64.deb` (Linux)
   - `mindload.app` (macOS)
3. Ejecuta el instalador

### Desde código fuente

```bash
# Clonar el repositorio
git clone https://github.com/STAYED5/mindload.git
cd mindload

# Instalar dependencias del frontend
npm install

# Instalar dependencias de Rust
cd src-tauri
cargo build

# Ejecutar en modo desarrollo
cd ..
npm run tauri dev

# Construir para producción
npm run tauri build
```
# 🧠 Mindload

Descargador de contenido de YouTube (videos, playlists y música) con interfaz gráfica moderna construida con Tauri.

## 🎯 Cómo usar

1. Pega una URL de YouTube (video, playlist o música)
2. Selecciona el formato deseado:
   - MP3 Audio (con metadatos)
   - Video en diferentes resoluciones
3. Haz clic en "Descargar"
4. Los archivos se guardan en `Descargas/Mindload/`

## 📦 Formatos soportados

| Formato       | Calidad      | FPS  |
|---------------|--------------|------|
| MP3 Audio     | 320kbps      | -    |
| 4K (2160p)    | Ultra HD     | 60   |
| 2K (1440p)    | Muy alta     | 60   |
| 1080p         | Full HD      | 60/30|
| 720p          | HD           | 60/30|
| 480p          | Media        | -    |
| 360p          | Baja         | -    |

## 📁 Estructura de archivos

Las descargas se organizan automáticamente:
Descargas/
└── Mindload/
├── Artista - Canción.mp3 (canciones sueltas)
└── Nombre Playlist/ (playlists)
├── 01 - Artista - Canción 1.mp3
├── 02 - Artista - Canción 2.mp3
└── ...

## 🛠️ Tecnologías

| Tecnología   | Uso                                      |
|--------------|------------------------------------------|
| Tauri        | Framework de escritorio                  |
| Rust         | Backend y lógica de descargas            |
| yt-dlp       | Extracción y descarga de contenido       |
| ffmpeg       | Procesamiento de audio y video           |
| HTML/CSS/JS  | Interfaz de usuario                      |

## 📂 Estructura del proyecto
mindload/
├── src-tauri/
│ ├── src/
│ │ └── main.rs # Código Rust (backend)
│ ├── Cargo.toml # Dependencias Rust
│ └── tauri.conf.json # Configuración de Tauri
├── src/
│ ├── index.html # Interfaz principal
│ ├── styles.css # Estilos
│ └── main.js # Lógica del frontend
├── package.json # Dependencias Node.js
└── README.md # Este archivo

## ⚠️ Limitaciones conocidas

- **Spotify**: El soporte requiere API key y cuenta Premium. Actualmente no implementado.
- **YouTube**: Descargas limitadas por políticas de YouTube (se recomienda usar con responsabilidad).
- **Windows**: Puede requerir permisos de administrador ocasionalmente.

## 🔧 Solución de problemas

| Problema                         | Solución                                           |
|----------------------------------|----------------------------------------------------|
| "yt-dlp not found"               | Instala yt-dlp y agrégalo al PATH                 |
| "ffmpeg not found"               | Instala ffmpeg y agrégalo al PATH                 |
| Error 429 (Too Many Requests)    | Espera unos minutos y vuelve a intentar           |
| Las descargas no comienzan       | Verifica tu conexión a internet                   |
| No se descargan playlists completas | Asegúrate de que la playlist sea pública       |

## 🚀 Próximas mejoras

- Barra de progreso en tiempo real
- Descarga simultánea de múltiples archivos
- Soporte para más plataformas (SoundCloud, Bandcamp)
- Cola de descargas
- Historial de descargas
- Modo oscuro/claro
- Configuración de carpeta de destino

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Por favor:

1. Haz un Fork del proyecto
2. Crea tu rama de características (`git checkout -b feature/nueva-caracteristica`)
3. Commit tus cambios (`git commit -m 'Agrego nueva caracteristica'`)
4. Push a la rama (`git push origin feature/nueva-caracteristica`)
5. Abre un Pull Request

## 📄 Licencia

Este proyecto está bajo la licencia MIT. Ver el archivo `LICENSE` para más detalles.

## 👤 Autor

**LUZY (IAMLUZY)**  
GitHub: [@STAYED5](https://github.com/STAYED5)

## 🙏 Agradecimientos

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - Por la increíble herramienta de descarga
- [Tauri](https://tauri.app/) - Por el framework de escritorio
- Discord - Por la comunidad que inspiró esta app

---

<div align="center">
  Hecho con ❤️ para la comunidad <br>
  <sub>Mindload - Descarga lo que amas</sub>
</div>
