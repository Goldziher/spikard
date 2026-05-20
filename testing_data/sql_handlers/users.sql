-- @name GetUser
-- @returns :one
-- @http GET /users/{id}
-- @http_auth bearer:jwt
-- @http_status 200,404
-- @http_summary Fetch one user by id
-- @http_tags users
SELECT id, email, name, active, created_at
FROM users
WHERE id = $1;

-- @name CreateUser
-- @returns :exec_rows
-- @http POST /users
-- @http_auth bearer:jwt
-- @http_status 201
-- @http_summary Create a new user
-- @http_tags users
-- @http_param email body
-- @http_param name body
INSERT INTO users (email, name) VALUES ($1, $2);

-- @name ListUsersByStatus
-- @returns :many
-- @http GET /users
-- @http_auth bearer:jwt
-- @http_summary List users filtered by active status
-- @http_tags users
-- @http_param active query
SELECT id, email, name, active, created_at
FROM users
WHERE active = $1;
