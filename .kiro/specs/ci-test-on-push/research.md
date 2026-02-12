# Research & Design Decisions

## Summary
- **Feature**: `ci-test-on-push`
- **Discovery Scope**: Simple Addition
- **Key Findings**:
  - `dtolnay/rust-toolchain@stable`が現在のRust GitHub Actions標準であり、`actions-rs/toolchain`は非推奨
  - `Swatinem/rust-cache@v2`でCargo依存関係のキャッシュが簡潔に実現可能
  - 単一ジョブ構成で十分（build/test/fmt/clippy を順次実行）

## Research Log

### GitHub ActionsにおけるRustツールチェインセットアップ
- **Context**: CI環境でRust stable toolchainを構成する最適な方法の調査
- **Sources Consulted**: dtolnay/rust-toolchain GitHub リポジトリ、GitHub Actions公式ドキュメント
- **Findings**:
  - `dtolnay/rust-toolchain@stable`が推奨。`with: components: rustfmt, clippy`で追加コンポーネントを指定
  - `actions-rs/toolchain`は2023年10月に非推奨化済み
  - `actions/checkout@v4`が最新のチェックアウトアクション
- **Implications**: `dtolnay/rust-toolchain@stable`を採用、コンポーネント指定でfmt/clippy対応

### Cargo依存関係キャッシュ
- **Context**: CI実行時間短縮のためのキャッシュ戦略
- **Sources Consulted**: Swatinem/rust-cache GitHub リポジトリ
- **Findings**:
  - `Swatinem/rust-cache@v2`がデフォルト設定で`~/.cargo`とtargetディレクトリをキャッシュ
  - `Cargo.lock`のハッシュ値ベースでキャッシュキーを自動生成
  - 追加設定なしで基本的なユースケースに対応
- **Implications**: デフォルト設定で十分。`shared-key`等のオプションは単一ジョブ構成では不要

### ワークフロー構造
- **Context**: build/test/fmt/clippyの実行順序と構成
- **Sources Consulted**: Rust CI ベストプラクティス記事、GitHub Actions公式ガイド
- **Findings**:
  - 単一ジョブで`fmt --check` → `clippy` → `build` → `test`の順が推奨
  - fmtを最初に実行することで、フォーマット違反を最速で検出
  - `CARGO_TERM_COLOR: always`でCI上でもカラー出力を維持
  - `--verbose`フラグでデバッグ時の情報量を確保
- **Implications**: 単一ジョブ・順次実行で十分。並列ジョブ化は現段階では不要

## Design Decisions

### Decision: 単一ジョブ vs 複数ジョブ構成
- **Context**: CI pipeline の構成方法
- **Alternatives Considered**:
  1. 単一ジョブ — すべてのステップを1ジョブ内で順次実行
  2. 複数ジョブ — fmt/clippy/build/testを個別ジョブに分離
- **Selected Approach**: 単一ジョブ構成
- **Rationale**: プロジェクト規模が小さく、ジョブ分離によるオーバーヘッド（キャッシュ共有、ジョブ間依存）が利点を上回る
- **Trade-offs**: 1ステップの失敗で後続ステップがスキップされるが、これは早期失敗の原則に合致
- **Follow-up**: プロジェクト規模拡大時にジョブ分離を再検討

### Decision: pushトリガーの対象ブランチ
- **Context**: どのブランチへのpushでCIを実行するか
- **Alternatives Considered**:
  1. 全ブランチ — `on: push`（ブランチ指定なし）
  2. 特定ブランチのみ — `branches: [master]`
- **Selected Approach**: 全ブランチへのpush
- **Rationale**: 要件は「pushした時にtestできるようにする」であり、ブランチ制限は要求されていない。すべてのpushでテストを実行することでリグレッション検出を最大化
- **Trade-offs**: GitHub Actions の無料枠消費が増加するが、小規模プロジェクトでは問題にならない

## Risks & Mitigations
- Rust Edition 2024対応 — `dtolnay/rust-toolchain@stable`は最新stableを自動取得するため対応済み
- キャッシュ肥大化 — `Swatinem/rust-cache@v2`がキャッシュサイズを自動管理
- ワークフロー構文エラー — 初回push時にGitHub Actionsのログで即座に検出可能

## References
- [dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain) — Rust toolchain GitHub Action
- [Swatinem/rust-cache](https://github.com/Swatinem/rust-cache) — Cargo キャッシュ GitHub Action
- [actions/checkout](https://github.com/actions/checkout) — リポジトリチェックアウト Action
