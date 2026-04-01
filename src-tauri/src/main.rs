#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::os::windows::process::CommandExt;
use serde::{Serialize, Deserialize};

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Serialize, Deserialize, Debug)]
struct VideoInfo {
    title: String,
    duration: String,
    thumbnail: String, 
    formats: Vec<Format>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Format {
    id: String,
    quality: String,
    size: Option<String>,
    format_type: String,
}

fn limpiar_url_playlist(url: &str) -> String {
    if let Some(pos) = url.find("&list=") {
        url[..pos].to_string()
    } else {
        url.to_string()
    }
}

#[tauri::command]
async fn get_video_info(url: String) -> Result<VideoInfo, String> {
    let clean_url = limpiar_url_playlist(&url);
    
    let output = Command::new("yt-dlp")
        .args(["-j", &clean_url])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| format!("Error al ejecutar yt-dlp: {}", e))?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Error en yt-dlp: {}", error));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    println!("JSON recibido: {}", stdout);
    
    let json: serde_json::Value = serde_json::from_str(&stdout)
        .map_err(|e| format!("Error al parsear JSON: {}", e))?;
    
    let title = json["title"].as_str().unwrap_or("Desconocido").to_string();
    let duration = format_duration(json["duration"].as_u64().unwrap_or(0));
    let thumbnail = json["thumbnail"].as_str().unwrap_or("").to_string();

    println!("Título extraído: {}", title);
    println!("Duración extraída: {}", duration);
    
    let is_playlist = url.contains("list=");
    
    let mut formats = Vec::new();
    
    formats.push(Format {
        id: "audio_mp3".to_string(),
        quality: "MP3 Audio (mejor calidad)".to_string(),
        size: None,
        format_type: "audio".to_string(),
    });
    
    formats.push(Format {
    id: "video_2160p".to_string(),
    quality: "MP4 2160p (4K) - Calidad máxima".to_string(),
    size: None,
    format_type: "video".to_string(),
    });

    formats.push(Format {
    id: "video_1440p".to_string(),
    quality: "MP4 1440p (2K) - Muy alta calidad".to_string(),
    size: None,
    format_type: "video".to_string(),
    });

    formats.push(Format {
        id: "video_1080p".to_string(),
        quality: "MP4 1080p (Full HD) - Alta calidad".to_string(),
        size: None,
        format_type: "video".to_string(),
    });
    
    formats.push(Format {
        id: "video_720p".to_string(),
        quality: "MP4 720p (HD) - Buena calidad".to_string(),
        size: None,
        format_type: "video".to_string(),
    });
    
    formats.push(Format {
        id: "video_480p".to_string(),
        quality: "MP4 480p - Calidad media".to_string(),
        size: None,
        format_type: "video".to_string(),
    });
    
    formats.push(Format {
        id: "video_360p".to_string(),
        quality: "MP4 360p - Calidad baja".to_string(),
        size: None,
        format_type: "video".to_string(),
    });
    
    if is_playlist {
        formats.push(Format {
            id: "playlist".to_string(),
            quality: "Playlist completa (todos los videos)".to_string(),
            size: None,
            format_type: "video".to_string(),
        });
    }
    
    Ok(VideoInfo {
    title,
    duration,
    thumbnail,
    formats,
})
}

#[tauri::command]
async fn descargar_video(url: String, format_id: String) -> Result<String, String> {
    let clean_url = limpiar_url_playlist(&url);
    
    // Obtener la carpeta Mindload
    let descargas = dirs::download_dir().ok_or("No se pudo obtener la carpeta de descargas")?;
    let carpeta_mindload = descargas.join("Mindload");
    
    // Asegurar que la carpeta existe (por si acaso)
    if !carpeta_mindload.exists() {
        std::fs::create_dir_all(&carpeta_mindload)
            .map_err(|e| format!("Error al crear la carpeta Mindload: {}", e))?;
    }
    
    let mut cmd = Command::new("yt-dlp"); 
    cmd.creation_flags(CREATE_NO_WINDOW);
    
    match format_id.as_str() {
        "audio_mp3" => {
            cmd.args([
                "-f", "bestaudio",
                "--extract-audio",
                "--audio-format", "mp3",
                "--sleep-interval", "5",              
                "--retries", "5", 
                "-o", &format!("{}/%(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &clean_url
            ]);
        },
        "video_2160p" => {
            cmd.args([
                "-f", "bestvideo[height<=2160][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "5",            
                "--retries", "5", 
                "-o", &format!("{}/%(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &clean_url
            ]);
        },
        "video_1440p" => {
            cmd.args([
                "-f", "bestvideo[height<=1440][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "5",           
                "--retries", "5", 
                "-o", &format!("{}/%(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &clean_url
            ]);
        },
        "video_1080p" => {
            cmd.args([
                "-f", "bestvideo[height<=1080][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "5",            
                "--retries", "5", 
                "-o", &format!("{}/%(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &clean_url
            ]);
        },
        "video_720p" => {
            cmd.args([
                "-f", "bestvideo[height<=720][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "5",                       
                "--retries", "5", 
                "-o", &format!("{}/%(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &clean_url
            ]);
        },
        "video_480p" => {
            cmd.args([
                "-f", "bestvideo[height<=480][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "5",            
                "--retries", "5", 
                "-o", &format!("{}/%(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &clean_url
            ]);
        },
        "video_360p" => {
            cmd.args([
                "-f", "bestvideo[height<=360][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "5",              
                "--retries", "5", 
                "-o", &format!("{}/%(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &clean_url
            ]);
        },
        "playlist" => {
            cmd.args([
                "--yes-playlist",
                "-f", "bestvideo[height<=1080][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "5",         
                "--retries", "5", 
                "-o", &format!("{}/%(playlist)s/%(playlist_index)s - %(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &url
            ]);
        },
        _ => {
            return Err("Formato no válido".to_string());
        }
    };
    
    let output = cmd.output()
        .map_err(|e| format!("Error al ejecutar yt-dlp: {}", e))?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Error en yt-dlp: {}", error));
    }
    
    Ok("Descarga completada".to_string())
}


fn format_duration(seconds: u64) -> String {

    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    
    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{}:{:02}", minutes, secs)
    }
}

fn crear_carpeta_mindload() -> Result<String, String> {
    // Obtener la carpeta de descargas del sistema
    let descargas = dirs::download_dir().ok_or("No se pudo obtener la carpeta de descargas")?;
    let carpeta_mindload = descargas.join("Mindload");
    
    // Crear la carpeta si no existe
    if !carpeta_mindload.exists() {
        std::fs::create_dir_all(&carpeta_mindload)
            .map_err(|e| format!("Error al crear la carpeta Mindload: {}", e))?;
        println!("📁 Carpeta Mindload creada en: {:?}", carpeta_mindload);
    } else {
        println!("📁 Carpeta Mindload ya existe en: {:?}", carpeta_mindload);
    }
    
    Ok(carpeta_mindload.to_string_lossy().to_string())
}

fn main() {
    // Crear la carpeta Mindload al iniciar
    match crear_carpeta_mindload() {
        Ok(ruta) => println!("✅ Carpeta lista en: {}", ruta),
        Err(e) => eprintln!("❌ Error: {}", e),
    }
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_video_info, descargar_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}