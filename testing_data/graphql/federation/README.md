# GraphQL Federation Fixtures

Comprehensive test fixtures for Apollo Federation v2 features, including subgraph delegation, entity resolution, and federation directives.

## Overview

This directory contains 10 fixtures that test Apollo Federation v2 core features:
- **Entity Resolution** via `_entities` query
- **Entity Keys** with `@key` directive
- **Compound Keys** (multiple-field keys)
- **Field Dependencies** via `@requires` directive
- **Field Provisions** via `@provides` directive
- **External Field References** via `@external` directive
- **Subgraph Introspection** via `_service` query
- **Cross-Subgraph Queries** with entity references
- **Error Handling** (missing entities, type mismatches)

## Fixtures

### 1. entity_resolution_basic.json
**Tests:** Basic `_entities` query for entity resolution

```graphql
query {
  _entities(representations: [{__typename: "User", id: "1"}]) {
    ... on User {
      id
      name
      email
    }
  }
}
```

**Expected:** 200 - Resolved User entity with all requested fields

**Federation Concepts:**
- Basic entity representation format
- Entity type union resolution
- Simple key field resolution

---

### 2. entity_with_key.json
**Tests:** Entity with `@key` directive

```graphql
type User @key(fields: "id") {
  id: ID!
  name: String!
  username: String!
  profile: UserProfile!
}
```

**Expected:** 200 - Resolved User entity with nested profile object

**Federation Concepts:**
- Single-field entity keys
- Nested object resolution in entity context
- Key-based entity lookup across subgraphs

---

### 3. entity_with_compound_key.json
**Tests:** Entity with compound `@key` spanning multiple fields

```graphql
type Product @key(fields: "sku category") {
  sku: String!
  category: String!
  name: String!
  price: Float!
  stock: Int!
}
```

**Query:**
```graphql
_entities(representations: [{__typename: "Product", sku: "ABC123", category: "electronics"}])
```

**Expected:** 200 - Product resolved using composite key (sku + category)

**Federation Concepts:**
- Multiple-field entity keys
- Compound key representation in Apollo Federation
- Field ordering in composite keys

---

### 4. requires_directive.json
**Tests:** Field with `@requires` directive for dependent field resolution

```graphql
type Shipment @key(fields: "id") {
  id: ID!
  weight: Float!
  destination: String!
  shippingEstimate: Float! @requires(fields: "weight destination")
}
```

**Query:**
```graphql
_entities(representations: [{__typename: "Shipment", id: "ship-001", weight: 5.5, destination: "NYC"}])
```

**Expected:** 200 - Computed `shippingEstimate` field (24.75) calculated from required fields

**Federation Concepts:**
- Field-level dependencies via `@requires`
- Computed fields requiring other fields
- Gateway-side field computation
- Zero-cost field selection optimization

---

### 5. provides_directive.json
**Tests:** Field with `@provides` directive for optimized nested field resolution

```graphql
type Post @key(fields: "id") {
  id: ID!
  reviews: [Review!]! @provides(fields: "author { id name }")
}
```

**Expected:** 200 - Reviews with pre-resolved author data without additional subgraph fetch

**Federation Concepts:**
- `@provides` directive for field optimization
- Reducing subgraph round-trips
- Nested field provision patterns
- Gateway optimization with provided data

---

### 6. external_field.json
**Tests:** Reference to `@external` field used by `@requires`

```graphql
type Parcel @key(fields: "id") {
  id: ID!
  weight: Float! @external
  dimensions: String! @external
  label: String! @requires(fields: "weight dimensions")
}
```

**Query:**
```graphql
_entities(representations: [{__typename: "Parcel", id: "parcel-x1", weight: 2.5, dimensions: "10x8x6"}])
```

**Expected:** 200 - Generated `label` field ("SMALL_PACKAGE_2.5KG") from external fields

**Federation Concepts:**
- `@external` directive for field references from other subgraphs
- Cross-subgraph field dependencies
- External field composition in computed fields
- Reference entity patterns

