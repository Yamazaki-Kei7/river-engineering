# プロジェクト構成

## 構成方針

`src/`内のフラットモジュール構成で、レイヤーの責務ごとに整理。各モジュールは1ファイル1責務。CLIツールに適したサイズであり、ネストしたモジュールディレクトリは不要。

## ディレクトリパターン

### ソースモジュール (`src/`)
**目的**: すべてのアプリケーションコードをフラットなRustモジュールとして配置
**パターン**: 1ファイル1責務、`main.rs`で`mod`宣言
**レイヤー構成**:
- CLI層: `cli.rs`（引数定義）、`validator.rs`（ドメインバリデーション）
- Core層: `rainfall.rs`（計算）、`distribution.rs`（配置）、`types.rs`（共有データ型）
- Output層: `chart.rs`（PNG描画）、`csv_writer.rs`（CSV出力）
- エントリポイント: `main.rs`（パイプラインの統合）

### 統合テスト (`tests/`)
**目的**: コンパイル済みバイナリを実行するエンドツーエンドテスト
**パターン**: `env!("CARGO_BIN_EXE_hyetograph-cli")`でバイナリを特定、`tempfile`で出力を隔離

### 仕様書 (`.kiro/specs/`)
**目的**: Spec-Driven Developmentワークフローに基づくフィーチャー仕様
**パターン**: `{feature-name}/`配下に`spec.json`、`requirements.md`、`design.md`、`tasks.md`、`research.md`

## 命名規則

- **ファイル**: `snake_case.rs`（Rust標準）
- **構造体/列挙型**: `PascalCase`（例: `RainfallParams`、`DistributionPattern`）
- **関数**: `snake_case`（例: `calculate`、`arrange`、`render`、`write`）
- **定数**: `SCREAMING_SNAKE_CASE`（例: `DEFAULT_WIDTH`、`VBA_EXPECTED`）

## モジュール依存ルール

```
main.rs
  -> cli.rs（引数解析）
  -> validator.rs（ドメインバリデーション、cli + typesに依存）
  -> rainfall.rs（計算、typesに依存）
  -> distribution.rs（配置、typesに依存）
  -> chart.rs（PNG出力、typesに依存）
  -> csv_writer.rs（CSV出力、typesに依存）
  -> types.rs（共有データ型、プロジェクト内依存なし）
```

- `types.rs`はプロジェクト内依存ゼロ（リーフモジュール）
- Core層モジュール（`rainfall`、`distribution`）は`types`のみに依存
- Output層モジュール（`chart`、`csv_writer`）は`types`のみに依存
- `validator`がCLI層とCore層を橋渡し

## コード構成原則

- **各モジュールにテストを内包**: ファイル末尾に`#[cfg(test)] mod tests`
- **公開APIは最小限**: 各モジュールは1〜2個のpublic関数のみ公開
- **データは一方向に流れる**: CLI引数 → バリデーション済みパラメータ → 増分雨量 → 配置済みエントリ → 出力ファイル
- **共有可変状態なし**: すべての関数は不変参照を受け取り、所有値を返す

---
_ファイルツリーではなくパターンを文書化すること。パターンに従う新規ファイルは更新不要_
