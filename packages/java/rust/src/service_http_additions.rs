// Error type constructor for NotFoundError
// Status: 404, ProblemDetails type: ""
#[no_mangle]
pub unsafe extern "system" fn Java_dev_spikard_Errors_createNotFoundError(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    message: jni::objects::JString,
) -> jni::objects::JObject {
    catch_exc_and_return!(env; {
        let msg: String = env.get_string(&message)?.into();

        // Create the exception instance on the Java side
        // The Java class NotFoundError is expected to have a constructor
        // accepting (String message, int statusCode, String problemDetailsType)
        let error_class = env.find_class("dev/spikard/errors/NotFoundError")?;

        let constructor_sig = "(Ljava/lang/String;ILjava/lang/String;)V";
        let constructor_id = env.get_method_id(&error_class, "<init>", constructor_sig)?;

        let msg_jstr = env.new_string(&msg)?;
        let problem_type_str = env.new_string("")?;

        let args = [
            jni::objects::JValue::from(msg_jstr),
            jni::objects::JValue::from(404),
            jni::objects::JValue::from(problem_type_str),
        ];

        let instance = env.new_object_unchecked(&error_class, constructor_id, &args)?;

        instance.into()
    })
}

// Error type constructor for ValidationError
// Status: 422, ProblemDetails type: ""
#[no_mangle]
pub unsafe extern "system" fn Java_dev_spikard_Errors_createValidationError(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    message: jni::objects::JString,
) -> jni::objects::JObject {
    catch_exc_and_return!(env; {
        let msg: String = env.get_string(&message)?.into();

        // Create the exception instance on the Java side
        // The Java class ValidationError is expected to have a constructor
        // accepting (String message, int statusCode, String problemDetailsType)
        let error_class = env.find_class("dev/spikard/errors/ValidationError")?;

        let constructor_sig = "(Ljava/lang/String;ILjava/lang/String;)V";
        let constructor_id = env.get_method_id(&error_class, "<init>", constructor_sig)?;

        let msg_jstr = env.new_string(&msg)?;
        let problem_type_str = env.new_string("")?;

        let args = [
            jni::objects::JValue::from(msg_jstr),
            jni::objects::JValue::from(422),
            jni::objects::JValue::from(problem_type_str),
        ];

        let instance = env.new_object_unchecked(&error_class, constructor_id, &args)?;

        instance.into()
    })
}

// Error type constructor for UnauthorizedError
// Status: 401, ProblemDetails type: ""
#[no_mangle]
pub unsafe extern "system" fn Java_dev_spikard_Errors_createUnauthorizedError(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    message: jni::objects::JString,
) -> jni::objects::JObject {
    catch_exc_and_return!(env; {
        let msg: String = env.get_string(&message)?.into();

        // Create the exception instance on the Java side
        // The Java class UnauthorizedError is expected to have a constructor
        // accepting (String message, int statusCode, String problemDetailsType)
        let error_class = env.find_class("dev/spikard/errors/UnauthorizedError")?;

        let constructor_sig = "(Ljava/lang/String;ILjava/lang/String;)V";
        let constructor_id = env.get_method_id(&error_class, "<init>", constructor_sig)?;

        let msg_jstr = env.new_string(&msg)?;
        let problem_type_str = env.new_string("")?;

        let args = [
            jni::objects::JValue::from(msg_jstr),
            jni::objects::JValue::from(401),
            jni::objects::JValue::from(problem_type_str),
        ];

        let instance = env.new_object_unchecked(&error_class, constructor_id, &args)?;

        instance.into()
    })
}

