# Research & Design Decisions

## Summary
- **Feature**: `hyetograph-cli`
- **Discovery Scope**: New Feature（グリーンフィールドのRust CLIツール）
- **Key Findings**:
  - `plotters`クレートがPure RustでPNG棒グラフ出力に最適（BitMapBackend利用）
  - `clap` v4.5（derive macro）がCLI引数解析の標準的選択肢
  - VBAの降雨強度式はf64浮動小数点で十分な精度を確保可能

## Research Log

### Rustプロッティングライブラリの選定
- **Context**: PNG形式の棒グラフ出力が必要（要件3.1）
- **Sources Consulted**:
  - [plotters GitHub](https://github.com/plotters-rs/plotters) — Pure Rust描画ライブラリ
  - [charming GitHub](https://github.com/yuankunzhang/charming) — Apache EChartsベースのRustラッパー
- **Findings**:
  - `plotters`: Pure Rust実装、BitMapBackendでPNG直接出力、外部ランタイム不要。Histogramサポートあり。ChartBuilder APIで軸ラベル・タイトル設定が容易
  - `charming`: ECharts準拠の高品質チャートだが、PNG出力にヘッドレスブラウザまたはSSR環境が必要で依存関係が重い
- **Implications**: `plotters`を採用。CLIツールとして外部依存を最小化し、クロスプラットフォーム対応を容易にする

### CLIフレームワークの選定
- **Context**: コマンドライン引数の解析・バリデーション（要件5, 6）
- **Sources Consulted**:
  - [clap crates.io](https://crates.io/crates/clap) — v4.5.58
  - [clap docs.rs](https://docs.rs/clap/latest/clap/)
- **Findings**:
  - `clap` v4.5: derive macroで型安全な引数解析、ValueEnumで列挙型のバリデーション、自動ヘルプ生成・バージョン表示
  - `--help`, `--version`がデフォルトで提供される
  - CLAUDE.mdの「classを使わない」制約はRustには該当しない（構造体+traitパターンが標準）
- **Implications**: `clap` v4.5をderive featureで使用。DistributionPatternをValueEnumで定義

### CSV出力ライブラリ
- **Context**: 数値データのCSVエクスポート（要件4）
- **Sources Consulted**:
  - [csv crates.io](https://crates.io/crates/csv)
  - [BurntSushi/rust-csv GitHub](https://github.com/BurntSushi/rust-csv)
- **Findings**:
  - `csv`クレート: Serdeサポートによる構造体の直接シリアライズ、Writer APIでヘッダー行自動出力
  - 事実上の標準ライブラリ
- **Implications**: `csv` + `serde`でCSV出力を実装

### 数値精度の検討
- **Context**: VBAのDouble型との互換性（要件1.3）
- **Findings**:
  - VBA Double = IEEE 754 64ビット浮動小数点 = Rustのf64と同一
  - 演算順序が同一であれば数値結果は一致する
- **Implications**: f64を全面的に使用。VBAと同一の演算順序を維持する

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| Layered (CLI → Core → Output) | 3層構成: 入力解析、計算ロジック、出力生成 | シンプル、テスト容易、依存方向が明確 | 大規模拡張には柔軟性不足 | CLIツールとして最適なサイズ |
| Hexagonal | ポート&アダプター | 出力先の差し替えが容易 | 小規模CLIには過剰 | 不採用 |

## Design Decisions

### Decision: アーキテクチャパターン
- **Context**: 単一バイナリのCLIツールとしてのアーキテクチャ
- **Alternatives Considered**:
  1. Layered — CLI層、Core層（計算）、Output層（出力）の3層
  2. Hexagonal — ポート&アダプターによる抽象化
- **Selected Approach**: Layered（3層構成）
- **Rationale**: 単一目的のCLIツールであり、入出力のバリエーションが限定的。シンプルな層構成が実装・テスト・保守すべてにおいて合理的
- **Trade-offs**: 将来的に出力先を大幅に増やす場合はリファクタリングが必要だが、現スコープでは不要

### Decision: プロッティングライブラリ
- **Context**: PNG棒グラフ出力の実現手段
- **Alternatives Considered**:
  1. `plotters` — Pure Rust、BitMapBackend
  2. `charming` — EChartsベース、高品質だが重い依存
- **Selected Approach**: `plotters`
- **Rationale**: 外部依存なしでPNG出力可能。CLIツールの配布・クロスプラットフォーム対応に有利
- **Trade-offs**: EChartsほどの見た目の洗練さはないが、技術報告書向けとしては十分

### Decision: エラーハンドリング戦略
- **Context**: CLIツールとしてのエラー報告方法
- **Selected Approach**: `anyhow`（アプリケーションエラー）+ カスタムバリデーションエラー型
- **Rationale**: CLIツールではエラー型の外部公開が不要。`anyhow`でコンテキスト付きエラーチェインを簡潔に実現

## Risks & Mitigations
- `plotters`の棒グラフ描画はRectangle要素の手動配置が必要 → サンプルコードを参考に実装
- VBAとの数値一致性検証が必要 → 既知のパラメータでの検算テストを作成
- 日本語フォントの文字化け可能性（グラフタイトル・ラベル） → 英語ラベルをデフォルトとし、フォント設定はオプション

## References
- [plotters GitHub](https://github.com/plotters-rs/plotters) — Rust描画ライブラリ
- [clap docs.rs](https://docs.rs/clap/latest/clap/) — CLI引数解析
- [csv crates.io](https://crates.io/crates/csv) — CSV読み書き
- [plotters ドキュメント](https://plotters-rs.github.io/) — 公式ドキュメント
