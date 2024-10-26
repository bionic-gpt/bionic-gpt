#!/bin/bash

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "DATABASE_URL is not set."
    exit 1
fi

# Get the database name
DB_NAME=$(psql "$DATABASE_URL" -Atc "SELECT current_database();")

echo "## Database Schema: $DB_NAME"
echo

# Get the list of tables in the public schema
TABLES=$(psql "$DATABASE_URL" -Atc "
    SELECT table_name 
    FROM information_schema.tables 
    WHERE table_schema='public' AND table_type='BASE TABLE';
")

# For each table, get its columns, data types, and include table comments
for TABLE in $TABLES; do
    echo "### Table: $TABLE"
    
    # Get the table comment
    TABLE_COMMENT=$(psql "$DATABASE_URL" -Atc "
        SELECT obj_description('\"public\".\"$TABLE\"'::regclass, 'pg_class');
    ")

    # If there is a comment, include it under the table heading
    if [ -n "$TABLE_COMMENT" ]; then
        echo
        echo "_${TABLE_COMMENT}_"
        echo
    fi

    echo "| Column Name | Data Type | Nullable | Default Value |"
    echo "|-------------|-----------|----------|---------------|"
    psql "$DATABASE_URL" -F"|" --no-align -c "
        SELECT 
            column_name, 
            data_type, 
            is_nullable,
            column_default
        FROM information_schema.columns 
        WHERE table_name='$TABLE' AND table_schema='public'
        ORDER BY ordinal_position;
    " | while IFS="|" read -r COLUMN_NAME DATA_TYPE IS_NULLABLE COLUMN_DEFAULT; do
        # Replace empty default with 'NULL'
        [ -z "$COLUMN_DEFAULT" ] && COLUMN_DEFAULT="NULL"
        echo "| $COLUMN_NAME | $DATA_TYPE | $IS_NULLABLE | $COLUMN_DEFAULT |"
    done
    echo
done
