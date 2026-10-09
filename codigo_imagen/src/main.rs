use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::sync::Semaphore;
use tesseract_rs::TesseractAPI;
use regex::Regex;
use tracing::{info, warn, error, debug, instrument};

struct Config {
    target_dir: String,
    tessdata_path: String,
    ocr_lang: String,
    max_concurrent: usize,
    codigo_pattern: String,
    adjust_contraste: f32, // Nombre corregido
}

impl Config {
    fn from_env() -> Self {
        Self {
            target_dir: env::var("TARGET_DIR").unwrap_or_else(|_| "./mis_imagenes".to_string()),
            tessdata_path: env::var("TESSDATA_PREFIX").unwrap_or_else(|_| "/usr/share/tessdata".to_string()),
            ocr_lang: env::var("OCR_LANG").unwrap_or_else(|_| "eng".to_string()),
            max_concurrent: env::var("MAX_CONCURRENT_TASKS")
                .unwrap_or_else(|_| "4".to_string())
                .parse::<usize>()
                .unwrap_or(4),
            codigo_pattern: env::var("CODIGO_PATTERN")
                .unwrap_or_else(|_| r"CON-[a-zA-Z0-9]+-[a-zA-Z0-9]+-[a-zA-Z0-9]+".to_string()),
            adjust_contraste: env::var("ADJUST_CONTRASTE")
                .unwrap_or_else(|_| "0.0".to_string())
                .parse::<f32>()
                .unwrap_or(0.0),
        }
    }
}

fn procesar_ocr_con_rotacion(
    image_path: PathBuf,
    tessdata_path: String,
    lang: String,
    contraste: f32,
    regex_pattern: &str
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {

    // 1. Cargar la imagen base
    let mut img_dynamic = image::open(&image_path)?;

    // 2. Aplicar el filtro de contraste corregido
    if contraste != 0.0 {
        img_dynamic = img_dynamic.adjust_contrast(contraste);
    }

    let api = TesseractAPI::new();
    api.init(&tessdata_path, &lang)?;
    let re = Regex::new(regex_pattern)?;

    // Probar las 4 rotaciones ortogonales (0°, 90°, 180°, 270°)
    for angulo in [0, 90, 180, 270] {
        debug!("Intentando OCR con rotación de {} grados...", angulo);

        let img_rotada = match angulo {
            0 => img_dynamic.clone(),
            90 => img_dynamic.rotate90(),
            180 => img_dynamic.rotate180(),
            270 => img_dynamic.rotate270(),
            _ => unreachable!(),
        };

        let img_rgba = img_rotada.to_rgba8();
        let (width, height) = img_rgba.dimensions();
        let raw_pixels = img_rgba.into_raw();

        api.set_image(&raw_pixels, width as i32, height as i32, 4, (width * 4) as i32);

        if let Ok(text) = api.get_utf8_text() {
            if let Some(mat) = re.find(&text) {
                let codigo_encontrado = mat.as_str().to_string();
                info!("¡Código encontrado en rotación de {}°!: {}", angulo, codigo_encontrado);
                return Ok(codigo_encontrado);
            }
        }
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No se detectó el patrón en ningún ángulo tras ajustar contraste"
    )))
}

#[tracing::instrument(skip(config), fields(archivo = %image_path.file_name().unwrap().to_string_lossy()))]
async fn procesar_y_renombrar_imagen(image_path: PathBuf, config: Arc<Config>) -> Result<(), Box<dyn std::error::Error>> {
    let filename = image_path.file_name().unwrap().to_string_lossy().into_owned();

    let path_clone = image_path.clone();
    let tessdata = config.tessdata_path.clone();
    let lang = config.ocr_lang.clone();
    let contraste = config.adjust_contraste;
    let pattern = config.codigo_pattern.clone();

    let ocr_result = tokio::task::spawn_blocking(move || {
        procesar_ocr_con_rotacion(path_clone, tessdata, lang, contraste, &pattern)
    }).await?;

    let mut new_name = match ocr_result {
        Ok(codigo) => codigo,
        Err(_) => {
            warn!("Patrón no detectado tras aplicar contraste y rotar 360 grados.");
            "codigo_no_detectado".to_string()
        }
    };

    new_name = new_name.chars().take(50).collect::<String>();

    let extension = image_path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("png");

    let mut destination_path = image_path.clone();
    destination_path.set_file_name(format!("{}.{}", new_name, extension));
    let dest_filename = destination_path.file_name().unwrap().to_string_lossy().into_owned();

    if fs::try_exists(&destination_path).await? {
        warn!(destino = %dest_filename, "El destino ya existe, ignorando.");
    } else {
        fs::rename(&image_path, &destination_path).await?;
        info!(de = %filename, a = %dest_filename, "¡Imagen renombrada exitosamente!");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    dotenvy::dotenv().ok();
    let config = Arc::new(Config::from_env());

    info!("=== INICIALIZANDO PROCESADOR DE IMÁGENES ===");
    info!("Directorio objetivo : {}", config.target_dir);
    info!("Ajuste de Contraste : {}", config.adjust_contraste);
    info!("Patrón RegEx        : {}", config.codigo_pattern);
    info!("============================================");

    if !fs::try_exists(&config.target_dir).await? {
        error!("El directorio objetivo '{}' no existe.", config.target_dir);
        return Ok(());
    }

    let semaforo = Arc::new(Semaphore::new(config.max_concurrent));
    let mut entries = fs::read_dir(&config.target_dir).await?;
    let mut tareas = vec![];

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_lower = ext.to_lowercase();
                if ["png", "jpg", "jpeg", "bmp"].contains(&ext_lower.as_str()) {
                    let config_clone = Arc::clone(&config);
                    let sem_clone = Arc::clone(&semaforo);

                    let tarea = tokio::spawn(async move {
                        let _permiso = sem_clone.acquire().await.unwrap();
                        if let Err(e) = procesar_y_renombrar_imagen(path, config_clone).await {
                            error!("Error en la tarea de procesamiento: {}", e);
                        }
                    });
                    tareas.push(tarea);
                }
            }
        }
    }

    for tarea in tareas {
        let _ = tarea.await;
    }

    info!("¡Procesamiento de directorio finalizado con éxito!");
    Ok(())
}
