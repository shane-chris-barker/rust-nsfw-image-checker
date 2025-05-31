# NSFW Image Checker (Rust)
![MIT License](https://img.shields.io/badge/license-MIT-green)

A lightweight REST API server written in Rust to classify images for NSFW content using a pre-trained ONNX model.

This project is my **first hands-on experience with Rust**, built as a learning exercise and as a practical tool for image moderation. Feedback is very welcome!

---

## 🚀 Features

* 🔍 Detects NSFW content in user-supplied images
* 🧠 Powered by ONNX Runtime and a robust pre-trained classifier
* ✅ Exposes simple `/ping` and `/classify` endpoints
* ⚙️ Clean architecture with modular structure
* 🎯 Built-in integration tests

---

## 📆 Categories Explained

The model classifies each image with scores across five categories

The higher the score, the more reliable the result.

| Category        | Description                                                                    |
| --------------- | ------------------------------------------------------------------------------ |
| `porn_score`    | Highly explicit adult imagery including nudity and sexual acts                 |
| `sexy_score`    | Provocative content such as lingerie, revealing clothing, or suggestive poses  |
| `hentai_score`  | Drawn or animated NSFW content (e.g anime or cartoons)                       |
| `neutral_score` | Safe-for-work content such as normal clothing, faces, objects, landscapes      |
| `drawing_score` | Artistic or non-photographic images not intended to be sexual (e.g sketches) |

---

## 📁 Setup

### 1. Install Rust

If you don't already have Rust installed:

```
curl https://sh.rustup.rs -sSf | sh
```
More info at [https://rustup.rs](https://rustup.rs)

### 2. Clone the Repository

```
git clone https://github.com/shane-chris-barker/rust-nsfw-image-checker.git
```

### 3. Download the Model

```
cd rust-nsfw-image-checker
sh download_model.sh
```

This script downloads the ONNX model into `models/model.onnx`. 

The `models/` folder is `.gitignored` but preserved via a `.gitkeep` file.
> The model is approximately 85MB and may take a few seconds to download

#### 🛠️Dependencies  
**Note:** Ensure `wget` is installed on your system to run the model download script.

### 4. Run the Server

```
cargo run
```

Server will start at `http://localhost:3000`

## 🔮 API Endpoints

### `GET /ping`

Returns a basic JSON response indicating that the server is up and running:

```json
{"status":200,"result":true}
```

### `POST /classify`

Accepts an image file using `multipart/form-data`. Example using `curl`:

```bash
curl -X POST http://localhost:3000/classify \
  -F image=@example.jpg
```

**Response:**

```json
{
  "porn_score": :0.98313576,
  "sexy_score": 0.006444183,
  "hentai_score": 0.010374044,
  "neutral_score": 0.000044660992,
  "drawing_score": :0.0000013174929
}
```

---

## 🔧 Testing

Run integration tests with:

```bash
cargo test
```

---

## 😍 Thanks

* [iola1999/nsfw-detect-onnx](https://github.com/iola1999/nsfw-detect-onnx) for the ONNX model
* The __Rust community__ and documentation for being excellent
* [ChatGPT](https://openai.com/chatgpt) for helping me to learn the basics of Rust and how to translate my knowledge from PHP. 

---

## 📄 License

This project is licensed under the **MIT License**, which permits commercial and non-commercial use, modification, distribution, and private use.

See [LICENSE](./LICENSE) for full details.

---

## 🚫 Disclaimer

This project is provided as-is. Use responsibly. The model may not be 100% accurate and should not be relied upon for legal or high-stakes decisions.
