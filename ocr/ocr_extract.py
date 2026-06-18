"""
ocr_extract.py
Extrae texto de una imagen usando PaddleOCR 3.x (PP-OCRv5)
"""

import sys
from pathlib import Path

def load_image(image_path: str) -> str:
    """
    Verifica que el archivo exista y sea una imagen en formato soportado
    
    Args:
        image_path (str): Ruta a la imagen a validar
        
    Returns:
        str: Ruta absoluta del archivo validado
        
    Raises:
        SystemExit: Si el archivo no existe o formato no es soportado
        
    Formatos soportados: PNG, JPG, JPEG, BMP, TIFF, WebP
    """
    path = Path(image_path)
    if not path.exists():
        print(f"[ERROR] No se encontro el archivo: {image_path}")
        sys.exit(1)

    supported = {".png", ".jpg", ".jpeg", ".bmp", ".tiff", ".tif", ".webp"}
    if path.suffix.lower() not in supported:
        print(f"[ERROR] Formato no soportado: {path.suffix}")
        print(f"        Formatos validos: {', '.join(supported)}")
        sys.exit(1)

    return str(path)


def init_ocr(lang: str):
    """
    Inicializa una instancia de PaddleOCR 3.x con el idioma especificado
    
    Args:
        lang (str): Codigo de idioma ISO 639-1 (ej: 'es', 'en', 'ch', 'fr')
        
    Returns:
        PaddleOCR: Instancia de PaddleOCR configurada
        
    Raises:
        SystemExit: Si PaddleOCR no esta instalado
        
    Notas:
        - Usa 'use_textline_orientation=True' para detectar texto rotado
        - PaddleOCR 3.x descargara automáticamente modelos si es necesario
    """
    try:
        from paddleocr import PaddleOCR
    except ImportError:
        print("[ERROR] PaddleOCR no está instalado")
        print("        Instalalo con: pip install paddleocr")
        sys.exit(1)

    print(f"[INFO] Inicializando PaddleOCR (idioma: {lang}) ...")

    # PaddleOCR 3.x: use_angle_cls → use_textline_orientation
    # show_log ya no es parámetro válido en 3.x
    ocr = PaddleOCR(
        use_textline_orientation=True,  # corrige texto rotado
        lang=lang,
    )
    return ocr


def extract_text(ocr, image_path: str, min_confidence: float, show_boxes: bool) -> str:
    """
    Ejecuta OCR en una imagen usando PaddleOCR 3.x y extrae el texto
    
    Args:
        ocr (PaddleOCR): Instancia de PaddleOCR inicializada
        image_path (str): Ruta a la imagen para procesar
        min_confidence (float): Umbral mínimo de confianza (0.0-1.0)
        show_boxes (bool): Si True, muestra coordenadas de bounding boxes
        
    Returns:
        str: Texto extraido, con una línea por resultado detectado
        
    Estructura de resultados de PaddleOCR 3.x:
        res["rec_texts"]  → lista de strings detectados
        res["rec_scores"] → lista de floats (confianza por texto)
        res["rec_polys"]  → coordenadas de los polígonos (bounding boxes)
        
    Nota: Los resultados con confianza < min_confidence se filtran
    """
    print(f"[INFO] Procesando imagen: {image_path}")

    # predict() devuelve un generador; convertir a lista
    results = list(ocr.predict(image_path))

    if not results:
        print("[WARN] No se detecto texto en la imagen.")
        return ""

    lines = []
    print("\n" + "=" * 60)
    print("  TEXTO EXTRAIDO")
    print("=" * 60)

    for res in results:
        # res es un dict-like con claves: rec_texts, rec_scores, rec_polys, etc.
        texts  = res.get("rec_texts",  [])
        scores = res.get("rec_scores", [])
        polys  = res.get("rec_polys",  [])

        for i, (text, score) in enumerate(zip(texts, scores)):
            if score < min_confidence:
                continue

            lines.append(text)

            if show_boxes and i < len(polys):
                box = polys[i]
                coords = " | ".join(f"({int(p[0])},{int(p[1])})" for p in box)
                print(f"  [{score:.2f}]  {text}")
                print(f"         Coords: {coords}")
            else:
                print(f"  [{score:.2f}]  {text}")

    print("=" * 60)
    return "\n".join(lines)


def save_output(text: str, output_path: str):
    """
    Guarda el texto extraido en un archivo de texto UTF-8
    
    Args:
        text (str): Texto a guardar
        output_path (str): Ruta del archivo de destino
        
    Nota: Crea el archivo o sobrescribe uno existente
    """
    path = Path(output_path)
    path.write_text(text, encoding="utf-8")
    print(f"\n[INFO] Texto guardado en: {path}")
