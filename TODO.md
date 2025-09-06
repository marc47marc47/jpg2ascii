# TODO

- [x] 初始功能：讀圖、縮放、字元映射、輸出

## 基礎功能
- [x] 讀取常見影像格式（先支援 JPG/PNG，之後視需要擴充）
- [x] 圖片縮放：保持寬高比；提供輸出寬度/高度或縮放比例
- [x] 字元寬高比補償（字元通常偏高）：避免輸出被壓扁
- [x] 灰階計算：採用感知亮度（0.2126R + 0.7152G + 0.0722B）
- [x] ASCII 映射：可選字元階梯、支援反相與自定義字元集
- [x] 輸出：終端與檔案（純文字；可選含 ANSI 顏色）

## 色彩與顯示
- [x] ANSI TrueColor 渲染前景；提供 `--no-color`/`--color`
- [x] 亮度/對比/伽瑪/閾值等可調參數（新增 `--brightness`）
- [x] 端末相容性偵測與降級策略（TTY/`NO_COLOR`/`TERM` + Windows 啟用 VT）

## CLI 介面（使用 clap）
- [x] 參數：`--width/--height/--scale`、`--charset`、`--invert`、`--color`、`--no-color`、`--gamma`、`--contrast`、`--brightness`、`--threshold`、`--aspect`
- [x] 輸入：檔案路徑；輸出到 STDOUT 或指定檔
- [x] 輸入：支援從 STDIN 讀取圖片資料（以 `-` 表示）
- [x] 例子程式：`examples/basic.rs`

## 效能
- [x] 以 `rayon` 併行處理像素行
- [x] 縮放演算法：預設三角濾波（可再提供選項）
- [x] 避免重複配置與複製：一次轉為 `rgba8` 後計算

## 進階格式（選做）
- [x] GIF 動畫逐幀轉換（初版：支援 `--animate` 與 `--fps`，清屏；不循環）
- [ ] 影片/攝影機串流（僅在明確需求下）

## 函式庫 API
- [x] `Config` 結構（尺寸、色彩、字元集等）
- [x] `convert_path_to_ascii` / `convert_image_to_ascii(String)`
- [x] `convert_image_to_ascii_lines(img, &Config) -> Vec<String>`（快照/逐行處理友好）
- [x] 清晰的錯誤型別與結果（`anyhow::Result`）

## 測試與基準
- [x] 單元測試：灰階、映射、縮放（基礎）
- [x] 快照測試：固定輸入圖比對輸出文本（注意跨平台差異）
- [x] 基準測試：使用 `criterion` 衡量不同圖片尺寸與模式
- [x] 測試樣本：暫用 `image2ascii/convert/testdata` 中的圖片

## 文件與示例
- [x] README：使用方式、範例輸出截圖、參數說明（初版）
- [x] DEVELOP：隨實作更新細節與指南（已建立）
- [x] 範例程式：`examples/` 內提供最小可行示例

## CI/CD 與品質
- [x] GitHub Actions：`fmt`、`clippy`、`test` 初版流程
- [x] PR Gate：禁止含警告提交與未格式化程式碼（fmt 檢查 + clippy -D warnings + build -D warnings）

## 相容性與平台
- [x] Windows PowerShell/Terminal 的 ANSI 支援：嘗試啟用 VT 模式
- [x] 等寬字體與換行注意事項：已在 README/DEVELOP 標註

## 清理與結構
- [ ] 決定 `image2ascii/`（Go 參考）如何保留：
  - 標明僅作參考、不參與編譯；或
  - 只保留測試圖片到 `assets/`，其餘移除/子模組化
- [x] 規劃將核心邏輯抽離到 `src/lib.rs`，`main.rs` 只做 CLI

## 元問題
- [x] 修正 `Cargo.toml` 中 `edition = "2024"` → `2021`
- [ ] 補齊 `Cargo.toml` metadata（description、repository、license 等）
