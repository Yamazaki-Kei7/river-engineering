# 技術スタック

## アーキテクチャ

レイヤードアーキテクチャ（3層構成）: CLI層 → Core層 → Output層の一方向依存。単一目的のCLIツールに適したシンプルさを重視して選定。

## コア技術

- **言語**: Rust（Edition 2024）
- **ビルド**: Cargo
- **ランタイム**: ネイティブバイナリ（外部ランタイム不要）

## 主要ライブラリ

| ライブラリ | 役割 | 使用パターン |
|-----------|------|-------------|
| clap v4.5 (derive) | CLI引数解析 | 構造体へのderive macro、列挙型に`ValueEnum` |
| plotters v0.3 | PNGチャート描画 | `BitMapBackend` + `ChartBuilder` + `Rectangle`要素 |
| csv + serde | CSV出力 | データ構造体に`Serialize` derive |
| anyhow | エラーハンドリング | 戻り値に`Result<()>`、バリデーションに`bail!`、I/Oに`.with_context()` |

## 開発規約

### 型安全性
- すべての数値計算にf64を使用（IEEE 754倍精度、VBA Doubleと一致）
- `any`や`unknown`相当の型は使用しない
- 制約付きCLI入力には`ValueEnum`付き列挙型

### エラーハンドリングパターン
- すべての失敗可能関数に`anyhow::Result`
- バリデーションエラー: パラメータ名と有効範囲を含む`bail!`
- I/Oエラー: `.with_context()`でファイルパス情報を付加
- すべてのエラーはstderrに出力、終了コード1

### テスト
- 各モジュール内にユニットテスト（`#[cfg(test)] mod tests`）
- `tests/`ディレクトリにバイナリ実行による統合テスト
- 既知パラメータセットによるVBA数値互換テスト
- テスト用ファイルI/O隔離に`tempfile`クレート

## 開発環境

### 必須ツール
- Rust stable toolchain（Edition 2024）
- Cargo

### よく使うコマンド
```bash
# ビルド: cargo build
# テスト: cargo test
# 実行:   cargo run -- 0.75 5.411 1557.825 10 2 --pattern center --format both
```

## 主要な技術的決定

- **Core層は純粋関数**: `rainfall::calculate`と`distribution::arrange`は状態なし・副作用なしの関数としてテスト容易性を確保
- **バリデーションと解析の分離**: clapが構文処理、`validator`モジュールがドメインルール（正値、NT整数、ディレクトリ存在）を処理
- **VBAの演算順序を保持**: f64の数値同一性を保証するため、VBAと同じ評価順序で算術演算を実行

---
_すべての依存関係ではなく、規約とパターンを文書化すること_
