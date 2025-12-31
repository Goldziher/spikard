# UserService Proto Definition

Basic protobuf definition for a UserService with GetUser and CreateUser methods.

```protobuf
syntax = "proto3";

package userservice;

service UserService {
  rpc GetUser(GetUserRequest) returns (User);
  rpc CreateUser(CreateUserRequest) returns (User);
}

message GetUserRequest {
  int32 id = 1;
}

message CreateUserRequest {
  string name = 1;
  string email = 2;
}

message User {
  int32 id = 1;
  string name = 2;
  string email = 3;
  string created_at = 4;
}
```

## Usage

This proto definition is used across all language examples in the Spikard documentation. It demonstrates:

- **Service definition**: Defines the RPC methods available
- **Request messages**: Input types for each RPC method
- **Response messages**: Output types returned by the service
- **Field numbering**: Each field has a unique number for wire format compatibility
