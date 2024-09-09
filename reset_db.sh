#!/bin/bash

# エラーが発生した場合にスクリプトを終了する
set -e

# .envファイルを読み込む
if [ -f .env ]; then
    export $(cat .env | xargs)
else
    echo ".envファイルが見つかりません"
    exit 1
fi

# データベース接続情報を設定 (環境変数から埋め込む)
export DATABASE_URL="postgres://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"

# データベースをドロップ
echo "Dropping the database..."
sqlx db drop -y

# データベースを再作成
echo "Creating the database..."
sqlx db create

# マイグレーションの実行
echo "Running migrations..."
sqlx migrate run

echo "Database reset and migrations applied successfully."