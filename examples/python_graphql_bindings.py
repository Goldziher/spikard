#!/usr/bin/env python3
"""Example: Python GraphQL Bindings for Spikard.

This example demonstrates how to use the Python GraphQL bindings
to configure GraphQL schemas with spikard.

The bindings are available when the `graphql` feature is enabled.

To run this example (requires building the Python extension):
    maturin build --release
    python3 examples/python_graphql_bindings.py
"""

from spikard import GraphQLSchemaBuilder, GraphQLSchemaConfig

# This example shows the API design
# Actual execution requires the Python extension to be built


def example_schema_config() -> GraphQLSchemaConfig | None:
    """Example of creating a GraphQL schema configuration."""
    # Note: This requires the spikard Python extension to be built
    # If running without the extension, this will raise ImportError

    try:
        # Create a new configuration with defaults
        config = GraphQLSchemaConfig()

        # Modify configuration
        config.introspection_enabled = False
        config.complexity_limit = 5000
        config.depth_limit = 50

        # Validate configuration
        config.validate()

        return config

    except ImportError:
        return None


def example_schema_builder() -> GraphQLSchemaConfig | None:
    """Example of using the GraphQL schema builder."""
    try:
        # Create a builder with method chaining
        builder = GraphQLSchemaBuilder()

        # Configure settings (mutating methods return nothing, use for side effects)
        builder.enable_introspection(True)
        builder.complexity_limit(5000)
        builder.depth_limit(50)

        # Build the final configuration
        return builder.build()

    except ImportError:
        return None


def example_integration_with_server() -> None:
    """Example of integrating GraphQL with a Spikard server."""


def main() -> None:
    """Run all examples."""
    example_schema_config()

    example_schema_builder()

    example_integration_with_server()


if __name__ == "__main__":
    main()
