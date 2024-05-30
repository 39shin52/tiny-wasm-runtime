# RustでWasm Runtimeを実装する
[こちら](https://zenn.dev/skanehira/books/writing-wasm-runtime-in-rust)を写経した
14章まで写経し、Hello Worldが表示されるのを確認した

## 動作(テスト)
1. `fixtures/`に`.wat`を配置
2. `#[test]`をつけてテスト関数作成、実行

## わかったこと
- wasm runtimeの動作(スタックとか)
- ランタイムがやっていること
- Rust言語をたくさん書けた