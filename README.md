# RHR-Stack Demo Site

一個基於高併發 Rust 後端與超媒體驅動前端的架構示範專案。本專案展示了如何將 Rust 的極致安全效能與 Datastar 的即時反應能力無縫結合，構建一個無需繁瑣 JSON 協議的現代化網站。

## 技術棧 (The RHR-Stack)

*   **Backend:** [Salvo](https://salvo.rs/) - 高效能 Web 服務框架，處理業務邏輯與 Server-Sent Events (SSE)。
*   **ORM:** [SeaORM](https://www.sea-ql.org/SeaORM/) - A powerful ORM for building web services in Rust.
*   **Frontend (UI):** [SolidStart](https://start.solidjs.com/) - 處理 SSR 與基礎架構。
*   **Frontend (Interaction):** [Datastar](https://data-star.dev/) - 透過 HTML-over-the-wire 技術直接進行即時 UI 更新。
*   **Runtime:** [Bun](https://bun.sh/) - 管理 Monorepo 與高效能 JavaScript 運行環境。

## 專案結構 (Monorepo)

```text
/src
├── /backend          # Rust Salvo 服務
├── /frontend         # SolidStart + daisyUI
├── /shared           # Rust 與 TypeScript 型別共享定義
└── /scripts          # 開發自動化腳本

```

## 核心亮點

1. **超低延遲：** 直接從 Rust 後端串流 HTML 片段至前端，跳過 JSON 序列化與複雜的客戶端重繪過程。
2. **型別同步：** 自動化的 `shared` 層設計，確保後端資料結構變動時，前端型別同步更新。
3. **即時反應：** 利用 SSE 與 Datastar Signals，實現無需前端複雜邏輯的即時狀態同步。

## 快速開始

### 環境需求

* [Rust](https://rustup.rs/)
* [Bun](https://bun.sh/)

### 初始化

1. **安裝依賴：**
```bash
bun install

```


2. **啟動開發環境：**
```bash
# 啟動後端
cd src/backend && cargo run

# 啟動前端
cd src/frontend && bun run dev

```



## 專案願景

本專案旨在展示一種比傳統 SPA 更簡潔、比原生 SSR 更具備互動性的架構模型。我們將「反應性」下放至瀏覽器 DOM，將「邏輯」上推至 Rust 後端，實現架構的極致簡化與高效能。

---

*Powered by RHR-Stack: Rust, Hypermedia, and Reactive.*

