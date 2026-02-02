# GraphQL Integration

Spikard provides first-class GraphQL support through the `spikard-graphql` crate, which integrates with the HTTP server and middleware stack. GraphQL queries, mutations, and subscriptions work with tower-http middleware like compression, rate limiting, and authentication.

## What You Get

The `spikard-graphql` crate provides:

- **async-graphql integration** - Full-featured GraphQL executor built on async-graphql
- **Handler trait implementation** - GraphQL routes integrate with Spikard's router like any HTTP handler
- **Middleware compatibility** - All tower-http middleware applies transparently to GraphQL requests
- **Language-agnostic design** - Thin bindings expose GraphQL through Python, TypeScript, Ruby, and PHP
- **Schema building** - Type-safe schema construction with query, mutation, and subscription support
- **Error handling** - GraphQL-spec-compliant error responses with structured JSON

## Setting Up a GraphQL Schema

Every GraphQL server starts with a schema. Define your types, then create a schema builder.

=== "Rust"

    ```rust
    use spikard_graphql::{SchemaBuilder, QueryOnlyConfig};

    // Define a simple query type
    pub struct Query;

    impl Query {
        pub async fn hello(&self) -> String {
            "Hello, GraphQL!".to_string()
        }

        pub async fn greet(&self, name: String) -> String {
            format!("Hello, {}!", name)
        }
    }

    // Create a schema with query-only support
    let config = QueryOnlyConfig::builder()
        .introspection_enabled(true)
        .build();

    let schema = SchemaBuilder::new()
        .query_only(config)
        .build();
    ```

=== "Python"

    ```python
    from spikard import GraphQLSchemaBuilder, GraphQLSchemaConfig

    # Create a GraphQL schema configuration
    builder = GraphQLSchemaBuilder()
    builder.enable_introspection(True)
    builder.complexity_limit(5000)
    builder.depth_limit(50)

    config = builder.build()

    # Note: Python bindings expose schema configuration;
    # actual schema types are defined in Python classes with
    # appropriate type hints and decorators.
    ```

=== "TypeScript"

    ```typescript
    import { GraphQLSchemaBuilder, GraphQLSchemaConfig } from 'spikard';

    // Define query type using TypeScript classes
    class Query {
      async hello(): Promise<string> {
        return 'Hello, GraphQL!';
      }

      async greet(name: string): Promise<string> {
        return `Hello, ${name}!`;
      }
    }

    // Create schema configuration
    const builder = new GraphQLSchemaBuilder();
    builder.enableIntrospection(true);
    builder.complexityLimit(5000);
    builder.depthLimit(50);

    const config = builder.build();
    ```

=== "Ruby"

    ```ruby
    require 'spikard'

    # Define query type using Ruby methods
    class Query
      def hello
        'Hello, GraphQL!'
      end

      def greet(name:)
        "Hello, #{name}!"
      end
    end

    # Create schema configuration
    builder = Spikard::GraphQLSchemaBuilder.new
    builder.enable_introspection(true)
    builder.complexity_limit(5000)
    builder.depth_limit(50)

    config = builder.build
    ```

=== "PHP"

    ```php
    <?php

    use Spikard\GraphQL\SchemaBuilder;
    use Spikard\GraphQL\SchemaConfig;

    // Define query type using PHP class
    class Query {
        public function hello(): string {
            return 'Hello, GraphQL!';
        }

        public function greet(string $name): string {
            return "Hello, {$name}!";
        }
    }

    // Create schema configuration
    $builder = new SchemaBuilder();
    $builder->enableIntrospection(true);
    $builder->complexityLimit(5000);
    $builder->depthLimit(50);

    $config = $builder->build();
    ```

## Registering the GraphQL Handler

Register your GraphQL endpoint with the Spikard router. The handler implements the `Handler` trait, so it integrates like any HTTP route.

