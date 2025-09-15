# Memory Memo

シンプルで美しいメモ管理アプリケーション。永続メモと24時間で自動削除されるフラッシュメモの両方をサポートします。

## ✨ 特徴

- **永続メモ**: 削除するまで永続的に保存されるメモ
- **フラッシュメモ**: 24時間後に自動削除されるメモ  
- **ユーザー認証**: セキュアなユーザー登録・ログイン
- **レスポンシブデザイン**: デスクトップ・モバイル対応
- **リアルタイム更新**: 30秒ごとの自動データ更新
- **高速パフォーマンス**: <200msの応答時間

## 🛠 技術スタック

### Backend
- **Rust** + **Axum** - 高性能Web API
- **SQLite** + **SQLx** - 軽量データベース
- **bcrypt** - パスワードハッシュ化
- **UUID** - ユニークID生成
- **TDD** - テスト駆動開発

### Frontend  
- **Vanilla JavaScript** - シンプルなSPA
- **HTML5** + **CSS3** - セマンティックマークアップ
- **Inter & Noto Sans JP** - 美しいタイポグラフィ
- **Flat Design** - ミニマルUI

### Infrastructure
- **Docker** + **Docker Compose** - コンテナ化
- **SQLite** - ファイルベースDB (本番環境対応)

## 🚀 デプロイメント

### 自動デプロイ (推奨)

```bash
# リポジトリをクローン
git clone https://github.com/riku-miura/memory_memo.git
cd memory_memo

# 本番環境でデプロイ
./deploy.sh production
```

### 手動デプロイ

```bash
# 環境設定
cp .env.example .env
# .envファイルを編集

# Docker Composeでデプロイ
docker-compose up -d --build

# ヘルスチェック
curl http://localhost:3000/health
```

## 🔧 開発環境セットアップ

### 必要な環境
- Rust 1.75+
- Docker & Docker Compose
- curl (テスト用)

### 開発サーバー起動

```bash
# バックエンド (ターミナル1)
cd backend
cargo run

# フロントエンド確認
# http://localhost:3000 でアプリにアクセス
```

### テスト実行

```bash
# 全テスト実行
cd backend
cargo test

# 統合テスト
./test_integration.sh

# E2Eテスト  
cargo test --test e2e_test -- --nocapture
```

## 📊 API エンドポイント

### 認証
- `POST /api/auth/register` - ユーザー登録
- `POST /api/auth/login` - ログイン
- `POST /api/auth/logout` - ログアウト

### メモ管理
- `GET /api/memos` - メモ一覧取得
- `POST /api/memos/forever` - 永続メモ作成
- `POST /api/memos/flush` - フラッシュメモ作成
- `PUT /api/memos/forever/:id` - 永続メモ更新
- `DELETE /api/memos/forever/:id` - 永続メモ削除
- `DELETE /api/memos/flush/:id` - フラッシュメモ削除

### システム
- `GET /health` - ヘルスチェック

## 🔐 セキュリティ

- **パスワードハッシュ化**: bcryptによるセキュアなハッシュ
- **セッション管理**: HttpOnlyクッキーによる認証
- **CORS設定**: 適切なクロスオリジン設定
- **入力検証**: フロントエンド・バックエンド両方で検証
- **SQLインジェクション対策**: SQLxによるプリペアドステートメント

## 💾 データバックアップ

```bash
# データベースバックアップ
./scripts/backup.sh

# バックアップから復元
./scripts/restore.sh ./backups/memory_memo_backup_YYYYMMDD_HHMMSS.db
```

## 📈 監視・メンテナンス

### ログ確認
```bash
# アプリケーションログ
docker-compose logs -f

# リアルタイムログ
docker-compose logs -f --tail=100
```

### パフォーマンス監視
```bash
# ヘルスチェック
curl http://localhost:3000/health

# レスポンス時間測定
curl -w "@curl-format.txt" -o /dev/null -s http://localhost:3000/health
```

## 🎯 本番環境設定

### 環境変数

```bash
# .env ファイル
DATABASE_URL=sqlite:///app/data/memory_memo.db
PORT=3000
RUST_LOG=info
RUST_ENV=production
```

### リバースプロキシ設定 (Nginx例)

```nginx
server {
    listen 80;
    server_name rikumiura.com;
    
    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## 📋 要件仕様

このアプリケーションは以下の仕様に基づいて開発されています：

- **Constitutional Principles**: Simplicity First, Performance <200ms
- **TDD Development**: 全機能がテスト駆動で開発
- **Rust Best Practices**: エラーハンドリング、型安全性
- **Modern Web Standards**: セマンティックHTML、アクセシビリティ

## 🤝 貢献

1. フォークしてクローン
2. フィーチャーブランチ作成 (`git checkout -b feature/amazing-feature`)
3. テスト付きでコミット (`git commit -m 'Add amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエスト作成

## 📄 ライセンス

このプロジェクトはMITライセンスの下で公開されています。

## 👤 作者

**Riku Miura**
- GitHub: [@riku-miura](https://github.com/riku-miura)
- Website: https://rikumiura.com

---

**Memory Memo** - シンプルで美しいメモ管理 ✨