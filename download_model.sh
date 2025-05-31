#!/bin/bash

set -e
echo "Checking for existing model.onnx file"

MODEL_PATH="models/model.onnx"
MODEL_URL="https://github.com/iola1999/nsfw-detect-onnx/releases/download/v1.0.0/model.onnx"

if [ ! -f "$MODEL_PATH" ]; then
    echo "Model not found. Downloading..."
    if wget -O "$MODEL_PATH" "$MODEL_URL"; then
        echo "Download successful."
    else
        echo "Download failed. Please check your internet connection or the URL."
        exit 1
    fi
else
    echo "Model already exists at $MODEL_PATH"
fi
