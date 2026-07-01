@echo off
REM Download Embedding and Reranker Model Primitives
REM Uses curl to download from Hugging Face

setlocal enabledelayedexpansion

set OUTPUT_DIR=F:\embedding-and-reranker-primitives\models\weights

echo ============================================================
echo Embedding ^& Reranker Primitives Download
echo Output: %OUTPUT_DIR%
echo ============================================================

REM Create output directory
if not exist "%OUTPUT_DIR%" mkdir "%OUTPUT_DIR%"

REM Download BGE-M3 (Embedding)
echo.
echo [1/4] Downloading BGE-M3 (Embedding Model)
echo URL: https://huggingface.co/BAAI/bge-m3
echo Size: ~2.2 GB
echo.

set BGE_M3_DIR=%OUTPUT_DIR%\BAAI_bge-m3
if not exist "%BGE_M3_DIR%" mkdir "%BGE_M3_DIR%"

echo Downloading config.json...
curl -L "https://huggingface.co/BAAI/bge-m3/resolve/main/config.json" -o "%BGE_M3_DIR%\config.json"

echo Downloading tokenizer.json...
curl -L "https://huggingface.co/BAAI/bge-m3/resolve/main/tokenizer.json" -o "%BGE_M3_DIR%\tokenizer.json"

echo Downloading tokenizer_config.json...
curl -L "https://huggingface.co/BAAI/bge-m3/resolve/main/tokenizer_config.json" -o "%BGE_M3_DIR%\tokenizer_config.json"

echo Downloading special_tokens_map.json...
curl -L "https://huggingface.co/BAAI/bge-m3/resolve/main/special_tokens_map.json" -o "%BGE_M3_DIR%\special_tokens_map.json"

echo Downloading vocab.txt...
curl -L "https://huggingface.co/BAAI/bge-m3/resolve/main/vocab.txt" -o "%BGE_M3_DIR%\vocab.txt"

echo Downloading model.safetensors (this may take a while)...
curl -L "https://huggingface.co/BAAI/bge-m3/resolve/main/model.safetensors" -o "%BGE_M3_DIR%\model.safetensors"

echo.
echo [✓] BGE-M3 download complete
echo.

REM Download mxbai-embed-large-v1
echo [2/4] Downloading mxbai-embed-large-v1 (Embedding Model)
echo URL: https://huggingface.co/mixedbread-ai/mxbai-embed-large-v1
echo Size: ~1.3 GB
echo.

set MXBAI_DIR=%OUTPUT_DIR%\mixedbread-ai_mxbai-embed-large-v1
if not exist "%MXBAI_DIR%" mkdir "%MXBAI_DIR%"

for %%f in (config.json tokenizer.json tokenizer_config.json special_tokens_map.json vocab.txt model.safetensors) do (
    echo Downloading %%f...
    curl -L "https://huggingface.co/mixedbread-ai/mxbai-embed-large-v1/resolve/main/%%f" -o "%MXBAI_DIR%\%%f"
)

echo.
echo [✓] mxbai-embed-large-v1 download complete
echo.

REM Download BGE-Reranker-v2-M3
echo [3/4] Downloading BGE-Reranker-v2-M3 (Reranker Model)
echo URL: https://huggingface.co/BAAI/bge-reranker-v2-m3
echo Size: ~1.2 GB
echo.

set RERANKER_V2M3_DIR=%OUTPUT_DIR%\BAAI_bge-reranker-v2-m3
if not exist "%RERANKER_V2M3_DIR%" mkdir "%RERANKER_V2M3_DIR%"

for %%f in (config.json tokenizer.json tokenizer_config.json special_tokens_map.json vocab.txt model.safetensors) do (
    echo Downloading %%f...
    curl -L "https://huggingface.co/BAAI/bge-reranker-v2-m3/resolve/main/%%f" -o "%RERANKER_V2M3_DIR%\%%f"
)

echo.
echo [✓] BGE-Reranker-v2-M3 download complete
echo.

REM Download BGE-Reranker-v2.5-Gemma2-Lightweight (WARNING: LARGE)
echo [4/4] Downloading BGE-Reranker-v2.5-Gemma2-Lightweight (Reranker Model)
echo URL: https://huggingface.co/BAAI/bge-reranker-v2.5-gemma2-lightweight
echo Size: ~18 GB (sharded)
echo WARNING: This is a large download!
echo.

set GEMMA2_DIR=%OUTPUT_DIR%\BAAI_bge-reranker-v2.5-gemma2-lightweight
if not exist "%GEMMA2_DIR%" mkdir "%GEMMA2_DIR%"

for %%f in (config.json tokenizer.json tokenizer_config.json special_tokens_map.json model.safetensors.index.json) do (
    echo Downloading %%f...
    curl -L "https://huggingface.co/BAAI/bge-reranker-v2.5-gemma2-lightweight/resolve/main/%%f" -o "%GEMMA2_DIR%\%%f"
)

echo.
echo NOTE: Gemma2 model uses sharded safetensors.
echo Additional shard files may need to be downloaded separately.
echo Check: https://huggingface.co/BAAI/bge-reranker-v2.5-gemma2-lightweight/tree/main
echo.

echo ============================================================
echo Download Summary
echo ============================================================
echo.
echo Models downloaded to: %OUTPUT_DIR%
echo.
echo To use these models:
echo   1. Install Python dependencies: pip install huggingface_hub transformers
echo   2. See DOWNLOAD.md for usage examples
echo   3. Or use the Rust primitives in embedding-primitives/ and reranker-primitives/
echo.
echo Done!
pause
