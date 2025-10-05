# crypto-checker-wasm

# Crypto Checker (Rust + WASM) 💰

[![GitHub Pages](https://img.shields.io/badge/GitHub%20Pages-Deployed-blue?logo=github)](https://[USERNAME].github.io/crypto-checker-wasm/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> Aplikasi crypto checker yang dibuat dengan Rust + WebAssembly.  
> *100% di-browser—nggak perlu install apa-apa!* 😎

## 🚀 Features

| Fitur | Deskripsi |
|-------|-----------|
| ✅ **Real-time Data** | Harga crypto update setiap 1 menit |
| 📦 **Multi-coin Support** | Bitcoin, Ethereum, Binance Coin |
| 🌐 **No API Key** | Pake CoinGecko API (gratis) |
| 🛠️ **Rust + WASM** | Performa tinggi dengan Rust, kompatibel browser |

## 🧰 Tech Stack

- **Rust**: Core logic & API calls
- **WebAssembly (WASM)**: Compile Rust ke browser
- **CoinGecko API**: Sumber data harga crypto
- **HTML/CSS/JS**: UI & rendering

## 🌐 Demo Live

[Live Demo](https://[USERNAME].github.io/crypto-checker-wasm/)  
*(Tunggu 10-30 detik setelah deploy)*

![Screenshot](https://i.imgur.com/ABC123.png)  
> *Contoh tampilan aplikasi di browser*

## 📦 Struktur Project

crypto-checker-wasm/
├── Cargo.toml        # Konfigurasi Rust
├── index.html        # UI & WASM loader
├── src/
│   └── lib.rs        # Rust code
└── README.md         # Dokumentasi

## 🧪 API Documentation

- **Endpoint**: `get_crypto_price(coin: &str) -> f64`
- **Contoh**:
  ```rust
  let price = get_crypto_price("bitcoin"); // $60,000
  
