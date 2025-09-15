# 本番環境デプロイメントチェックリスト

## 🔍 デプロイ前チェック

### システム要件
- [ ] Docker 20.10+ がインストール済み
- [ ] Docker Compose 2.0+ がインストール済み
- [ ] 十分なディスク容量 (最低2GB)
- [ ] メモリ 512MB以上

### セキュリティ設定
- [ ] `.env`ファイルが適切に設定されている
- [ ] 本番用データベースパスが設定されている
- [ ] CORS設定が本番ドメインに適合している
- [ ] ファイアウォール設定が適切 (ポート3000)

### データ準備
- [ ] データベースマイグレーションが正常動作
- [ ] バックアップディレクトリが作成済み
- [ ] ログディレクトリの権限が正しい

## 🚀 デプロイ手順

### 1. 初回デプロイ
```bash
# 1. リポジトリクローン
git clone https://github.com/riku-miura/memory_memo.git
cd memory_memo

# 2. 環境設定
cp .env.example .env
# .envファイルを編集

# 3. 本番デプロイ実行
./deploy.sh production
```

### 2. 動作確認
```bash
# ヘルスチェック
curl http://localhost:3000/health

# レスポンス時間確認
curl -w "%{time_total}\n" -o /dev/null -s http://localhost:3000/health

# フロントエンド確認
curl -I http://localhost:3000/
```

### 3. 機能テスト
- [ ] ユーザー登録が動作する
- [ ] ログインが動作する
- [ ] 永続メモの作成・編集・削除が動作する
- [ ] フラッシュメモの作成・削除が動作する
- [ ] レスポンシブデザインが正常表示される

## 📊 パフォーマンス検証

### レスポンス時間要件
- [ ] `/health` < 50ms
- [ ] `/api/auth/login` < 200ms
- [ ] `/api/memos` < 200ms
- [ ] 静的ファイル配信 < 100ms

### 負荷テスト (簡易版)
```bash
# 同時10リクエスト
for i in {1..10}; do
  curl -s http://localhost:3000/health > /dev/null &
done
wait
```

## 🔧 運用設定

### 定期バックアップ設定
```bash
# crontabに追加
0 2 * * * /path/to/memory_memo/scripts/backup.sh

# 週次バックアップ確認
0 9 * * 0 ls -la /path/to/memory_memo/backups/
```

### ログローテーション
```bash
# Docker logsの設定確認
docker-compose logs --tail=1000 > logs/app-$(date +%Y%m%d).log
```

### 監視設定
```bash
# ヘルスチェックcron (5分毎)
*/5 * * * * curl -f http://localhost:3000/health || echo "Health check failed: $(date)" >> /var/log/memory_memo_health.log
```

## 🚨 トラブルシューティング

### よくある問題と解決策

#### アプリケーションが起動しない
```bash
# ログ確認
docker-compose logs

# コンテナ状態確認
docker-compose ps

# 再起動
docker-compose restart
```

#### データベース接続エラー
```bash
# ボリューム確認
docker volume ls

# データベースファイル確認
docker exec memory_memo_memory-memo_1 ls -la /app/data/
```

#### ポート競合
```bash
# ポート使用状況確認
netstat -tuln | grep 3000

# プロセス特定・終了
lsof -ti:3000 | xargs kill -9
```

#### パフォーマンス低下
```bash
# リソース使用量確認
docker stats

# データベースサイズ確認
docker exec memory_memo_memory-memo_1 du -h /app/data/memory_memo.db
```

## 📈 スケーリング対応

### 負荷増加時の対応
1. **リソース監視**: CPU・メモリ使用率を定期チェック
2. **データベース最適化**: VACUUM、インデックス最適化
3. **ログローテーション**: 古いログファイルの削除
4. **コンテナリソース調整**: docker-compose.ymlでlimits設定

### 将来の改善案
- **PostgreSQL移行**: より高性能なデータベース
- **Redis導入**: セッションストレージとキャッシュ
- **負荷分散**: 複数コンテナでの分散処理
- **CDN導入**: 静的ファイルの配信最適化

## ✅ デプロイ完了確認

全ての項目が完了したら、本番環境への正式デプロイが完了です：

- [ ] 全てのヘルスチェックが通過
- [ ] パフォーマンス要件を満たしている
- [ ] セキュリティ設定が適切
- [ ] バックアップ・復旧手順が確立済み
- [ ] 監視・アラート体制が整備済み
- [ ] ドキュメントが最新状態

🎉 **Memory Memo 本番環境デプロイ完了！**

---
*最終更新: $(date +"%Y-%m-%d")*