=== "Rust"

    ```rust
    use spikard_graphql::{GraphQLExecutor, GraphQLHandler};
    use spikard_http::{Route, Router, Server, ServerConfig, Method};
    use std::sync::Arc;

    // Create executor with your schema
    let executor = Arc::new(GraphQLExecutor::new(schema));

    // Create handler
    let handler = GraphQLHandler::new(executor);

    // Register route
    let route = Route::new(
        "/graphql".to_string(),
        Method::Post,
        Arc::new(handler) as Arc<dyn spikard_http::Handler>,
    );

    let mut router = Router::new();
    router.register_route(route);

    // Configure and start server
    let config = ServerConfig::builder()
        .host("127.0.0.1")
        .port(8000)
        .build();

    let server = Server::new(config, router);
    server.run().await?;
    ```

=== "Python"

    ```python
    from spikard import Spikard, GraphQLHandler

    app = Spikard()

    # Create GraphQL handler
    graphql_handler = GraphQLHandler(schema_config)

    # Register route (similar to HTTP handlers)
    app.register_route(
        path="/graphql",
        method="POST",
        handler=graphql_handler
    )

    # Start server
    app.run(host="127.0.0.1", port=8000)
    ```

=== "TypeScript"

    ```typescript
    import { Spikard, GraphQLHandler } from 'spikard';

    const app = new Spikard();

    // Create GraphQL handler
    const graphqlHandler = new GraphQLHandler(schemaConfig);

    // Register route
    app.registerRoute({
      path: '/graphql',
      method: 'POST',
      handler: graphqlHandler,
    });

    // Start server
    await app.run({
      host: '127.0.0.1',
      port: 8000,
    });
    ```

=== "Ruby"

    ```ruby
    require 'spikard'

    app = Spikard::App.new

    # Create GraphQL handler
    graphql_handler = Spikard::GraphQLHandler.new(schema_config)

    # Register route
    app.register_route(
      path: '/graphql',
      method: 'POST',
      handler: graphql_handler
    )

    # Start server
    app.run(host: '127.0.0.1', port: 8000)
    ```

=== "PHP"

    ```php
    <?php

    use Spikard\App;
    use Spikard\GraphQL\Handler as GraphQLHandler;

    $app = new App();

    // Create GraphQL handler
    $graphqlHandler = new GraphQLHandler($schemaConfig);

    // Register route
    $app->registerRoute([
        'path' => '/graphql',
        'method' => 'POST',
        'handler' => $graphqlHandler,
    ]);

    // Start server
    $app->run(['host' => '127.0.0.1', 'port' => 8000]);
    ```

## Query and Mutation Support

Beyond queries, define mutations for state-changing operations and subscriptions for real-time updates.

=== "Rust"

    ```rust
    pub struct Query;
    pub struct Mutation;
    pub struct Subscription;

    impl Query {
        pub async fn user(&self, id: String) -> User {
            User {
                id,
                name: "Alice".to_string(),
            }
        }
    }

    impl Mutation {
        pub async fn create_user(&self, name: String) -> User {
            User {
                id: uuid::Uuid::new_v4().to_string(),
                name,
            }
        }
    }

    impl Subscription {
        pub async fn user_created(&self) -> String {
            "User created".to_string()
        }
    }

    // Build schema with all three types
    let schema = SchemaBuilder::new()
        .with_query(Query)
        .with_mutation(Mutation)
        .with_subscription(Subscription)
        .build();
    ```

=== "Python"

    ```python
    from spikard import GraphQLSchemaBuilder
    from dataclasses import dataclass

    @dataclass
    class User:
        id: str
        name: str

    class Query:
        @staticmethod
        async def user(id: str) -> User:
            return User(id=id, name="Alice")

    class Mutation:
        @staticmethod
        async def create_user(name: str) -> User:
            import uuid
            return User(id=str(uuid.uuid4()), name=name)

    class Subscription:
        @staticmethod
        async def user_created(self):
            yield "User created"

    # Build schema with all three types
    builder = GraphQLSchemaBuilder()
    builder.with_query(Query)
    builder.with_mutation(Mutation)
    builder.with_subscription(Subscription)

    schema = builder.build()
    ```

