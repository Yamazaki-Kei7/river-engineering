# Requirements Document

## Introduction
本ドキュメントは、ハイエトグラフ（降雨時間分布図）を作成するRust製コマンドラインツール「hyetograph-cli」の要件を定義する。ハイエトグラフは降雨強度の時間変化を棒グラフで表現し、河川工学・都市排水設計・洪水解析において広く利用される。本ツールは、既存のExcel VBAマクロと同等の降雨強度計算・雨量分布配置ロジックをRustで再実装する。

## VBAアルゴリズム参照
本要件は以下のExcel VBAロジックに基づく:
- **降雨強度計算（RKEISAN）**: 降雨強度式 `K = C / ((T * I)^A + B)` により各時間ステップの増分雨量 `R(I) = K * I - ZK` を算出（ZKは前ステップまでの累計値 `K * I`）
- **時間ステップ数**: `NT = TT * 60 / T`（TT: 降雨継続時間[時間], T: 計算時間刻み[分]）
- **雨量分布配置**: 前方集中型（Case 1）、中央集中型（Case 2）、後方集中型（Case 3）の3パターン

## Requirements

### Requirement 1: 降雨強度計算
**Objective:** As a 河川技術者, I want 降雨強度式のパラメータを指定して各時間ステップの増分雨量を算出したい, so that ハイエトグラフの元データを得られる

#### Acceptance Criteria
1. When ユーザーが降雨強度係数A, B, Cと計算時間刻みT[分]および降雨継続時間TT[時間]を指定した場合, the hyetograph-cli shall 時間ステップ数 `NT = TT * 60 / T` を算出し、各ステップI（1〜NT）について降雨強度 `K = C / ((T * I)^A + B)` と増分雨量 `R(I) = K * I - ZK`（ZKは前ステップの累計値）を計算する
2. The hyetograph-cli shall 増分雨量R(I)を降雨強度の大きい順（I=1が最大）で保持する
3. The hyetograph-cli shall VBAの`RKEISAN`サブルーチンと同一の数値結果を出力する

### Requirement 2: 雨量分布配置
**Objective:** As a 河川技術者, I want 計算された増分雨量を時間軸上に任意のパターンで配置したい, so that 前方集中・中央集中・後方集中の降雨波形を作成できる

#### Acceptance Criteria
1. When ユーザーが前方集中型（パターン1）を選択した場合, the hyetograph-cli shall R(I)を時間軸の先頭から順に配置する（R(1)が最初のステップ、R(NT)が最後のステップ）
2. When ユーザーが中央集中型（パターン2）を選択した場合, the hyetograph-cli shall 奇数番目のR(I)を中央から後方へ、偶数番目のR(I)を中央から前方へ交互に配置する（VBAのCase 2と同一のロジック）
3. When ユーザーが後方集中型（パターン3）を選択した場合, the hyetograph-cli shall R(I)を時間軸の末尾から順に配置する（R(1)が最後のステップ、R(NT)が最初のステップ）
4. The hyetograph-cli shall デフォルトの雨量分布を中央集中型（パターン2）とする

### Requirement 3: グラフ出力
**Objective:** As a 河川技術者, I want ハイエトグラフを画像ファイルとして出力したい, so that 報告書やプレゼン資料に使用できる

#### Acceptance Criteria
1. The hyetograph-cli shall ハイエトグラフをPNG形式の棒グラフとして出力する
2. The hyetograph-cli shall グラフの横軸に経過時間（計算時間刻みに応じた表記）、縦軸に降雨強度（mm/h）を表示する
3. When ユーザーが出力ファイルパスを指定した場合, the hyetograph-cli shall 指定されたパスに画像ファイルを保存する
4. When ユーザーが出力ファイルパスを指定しなかった場合, the hyetograph-cli shall デフォルトのファイル名（`hyetograph.png`）でカレントディレクトリに出力する
5. The hyetograph-cli shall グラフのタイトル・軸ラベルを自動設定する

### Requirement 4: データ出力
**Objective:** As a 河川技術者, I want ハイエトグラフの数値データをCSVとして出力したい, so that 他のツールや計算で利用できる

#### Acceptance Criteria
1. When ユーザーがCSV出力オプションを指定した場合, the hyetograph-cli shall 経過時間[分]と降雨強度[mm/h]の列を持つCSVファイルを出力する
2. The hyetograph-cli shall CSV出力時にヘッダー行を含める
3. When ユーザーがグラフ出力とCSV出力の両方を指定した場合, the hyetograph-cli shall 両方の出力を生成する

### Requirement 5: コマンドラインインターフェース
**Objective:** As a ユーザー, I want 直感的なコマンドライン引数でツールを操作したい, so that 効率的にハイエトグラフを生成できる

#### Acceptance Criteria
1. The hyetograph-cli shall 降雨強度係数A, B, C、計算時間刻みT、降雨継続時間TTを必須引数として受け取る
2. The hyetograph-cli shall 雨量分布パターン（1: 前方集中, 2: 中央集中, 3: 後方集中）をオプション引数として受け取る（デフォルト: 2）
3. The hyetograph-cli shall 出力ファイルパスおよび出力形式（PNG/CSV/両方）をオプション引数として受け取る
4. When ユーザーが`--help`フラグを指定した場合, the hyetograph-cli shall 利用可能なオプション・使用例を表示する
5. When ユーザーが`--version`フラグを指定した場合, the hyetograph-cli shall バージョン情報を表示する
6. If 必須の引数が不足している場合, then the hyetograph-cli shall 不足している引数名を含むエラーメッセージとヘルプへの参照を表示する
7. The hyetograph-cli shall 終了コード0を正常終了、0以外をエラー終了として返す

### Requirement 6: エラーハンドリングとバリデーション
**Objective:** As a ユーザー, I want 誤った入力に対して明確なエラーメッセージを受け取りたい, so that 入力を修正して再実行できる

#### Acceptance Criteria
1. If 降雨強度係数A, B, Cまたは計算時間刻みTに0以下の値が指定された場合, then the hyetograph-cli shall パラメータ名と有効な範囲を含むエラーメッセージを表示する
2. If 降雨継続時間TTに0以下の値が指定された場合, then the hyetograph-cli shall エラーメッセージを表示する
3. If 雨量分布パターンに1〜3以外の値が指定された場合, then the hyetograph-cli shall 有効な選択肢を含むエラーメッセージを表示する
4. If `TT * 60 / T` が整数にならない場合, then the hyetograph-cli shall 時間刻みと降雨継続時間の組み合わせが不適切である旨のエラーメッセージを表示する
5. If 出力先ディレクトリが存在しない場合, then the hyetograph-cli shall ディレクトリパスを含むエラーメッセージを表示する
6. The hyetograph-cli shall すべてのエラーメッセージを標準エラー出力（stderr）に出力する