// Error type constructor for ForbiddenError
// Status: 403, ProblemDetails type: ""
#[no_mangle]
pub unsafe extern "system" fn Java_dev_spikard_Errors_createForbiddenError(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    message: jni::objects::JString,
) -> jni::objects::JObject {
    catch_exc_and_return!(env; {
        let msg: String = env.get_string(&message)?.into();

        // Create the exception instance on the Java side
        // The Java class ForbiddenError is expected to have a constructor
        // accepting (String message, int statusCode, String problemDetailsType)
        let error_class = env.find_class("dev/spikard/errors/ForbiddenError")?;

        let constructor_sig = "(Ljava/lang/String;ILjava/lang/String;)V";
        let constructor_id = env.get_method_id(&error_class, "<init>", constructor_sig)?;

        let msg_jstr = env.new_string(&msg)?;
        let problem_type_str = env.new_string("")?;

        let args = [
            jni::objects::JValue::from(msg_jstr),
            jni::objects::JValue::from(403),
            jni::objects::JValue::from(problem_type_str),
        ];

        let instance = env.new_object_unchecked(&error_class, constructor_id, &args)?;

        instance.into()
    })
}

// Error type constructor for RateLimitedError
// Status: 429, ProblemDetails type: ""
#[no_mangle]
pub unsafe extern "system" fn Java_dev_spikard_Errors_createRateLimitedError(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    message: jni::objects::JString,
) -> jni::objects::JObject {
    catch_exc_and_return!(env; {
        let msg: String = env.get_string(&message)?.into();

        // Create the exception instance on the Java side
        // The Java class RateLimitedError is expected to have a constructor
        // accepting (String message, int statusCode, String problemDetailsType)
        let error_class = env.find_class("dev/spikard/errors/RateLimitedError")?;

        let constructor_sig = "(Ljava/lang/String;ILjava/lang/String;)V";
        let constructor_id = env.get_method_id(&error_class, "<init>", constructor_sig)?;

        let msg_jstr = env.new_string(&msg)?;
        let problem_type_str = env.new_string("")?;

        let args = [
            jni::objects::JValue::from(msg_jstr),
            jni::objects::JValue::from(429),
            jni::objects::JValue::from(problem_type_str),
        ];

        let instance = env.new_object_unchecked(&error_class, constructor_id, &args)?;

        instance.into()
    })
}

// Error type constructor for ConflictError
// Status: 409, ProblemDetails type: ""
#[no_mangle]
pub unsafe extern "system" fn Java_dev_spikard_Errors_createConflictError(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    message: jni::objects::JString,
) -> jni::objects::JObject {
    catch_exc_and_return!(env; {
        let msg: String = env.get_string(&message)?.into();

        // Create the exception instance on the Java side
        // The Java class ConflictError is expected to have a constructor
        // accepting (String message, int statusCode, String problemDetailsType)
        let error_class = env.find_class("dev/spikard/errors/ConflictError")?;

        let constructor_sig = "(Ljava/lang/String;ILjava/lang/String;)V";
        let constructor_id = env.get_method_id(&error_class, "<init>", constructor_sig)?;

        let msg_jstr = env.new_string(&msg)?;
        let problem_type_str = env.new_string("")?;

        let args = [
            jni::objects::JValue::from(msg_jstr),
            jni::objects::JValue::from(409),
            jni::objects::JValue::from(problem_type_str),
        ];

        let instance = env.new_object_unchecked(&error_class, constructor_id, &args)?;

        instance.into()
    })
}

// Error type constructor for InternalError
// Status: 500, ProblemDetails type: ""
#[no_mangle]
pub unsafe extern "system" fn Java_dev_spikard_Errors_createInternalError(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    message: jni::objects::JString,
) -> jni::objects::JObject {
    catch_exc_and_return!(env; {
        let msg: String = env.get_string(&message)?.into();

        // Create the exception instance on the Java side
        // The Java class InternalError is expected to have a constructor
        // accepting (String message, int statusCode, String problemDetailsType)
        let error_class = env.find_class("dev/spikard/errors/InternalError")?;

        let constructor_sig = "(Ljava/lang/String;ILjava/lang/String;)V";
        let constructor_id = env.get_method_id(&error_class, "<init>", constructor_sig)?;

        let msg_jstr = env.new_string(&msg)?;
        let problem_type_str = env.new_string("")?;

        let args = [
            jni::objects::JValue::from(msg_jstr),
            jni::objects::JValue::from(500),
            jni::objects::JValue::from(problem_type_str),
        ];

        let instance = env.new_object_unchecked(&error_class, constructor_id, &args)?;

        instance.into()
    })
}