=== "TypeScript"

    ```typescript
    import { GraphQLSchemaBuilder } from 'spikard';

    interface User {
      id: string;
      name: string;
    }

    class Query {
      async user(id: string): Promise<User> {
        return { id, name: 'Alice' };
      }
    }

    class Mutation {
      async createUser(name: string): Promise<User> {
        return { id: crypto.randomUUID(), name };
      }
    }

    class Subscription {
      async *userCreated(): AsyncGenerator<string> {
        yield 'User created';
      }
    }

    const builder = new GraphQLSchemaBuilder();
    builder.withQuery(Query);
    builder.withMutation(Mutation);
    builder.withSubscription(Subscription);

    const schema = builder.build();
    ```

=== "Ruby"

    ```ruby
    require 'spikard'
    require 'securerandom'

    User = Struct.new(:id, :name, keyword_init: true)

    class Query
      def user(id:)
        User.new(id: id, name: 'Alice')
      end
    end

    class Mutation
      def create_user(name:)
        User.new(id: SecureRandom.uuid, name: name)
      end
    end

    class Subscription
      def user_created
        yield 'User created'
      end
    end

    builder = Spikard::GraphQLSchemaBuilder.new
    builder.with_query(Query)
    builder.with_mutation(Mutation)
    builder.with_subscription(Subscription)

    schema = builder.build
    ```

=== "PHP"

    ```php
    <?php

    use Spikard\GraphQL\SchemaBuilder;

    class User {
        public function __construct(
            public string $id,
            public string $name,
        ) {}
    }

    class Query {
        public function user(string $id): User {
            return new User($id, 'Alice');
        }
    }

    class Mutation {
        public function createUser(string $name): User {
            return new User(bin2hex(random_bytes(16)), $name);
        }
    }

    class Subscription {
        public function userCreated(): string {
            return 'User created';
        }
    }

    $builder = new SchemaBuilder();
    $builder->withQuery(Query::class);
    $builder->withMutation(Mutation::class);
    $builder->withSubscription(Subscription::class);

    $schema = $builder->build();
    ```

## Making GraphQL Requests

The GraphQL endpoint accepts POST requests with JSON bodies containing the query, variables, and operation name.

```bash
curl -X POST http://localhost:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "{ hello }",
    "operationName": null,
    "variables": {}
  }'
```

With variables:

```bash
curl -X POST http://localhost:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query GetUser($id: ID!) { user(id: $id) { name } }",
    "variables": {"id": "123"},
    "operationName": "GetUser"
  }'
```

With mutations:

```bash
curl -X POST http://localhost:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "mutation CreateUser($name: String!) { createUser(name: $name) { id name } }",
    "variables": {"name": "Bob"},
    "operationName": "CreateUser"
  }'
```

## GraphQL Playground and IDE Integration

Enable the GraphQL Playground IDE for interactive development and schema exploration.

=== "Rust"

    ```rust
    use spikard_graphql::routes::GraphQLRouteConfig;

    let config = GraphQLRouteConfig::builder()
        .path("/graphql")
        .enable_playground(true)  // Enable Playground UI
        .playground_path("/graphql/ui")
        .build();

    // Playground UI is served at http://localhost:8000/graphql/ui
    ```

=== "Python"

    ```python
    from spikard import GraphQLHandler

    handler = GraphQLHandler(schema)
    handler.enable_playground = True
    handler.playground_path = "/graphql/ui"

    # Playground UI is served at http://localhost:8000/graphql/ui
    ```

=== "TypeScript"

    ```typescript
    import { GraphQLHandler } from 'spikard';

    const handler = new GraphQLHandler(schema);
    handler.enablePlayground = true;
    handler.playgroundPath = '/graphql/ui';

    // Playground UI is served at http://localhost:8000/graphql/ui
    ```

