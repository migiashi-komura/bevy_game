# リンカのインストール（Windows）
詳しくはhttps://bevyengine.org/learn/quick-start/getting-started/setup/ を参照してください。

```bash
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

# 実行
```bash
git clone https://github.com/migiashi-komura/bevy_game.git
cd bevy_game
cargo run
``` 