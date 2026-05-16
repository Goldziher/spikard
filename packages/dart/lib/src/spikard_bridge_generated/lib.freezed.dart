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
