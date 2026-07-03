from ocr_extract import load_image, init_ocr, extract_text, save_output

def start_extract_text(image: str) -> str:
    """
    Funcion principal que orquesta todo el flujo de extracción de OCR.
    
    Flujo:
        1. Inicializa argumentos
        2. Valida que la imagen exista
        3. Inicializa PaddleOCR con el idioma especificado
        4. Extrae texto de la imagen
        5. (Opcional) Guarda el resultado en un archivo
        6. Muestra estadisticas de líneas extraidas
    """

    #Initialize arguments
    #image = "/home/bryan/capture-ocr/ocr/captures/pdf_screenshot.png"
    lang = "en"
    show_boxes = False
    min_confidence = 0.5
    output_path = "/home/bryan/capture-ocr/ocr/output/out.txt"

    image_path = load_image(image)
    ocr = init_ocr(lang)
    text = extract_text(
        ocr,
        image_path,
        min_confidence,
        show_boxes
    )

    if output_path and text:
        save_output(text, output_path)

    if not text:
        #print("\n[RESULTADO] Sin texto extraido.")
        return  "[RESULTADO] Sin texto extraido."
    else:
        #print(f"\n[RESULTADO] {len(text.splitlines())} lineas extraidas.")
        return text

#if __name__ == "__main__":
    #main()