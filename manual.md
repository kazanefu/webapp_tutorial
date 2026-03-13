# 環境構築

```bash
# ルートに移動
cd ~

# 必要なものをインストール
sudo apt update
sudo apt install build-essential pkg-config libssl-dev sqlite3 libsqlite3-dev

# Rustをインストール
curl https://sh.rustup.rs -sSf | sh
# 環境変数にRust関連のもの一式を追加
. "$HOME/.cargo/env"
# バージョンをださせてインストール成功を確認する
rustc --version

# nodeのインストール
curl -fsSL https://deb.nodesource.com/setup_24.x | sudo -E bash - 

sudo apt install nodejs -y

# バージョンを出させてインストール成功を確認する
node -v

# なければprojectsディレクトリを作成して移動
mkdir projects && cd projects
 
# projectのディレクトリを作成する(ここではweb_testとする)
mkdir web_test && cd web_test

# gitで管理する
git init
git branch -M main

#任意のテキストエディタで編集する(ここではVSCode)
code .

# frontendをプロジェクトを作る
npm create vite@latest frontend
Need to install the following packages:
create-vite@9.0.1
Ok to proceed? (y) y

> npx
> "create-vite" frontend

│
◇  Select a framework:
│  React
│
◇  Select a variant:
│  TypeScript


added 173 packages, and audited 174 packages in 13s

49 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
│
◇  Starting dev server...

> frontend@0.0.0 dev
> vite

cd frontend
npm install
npm install react-router-dom
# ここまででフロントエンドのプロジェクトが完成

# プロジェクトのルートディレクトリに移動(ここではweb_test/)してください
# バックエンドのプロジェクトを作る(ここでは上位のディレクトリでgitが動いている想定で--vcs none)
cargo new backend --vcs none
cd backend
```
## VScodeの入れることを推奨する拡張機能

- rust-analyzer
- Dependi

# バックエンドを作る

まず`backend/`の中身を見てみましょう。

- src/ ソースコードをここに書く
- Cargo.toml 依存関係だったりビルド情報だったりを書く

まずはCargo.tomlに依存関係を追加してください。今回追加するものは以下の通りです。Cargo.tomlの`dependencies`の欄に書いてください
```toml
axum = "0.8.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4"] }
tower-http = { version = "0.6.8", features = ["cors"] }
sqlx = { version = "0.8.6", features = ["sqlite", "runtime-tokio","runtime-tokio-rustls", "macros"] }
bcrypt = "0.19.0"
anyhow = "1"
```
これを保存すると`Cargo.lock`というファイルが自動で作られます。ここには依存の厳密なものが書かれています。人間がこれを直接編集することはありません。

また、`target`っていうフォルダも作られます。ここにはビルドしてできたものとかキャッシュとかいろいろ入りますがgitでは無視したいです。そのため`backend/`に`.gitignore`ファイルを作成してそこに`target/`と書き込んでください。

次にソースコードを書いていきます。`src/`の中に`main.rs`というファイルがあることを確認してください。多分最初は`Hello, world!`を表示するコードが書いてあると思います。それを確認したうえで`backend/`で`cargo run`というコマンドを打ってみてください。Hello, world!が表示されればちゃんと環境が作られていることが確認できます。`main.rs`の中の`main`関数がRustコードのエントリーポイントです。今回は小さなチュートリアルなので`main.rs`ファイルだけでコードを書きますが実際は複数のファイルで機能別に分けて開発します。ではコードをバックエンドのためのコードに書き換えて行きましょう。github上の完成しているコードを見ながら書いていってください。簡単なチュートリアルなので一応このコードの説明ができるくらいにはちゃんと理解しておいてほしいです。またできれば[The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/)を読んでおいてほしいです。

# フロントエンドを作る

まずはちゃんと動作する環境担っているかを確認するために`frontend/`で`npm run dev`というコマンドを実行してブラウザで表示してみてくださいボタンを押したらカウントアップされていくものが実行されると思います。

フロントエンドではまず一番最初に`index.html`が読み込まれます。`index.html`のbodyの中にscriptがありそこで`src="/src/main.tsx"`とソースコードが指定されていて見ての通り、`/src/main.tsx`が指定されているのでこれがエントリーポイントになります。

ではソースコードを編集していきましょう。まず`main.tsx`をgithub上のと同じように書いてください。次に`App.tsx`もgithub上のと同じように書いてください。そしたら`pages/Signup.tsx`と`pages/Login.tsx`を作ってこれらもgithub上のと同じように書いてください。そしてここまで書いたものについてもコードを説明できる程度には理解してください。

# 実行してみよう

まず、`backend/`のなかで`cargo run`でバックエンドを実行してください。次に`frontend`のなかで`npm run dev`でフロントエンドを実行する。

ここではusernameとpasswordを入力してアカウントを作ってUIDが取得でき、Login画面でUIDとpasswordを入力することでusernameを取得できるというものができていることが確認できるはずです。

もしバグとかがあったりエラーが出たら自分で調べながら頑張って直してみてください

# 追加課題

自力でLoginできている状態であればusernameを更新できる機能を追加してみてください