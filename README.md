# Table Snapshot

## 主キー判定について ( MySQL )

Null にできない重複のないカラム ( = `information_schema.columns.column_key` が `PRI` ) を主キーとして扱う

複合主キーも扱えるが、主キーがないテーブルは抽出対象外となる

### パターン: 1 カラム

| col      | constraint       | column_key | is_nullable | pick up |
|:---------|:-----------------|:-----------|:------------|:--------|
|          | primary key(col) | PRI        | NO          | col     |
|          | unique(col)      | UNI        | YES         | x       |
| not null | unique(col)      | PRI        | NO          | col     |

### パターン: 2 カラム ( multi primary )

| col1 | col2 | constraint              | column_key | is_nullable | pick up   |
|:-----|:-----|:------------------------|:-----------|:------------|:----------|
|      |      | primary key(col1, col2) | PRI<br>PRI | NO<br>NO    | col1-col2 |

### パターン: 2 カラム ( primary + unique )

| col1 | col2     | constraint                        | column_key | is_nullable | pick up |
|:-----|:---------|:----------------------------------|:-----------|:------------|:--------|
|      |          | primary key(col1)<br>unique(col2) | PRI<br>UNI | NO<br>YES   | col1    |
|      | not null | primary key(col1)<br>unique(col2) | PRI<br>UNI | NO<br>NO    | col1    |

### パターン: 2 カラム ( unique + unique )

| col1     | col2     | constraint                   | column_key | is_nullable | pick up |
|:---------|:---------|:-----------------------------|:-----------|:------------|:--------|
|          |          | unique(col1)<br>unique(col2) | UNI<br>UNI | YES<br>YES  | x       |
| not null |          | unique(col1)<br>unique(col2) | PRI<br>UNI | NO<br>YES   | col1    |
| not null | not null | unique(col1)<br>unique(col2) | PRI<br>UNI | NO<br>NO    | col1    |

### パターン: 2 カラム ( multi unique )

| col1     | col2     | constraint         | column_key | is_nullable | pick up   |
|:---------|:---------|:-------------------|:-----------|:------------|:----------|
|          |          | unique(col1, col2) | MUL        | YES<br>YES  | x         |
| not null |          | unique(col1, col2) | MUL        | NO<br>YES   | x         |
| not null | not null | unique(col1, col2) | PRI<br>PRI | NO<br>NO    | col1-col2 |