---

### 7. subgraph_introspection.json
**Tests:** Federation `_service` query returning Schema Definition Language (SDL)

```graphql
query {
  _service {
    sdl
  }
}
```

**Expected:** 200 - Subgraph SDL string for gateway composition

**Federation Concepts:**
- Subgraph introspection mechanism
- SDL retrieval for Apollo Gateway composition
- Schema composition and stitching
- Dynamic gateway schema building

---

### 8. cross_subgraph_query.json
**Tests:** Query spanning multiple subgraphs with federation entity references

**Schema combines two entity types:**
- `User @key(fields: "id")` - from users subgraph
- `Order @key(fields: "id")` - from orders subgraph

```graphql
query {
  user(id: "usr-42") {
    id
    name
    email
    orders {
      id
      orderId
      total
      status
      createdAt
    }
  }
}
```

**Expected:** 200 - User with associated orders from multiple subgraphs

**Federation Concepts:**
- Entity references across subgraphs
- Complex multi-entity queries
- Gateway entity resolution orchestration
- Cross-subgraph relationship traversal

---

### 9. federation_error_missing_entity.json
**Tests:** Entity not found handling (returns null per spec)

```graphql
_entities(representations: [{__typename: "Customer", id: "999999"}])
```

**Expected:** 200 - Response with `null` in `_entities` array (no error thrown)

**Federation Concepts:**
- Federation spec compliance for missing entities
- Null handling in entity array responses
- Gateway-side null resolution handling
- Proper error vs. missing entity distinction

---

### 10. federation_type_mismatch.json
**Tests:** Wrong `__typename` in entity representation (returns 400 error)

```graphql
_entities(representations: [{__typename: "InvalidType", id: "1"}])
```

**Expected:** 400 - GraphQL error with "Unknown type 'InvalidType'" message

**Federation Concepts:**
- Type validation in entity representations
- Error handling for invalid types
- Validation before entity resolution
- Type safety in federation gateway

---

## Federation Directives Reference

### @key
- **Purpose:** Designates an object type as an entity and defines uniqueness fields
- **Syntax:** `@key(fields: "fieldName")`  or `@key(fields: "field1 field2")`
- **Used in:** 8 fixtures

```graphql
type User @key(fields: "id") {
  id: ID!
  name: String!
}
```

### @requires
- **Purpose:** Indicates that a field requires other fields to be resolved
- **Syntax:** `@requires(fields: "fieldName")` or `@requires(fields: "field1 field2")`
- **Used in:** 2 fixtures (requires_directive, external_field)

```graphql
type Shipment {
  weight: Float!
  shippingEstimate: Float! @requires(fields: "weight")
}
```

### @provides
- **Purpose:** Indicates that a field may provide data available from another subgraph
- **Syntax:** `@provides(fields: "fieldName")`
- **Used in:** 1 fixture (provides_directive)

```graphql
type Post {
  reviews: [Review!]! @provides(fields: "author { id name }")
}
```

### @external
- **Purpose:** Indicates that a field is defined and owned by another subgraph
- **Syntax:** `@external` (applied directly to field)
- **Used in:** 1 fixture (external_field)

```graphql
type Parcel {
  weight: Float! @external
}
```

---

## Federation Queries Reference

### _entities
**Purpose:** Core federation query for resolving entities by their key representations

**Signature:**
```graphql
_entities(representations: [_Any!]!): [_Entity]!
```

**Used in:** 9 fixtures (all except subgraph_introspection)

**Example:**
```graphql
{
  _entities(representations: [
    {__typename: "User", id: "1"},
    {__typename: "Product", sku: "ABC", category: "electronics"}
  ]) {
    ... on User { id name }
    ... on Product { sku category name }
  }
}
```

### _service
**Purpose:** Subgraph introspection query for schema composition

**Signature:**
```graphql
_service: _Service { sdl: String! }
```

**Used in:** 10 fixtures (all fixtures include _service in schema)

**Example:**
```graphql
{
  _service {
    sdl
  }
}
```

