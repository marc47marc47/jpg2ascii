# 開發文檔（jpg2ascii）

本專案目標：以 Rust 實作「將圖片轉成 ASCII 藝術」的函式庫與命令列工具。倉庫中同時包含 `image2ascii/`（Go 版本參考實作），僅作為演算法與行為參考，不參與 Rust 專案的編譯或發佈。

## 環境需求
- Rust 稳定版工具鏈：建議以 `rustup` 安裝與管理
- 支援系統：Windows / macOS / Linux（終端需等寬字型）
- 建議工具：`cargo`、`rustfmt`、`clippy`

快速安裝（示例）
- 安裝 rustup：https://rustup.rs
- 更新工具鏈：`rustup update`

注意：目前 `Cargo.toml` 中 `edition = "2024"` 不是穩定可用版本，實作時請調整為 `2021`（見 TODO）。

## 專案結構
- `Cargo.toml`：Rust 專案定義
- `src/main.rs`：目前為範例入口（尚未接上功能）
- `image2ascii/`：Go 參考實作與測試資料（不參與 Rust 編譯）

建議未來結構
- 建立 `src/lib.rs` 作為核心庫（轉換流程、配置、字元映射、顏色渲染）
- `src/main.rs` 專注於 CLI（參數解析與 I/O）

## 常用指令
- 編譯除錯版：`cargo build`
- 執行：`cargo run -- <參數>`（功能完成後生效）
- 編譯釋出版：`cargo build --release`
- 格式化：`cargo fmt`
- 靜態檢查：`cargo clippy -- -D warnings`
- 測試：`cargo test`

Windows PowerShell 提示
- 若使用彩色輸出，需確保終端支援 ANSI/VT；Windows 10+ 預設通常可用。

## 開發工作流建議
1. 建立分支：以功能/修復為單位
2. 開發過程保持可編譯、可測試
3. 提交前執行：`cargo fmt`、`cargo clippy -- -D warnings`、`cargo test`
4. PR 說明清楚動機、設計與測試覆蓋點

## 設計要點（建議）
- 讀圖：使用 `image` crate（支援 JPG/PNG/…）
- 縮放：保持寬高比，考慮字元寬高比（一般字元約高是寬的 ~2 倍）
- 灰階：採用感知亮度（例：0.2126R + 0.7152G + 0.0722B）
- 字元映射：提供可調字元階梯（例：` .:-=+*#%@`）與反相選項
- 彩色：以 ANSI 256 色或 TrueColor 渲染前景（可選關閉）
- 輸出：終端輸出與檔案輸出（純文字或含 ANSI 碼）
- 效能：可用 `rayon` 併行處理像素行/區塊
- CLI：以 `clap` 解析參數，提供寬度/高度/比例、字元集、反相、顏色、對比/伽瑪等

## 測試策略
- 單元測試：灰階計算、字元映射、尺寸計算
- 快照測試：對固定輸入圖片產生穩定 ASCII 結果（注意跨平台差異）
- 基準測試：以 `criterion` 量測在不同尺寸/模式下的效能
- 參考對照：可對照 `image2ascii/` 輸出行為（僅作輔助）

## 版本與發佈
- 標記版本：遵循 SemVer
- 產出二進位：`target/release/jpg2ascii`
- 如需發佈 crate：補齊 `Cargo.toml` metadata、README、範例與文件

## 常見問題
- 終端顏色亂碼或無色：確認終端支援 ANSI；必要時提供 `--no-color`
- 圖片看起來被壓扁：調整「字元寬高比補償」或指定輸出寬度
- 等寬字體：請確保終端使用等寬字體，避免對齊錯亂

## 貢獻
- 歡迎 Issue/PR，請遵守上述工作流與檢查清單
- 變更涉及公共 API 時，請更新文件與範例並附測試

