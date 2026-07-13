use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub python_path: PathBuf,
    pub venv_path: PathBuf,
    pub ocr_script_path: PathBuf,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

impl RuntimeConfig {
    pub fn from_env() -> Self {
        let venv_path = env::var_os("OCR_VENV_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(default_venv_path);
        let python_path = env::var_os("OCR_PYTHON")
            .map(PathBuf::from)
            .unwrap_or_else(|| default_python_path(&venv_path));
        let ocr_script_path = env::var_os("OCR_SCRIPT_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(default_ocr_script_path);

        Self {
            python_path,
            venv_path,
            ocr_script_path,
        }
    }
}

fn default_python_path(venv_path: &PathBuf) -> PathBuf {
    let venv_python = venv_path.join("bin/python3");
    if venv_python.is_file() {
        return venv_python;
    }

    let venv_python_fallback = venv_path.join("bin/python");
    if venv_python_fallback.is_file() {
        return venv_python_fallback;
    }

    PathBuf::from("python3")
}

fn default_ocr_script_path() -> PathBuf {
    resolve_workspace_path("ocr/main.py")
}

fn default_venv_path() -> PathBuf {
    resolve_workspace_path("ocr/.venv_paddleocr")
}

fn resolve_workspace_path(relative_path: &str) -> PathBuf {
    if let Ok(executable_path) = env::current_exe() {
        if let Some(crate_root) = executable_path
            .parent()
            .and_then(|path| path.parent())
            .and_then(|path| path.parent())
        {
            let candidate = crate_root
                .parent()
                .map(|workspace_root| workspace_root.join(relative_path))
                .unwrap_or_else(|| crate_root.join(relative_path));

            if candidate.exists() {
                return candidate;
            }
        }
    }

    env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(relative_path)
}