# CaptureOCR

CaptureOCR is a Linux desktop utility that captures a selected area of the screen, extracts its text with PaddleOCR, and copies the result to the clipboard.

It combines a Rust/GTK interface with a Python OCR worker. The two processes communicate through a Unix socket pair, so the OCR work remains isolated from the desktop application.

## Features

- Interactive screen-area selection through the desktop screenshot portal.
- Optional OCR after each capture.
- Text copied directly to the system clipboard.
- PaddleOCR-based text recognition with a configurable confidence threshold.
- Configurable Python interpreter, virtual environment, and OCR worker paths.

## Requirements

- A Linux desktop session with support for the XDG Desktop Portal screenshot API.
- Rust and Cargo (edition 2021 or later).
- Python 3.
- GTK 4 and libadwaita development packages required by the Rust dependencies.
- PaddleOCR and its runtime dependencies installed in the Python environment used by the app.

The first OCR run may download PaddleOCR model files, depending on the local cache and PaddleOCR configuration.

## Project layout

```text
.
├── screen-captures/       # Rust desktop application
│   └── src/
└── ocr/                   # Python OCR worker
    ├── main.py            # Worker entry point
    ├── socket_pair.py     # IPC protocol handler
    └── ocr_extract.py     # PaddleOCR integration
```

## Setup

Create and activate a Python virtual environment, then install PaddleOCR and the compatible PaddlePaddle runtime for your platform. Refer to the official PaddleOCR installation guidance when selecting the PaddlePaddle package, particularly when GPU support is needed.

For a CPU-oriented environment, the Python dependency installation typically begins with:

```bash
python3 -m venv ocr/.venv_paddleocr
source ocr/.venv_paddleocr/bin/activate
pip install paddleocr
```

Install the appropriate PaddlePaddle package if it was not installed as a dependency in your environment.

## Run

From the repository root, start the application with:

```bash
cargo run --manifest-path screen-captures/Cargo.toml
```

When prompted:

1. Select **Yes** to capture an area and run OCR. The extracted text is copied to the clipboard.
2. Select **No** to capture an area without running OCR.
3. Cancel the desktop capture dialog to exit that operation without producing a result.

## Configuration

The application resolves its runtime paths automatically, but each can be overridden through environment variables:

| Variable | Purpose | Default |
| --- | --- | --- |
| `OCR_VENV_PATH` | Python virtual-environment directory | `ocr/.venv_paddleocr` |
| `OCR_PYTHON` | Python executable used for the OCR worker | `<OCR_VENV_PATH>/bin/python3`, then `python3` |
| `OCR_SCRIPT_PATH` | Python OCR worker entry point | `ocr/main.py` |

Example:

```bash
OCR_PYTHON=/opt/ocr-venv/bin/python \
OCR_SCRIPT_PATH="$PWD/ocr/main.py" \
cargo run --manifest-path screen-captures/Cargo.toml
```

## How it works

```text
Desktop UI → Screenshot portal → Selected image
                                  ↓
Rust application → Python OCR worker → PaddleOCR → Clipboard
```

The Rust process passes the image path to the Python worker over a Unix socket pair. The worker returns recognized text, which the application writes to the clipboard. OCR output is also written by the Python worker to `ocr/output/out.txt` when text is found.

## Development

Run the Rust test suite with:

```bash
cargo test --manifest-path screen-captures/Cargo.toml
```

The OCR worker currently recognizes English text and filters detections below a confidence score of `0.5`. These settings are defined in `ocr/extract_text_main.py`.

## Troubleshooting

- **Capture dialog does not appear:** confirm that your desktop environment provides the screenshot portal and that its portal backend is installed and running.
- **`PaddleOCR is not installed`:** ensure `OCR_PYTHON` points to the virtual environment where PaddleOCR is installed.
- **The app cannot find the worker:** set `OCR_SCRIPT_PATH` to the absolute path of `ocr/main.py`.
- **No text is extracted:** use a sharper capture with sufficiently large, high-contrast text; low-confidence detections are intentionally discarded.
