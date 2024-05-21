#!/bin/bash

# エラーが発生した場合にスクリプトを終了する
set -e

# データベース接続情報を設定
export DATABASE_URL="postgres://username:password@localhost/database_name"

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