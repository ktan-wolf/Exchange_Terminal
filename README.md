# 🧠 Web3 Terminal — High-Performance Arbitrage Dashboard

**Web3 Terminal** is a high-performance, real-time cryptocurrency arbitrage dashboard built with **Rust**, **Axum**, **Solana**, and **Next.js**.  
It provides a unified interface to monitor fragmented liquidity across **Centralized Exchanges (CEXs)** and **Decentralized Exchanges (DEXs)** in real-time.

> ⚡ Built for speed, concurrency, and reliability — powered by Rust on the backend and Next.js on the frontend.

---

## 🚀 Current Status (End of Sprint 2)

🟢 **In Active Development**

The terminal successfully:
- Streams and normalizes **live price data** from **Binance (CEX)** and a **Solana (DEX)** liquidity pool (Raydium).
- Displays unified, real-time data in a single dashboard.

---

## ✨ Features

### 🦀 High-Performance Rust Backend
- Built with **[Axum](https://github.com/tokio-rs/axum)** and **Tokio** for maximum concurrency.
- Designed for low-latency, high-throughput real-time data handling.

### 📡 Real-Time WebSocket Broadcasting
- A concurrent-safe **WebSocket server** broadcasts normalized price updates to all connected clients.

### 💱 Multi-Source Data Pipelines
- **CEX Data Pipeline:** Streams live trades from **Binance** WebSocket (`sol/usdt`).
- **DEX Data Pipeline:** Streams on-chain updates from **Raydium** (Solana `SOL/USDC` pool) using `solana-pubsub-client`.
