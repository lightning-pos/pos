# Database Migrations Guide

## Quick Start

```bash
# Create new migration
diesel migration generate <migration_name>

# Run migrations
DATABASE_URL="minnal.db" diesel migration run

# Revert last migration
DATABASE_URL="minnal.db" diesel migration revert
```

## Migration Structure

Each migration consists of:
- `up.sql`: Forward migration
- `down.sql`: Rollback migration

## Best Practices

### Table Structure
- Always include: `id` (TEXT), `created_at` (TIMESTAMP), `updated_at` (TIMESTAMP)
- Use appropriate constraints and foreign keys
- Create indexes for frequently queried columns
- Add triggers for automatic timestamp updates

### Example: Creating a Table

```sql
-- up.sql
CREATE TABLE table_name (
    id TEXT NOT NULL PRIMARY KEY,
    field TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY(field) REFERENCES other_table(id)
);

CREATE INDEX idx_table_name_field ON table_name(field);

CREATE TRIGGER trigger_table_name_updated_at
AFTER UPDATE ON table_name
BEGIN
    UPDATE table_name SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;

-- down.sql
DROP TRIGGER IF EXISTS trigger_table_name_updated_at;
DROP INDEX IF EXISTS idx_table_name_field;
DROP TABLE IF EXISTS table_name;
```

## SQLite ALTER TABLE Operations

### Supported Operations
1. **RENAME TABLE**
```sql
ALTER TABLE table_name RENAME TO new_table_name;
```

2. **RENAME COLUMN**
```sql
ALTER TABLE table_name RENAME COLUMN column_name TO new_column_name;
```

3. **ADD COLUMN**
```sql
ALTER TABLE table_name ADD COLUMN column_def;
```
Restrictions:
- No PRIMARY KEY/UNIQUE constraints
- NOT NULL requires default value
- Foreign keys must default to NULL
- No STORED generated columns (VIRTUAL allowed)

4. **DROP COLUMN**
```sql
ALTER TABLE table_name DROP COLUMN column_name;
```
Cannot drop columns that are:
- Part of PRIMARY KEY/UNIQUE constraints
- Referenced in indexes/foreign keys
- Used in triggers/views

## Complex Schema Changes Template

For unsupported operations, use this pattern:

```sql
PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;

-- Create new table with desired schema
CREATE TABLE new_table (
    -- new schema
);

-- Copy data
INSERT INTO new_table SELECT ... FROM table_name;

-- Replace old table
DROP TABLE table_name;
ALTER TABLE new_table RENAME TO table_name;

-- Verify and commit
PRAGMA foreign_key_check;
COMMIT;
PRAGMA foreign_keys=ON;
