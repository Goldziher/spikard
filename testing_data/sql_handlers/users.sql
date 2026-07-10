-- @returns :one
SELECT id, email, name, active, created_at
FROM users
WHERE id =
$1;

-- @returns :exec_rows
INSERT INTO users (email, name) VALUES ($1, $2);

-- @returns :many
SELECT id, email, name, active, created_at
FROM users
WHERE active = $1;
