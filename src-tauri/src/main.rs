#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use serde::{Serialize, Deserialize};

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoInfo {
    title: String,
    duration: String,
    thumbnail: String,
    formats: Vec<Format>,
    is_spotify: bool,
    is_playlist: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Format {
    id: String,
    quality: String,
    size: Option<String>,
    format_type: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct SpotifyTrack {
    name: String,
    artists: Vec<SpotifyArtist>,
    album: SpotifyAlbum,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpotifyArtist {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpotifyAlbum {
    name: String,
    images: Vec<SpotifyImage>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpotifyImage {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpotifyTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
}

fn limpiar_url_playlist(url: &str) -> String {
    if let Some(pos) = url.find("&list=") {
        url[..pos].to_string()
    } else {
        url.to_string()
    }
}

fn detectar_plataforma(url: &str) -> &'static str {
    if url.contains("spotify.com") {
        if url.contains("/album/") {
            "spotify_album"
        } else if url.contains("/playlist/") {
            "spotify_playlist"
        } else if url.contains("/track/") {
            "spotify_track"
        } else {
            "spotify"
        }
    } else if url.contains("music.youtube.com") {
        if url.contains("playlist?list=") || url.contains("&list=") {
            "youtube_music_playlist"
        } else {
            "youtube"
        }
    } else if url.contains("youtube.com") || url.contains("youtu.be") {
        if url.contains("list=") && !url.contains("&index=") {
            "youtube_playlist"
        } else {
            "youtube"
        }
    } else {
        "unknown"
    }
}

fn es_playlist(url: &str) -> bool {
    url.contains("list=") && !url.contains("&index=") || url.contains("playlist?list=")
}

fn extraer_id_spotify(url: &str) -> &str {
    let parts: Vec<&str> = url.split('/').collect();
    let track_part = parts.last().unwrap_or(&"");
    track_part.split('?').next().unwrap_or(track_part)
}

async fn get_spotify_token() -> Result<String, String> {
    let client_id = "";  
    let client_secret = "";  
    
    let client = reqwest::Client::new();
    let response = client
        .post("https://accounts.spotify.com/api/token")
        .basic_auth(client_id, Some(client_secret))
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await
        .map_err(|e| format!("Error Url de Spotify no soportada: {}", e))?;
    
    if !response.status().is_success() {
        let status = response.status();
        return Err(format!("Error en API de Spotify: {}", status));
    }
    
    let token_data: SpotifyTokenResponse = response
        .json()
        .await
        .map_err(|e| format!("Error al parsear token: {}", e))?;
    
    Ok(token_data.access_token)
}

async fn buscar_spotify_por_url(url: &str) -> Result<String, String> {
    let token = get_spotify_token().await?;
    let track_id = extraer_id_spotify(url);
    
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://api.spotify.com/v1/tracks/{}", track_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Error al buscar en Spotify: {}", e))?;
    
    if !response.status().is_success() {
        let status = response.status();
        return Err(format!("Error en API de Spotify: {}", status));
    }
    
    let track: SpotifyTrack = response
        .json()
        .await
        .map_err(|e| format!("Error al parsear respuesta: {}", e))?;
    
    let artistas: Vec<String> = track.artists.iter().map(|a| a.name.clone()).collect();
    let query = format!("{} {} {}", artistas.join(" "), track.name, track.album.name);
    
    println!("Buscando: {}", query);
    
    Ok(query)
}

#[tauri::command]
async fn buscar_spotify_manual(query: String) -> Result<VideoInfo, String> {
    println!("Buscando: {}", query);
    
    #[cfg(target_os = "windows")]
    let mut cmd = Command::new("yt-dlp");
    #[cfg(not(target_os = "windows"))]
    let mut cmd = Command::new("yt-dlp");
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);
    
    let output = cmd
        .args([
            "-j",
            "--default-search", "ytsearch",
            "--match-filter", "duration < 600",
            &format!("ytsearch1:{}", query)
        ])
        .output()
        .map_err(|e| format!("Error al ejecutar yt-dlp: {}", e))?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Error en busqueda: {}", error));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout)
        .map_err(|e| format!("Error al parsear JSON: {}", e))?;
    
    let title = json["title"].as_str().unwrap_or("Desconocido").to_string();
    let duration = format_duration(json["duration"].as_u64().unwrap_or(0));
    let thumbnail = json["thumbnail"].as_str().unwrap_or("").to_string();
    
    let formats = vec![
        Format {
            id: "audio_mp3_spotify".to_string(),
            quality: "MP3 320kbps (Calidad Spotify)".to_string(),
            size: None,
            format_type: "audio".to_string(),
        },
        Format {
            id: "audio_mp3_high".to_string(),
            quality: "MP3 Mejor Calidad".to_string(),
            size: None,
            format_type: "audio".to_string(),
        }
    ];
    
    Ok(VideoInfo {
        title,
        duration,
        thumbnail,
        formats,
        is_spotify: true,
        is_playlist: false,
    })
}

#[tauri::command]
async fn get_video_info(url: String) -> Result<VideoInfo, String> {
    let plataforma = detectar_plataforma(&url);
    
    if plataforma == "spotify_track" {
        let query = buscar_spotify_por_url(&url).await?;
        return buscar_spotify_manual(query).await;
    }
    
    if plataforma == "spotify_album" || plataforma == "spotify_playlist" {
        return Err("SPOTIFY_PLAYLIST_NOT_SUPPORTED".to_string());
    }
    
    let clean_url = if plataforma == "youtube_playlist" || plataforma == "youtube_music_playlist" {
        url.clone()
    } else {
        limpiar_url_playlist(&url)
    };
    
    let is_playlist = es_playlist(&url);
    
    println!("URL detectada: {}", url);
    println!("Es playlist: {}", is_playlist);
    println!("Plataforma: {}", plataforma);
    
    #[cfg(target_os = "windows")]
    let mut cmd = Command::new("yt-dlp");
    #[cfg(not(target_os = "windows"))]
    let mut cmd = Command::new("yt-dlp");
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);
    
    let output = if is_playlist {
        cmd.args([
            "-j",
            "--flat-playlist",
            "--playlist-items", "1",
            &clean_url
        ]).output()
    } else {
        cmd.args(["-j", &clean_url]).output()
    }.map_err(|e| format!("Error al ejecutar yt-dlp: {}", e))?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Error en yt-dlp: {}", error));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout)
        .map_err(|e| format!("Error al parsear JSON: {}", e))?;
    
    let title = if is_playlist {
        json["playlist_title"].as_str().unwrap_or("Playlist").to_string()
    } else {
        json["title"].as_str().unwrap_or("Desconocido").to_string()
    };
    
    let duration = format_duration(json["duration"].as_u64().unwrap_or(0));
    let thumbnail = if is_playlist {
        json["thumbnails"][0]["url"].as_str().unwrap_or("").to_string()
    } else {
        json["thumbnail"].as_str().unwrap_or("").to_string()
    };
    
    let mut formats = Vec::new();
    
    formats.push(Format {
        id: "audio_mp3".to_string(),
        quality: "MP3 Audio".to_string(),
        size: None,
        format_type: "audio".to_string(),
    });
   
    let video_formats = vec![
        ("video_2160p", "MP4 2160p (4K) - Alta calidad 60FPS"),
        ("video_1440p", "MP4 1440p (2K) - Alta calidad 60FPS"),
        ("video_1080p_60fps", "MP4 1080p (Full HD) - 60 FPS"),
        ("video_1080p", "MP4 1080p (Full HD) - 30 FPS"),
        ("video_720p_60fps", "MP4 720p (HD) - 60 FPS"),
        ("video_720p", "MP4 720p (HD) - 30 FPS"),
        ("video_480p", "MP4 480p - Calidad media"),
        ("video_360p", "MP4 360p - Calidad baja"),
    ];
    
    for (id, quality) in video_formats {
        formats.push(Format {
            id: id.to_string(),
            quality: quality.to_string(),
            size: None,
            format_type: "video".to_string(),
        });
    }
    
    if is_playlist {
        
        formats.push(Format {
            id: "playlist_mp3".to_string(),
            quality: "Playlist completa en MP3".to_string(),
            size: None,
            format_type: "audio".to_string(),
        });
        
        formats.push(Format {
            id: "playlist".to_string(),
            quality: "Playlist completa en video 1080p 60FPS".to_string(),
            size: None,
            format_type: "video".to_string(),
        });
    }
    
    Ok(VideoInfo {
        title,
        duration,
        thumbnail,
        formats,
        is_spotify: false,
        is_playlist,
    })
}

#[tauri::command]
async fn descargar_video(url: String, format_id: String, spotify_query: Option<String>) -> Result<String, String> {
    let plataforma = detectar_plataforma(&url);
    let is_spotify = plataforma == "spotify";
    let is_spotify_album = plataforma == "spotify_album";
    let is_spotify_playlist = plataforma == "spotify_playlist";
    let is_youtube_music_playlist = plataforma == "youtube_music_playlist";
    
    let clean_url = if is_spotify || is_spotify_album || is_spotify_playlist {
        let query = spotify_query.ok_or("Se necesita el nombre de la cancion para Spotify")?;
        format!("ytsearch1:{}", query)
    } else {
        url.clone()
    };
    
    let descargas = dirs::download_dir().ok_or("No se pudo obtener la carpeta de descargas")?;
    let carpeta_mindload = descargas.join("Mindload");
    
    if !carpeta_mindload.exists() {
        std::fs::create_dir_all(&carpeta_mindload)
            .map_err(|e| format!("Error al crear la carpeta Mindload: {}", e))?;
    }
    
    #[cfg(target_os = "windows")]
    let mut cmd = Command::new("yt-dlp");
    #[cfg(not(target_os = "windows"))]
    let mut cmd = Command::new("yt-dlp");
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);
    
    let is_playlist = es_playlist(&url) || is_spotify_album || is_spotify_playlist || is_youtube_music_playlist;
    
    match format_id.as_str() {
        "playlist_mp3" => {
            let output_template = format!("{}/%(playlist_title)s/%(playlist_index)s - %(title)s.%(ext)s", 
                carpeta_mindload.to_string_lossy());
            
            let mut args = vec![
                "--yes-playlist",
                "--ignore-errors",
                "--no-abort-on-error",
                "--extract-audio",
                "--audio-format", "mp3",
                "--audio-quality", "0",
                "--embed-thumbnail",
                "--embed-metadata",
                "--parse-metadata", "playlist_index:%(track_number)s",
                "--parse-metadata", "title:%(meta_title)s",
                "--parse-metadata", "uploader:%(artist)s",
                "--replace-in-metadata", "artist", r" - ", ", ",
                "--replace-in-metadata", "artist", r" & ", ", ",
                "--sleep-interval", "3",
                "--retries", "10",
                "--fragment-retries", "10",
                "-o", &output_template,
            ];
            args.push(&clean_url);
            cmd.args(args);
        },
        "audio_mp3" | "audio_mp3_spotify" | "audio_mp3_high" => {
            let output_template = if is_playlist {
                format!("{}/%(playlist_title)s/%(playlist_index)s - %(title)s.%(ext)s", 
                    carpeta_mindload.to_string_lossy())
            } else {
                format!("{}/%(artist)s - %(title)s.%(ext)s", carpeta_mindload.to_string_lossy())
            };
            
            let mut args = vec![
                "-f", "bestaudio",
                "--extract-audio",
                "--audio-format", "mp3",
                "--audio-quality", "0",
                "--embed-thumbnail",
                "--embed-metadata",
            ];
            
            if is_playlist {
                args.push("--parse-metadata");
                args.push("playlist_index:%(track_number)s");
                args.push("--parse-metadata");
                args.push("uploader:%(artist)s");
                args.push("--replace-in-metadata");
                args.push("artist");
                args.push(r" - ");
                args.push(", ");
                args.push("--replace-in-metadata");
                args.push("artist");
                args.push(r" & ");
                args.push(", ");
            } else {
                args.push("--parse-metadata");
                args.push("artist:%(artist)s");
            }
            
            args.push("--parse-metadata");
            args.push("title:%(title)s");
            args.push("--sleep-interval");
            args.push("3");
            args.push("--retries");
            args.push("10");
            args.push("--fragment-retries");
            args.push("10");
            args.push("-o");
            args.push(&output_template);
            
            if is_playlist {
                args.insert(0, "--yes-playlist");
                args.insert(1, "--ignore-errors");
                args.insert(2, "--no-abort-on-error");
            }
            
            if format_id == "audio_mp3_high" {
                args.push("--audio-quality");
                args.push("0");
            }
            
            args.push(&clean_url);
            cmd.args(args);
        },
        "playlist" => {
            let output_template = format!("{}/%(playlist_title)s/%(playlist_index)s - %(title)s.%(ext)s", 
                carpeta_mindload.to_string_lossy());
            
            let mut args = vec![
                "--yes-playlist",
                "--ignore-errors",
                "--no-abort-on-error",
                "-f", "bestvideo[height<=1080][fps<=60][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "3",
                "--retries", "10",
                "--fragment-retries", "10",
                "-o", &output_template,
            ];
            args.push(&clean_url);
            cmd.args(args);
        },
        "video_2160p" => {
            cmd.args([
                "-f", "bestvideo[height<=2160][fps<=60][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "3",
                "--retries", "10",
                "--fragment-retries", "10",
                "-o", &format!("{}/%(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &clean_url
            ]);
        },
        "video_1440p" => {
            cmd.args([
                "-f", "bestvideo[height<=1440][fps<=60][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "3",
                "--retries", "10",
                "--fragment-retries", "10",
                "-o", &format!("{}/%(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &clean_url
            ]);
        },
        "video_1080p_60fps" => {
            cmd.args([
                "-f", "bestvideo[height<=1080][fps<=60][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "3",
                "--retries", "10",
                "--fragment-retries", "10",
                "-o", &format!("{}/%(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &clean_url
            ]);
        },
        "video_1080p" => {
            cmd.args([
                "-f", "bestvideo[height<=1080][fps<=30][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "3",
                "--retries", "10",
                "--fragment-retries", "10",
                "-o", &format!("{}/%(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &clean_url
            ]);
        },
        "video_720p_60fps" => {
            cmd.args([
                "-f", "bestvideo[height<=720][fps<=60][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "3",
                "--retries", "10",
                "--fragment-retries", "10",
                "-o", &format!("{}/%(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &clean_url
            ]);
        },
        "video_720p" => {
            cmd.args([
                "-f", "bestvideo[height<=720][fps<=30][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "3",
                "--retries", "10",
                "--fragment-retries", "10",
                "-o", &format!("{}/%(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &clean_url
            ]);
        },
        "video_480p" => {
            cmd.args([
                "-f", "bestvideo[height<=480][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "3",
                "--retries", "10",
                "--fragment-retries", "10",
                "-o", &format!("{}/%(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &clean_url
            ]);
        },
        "video_360p" => {
            cmd.args([
                "-f", "bestvideo[height<=360][vcodec^=avc]+bestaudio[ext=m4a]/best[ext=mp4]",
                "--merge-output-format", "mp4",
                "--sleep-interval", "3",
                "--retries", "10",
                "--fragment-retries", "10",
                "-o", &format!("{}/%(title)s.%(ext)s", carpeta_mindload.to_string_lossy()),
                &clean_url
            ]);
        },
        _ => {
            return Err("Formato no valido".to_string());
        }
    };
    
    println!("Ejecutando: {:?}", cmd);
    
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
    let descargas = dirs::download_dir().ok_or("No se pudo obtener la carpeta de descargas")?;
    let carpeta_mindload = descargas.join("Mindload");
    
    if !carpeta_mindload.exists() {
        std::fs::create_dir_all(&carpeta_mindload)
            .map_err(|e| format!("Error al crear la carpeta Mindload: {}", e))?;
        println!("Carpeta Mindload creada en: {:?}", carpeta_mindload);
    } else {
        println!("Carpeta Mindload ya existe en: {:?}", carpeta_mindload);
    }
    
    Ok(carpeta_mindload.to_string_lossy().to_string())
}

#[tauri::command]
fn get_download_folder() -> Result<String, String> {
    crear_carpeta_mindload()
}

fn main() {
    match crear_carpeta_mindload() {
        Ok(ruta) => println!("Carpeta lista en: {}", ruta),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_video_info, 
            descargar_video,
            buscar_spotify_manual,
            get_download_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}