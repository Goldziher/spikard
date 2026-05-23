// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'lib.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$GraphQLError {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'GraphQLError()';
}


}

/// @nodoc
class $GraphQLErrorCopyWith<$Res>  {
$GraphQLErrorCopyWith(GraphQLError _, $Res Function(GraphQLError) __);
}


/// Adds pattern-matching-related methods to [GraphQLError].
extension GraphQLErrorPatterns on GraphQLError {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( GraphQLError_ExecutionError value)?  executionError,TResult Function( GraphQLError_SchemaBuildError value)?  schemaBuildError,TResult Function( GraphQLError_RequestHandlingError value)?  requestHandlingError,TResult Function( GraphQLError_SerializationError value)?  serializationError,TResult Function( GraphQLError_JsonError value)?  jsonError,TResult Function( GraphQLError_ValidationError value)?  validationError,TResult Function( GraphQLError_ParseError value)?  parseError,TResult Function( GraphQLError_AuthenticationError value)?  authenticationError,TResult Function( GraphQLError_AuthorizationError value)?  authorizationError,TResult Function( GraphQLError_NotFound value)?  notFound,TResult Function( GraphQLError_RateLimitExceeded value)?  rateLimitExceeded,TResult Function( GraphQLError_InvalidInput value)?  invalidInput,TResult Function( GraphQLError_ComplexityLimitExceeded value)?  complexityLimitExceeded,TResult Function( GraphQLError_DepthLimitExceeded value)?  depthLimitExceeded,TResult Function( GraphQLError_InternalError value)?  internalError,required TResult orElse(),}){
final _that = this;
switch (_that) {
case GraphQLError_ExecutionError() when executionError != null:
return executionError(_that);case GraphQLError_SchemaBuildError() when schemaBuildError != null:
return schemaBuildError(_that);case GraphQLError_RequestHandlingError() when requestHandlingError != null:
return requestHandlingError(_that);case GraphQLError_SerializationError() when serializationError != null:
return serializationError(_that);case GraphQLError_JsonError() when jsonError != null:
return jsonError(_that);case GraphQLError_ValidationError() when validationError != null:
return validationError(_that);case GraphQLError_ParseError() when parseError != null:
return parseError(_that);case GraphQLError_AuthenticationError() when authenticationError != null:
return authenticationError(_that);case GraphQLError_AuthorizationError() when authorizationError != null:
return authorizationError(_that);case GraphQLError_NotFound() when notFound != null:
return notFound(_that);case GraphQLError_RateLimitExceeded() when rateLimitExceeded != null:
return rateLimitExceeded(_that);case GraphQLError_InvalidInput() when invalidInput != null:
return invalidInput(_that);case GraphQLError_ComplexityLimitExceeded() when complexityLimitExceeded != null:
return complexityLimitExceeded(_that);case GraphQLError_DepthLimitExceeded() when depthLimitExceeded != null:
return depthLimitExceeded(_that);case GraphQLError_InternalError() when internalError != null:
return internalError(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( GraphQLError_ExecutionError value)  executionError,required TResult Function( GraphQLError_SchemaBuildError value)  schemaBuildError,required TResult Function( GraphQLError_RequestHandlingError value)  requestHandlingError,required TResult Function( GraphQLError_SerializationError value)  serializationError,required TResult Function( GraphQLError_JsonError value)  jsonError,required TResult Function( GraphQLError_ValidationError value)  validationError,required TResult Function( GraphQLError_ParseError value)  parseError,required TResult Function( GraphQLError_AuthenticationError value)  authenticationError,required TResult Function( GraphQLError_AuthorizationError value)  authorizationError,required TResult Function( GraphQLError_NotFound value)  notFound,required TResult Function( GraphQLError_RateLimitExceeded value)  rateLimitExceeded,required TResult Function( GraphQLError_InvalidInput value)  invalidInput,required TResult Function( GraphQLError_ComplexityLimitExceeded value)  complexityLimitExceeded,required TResult Function( GraphQLError_DepthLimitExceeded value)  depthLimitExceeded,required TResult Function( GraphQLError_InternalError value)  internalError,}){
final _that = this;
switch (_that) {
case GraphQLError_ExecutionError():
return executionError(_that);case GraphQLError_SchemaBuildError():
return schemaBuildError(_that);case GraphQLError_RequestHandlingError():
return requestHandlingError(_that);case GraphQLError_SerializationError():
return serializationError(_that);case GraphQLError_JsonError():
return jsonError(_that);case GraphQLError_ValidationError():
return validationError(_that);case GraphQLError_ParseError():
return parseError(_that);case GraphQLError_AuthenticationError():
return authenticationError(_that);case GraphQLError_AuthorizationError():
return authorizationError(_that);case GraphQLError_NotFound():
return notFound(_that);case GraphQLError_RateLimitExceeded():
return rateLimitExceeded(_that);case GraphQLError_InvalidInput():
return invalidInput(_that);case GraphQLError_ComplexityLimitExceeded():
return complexityLimitExceeded(_that);case GraphQLError_DepthLimitExceeded():
return depthLimitExceeded(_that);case GraphQLError_InternalError():
return internalError(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( GraphQLError_ExecutionError value)?  executionError,TResult? Function( GraphQLError_SchemaBuildError value)?  schemaBuildError,TResult? Function( GraphQLError_RequestHandlingError value)?  requestHandlingError,TResult? Function( GraphQLError_SerializationError value)?  serializationError,TResult? Function( GraphQLError_JsonError value)?  jsonError,TResult? Function( GraphQLError_ValidationError value)?  validationError,TResult? Function( GraphQLError_ParseError value)?  parseError,TResult? Function( GraphQLError_AuthenticationError value)?  authenticationError,TResult? Function( GraphQLError_AuthorizationError value)?  authorizationError,TResult? Function( GraphQLError_NotFound value)?  notFound,TResult? Function( GraphQLError_RateLimitExceeded value)?  rateLimitExceeded,TResult? Function( GraphQLError_InvalidInput value)?  invalidInput,TResult? Function( GraphQLError_ComplexityLimitExceeded value)?  complexityLimitExceeded,TResult? Function( GraphQLError_DepthLimitExceeded value)?  depthLimitExceeded,TResult? Function( GraphQLError_InternalError value)?  internalError,}){
final _that = this;
switch (_that) {
case GraphQLError_ExecutionError() when executionError != null:
return executionError(_that);case GraphQLError_SchemaBuildError() when schemaBuildError != null:
return schemaBuildError(_that);case GraphQLError_RequestHandlingError() when requestHandlingError != null:
return requestHandlingError(_that);case GraphQLError_SerializationError() when serializationError != null:
return serializationError(_that);case GraphQLError_JsonError() when jsonError != null:
return jsonError(_that);case GraphQLError_ValidationError() when validationError != null:
return validationError(_that);case GraphQLError_ParseError() when parseError != null:
return parseError(_that);case GraphQLError_AuthenticationError() when authenticationError != null:
return authenticationError(_that);case GraphQLError_AuthorizationError() when authorizationError != null:
return authorizationError(_that);case GraphQLError_NotFound() when notFound != null:
return notFound(_that);case GraphQLError_RateLimitExceeded() when rateLimitExceeded != null:
return rateLimitExceeded(_that);case GraphQLError_InvalidInput() when invalidInput != null:
return invalidInput(_that);case GraphQLError_ComplexityLimitExceeded() when complexityLimitExceeded != null:
return complexityLimitExceeded(_that);case GraphQLError_DepthLimitExceeded() when depthLimitExceeded != null:
return depthLimitExceeded(_that);case GraphQLError_InternalError() when internalError != null:
return internalError(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String field0)?  executionError,TResult Function( String field0)?  schemaBuildError,TResult Function( String field0)?  requestHandlingError,TResult Function( String field0)?  serializationError,TResult Function( String field0)?  jsonError,TResult Function( String field0)?  validationError,TResult Function( String field0)?  parseError,TResult Function( String field0)?  authenticationError,TResult Function( String field0)?  authorizationError,TResult Function( String field0)?  notFound,TResult Function( String field0)?  rateLimitExceeded,TResult Function( String message,  String details)?  invalidInput,TResult Function()?  complexityLimitExceeded,TResult Function()?  depthLimitExceeded,TResult Function( String field0)?  internalError,required TResult orElse(),}) {final _that = this;
switch (_that) {
case GraphQLError_ExecutionError() when executionError != null:
return executionError(_that.field0);case GraphQLError_SchemaBuildError() when schemaBuildError != null:
return schemaBuildError(_that.field0);case GraphQLError_RequestHandlingError() when requestHandlingError != null:
return requestHandlingError(_that.field0);case GraphQLError_SerializationError() when serializationError != null:
return serializationError(_that.field0);case GraphQLError_JsonError() when jsonError != null:
return jsonError(_that.field0);case GraphQLError_ValidationError() when validationError != null:
return validationError(_that.field0);case GraphQLError_ParseError() when parseError != null:
return parseError(_that.field0);case GraphQLError_AuthenticationError() when authenticationError != null:
return authenticationError(_that.field0);case GraphQLError_AuthorizationError() when authorizationError != null:
return authorizationError(_that.field0);case GraphQLError_NotFound() when notFound != null:
return notFound(_that.field0);case GraphQLError_RateLimitExceeded() when rateLimitExceeded != null:
return rateLimitExceeded(_that.field0);case GraphQLError_InvalidInput() when invalidInput != null:
return invalidInput(_that.message,_that.details);case GraphQLError_ComplexityLimitExceeded() when complexityLimitExceeded != null:
return complexityLimitExceeded();case GraphQLError_DepthLimitExceeded() when depthLimitExceeded != null:
return depthLimitExceeded();case GraphQLError_InternalError() when internalError != null:
return internalError(_that.field0);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String field0)  executionError,required TResult Function( String field0)  schemaBuildError,required TResult Function( String field0)  requestHandlingError,required TResult Function( String field0)  serializationError,required TResult Function( String field0)  jsonError,required TResult Function( String field0)  validationError,required TResult Function( String field0)  parseError,required TResult Function( String field0)  authenticationError,required TResult Function( String field0)  authorizationError,required TResult Function( String field0)  notFound,required TResult Function( String field0)  rateLimitExceeded,required TResult Function( String message,  String details)  invalidInput,required TResult Function()  complexityLimitExceeded,required TResult Function()  depthLimitExceeded,required TResult Function( String field0)  internalError,}) {final _that = this;
switch (_that) {
case GraphQLError_ExecutionError():
return executionError(_that.field0);case GraphQLError_SchemaBuildError():
return schemaBuildError(_that.field0);case GraphQLError_RequestHandlingError():
return requestHandlingError(_that.field0);case GraphQLError_SerializationError():
return serializationError(_that.field0);case GraphQLError_JsonError():
return jsonError(_that.field0);case GraphQLError_ValidationError():
return validationError(_that.field0);case GraphQLError_ParseError():
return parseError(_that.field0);case GraphQLError_AuthenticationError():
return authenticationError(_that.field0);case GraphQLError_AuthorizationError():
return authorizationError(_that.field0);case GraphQLError_NotFound():
return notFound(_that.field0);case GraphQLError_RateLimitExceeded():
return rateLimitExceeded(_that.field0);case GraphQLError_InvalidInput():
return invalidInput(_that.message,_that.details);case GraphQLError_ComplexityLimitExceeded():
return complexityLimitExceeded();case GraphQLError_DepthLimitExceeded():
return depthLimitExceeded();case GraphQLError_InternalError():
return internalError(_that.field0);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String field0)?  executionError,TResult? Function( String field0)?  schemaBuildError,TResult? Function( String field0)?  requestHandlingError,TResult? Function( String field0)?  serializationError,TResult? Function( String field0)?  jsonError,TResult? Function( String field0)?  validationError,TResult? Function( String field0)?  parseError,TResult? Function( String field0)?  authenticationError,TResult? Function( String field0)?  authorizationError,TResult? Function( String field0)?  notFound,TResult? Function( String field0)?  rateLimitExceeded,TResult? Function( String message,  String details)?  invalidInput,TResult? Function()?  complexityLimitExceeded,TResult? Function()?  depthLimitExceeded,TResult? Function( String field0)?  internalError,}) {final _that = this;
switch (_that) {
case GraphQLError_ExecutionError() when executionError != null:
return executionError(_that.field0);case GraphQLError_SchemaBuildError() when schemaBuildError != null:
return schemaBuildError(_that.field0);case GraphQLError_RequestHandlingError() when requestHandlingError != null:
return requestHandlingError(_that.field0);case GraphQLError_SerializationError() when serializationError != null:
return serializationError(_that.field0);case GraphQLError_JsonError() when jsonError != null:
return jsonError(_that.field0);case GraphQLError_ValidationError() when validationError != null:
return validationError(_that.field0);case GraphQLError_ParseError() when parseError != null:
return parseError(_that.field0);case GraphQLError_AuthenticationError() when authenticationError != null:
return authenticationError(_that.field0);case GraphQLError_AuthorizationError() when authorizationError != null:
return authorizationError(_that.field0);case GraphQLError_NotFound() when notFound != null:
return notFound(_that.field0);case GraphQLError_RateLimitExceeded() when rateLimitExceeded != null:
return rateLimitExceeded(_that.field0);case GraphQLError_InvalidInput() when invalidInput != null:
return invalidInput(_that.message,_that.details);case GraphQLError_ComplexityLimitExceeded() when complexityLimitExceeded != null:
return complexityLimitExceeded();case GraphQLError_DepthLimitExceeded() when depthLimitExceeded != null:
return depthLimitExceeded();case GraphQLError_InternalError() when internalError != null:
return internalError(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class GraphQLError_ExecutionError extends GraphQLError {
  const GraphQLError_ExecutionError({required this.field0}): super._();


 final  String field0;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GraphQLError_ExecutionErrorCopyWith<GraphQLError_ExecutionError> get copyWith => _$GraphQLError_ExecutionErrorCopyWithImpl<GraphQLError_ExecutionError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError_ExecutionError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'GraphQLError.executionError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $GraphQLError_ExecutionErrorCopyWith<$Res> implements $GraphQLErrorCopyWith<$Res> {
  factory $GraphQLError_ExecutionErrorCopyWith(GraphQLError_ExecutionError value, $Res Function(GraphQLError_ExecutionError) _then) = _$GraphQLError_ExecutionErrorCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$GraphQLError_ExecutionErrorCopyWithImpl<$Res>
    implements $GraphQLError_ExecutionErrorCopyWith<$Res> {
  _$GraphQLError_ExecutionErrorCopyWithImpl(this._self, this._then);

  final GraphQLError_ExecutionError _self;
  final $Res Function(GraphQLError_ExecutionError) _then;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(GraphQLError_ExecutionError(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GraphQLError_SchemaBuildError extends GraphQLError {
  const GraphQLError_SchemaBuildError({required this.field0}): super._();


 final  String field0;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GraphQLError_SchemaBuildErrorCopyWith<GraphQLError_SchemaBuildError> get copyWith => _$GraphQLError_SchemaBuildErrorCopyWithImpl<GraphQLError_SchemaBuildError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError_SchemaBuildError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'GraphQLError.schemaBuildError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $GraphQLError_SchemaBuildErrorCopyWith<$Res> implements $GraphQLErrorCopyWith<$Res> {
  factory $GraphQLError_SchemaBuildErrorCopyWith(GraphQLError_SchemaBuildError value, $Res Function(GraphQLError_SchemaBuildError) _then) = _$GraphQLError_SchemaBuildErrorCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$GraphQLError_SchemaBuildErrorCopyWithImpl<$Res>
    implements $GraphQLError_SchemaBuildErrorCopyWith<$Res> {
  _$GraphQLError_SchemaBuildErrorCopyWithImpl(this._self, this._then);

  final GraphQLError_SchemaBuildError _self;
  final $Res Function(GraphQLError_SchemaBuildError) _then;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(GraphQLError_SchemaBuildError(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GraphQLError_RequestHandlingError extends GraphQLError {
  const GraphQLError_RequestHandlingError({required this.field0}): super._();


 final  String field0;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GraphQLError_RequestHandlingErrorCopyWith<GraphQLError_RequestHandlingError> get copyWith => _$GraphQLError_RequestHandlingErrorCopyWithImpl<GraphQLError_RequestHandlingError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError_RequestHandlingError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'GraphQLError.requestHandlingError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $GraphQLError_RequestHandlingErrorCopyWith<$Res> implements $GraphQLErrorCopyWith<$Res> {
  factory $GraphQLError_RequestHandlingErrorCopyWith(GraphQLError_RequestHandlingError value, $Res Function(GraphQLError_RequestHandlingError) _then) = _$GraphQLError_RequestHandlingErrorCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$GraphQLError_RequestHandlingErrorCopyWithImpl<$Res>
    implements $GraphQLError_RequestHandlingErrorCopyWith<$Res> {
  _$GraphQLError_RequestHandlingErrorCopyWithImpl(this._self, this._then);

  final GraphQLError_RequestHandlingError _self;
  final $Res Function(GraphQLError_RequestHandlingError) _then;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(GraphQLError_RequestHandlingError(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GraphQLError_SerializationError extends GraphQLError {
  const GraphQLError_SerializationError({required this.field0}): super._();


 final  String field0;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GraphQLError_SerializationErrorCopyWith<GraphQLError_SerializationError> get copyWith => _$GraphQLError_SerializationErrorCopyWithImpl<GraphQLError_SerializationError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError_SerializationError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'GraphQLError.serializationError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $GraphQLError_SerializationErrorCopyWith<$Res> implements $GraphQLErrorCopyWith<$Res> {
  factory $GraphQLError_SerializationErrorCopyWith(GraphQLError_SerializationError value, $Res Function(GraphQLError_SerializationError) _then) = _$GraphQLError_SerializationErrorCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$GraphQLError_SerializationErrorCopyWithImpl<$Res>
    implements $GraphQLError_SerializationErrorCopyWith<$Res> {
  _$GraphQLError_SerializationErrorCopyWithImpl(this._self, this._then);

  final GraphQLError_SerializationError _self;
  final $Res Function(GraphQLError_SerializationError) _then;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(GraphQLError_SerializationError(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GraphQLError_JsonError extends GraphQLError {
  const GraphQLError_JsonError({required this.field0}): super._();


 final  String field0;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GraphQLError_JsonErrorCopyWith<GraphQLError_JsonError> get copyWith => _$GraphQLError_JsonErrorCopyWithImpl<GraphQLError_JsonError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError_JsonError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'GraphQLError.jsonError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $GraphQLError_JsonErrorCopyWith<$Res> implements $GraphQLErrorCopyWith<$Res> {
  factory $GraphQLError_JsonErrorCopyWith(GraphQLError_JsonError value, $Res Function(GraphQLError_JsonError) _then) = _$GraphQLError_JsonErrorCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$GraphQLError_JsonErrorCopyWithImpl<$Res>
    implements $GraphQLError_JsonErrorCopyWith<$Res> {
  _$GraphQLError_JsonErrorCopyWithImpl(this._self, this._then);

  final GraphQLError_JsonError _self;
  final $Res Function(GraphQLError_JsonError) _then;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(GraphQLError_JsonError(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GraphQLError_ValidationError extends GraphQLError {
  const GraphQLError_ValidationError({required this.field0}): super._();


 final  String field0;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GraphQLError_ValidationErrorCopyWith<GraphQLError_ValidationError> get copyWith => _$GraphQLError_ValidationErrorCopyWithImpl<GraphQLError_ValidationError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError_ValidationError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'GraphQLError.validationError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $GraphQLError_ValidationErrorCopyWith<$Res> implements $GraphQLErrorCopyWith<$Res> {
  factory $GraphQLError_ValidationErrorCopyWith(GraphQLError_ValidationError value, $Res Function(GraphQLError_ValidationError) _then) = _$GraphQLError_ValidationErrorCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$GraphQLError_ValidationErrorCopyWithImpl<$Res>
    implements $GraphQLError_ValidationErrorCopyWith<$Res> {
  _$GraphQLError_ValidationErrorCopyWithImpl(this._self, this._then);

  final GraphQLError_ValidationError _self;
  final $Res Function(GraphQLError_ValidationError) _then;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(GraphQLError_ValidationError(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GraphQLError_ParseError extends GraphQLError {
  const GraphQLError_ParseError({required this.field0}): super._();


 final  String field0;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GraphQLError_ParseErrorCopyWith<GraphQLError_ParseError> get copyWith => _$GraphQLError_ParseErrorCopyWithImpl<GraphQLError_ParseError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError_ParseError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'GraphQLError.parseError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $GraphQLError_ParseErrorCopyWith<$Res> implements $GraphQLErrorCopyWith<$Res> {
  factory $GraphQLError_ParseErrorCopyWith(GraphQLError_ParseError value, $Res Function(GraphQLError_ParseError) _then) = _$GraphQLError_ParseErrorCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$GraphQLError_ParseErrorCopyWithImpl<$Res>
    implements $GraphQLError_ParseErrorCopyWith<$Res> {
  _$GraphQLError_ParseErrorCopyWithImpl(this._self, this._then);

  final GraphQLError_ParseError _self;
  final $Res Function(GraphQLError_ParseError) _then;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(GraphQLError_ParseError(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GraphQLError_AuthenticationError extends GraphQLError {
  const GraphQLError_AuthenticationError({required this.field0}): super._();


 final  String field0;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GraphQLError_AuthenticationErrorCopyWith<GraphQLError_AuthenticationError> get copyWith => _$GraphQLError_AuthenticationErrorCopyWithImpl<GraphQLError_AuthenticationError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError_AuthenticationError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'GraphQLError.authenticationError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $GraphQLError_AuthenticationErrorCopyWith<$Res> implements $GraphQLErrorCopyWith<$Res> {
  factory $GraphQLError_AuthenticationErrorCopyWith(GraphQLError_AuthenticationError value, $Res Function(GraphQLError_AuthenticationError) _then) = _$GraphQLError_AuthenticationErrorCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$GraphQLError_AuthenticationErrorCopyWithImpl<$Res>
    implements $GraphQLError_AuthenticationErrorCopyWith<$Res> {
  _$GraphQLError_AuthenticationErrorCopyWithImpl(this._self, this._then);

  final GraphQLError_AuthenticationError _self;
  final $Res Function(GraphQLError_AuthenticationError) _then;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(GraphQLError_AuthenticationError(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GraphQLError_AuthorizationError extends GraphQLError {
  const GraphQLError_AuthorizationError({required this.field0}): super._();


 final  String field0;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GraphQLError_AuthorizationErrorCopyWith<GraphQLError_AuthorizationError> get copyWith => _$GraphQLError_AuthorizationErrorCopyWithImpl<GraphQLError_AuthorizationError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError_AuthorizationError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'GraphQLError.authorizationError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $GraphQLError_AuthorizationErrorCopyWith<$Res> implements $GraphQLErrorCopyWith<$Res> {
  factory $GraphQLError_AuthorizationErrorCopyWith(GraphQLError_AuthorizationError value, $Res Function(GraphQLError_AuthorizationError) _then) = _$GraphQLError_AuthorizationErrorCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$GraphQLError_AuthorizationErrorCopyWithImpl<$Res>
    implements $GraphQLError_AuthorizationErrorCopyWith<$Res> {
  _$GraphQLError_AuthorizationErrorCopyWithImpl(this._self, this._then);

  final GraphQLError_AuthorizationError _self;
  final $Res Function(GraphQLError_AuthorizationError) _then;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(GraphQLError_AuthorizationError(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GraphQLError_NotFound extends GraphQLError {
  const GraphQLError_NotFound({required this.field0}): super._();


 final  String field0;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GraphQLError_NotFoundCopyWith<GraphQLError_NotFound> get copyWith => _$GraphQLError_NotFoundCopyWithImpl<GraphQLError_NotFound>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError_NotFound&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'GraphQLError.notFound(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $GraphQLError_NotFoundCopyWith<$Res> implements $GraphQLErrorCopyWith<$Res> {
  factory $GraphQLError_NotFoundCopyWith(GraphQLError_NotFound value, $Res Function(GraphQLError_NotFound) _then) = _$GraphQLError_NotFoundCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$GraphQLError_NotFoundCopyWithImpl<$Res>
    implements $GraphQLError_NotFoundCopyWith<$Res> {
  _$GraphQLError_NotFoundCopyWithImpl(this._self, this._then);

  final GraphQLError_NotFound _self;
  final $Res Function(GraphQLError_NotFound) _then;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(GraphQLError_NotFound(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GraphQLError_RateLimitExceeded extends GraphQLError {
  const GraphQLError_RateLimitExceeded({required this.field0}): super._();


 final  String field0;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GraphQLError_RateLimitExceededCopyWith<GraphQLError_RateLimitExceeded> get copyWith => _$GraphQLError_RateLimitExceededCopyWithImpl<GraphQLError_RateLimitExceeded>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError_RateLimitExceeded&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'GraphQLError.rateLimitExceeded(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $GraphQLError_RateLimitExceededCopyWith<$Res> implements $GraphQLErrorCopyWith<$Res> {
  factory $GraphQLError_RateLimitExceededCopyWith(GraphQLError_RateLimitExceeded value, $Res Function(GraphQLError_RateLimitExceeded) _then) = _$GraphQLError_RateLimitExceededCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$GraphQLError_RateLimitExceededCopyWithImpl<$Res>
    implements $GraphQLError_RateLimitExceededCopyWith<$Res> {
  _$GraphQLError_RateLimitExceededCopyWithImpl(this._self, this._then);

  final GraphQLError_RateLimitExceeded _self;
  final $Res Function(GraphQLError_RateLimitExceeded) _then;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(GraphQLError_RateLimitExceeded(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GraphQLError_InvalidInput extends GraphQLError {
  const GraphQLError_InvalidInput({required this.message, required this.details}): super._();


 final  String message;
 final  String details;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GraphQLError_InvalidInputCopyWith<GraphQLError_InvalidInput> get copyWith => _$GraphQLError_InvalidInputCopyWithImpl<GraphQLError_InvalidInput>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError_InvalidInput&&(identical(other.message, message) || other.message == message)&&(identical(other.details, details) || other.details == details));
}


@override
int get hashCode => Object.hash(runtimeType,message,details);

@override
String toString() {
  return 'GraphQLError.invalidInput(message: $message, details: $details)';
}


}

/// @nodoc
abstract mixin class $GraphQLError_InvalidInputCopyWith<$Res> implements $GraphQLErrorCopyWith<$Res> {
  factory $GraphQLError_InvalidInputCopyWith(GraphQLError_InvalidInput value, $Res Function(GraphQLError_InvalidInput) _then) = _$GraphQLError_InvalidInputCopyWithImpl;
@useResult
$Res call({
 String message, String details
});




}
/// @nodoc
class _$GraphQLError_InvalidInputCopyWithImpl<$Res>
    implements $GraphQLError_InvalidInputCopyWith<$Res> {
  _$GraphQLError_InvalidInputCopyWithImpl(this._self, this._then);

  final GraphQLError_InvalidInput _self;
  final $Res Function(GraphQLError_InvalidInput) _then;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? message = null,Object? details = null,}) {
  return _then(GraphQLError_InvalidInput(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,details: null == details ? _self.details : details // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GraphQLError_ComplexityLimitExceeded extends GraphQLError {
  const GraphQLError_ComplexityLimitExceeded(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError_ComplexityLimitExceeded);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'GraphQLError.complexityLimitExceeded()';
}


}




/// @nodoc


class GraphQLError_DepthLimitExceeded extends GraphQLError {
  const GraphQLError_DepthLimitExceeded(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError_DepthLimitExceeded);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'GraphQLError.depthLimitExceeded()';
}


}




/// @nodoc


class GraphQLError_InternalError extends GraphQLError {
  const GraphQLError_InternalError({required this.field0}): super._();


 final  String field0;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GraphQLError_InternalErrorCopyWith<GraphQLError_InternalError> get copyWith => _$GraphQLError_InternalErrorCopyWithImpl<GraphQLError_InternalError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GraphQLError_InternalError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'GraphQLError.internalError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $GraphQLError_InternalErrorCopyWith<$Res> implements $GraphQLErrorCopyWith<$Res> {
  factory $GraphQLError_InternalErrorCopyWith(GraphQLError_InternalError value, $Res Function(GraphQLError_InternalError) _then) = _$GraphQLError_InternalErrorCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$GraphQLError_InternalErrorCopyWithImpl<$Res>
    implements $GraphQLError_InternalErrorCopyWith<$Res> {
  _$GraphQLError_InternalErrorCopyWithImpl(this._self, this._then);

  final GraphQLError_InternalError _self;
  final $Res Function(GraphQLError_InternalError) _then;

/// Create a copy of GraphQLError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(GraphQLError_InternalError(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$SecuritySchemeInfo {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SecuritySchemeInfo);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SecuritySchemeInfo()';
}


}

/// @nodoc
class $SecuritySchemeInfoCopyWith<$Res>  {
$SecuritySchemeInfoCopyWith(SecuritySchemeInfo _, $Res Function(SecuritySchemeInfo) __);
}


/// Adds pattern-matching-related methods to [SecuritySchemeInfo].
extension SecuritySchemeInfoPatterns on SecuritySchemeInfo {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( SecuritySchemeInfo_Http value)?  http,TResult Function( SecuritySchemeInfo_ApiKey value)?  apiKey,required TResult orElse(),}){
final _that = this;
switch (_that) {
case SecuritySchemeInfo_Http() when http != null:
return http(_that);case SecuritySchemeInfo_ApiKey() when apiKey != null:
return apiKey(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( SecuritySchemeInfo_Http value)  http,required TResult Function( SecuritySchemeInfo_ApiKey value)  apiKey,}){
final _that = this;
switch (_that) {
case SecuritySchemeInfo_Http():
return http(_that);case SecuritySchemeInfo_ApiKey():
return apiKey(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( SecuritySchemeInfo_Http value)?  http,TResult? Function( SecuritySchemeInfo_ApiKey value)?  apiKey,}){
final _that = this;
switch (_that) {
case SecuritySchemeInfo_Http() when http != null:
return http(_that);case SecuritySchemeInfo_ApiKey() when apiKey != null:
return apiKey(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String scheme,  String bearerFormat)?  http,TResult Function( String location,  String name)?  apiKey,required TResult orElse(),}) {final _that = this;
switch (_that) {
case SecuritySchemeInfo_Http() when http != null:
return http(_that.scheme,_that.bearerFormat);case SecuritySchemeInfo_ApiKey() when apiKey != null:
return apiKey(_that.location,_that.name);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String scheme,  String bearerFormat)  http,required TResult Function( String location,  String name)  apiKey,}) {final _that = this;
switch (_that) {
case SecuritySchemeInfo_Http():
return http(_that.scheme,_that.bearerFormat);case SecuritySchemeInfo_ApiKey():
return apiKey(_that.location,_that.name);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String scheme,  String bearerFormat)?  http,TResult? Function( String location,  String name)?  apiKey,}) {final _that = this;
switch (_that) {
case SecuritySchemeInfo_Http() when http != null:
return http(_that.scheme,_that.bearerFormat);case SecuritySchemeInfo_ApiKey() when apiKey != null:
return apiKey(_that.location,_that.name);case _:
  return null;

}
}

}

/// @nodoc


class SecuritySchemeInfo_Http extends SecuritySchemeInfo {
  const SecuritySchemeInfo_Http({required this.scheme, required this.bearerFormat}): super._();


 final  String scheme;
 final  String bearerFormat;

/// Create a copy of SecuritySchemeInfo
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SecuritySchemeInfo_HttpCopyWith<SecuritySchemeInfo_Http> get copyWith => _$SecuritySchemeInfo_HttpCopyWithImpl<SecuritySchemeInfo_Http>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SecuritySchemeInfo_Http&&(identical(other.scheme, scheme) || other.scheme == scheme)&&(identical(other.bearerFormat, bearerFormat) || other.bearerFormat == bearerFormat));
}


@override
int get hashCode => Object.hash(runtimeType,scheme,bearerFormat);

@override
String toString() {
  return 'SecuritySchemeInfo.http(scheme: $scheme, bearerFormat: $bearerFormat)';
}


}

/// @nodoc
abstract mixin class $SecuritySchemeInfo_HttpCopyWith<$Res> implements $SecuritySchemeInfoCopyWith<$Res> {
  factory $SecuritySchemeInfo_HttpCopyWith(SecuritySchemeInfo_Http value, $Res Function(SecuritySchemeInfo_Http) _then) = _$SecuritySchemeInfo_HttpCopyWithImpl;
@useResult
$Res call({
 String scheme, String bearerFormat
});




}
/// @nodoc
class _$SecuritySchemeInfo_HttpCopyWithImpl<$Res>
    implements $SecuritySchemeInfo_HttpCopyWith<$Res> {
  _$SecuritySchemeInfo_HttpCopyWithImpl(this._self, this._then);

  final SecuritySchemeInfo_Http _self;
  final $Res Function(SecuritySchemeInfo_Http) _then;

/// Create a copy of SecuritySchemeInfo
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? scheme = null,Object? bearerFormat = null,}) {
  return _then(SecuritySchemeInfo_Http(
scheme: null == scheme ? _self.scheme : scheme // ignore: cast_nullable_to_non_nullable
as String,bearerFormat: null == bearerFormat ? _self.bearerFormat : bearerFormat // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class SecuritySchemeInfo_ApiKey extends SecuritySchemeInfo {
  const SecuritySchemeInfo_ApiKey({required this.location, required this.name}): super._();


 final  String location;
 final  String name;

/// Create a copy of SecuritySchemeInfo
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SecuritySchemeInfo_ApiKeyCopyWith<SecuritySchemeInfo_ApiKey> get copyWith => _$SecuritySchemeInfo_ApiKeyCopyWithImpl<SecuritySchemeInfo_ApiKey>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SecuritySchemeInfo_ApiKey&&(identical(other.location, location) || other.location == location)&&(identical(other.name, name) || other.name == name));
}


@override
int get hashCode => Object.hash(runtimeType,location,name);

@override
String toString() {
  return 'SecuritySchemeInfo.apiKey(location: $location, name: $name)';
}


}

/// @nodoc
abstract mixin class $SecuritySchemeInfo_ApiKeyCopyWith<$Res> implements $SecuritySchemeInfoCopyWith<$Res> {
  factory $SecuritySchemeInfo_ApiKeyCopyWith(SecuritySchemeInfo_ApiKey value, $Res Function(SecuritySchemeInfo_ApiKey) _then) = _$SecuritySchemeInfo_ApiKeyCopyWithImpl;
@useResult
$Res call({
 String location, String name
});




}
/// @nodoc
class _$SecuritySchemeInfo_ApiKeyCopyWithImpl<$Res>
    implements $SecuritySchemeInfo_ApiKeyCopyWith<$Res> {
  _$SecuritySchemeInfo_ApiKeyCopyWithImpl(this._self, this._then);

  final SecuritySchemeInfo_ApiKey _self;
  final $Res Function(SecuritySchemeInfo_ApiKey) _then;

/// Create a copy of SecuritySchemeInfo
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? location = null,Object? name = null,}) {
  return _then(SecuritySchemeInfo_ApiKey(
location: null == location ? _self.location : location // ignore: cast_nullable_to_non_nullable
as String,name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
