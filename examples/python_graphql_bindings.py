#!/usr/bin/env python3
"""
Example: Python GraphQL Bindings for Spikard

This example demonstrates how to use the Python GraphQL bindings
to configure GraphQL schemas with spikard.

The bindings are available when the `graphql` feature is enabled.

To run this example (requires building the Python extension):
    maturin build --release
    python3 examples/python_graphql_bindings.py
"""

# This example shows the API design
# Actual execution requires the Python extension to be built

def example_schema_config():
    """Example of creating a GraphQL schema configuration."""
    # Note: This requires the spikard Python extension to be built
    # If running without the extension, this will raise ImportError

    try:
        from spikard import GraphQLSchemaConfig

        # Create a new configuration with defaults
        config = GraphQLSchemaConfig()
        print(f"Created config: {config}")
        print(f"  Introspection enabled: {config.introspection_enabled}")
        print(f"  Complexity limit: {config.complexity_limit}")
        print(f"  Depth limit: {config.depth_limit}")

        # Modify configuration
        config.introspection_enabled = False
        config.complexity_limit = 5000
        config.depth_limit = 50

        print(f"\nModified config: {config}")
        print(f"  Introspection enabled: {config.introspection_enabled}")
        print(f"  Complexity limit: {config.complexity_limit}")
        print(f"  Depth limit: {config.depth_limit}")

        # Validate configuration
        is_valid = config.validate()
        print(f"\nConfiguration valid: {is_valid}")

        return config

    except ImportError as e:
        print(f"GraphQL feature not available: {e}")
        print("Build with: cargo build -p spikard-py --features graphql")
        return None


def example_schema_builder():
    """Example of using the GraphQL schema builder."""
    try:
        from spikard import GraphQLSchemaBuilder

        # Create a builder with method chaining
        builder = GraphQLSchemaBuilder()
        print(f"Created builder: {builder}")

        # Configure settings (mutating methods return nothing, use for side effects)
        builder.enable_introspection(True)
        builder.complexity_limit(5000)
        builder.depth_limit(50)

        print(f"\nConfigured builder settings:")
        print(f"  Introspection enabled: {builder.is_introspection_enabled()}")
        print(f"  Complexity limit: {builder.get_complexity_limit()}")
        print(f"  Depth limit: {builder.get_depth_limit()}")

        # Build the final configuration
        config = builder.build()
        print(f"\nBuilt configuration: {config}")
        print(f"  Introspection enabled: {config.introspection_enabled}")
        print(f"  Complexity limit: {config.complexity_limit}")
        print(f"  Depth limit: {config.depth_limit}")

        return config

    except ImportError as e:
        print(f"GraphQL feature not available: {e}")
        print("Build with: cargo build -p spikard-py --features graphql")
        return None


def example_integration_with_server():
    """Example of integrating GraphQL with a Spikard server."""
    print("""
Example: Integrating GraphQL with Spikard Server

from spikard import Spikard, Server, GraphQLSchemaBuilder

# Create GraphQL schema configuration
builder = GraphQLSchemaBuilder()
builder.enable_introspection(True)
builder.complexity_limit(5000)
builder.depth_limit(50)
config = builder.build()

# In the future, you'll be able to create a GraphQL route:
# server = Server(routes=[
#     GraphQLRoute("/graphql", schema_config=config)
# ])

# Then run the server:
# server.run()

Note: Full GraphQL integration with route handling is coming soon!
    """)


def main():
    """Run all examples."""
    print("=" * 60)
    print("Spikard Python GraphQL Bindings Examples")
    print("=" * 60)

    print("\n1. GraphQL Schema Configuration")
    print("-" * 60)
    config = example_schema_config()

    print("\n2. GraphQL Schema Builder")
    print("-" * 60)
    config = example_schema_builder()

    print("\n3. Integration with Spikard Server")
    print("-" * 60)
    example_integration_with_server()

    print("\n" + "=" * 60)
    print("Examples completed!")
    print("=" * 60)


if __name__ == "__main__":
    main()
