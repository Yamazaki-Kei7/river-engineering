# Requirements Document

## Introduction
本仕様は、hyetograph-cliプロジェクトにGitHub Actionsを導入し、pushイベント時にテストを自動実行するCIワークフローを定義するものである。現状、`.github/workflows/`ディレクトリは存在せず、CIは未構成である。プロジェクトはRust（Edition 2024）で構築されており、ユニットテスト（各モジュール内`#[cfg(test)]`）および統合テスト（`tests/e2e.rs`）が`cargo test`で実行可能な状態にある。

## Requirements

### Requirement 1: pushトリガーによるテスト自動実行
**Objective:** 開発者として、コードをpushした際にテストが自動実行されることで、リグレッションを早期に検出したい。

#### Acceptance Criteria
1. When コードがリポジトリにpushされた時, the CI Workflow shall `cargo test`を実行し、ユニットテストおよび統合テストをすべて実行する
2. When テストが全件パスした時, the CI Workflow shall ワークフローをsuccessステータスで完了する
3. If いずれかのテストが失敗した場合, the CI Workflow shall ワークフローをfailureステータスで完了し、失敗内容をログに出力する

### Requirement 2: Rustツールチェインのセットアップ
**Objective:** 開発者として、CI環境でRust stable toolchainが正しく構成されることで、ローカル環境と同一条件でテストを実行したい。

#### Acceptance Criteria
1. The CI Workflow shall Rust stable toolchainを使用してビルドおよびテストを実行する
2. The CI Workflow shall Cargo依存クレートのキャッシュを活用し、不要な再ダウンロードを抑制する

### Requirement 3: ビルド検証
**Objective:** 開発者として、テスト実行前にコンパイルが成功することを確認し、ビルドエラーとテストエラーを区別したい。

#### Acceptance Criteria
1. When テスト実行前に, the CI Workflow shall `cargo build`を実行し、コンパイルの成功を検証する
2. If コンパイルが失敗した場合, the CI Workflow shall ビルドエラーとしてワークフローをfailureステータスで完了する

### Requirement 4: コード品質チェック
**Objective:** 開発者として、pushのたびにフォーマットとlintが自動チェックされることで、コード品質を一定に保ちたい。

#### Acceptance Criteria
1. When テスト実行に加えて, the CI Workflow shall `cargo fmt -- --check`を実行し、フォーマット準拠を検証する
2. When テスト実行に加えて, the CI Workflow shall `cargo clippy -- -D warnings`を実行し、lint警告をエラーとして扱う
3. If フォーマット違反またはlint警告が検出された場合, the CI Workflow shall ワークフローをfailureステータスで完了する