=== "Ruby"

    ```ruby
    require 'spikard'

    handler = Spikard::GraphQLHandler.new(schema)
    handler.enable_playground = true
    handler.playground_path = '/graphql/ui'

    # Playground UI is served at http://localhost:8000/graphql/ui
    ```

=== "PHP"

    ```php
    <?php

    use Spikard\GraphQL\Handler as GraphQLHandler;

    $handler = new GraphQLHandler($schema);
    $handler->enablePlayground = true;
    $handler->playgroundPath = '/graphql/ui';

    // Playground UI is served at http://localhost:8000/graphql/ui
    ```

## Error Handling

GraphQL follows the GraphQL-over-HTTP specification for error responses. The server returns HTTP 200 for valid GraphQL requests, even if the query contains errors.

### Error Response Structure

```json
{
  "data": {
    "user": null
  },
  "errors": [
    {
      "message": "Field 'missing' not found on type 'User'",
      "extensions": {
        "code": "INTERNAL_ERROR"
      }
    }
  ]
}
```

### HTTP Status Codes

| Condition | HTTP Status | Example |
|-----------|-------------|---------|
| Invalid JSON | 400 Bad Request | Malformed request body |
| Valid GraphQL query (even with errors) | 200 OK | Field not found, validation error |
| Server error | 500 Internal Server Error | Unexpected panic, resource exhaustion |

=== "Rust"

    ```rust
    use spikard_graphql::error::GraphQLError;

    impl Query {
        pub async fn user(&self, id: String) -> Result<User, GraphQLError> {
            if id.is_empty() {
                return Err(GraphQLError::new("ID cannot be empty"));
            }

            Ok(User { id, name: "Alice".to_string() })
        }
    }

    // Errors are automatically formatted as GraphQL errors in responses
    ```

=== "Python"

    ```python
    from spikard import GraphQLError

    class Query:
        @staticmethod
        async def user(id: str) -> User:
            if not id:
                raise GraphQLError("ID cannot be empty")

            return User(id=id, name="Alice")

        # Errors are automatically formatted as GraphQL errors
    ```

=== "TypeScript"

    ```typescript
    import { GraphQLError } from 'spikard';

    class Query {
      async user(id: string): Promise<User> {
        if (!id) {
          throw new GraphQLError('ID cannot be empty');
        }

        return { id, name: 'Alice' };
      }

      // Errors are automatically formatted as GraphQL errors
    }
    ```

=== "Ruby"

    ```ruby
    require 'spikard'

    class Query
      def user(id:)
        raise Spikard::GraphQLError.new("ID cannot be empty") if id.empty?

        User.new(id: id, name: 'Alice')
      end

      # Errors are automatically formatted as GraphQL errors
    end
    ```

=== "PHP"

    ```php
    <?php

    use Spikard\GraphQL\GraphQLError;

    class Query {
        public function user(string $id): User {
            if (empty($id)) {
                throw new GraphQLError('ID cannot be empty');
            }

            return new User($id, 'Alice');
        }

        // Errors are automatically formatted as GraphQL errors
    }
    ```

### Custom Error Extensions

Add structured error details using extensions:

=== "Rust"

    ```rust
    use spikard_graphql::error::GraphQLError;

    impl Query {
        pub async fn user(&self, id: String) -> Result<User, GraphQLError> {
            Err(GraphQLError::new("User not found")
                .with_extension("code", "USER_NOT_FOUND")
                .with_extension("userId", id))
        }
    }
    ```

=== "Python"

    ```python
    from spikard import GraphQLError

    class Query:
        @staticmethod
        async def user(id: str) -> User:
            error = GraphQLError("User not found")
            error.extensions = {
                "code": "USER_NOT_FOUND",
                "userId": id,
            }
            raise error
    ```

## Testing GraphQL Endpoints

Test GraphQL handlers using standard HTTP testing patterns. Send POST requests with GraphQL queries and verify responses.