---

## Fixture Structure

Each fixture follows the standard Spikard GraphQL fixture schema:

```json
{
  "name": "fixture_name",
  "description": "Human-readable description",
  "operation_type": "query",
  "endpoint": "/graphql",
  "schema": "GraphQL SDL string",
  "request": {
    "query": "GraphQL query document"
  },
  "expected_response": {
    "status_code": 200,
    "data": { ... },
    "errors": [ ... ]
  },
  "resolvers": {
    "Type.field": {
      "type": "mock|factory|error",
      "return_value": { ... }
    }
  },
  "tags": ["federation", "feature-tag"],
  "complexity_limit": 100,
  "depth_limit": 5
}
```

---

## Resolver Types

### mock
Returns a static value
```json
{
  "type": "mock",
  "return_value": "Hello"
}
```

### factory
Calls a factory function for dynamic resolution
```json
{
  "type": "factory",
  "factory_fn": "resolve_user_entity"
}
```

### error
Returns a GraphQL error
```json
{
  "type": "error",
  "error_message": "Entity not found",
  "error_extensions": {"code": "NOT_FOUND"}
}
```

---

## Usage in Tests

### Load fixtures via conftest.py
```python
@pytest.fixture(params=load_federation_fixtures())
def federation_fixture(request):
    return request.param
```

### Test entity resolution
```python
def test_entity_resolution(graphql_client, federation_fixture):
    response = graphql_client.query(
        federation_fixture["request"]["query"],
        variables=federation_fixture["request"].get("variables")
    )
    assert response.status_code == federation_fixture["expected_response"]["status_code"]
    assert response.data == federation_fixture["expected_response"]["data"]
```

---

## Apollo Federation Specification Coverage

This fixture set provides comprehensive coverage of:

- ✓ **Entity Resolution** - Core _entities query mechanism
- ✓ **Entity Keys** - @key directive and key-based composition
- ✓ **Compound Keys** - Multiple-field key composition
- ✓ **Field Dependencies** - @requires directive for field computation
- ✓ **Field Provisions** - @provides directive for optimization
- ✓ **External Fields** - @external directive for cross-subgraph references
- ✓ **Subgraph Introspection** - _service SDL query
- ✓ **Cross-Subgraph Queries** - Multi-entity federation patterns
- ✓ **Error Handling** - Type validation and entity not found scenarios

**Specification:** [Apollo Federation v2 Documentation](https://www.apollographql.com/docs/federation)

---

## Integration Notes

### With Spikard Test Framework
All fixtures are compatible with the Spikard fixture-driven testing approach:
- Auto-generate Python, Node.js, Ruby, and PHP test suites
- Factory resolvers support async/await for language bindings
- Complexity and depth limits enforced at handler level

### With Apollo Gateway
Fixtures simulate subgraph responses that Apollo Federation Gateway expects:
- Proper entity representation format
- SDL response for gateway composition
- Error handling matching federation spec

### With Federation-Enabled Servers
Test any Federation v2-compliant server:
- Apollo Server with @apollo/subgraph
- GraphQL-core with federation plugins
- Custom federation-enabled GraphQL servers

---

## Summary

| Fixture | Feature | Directives | Status Code |
|---------|---------|-----------|------------|
| entity_resolution_basic | Entity Resolution | @key | 200 |
| entity_with_key | Entity Keys | @key | 200 |
| entity_with_compound_key | Compound Keys | @key | 200 |
| requires_directive | Field Dependencies | @key, @requires | 200 |
| provides_directive | Field Provisions | @key, @provides | 200 |
| external_field | External References | @key, @external, @requires | 200 |
| subgraph_introspection | SDL Introspection | - | 200 |
| cross_subgraph_query | Multi-Subgraph | @key | 200 |
| federation_error_missing_entity | Missing Entity | @key | 200 |
| federation_type_mismatch | Type Validation | - | 400 |

**Total Coverage:** 10 fixtures, 4 directives, 2 federation queries, Apollo Federation v2 specification compliance.
