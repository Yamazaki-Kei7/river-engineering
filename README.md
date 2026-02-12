# hyetograph-cli

降雨強度式のパラメータから交互ブロック法に基づくハイエトグラフ（降雨時間分布図）を生成するRust製CLIツール。

## 特徴

- Sherman型降雨強度式 `K = C / ((T * I)^A + B)` による増分雨量計算
- 3種の雨量分布パターン（前方集中・中央集中・後方集中）
- PNG棒グラフ / CSVデータの出力

## インストール

```bash
cargo install --path .
```

## 使い方

```bash
# 基本（中央集中型、PNG出力）
hyetograph-cli 0.75 5.411 1557.825 10 2

# パターンと出力形式を指定
hyetograph-cli 0.75 5.411 1557.825 10 2 --pattern front --format csv

# PNG + CSV 同時出力
hyetograph-cli 0.75 5.411 1557.825 10 2 --format both --output result.png
```

### 引数

| 引数 | 説明                       | 必須 |
| ---- | -------------------------- | ---- |
| `A`  | 降雨強度係数（べき乗指数） | Yes  |
| `B`  | 降雨強度係数（加算定数）   | Yes  |
| `C`  | 降雨強度係数（分子定数）   | Yes  |
| `T`  | 計算時間刻み [分]          | Yes  |
| `TT` | 降雨継続時間 [時間]        | Yes  |

### オプション

| オプション  | 説明                                         | デフォルト       |
| ----------- | -------------------------------------------- | ---------------- |
| `--pattern` | 雨量分布パターン (`front`, `center`, `rear`) | `center`         |
| `--output`  | 出力ファイルパス                             | `hyetograph.png` |
| `--format`  | 出力形式 (`png`, `csv`, `both`)              | `png`            |

## 開発

```bash
# ビルド
cargo build

# テスト
cargo test

# 実行（開発時）
cargo run -- 0.75 5.411 1557.825 10 2 --pattern center --format both
```

## ライセンス

MIT