=== "Rust"

    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn test_hello_query() {
            let schema = SchemaBuilder::new()
                .query_only(QueryOnlyConfig::builder().build())
                .build();

            let executor = Arc::new(GraphQLExecutor::new(schema));
            let handler = GraphQLHandler::new(executor);

            let request = RequestData {
                body: br#"{"query":"{ hello }"}"#.to_vec(),
                ..Default::default()
            };

            let response = handler.call(request).await.unwrap();

            assert_eq!(response.status, 200);
            let body = String::from_utf8(response.body).unwrap();
            assert!(body.contains("Hello"));
        }

        #[tokio::test]
        async fn test_query_with_variables() {
            let schema = SchemaBuilder::new()
                .query_only(QueryOnlyConfig::builder().build())
                .build();

            let executor = Arc::new(GraphQLExecutor::new(schema));
            let handler = GraphQLHandler::new(executor);

            let query = r#"
                query Greet($name: String!) {
                  greet(name: $name)
                }
            "#;

            let request = RequestData {
                body: format!(
                    r#"{{"query":"{}","variables":{{"name":"Bob"}}}}"#,
                    query
                ).into_bytes(),
                ..Default::default()
            };

            let response = handler.call(request).await.unwrap();
            let body = String::from_utf8(response.body).unwrap();
            assert!(body.contains("Bob"));
        }
    }
    ```

=== "Python"

    ```python
    import pytest
    from httpx import AsyncClient
    from spikard import Spikard, GraphQLHandler

    @pytest.fixture
    async def client():
        app = Spikard()
        app.register_route(
            path="/graphql",
            method="POST",
            handler=GraphQLHandler(schema_config),
        )
        async with AsyncClient(app=app, base_url="http://test") as client:
            yield client

    @pytest.mark.asyncio
    async def test_hello_query(client):
        response = await client.post("/graphql", json={
            "query": "{ hello }"
        })
        assert response.status_code == 200
        assert "Hello" in response.text

    @pytest.mark.asyncio
    async def test_query_with_variables(client):
        response = await client.post("/graphql", json={
            "query": "query Greet($name: String!) { greet(name: $name) }",
            "variables": {"name": "Bob"},
        })
        assert response.status_code == 200
        assert "Bob" in response.text
    ```

=== "TypeScript"

    ```typescript
    import { describe, it, expect } from 'vitest';
    import { Spikard, GraphQLHandler } from 'spikard';

    describe('GraphQL Handler', () => {
      let app: Spikard;

      beforeEach(() => {
        app = new Spikard();
        app.registerRoute({
          path: '/graphql',
          method: 'POST',
          handler: new GraphQLHandler(schemaConfig),
        });
      });

      it('should execute hello query', async () => {
        const response = await app.request({
          method: 'POST',
          path: '/graphql',
          body: JSON.stringify({ query: '{ hello }' }),
        });

        expect(response.status).toBe(200);
        expect(response.body).toContain('Hello');
      });

      it('should execute query with variables', async () => {
        const response = await app.request({
          method: 'POST',
          path: '/graphql',
          body: JSON.stringify({
            query: 'query Greet($name: String!) { greet(name: $name) }',
            variables: { name: 'Bob' },
          }),
        });

        expect(response.status).toBe(200);
        expect(response.body).toContain('Bob');
      });
    });
    ```

=== "Ruby"

    ```ruby
    require 'rspec'
    require 'spikard'

    describe 'GraphQL Handler' do
      let(:app) { Spikard::App.new }
      let(:handler) { Spikard::GraphQLHandler.new(schema_config) }

      before do
        app.register_route(
          path: '/graphql',
          method: 'POST',
          handler: handler,
        )
      end

      it 'executes hello query' do
        response = app.request(
          method: 'POST',
          path: '/graphql',
          body: { query: '{ hello }' },
        )

        expect(response.status).to eq(200)
        expect(response.body).to include('Hello')
      end

      it 'executes query with variables' do
        response = app.request(
          method: 'POST',
          path: '/graphql',
          body: {
            query: 'query Greet($name: String!) { greet(name: $name) }',
            variables: { name: 'Bob' },
          },
        )

        expect(response.status).to eq(200)
        expect(response.body).to include('Bob')
      end
    end
    ```

=== "PHP"

    ```php
    <?php

    use PHPUnit\Framework\TestCase;
    use Spikard\App;
    use Spikard\GraphQL\Handler as GraphQLHandler;

    class GraphQLHandlerTest extends TestCase {
        private App $app;
        private GraphQLHandler $handler;

        protected function setUp(): void {
            $this->app = new App();
            $this->handler = new GraphQLHandler($schemaConfig);
            $this->app->registerRoute([
                'path' => '/graphql',
                'method' => 'POST',
                'handler' => $this->handler,
            ]);
        }

        public function testHelloQuery(): void {
            $response = $this->app->request(
                'POST',
                '/graphql',
                json_encode(['query' => '{ hello }'])
            );

            $this->assertEquals(200, $response->getStatusCode());
            $this->assertStringContainsString('Hello', $response->getBody());
        }

        public function testQueryWithVariables(): void {
            $response = $this->app->request(
                'POST',
                '/graphql',
                json_encode([
                    'query' => 'query Greet($name: String!) { greet(name: $name) }',
                    'variables' => ['name' => 'Bob'],
                ])
            );

            $this->assertEquals(200, $response->getStatusCode());
            $this->assertStringContainsString('Bob', $response->getBody());
        }
    }
    ```

## Configuration Options

Configure GraphQL behavior with the `GraphQLRouteConfig` builder:

=== "Rust"

    ```rust
    use spikard_graphql::routes::GraphQLRouteConfig;

    let config = GraphQLRouteConfig::builder()
        .path("/graphql")                 // Endpoint path
        .enable_playground(true)          // Enable Playground UI
        .playground_path("/graphql/ui")   // Playground endpoint
        .introspection_enabled(true)      // Allow schema introspection
        .complexity_limit(5000)           // Max query complexity
        .depth_limit(50)                  // Max query depth
        .build();
    ```

=== "Python"

    ```python
    from spikard import GraphQLSchemaBuilder

    builder = GraphQLSchemaBuilder()
    builder.path("/graphql")
    builder.enable_playground(True)
    builder.playground_path("/graphql/ui")
    builder.introspection_enabled(True)
    builder.complexity_limit(5000)
    builder.depth_limit(50)

    config = builder.build()
    ```

=== "TypeScript"

    ```typescript
    import { GraphQLSchemaBuilder } from 'spikard';

    const builder = new GraphQLSchemaBuilder();
    builder.path('/graphql');
    builder.enablePlayground(true);
    builder.playgroundPath('/graphql/ui');
    builder.introspectionEnabled(true);
    builder.complexityLimit(5000);
    builder.depthLimit(50);

    const config = builder.build();
    ```

## Best Practices

- **Schema definition**: Define types with clear field descriptions; they appear in schema introspection and IDE autocompletion.
- **Error messages**: Return descriptive error messages; they help clients debug queries.
- **Resolver efficiency**: Keep resolvers fast; defer heavy computation to background jobs if needed.
- **Query depth and complexity**: Set reasonable limits to prevent malicious or accidentally expensive queries.
- **Testing**: Write integration tests for common queries and mutations using language-native test frameworks.
- **Caching**: Use HTTP caching headers (via middleware) for cacheable queries.
- **Monitoring**: Log slow queries and errors for debugging; integrate with observability platforms.

## See Also

- [GraphQL Specification](https://graphql.org/learn/)
- [async-graphql Documentation](https://async-graphql.rs/)
- [ADR 0013: GraphQL HTTP Integration](../adr/0013-graphql-http-integration.md)
- [Routing Basics](./routing.md)
- [Middleware](./middleware.md)
