<?php

declare(strict_types=1);

namespace E2E\Php;

use Spikard\App;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;

/**
 * Generated App factory for PHP e2e tests.
 */
final class AppFactory
{
    public static function create_auth(): App
    {
        $app = new App();
        $app = $app->addRoute('GET', '/api/data', new Handlerauth_1());
        $app = $app->addRoute('GET', '/api/data', new Handlerauth_2());
        $app = $app->addRoute('GET', '/api/data', new Handlerauth_3());
        $app = $app->addRoute('GET', '/api/data?api_key=sk_test_123456', new Handlerauth_4());
        $app = $app->addRoute('GET', '/api/data', new Handlerauth_5());
        $app = $app->addRoute('GET', '/api/data', new Handlerauth_6());
        $app = $app->addRoute('GET', '/api/protected', new Handlerauth_7());
        $app = $app->addRoute('GET', '/protected/user', new Handlerauth_8());
        $app = $app->addRoute('GET', '/protected/user', new Handlerauth_9());
        $app = $app->addRoute('GET', '/protected/user', new Handlerauth_10());
        $app = $app->addRoute('GET', '/protected/user', new Handlerauth_11());
        $app = $app->addRoute('GET', '/protected/user', new Handlerauth_12());
        $app = $app->addRoute('GET', '/api/protected', new Handlerauth_13());
        $app = $app->addRoute('GET', '/api/protected', new Handlerauth_14());
        $app = $app->addRoute('GET', '/api/admin', new Handlerauth_15());
        $app = $app->addRoute('GET', '/api/protected', new Handlerauth_16());
        $app = $app->addRoute('GET', '/api/protected', new Handlerauth_17());
        $app = $app->addRoute('GET', '/api/data', new Handlerauth_18());
        return $app;
    }

    public static function create_background(): App
    {
        $app = new App();
        $app = $app->addRoute('POST', '/background/events', new Handlerbackground_1());
        $app = $app->addRoute('POST', '/background/events', new Handlerbackground_2());
        return $app;
    }

    public static function create_body_limits(): App
    {
        $app = new App();
        $app = $app->addRoute('POST', '/body-limit/over', new Handlerbody_limits_1());
        $app = $app->addRoute('POST', '/body-limit/under', new Handlerbody_limits_2());
        return $app;
    }

    public static function create_compression(): App
    {
        $app = new App();
        $app = $app->addRoute('GET', '/compression/gzip', new Handlercompression_1());
        $app = $app->addRoute('GET', '/compression/skip', new Handlercompression_2());
        return $app;
    }

    public static function create_content_types(): App
    {
        $app = new App();
        $app = $app->addRoute('POST', '/data', new Handlercontent_types_1());
        $app = $app->addRoute('POST', '/data', new Handlercontent_types_2());
        $app = $app->addRoute('POST', '/upload', new Handlercontent_types_3());
        $app = $app->addRoute('POST', '/data', new Handlercontent_types_4());
        $app = $app->addRoute('POST', '/api/v1/resource', new Handlercontent_types_5());
        $app = $app->addRoute('POST', '/data', new Handlercontent_types_6());
        $app = $app->addRoute('POST', '/data', new Handlercontent_types_7());
        $app = $app->addRoute('POST', '/data', new Handlercontent_types_8());
        $app = $app->addRoute('POST', '/items/', new Handlercontent_types_9());
        $app = $app->addRoute('GET', '/download/file.bin', new Handlercontent_types_10());
        $app = $app->addRoute('GET', '/export/data.csv', new Handlercontent_types_11());
        $app = $app->addRoute('GET', '/accept-test/1', new Handlercontent_types_12());
        $app = $app->addRoute('GET', '/html', new Handlercontent_types_13());
        $app = $app->addRoute('GET', '/images/photo.jpg', new Handlercontent_types_14());
        $app = $app->addRoute('GET', '/items/json', new Handlercontent_types_15());
        $app = $app->addRoute('GET', '/items/unicode', new Handlercontent_types_16());
        $app = $app->addRoute('GET', '/download/document.pdf', new Handlercontent_types_17());
        $app = $app->addRoute('GET', '/images/logo.png', new Handlercontent_types_18());
        $app = $app->addRoute('GET', '/text', new Handlercontent_types_19());
        $app = $app->addRoute('GET', '/xml', new Handlercontent_types_20());
        return $app;
    }

    public static function create_cookies(): App
    {
        $app = new App();
        $app = $app->addRoute('GET', '/secure', new Handlercookies_1());
        $app = $app->addRoute('GET', '/data', new Handlercookies_2());
        $app = $app->addRoute('GET', '/secure', new Handlercookies_3());
        $app = $app->addRoute('GET', '/secure', new Handlercookies_4());
        $app = $app->addRoute('GET', '/users/me/auth', new Handlercookies_5());
        $app = $app->addRoute('GET', '/users/me', new Handlercookies_6());
        $app = $app->addRoute('GET', '/cookies/pattern', new Handlercookies_7());
        $app = $app->addRoute('GET', '/cookies/pattern', new Handlercookies_8());
        $app = $app->addRoute('GET', '/cookies/validated', new Handlercookies_9());
        $app = $app->addRoute('GET', '/cookies/min-length', new Handlercookies_10());
        $app = $app->addRoute('GET', '/items/', new Handlercookies_11());
        $app = $app->addRoute('GET', '/items/', new Handlercookies_12());
        $app = $app->addRoute('GET', '/users/me', new Handlercookies_13());
        $app = $app->addRoute('GET', '/items/', new Handlercookies_14());
        $app = $app->addRoute('GET', '/items/', new Handlercookies_15());
        $app = $app->addRoute('GET', '/items/cookies', new Handlercookies_16());
        $app = $app->addRoute('POST', '/cookies/delete', new Handlercookies_17());
        $app = $app->addRoute('POST', '/cookies/multiple', new Handlercookies_18());
        $app = $app->addRoute('POST', '/cookies/session', new Handlercookies_19());
        $app = $app->addRoute('POST', '/cookies/samesite-lax', new Handlercookies_20());
        $app = $app->addRoute('POST', '/cookies/samesite-none', new Handlercookies_21());
        $app = $app->addRoute('POST', '/cookies/samesite-strict', new Handlercookies_22());
        $app = $app->addRoute('GET', '/cookie/set', new Handlercookies_23());
        $app = $app->addRoute('POST', '/cookies/set-with-domain', new Handlercookies_24());
        $app = $app->addRoute('POST', '/cookies/set-with-path', new Handlercookies_25());
        $app = $app->addRoute('POST', '/cookie/', new Handlercookies_26());
        return $app;
    }

    public static function create_cors(): App
    {
        $app = new App();
        $app = $app->addRoute('OPTIONS', '/api/data', new Handlercors_1());
        $app = $app->addRoute('OPTIONS', '/api/data', new Handlercors_2());
        $app = $app->addRoute('OPTIONS', '/api/data', new Handlercors_3());
        $app = $app->addRoute('GET', '/api/data', new Handlercors_4());
        $app = $app->addRoute('GET', '/api/data', new Handlercors_5());
        $app = $app->addRoute('OPTIONS', '/api/local-resource', new Handlercors_6());
        $app = $app->addRoute('GET', '/api/cached-resource', new Handlercors_7());
        $app = $app->addRoute('GET', '/api/data', new Handlercors_8());
        $app = $app->addRoute('GET', '/api/data', new Handlercors_9());
        $app = $app->addRoute('OPTIONS', '/api/resource/456', new Handlercors_10());
        $app = $app->addRoute('OPTIONS', '/api/resource/123', new Handlercors_11());
        $app = $app->addRoute('OPTIONS', '/items/', new Handlercors_12());
        $app = $app->addRoute('GET', '/api/data', new Handlercors_13());
        $app = $app->addRoute('GET', '/items/', new Handlercors_14());
        $app = $app->addRoute('POST', '/api/form', new Handlercors_15());
        $app = $app->addRoute('GET', '/public/data', new Handlercors_16());
        $app = $app->addRoute('GET', '/api/user/profile', new Handlercors_17());
        $app = $app->addRoute('GET', '/items/', new Handlercors_18());
        return $app;
    }

    public static function create_di(): App
    {
        $app = new App();
        $app = $app->addRoute('GET', '/api/db-status', new Handlerdi_1());
        $app = $app->addRoute('GET', '/api/circular', new Handlerdi_2());
        $app = $app->addRoute('GET', '/api/hook-di-test', new Handlerdi_3());
        $app = $app->addRoute('GET', '/api/timestamp', new Handlerdi_4());
        $app = $app->addRoute('GET', '/api/missing-dep', new Handlerdi_5());
        $app = $app->addRoute('GET', '/api/mixed-caching', new Handlerdi_6());
        $app = $app->addRoute('GET', '/api/multi-cleanup-test', new Handlerdi_7());
        $app = $app->addRoute('GET', '/api/auth-status', new Handlerdi_8());
        $app = $app->addRoute('GET', '/api/node-destructure', new Handlerdi_9());
        $app = $app->addRoute('GET', '/api/request-id', new Handlerdi_10());
        $app = $app->addRoute('GET', '/api/python-name-inject', new Handlerdi_11());
        $app = $app->addRoute('GET', '/api/python-type-inject', new Handlerdi_12());
        $app = $app->addRoute('GET', '/api/cleanup-test', new Handlerdi_13());
        $app = $app->addRoute('GET', '/api/override-test', new Handlerdi_14());
        $app = $app->addRoute('GET', '/api/ruby-kwargs', new Handlerdi_15());
        $app = $app->addRoute('GET', '/api/app-counter', new Handlerdi_16());
        $app = $app->addRoute('GET', '/api/type-mismatch', new Handlerdi_17());
        $app = $app->addRoute('GET', '/api/config', new Handlerdi_18());
        return $app;
    }

    public static function create_edge_cases(): App
    {
        $app = new App();
        $app = $app->addRoute('GET', '/search?term=%22caf%C3%A9%22', new Handleredge_cases_1());
        $app = $app->addRoute('GET', '/search?term=hi%20there?term=%22hi%20there%22', new Handleredge_cases_2());
        $app = $app->addRoute('GET', '/items?filter=?filter=%22%22', new Handleredge_cases_3());
        $app = $app->addRoute('GET', '/items?id=%229007199254740991%22', new Handleredge_cases_4());
        $app = $app->addRoute('POST', '/calculate', new Handleredge_cases_5());
        $app = $app->addRoute('POST', '/data', new Handleredge_cases_6());
        $app = $app->addRoute('POST', '/text', new Handleredge_cases_7());
        $app = $app->addRoute('POST', '/users', new Handleredge_cases_8());
        $app = $app->addRoute('POST', '/messages', new Handleredge_cases_9());
        $app = $app->addRoute('POST', '/files', new Handleredge_cases_10());
        $app = $app->addRoute('POST', '/calculate', new Handleredge_cases_11());
        $app = $app->addRoute('GET', '/data?value=%220123%22', new Handleredge_cases_12());
        $app = $app->addRoute('POST', '/data', new Handleredge_cases_13());
        $app = $app->addRoute('POST', '/items', new Handleredge_cases_14());
        $app = $app->addRoute('POST', '/nested/', new Handleredge_cases_15());
        $app = $app->addRoute('POST', '/nulls/', new Handleredge_cases_16());
        $app = $app->addRoute('POST', '/calculations/', new Handleredge_cases_17());
        $app = $app->addRoute('POST', '/numbers/', new Handleredge_cases_18());
        $app = $app->addRoute('POST', '/strings/', new Handleredge_cases_19());
        $app = $app->addRoute('POST', '/items/', new Handleredge_cases_20());
        return $app;
    }

    public static function create_headers(): App
    {
        $app = new App();
        $app = $app->addRoute('GET', '/protected', new Handlerheaders_1());
        $app = $app->addRoute('GET', '/protected', new Handlerheaders_2());
        $app = $app->addRoute('GET', '/protected', new Handlerheaders_3());
        $app = $app->addRoute('GET', '/api/data', new Handlerheaders_4());
        $app = $app->addRoute('GET', '/api/data', new Handlerheaders_5());
        $app = $app->addRoute('GET', '/headers/accept', new Handlerheaders_6());
        $app = $app->addRoute('GET', '/headers/accept-encoding', new Handlerheaders_7());
        $app = $app->addRoute('GET', '/headers/accept-language', new Handlerheaders_8());
        $app = $app->addRoute('GET', '/users/me', new Handlerheaders_9());
        $app = $app->addRoute('GET', '/users/me', new Handlerheaders_10());
        $app = $app->addRoute('GET', '/users/me', new Handlerheaders_11());
        $app = $app->addRoute('GET', '/headers/basic-auth', new Handlerheaders_12());
        $app = $app->addRoute('GET', '/headers/bearer-auth', new Handlerheaders_13());
        $app = $app->addRoute('GET', '/headers/bearer-auth', new Handlerheaders_14());
        $app = $app->addRoute('GET', '/headers/content-type', new Handlerheaders_15());
        $app = $app->addRoute('POST', '/echo', new Handlerheaders_16());
        $app = $app->addRoute('GET', '/headers/pattern', new Handlerheaders_17());
        $app = $app->addRoute('GET', '/headers/pattern', new Handlerheaders_18());
        $app = $app->addRoute('GET', '/headers/max-length', new Handlerheaders_19());
        $app = $app->addRoute('GET', '/headers/validated', new Handlerheaders_20());
        $app = $app->addRoute('GET', '/headers/underscore', new Handlerheaders_21());
        $app = $app->addRoute('GET', '/headers/host', new Handlerheaders_22());
        $app = $app->addRoute('GET', '/headers/multiple', new Handlerheaders_23());
        $app = $app->addRoute('GET', '/items/', new Handlerheaders_24());
        $app = $app->addRoute('GET', '/items/', new Handlerheaders_25());
        $app = $app->addRoute('GET', '/headers/origin', new Handlerheaders_26());
        $app = $app->addRoute('GET', '/headers/referer', new Handlerheaders_27());
        $app = $app->addRoute('GET', '/items/', new Handlerheaders_28());
        $app = $app->addRoute('GET', '/items/', new Handlerheaders_29());
        $app = $app->addRoute('GET', '/users/me', new Handlerheaders_30());
        $app = $app->addRoute('GET', '/users/me', new Handlerheaders_31());
        $app = $app->addRoute('GET', '/users/me', new Handlerheaders_32());
        $app = $app->addRoute('GET', '/users/me', new Handlerheaders_33());
        return $app;
    }

    public static function create_http_methods(): App
    {
        $app = new App();
        $app = $app->addRoute('DELETE', '/items/1', new Handlerhttp_methods_1());
        $app = $app->addRoute('DELETE', '/items/999', new Handlerhttp_methods_2());
        $app = $app->addRoute('DELETE', '/items/1', new Handlerhttp_methods_3());
        $app = $app->addRoute('HEAD', '/items/1', new Handlerhttp_methods_4());
        $app = $app->addRoute('OPTIONS', '/items/', new Handlerhttp_methods_5());
        $app = $app->addRoute('PATCH', '/items/1', new Handlerhttp_methods_6());
        $app = $app->addRoute('PATCH', '/items/1', new Handlerhttp_methods_7());
        $app = $app->addRoute('PUT', '/items/1', new Handlerhttp_methods_8());
        $app = $app->addRoute('PUT', '/items/999', new Handlerhttp_methods_9());
        $app = $app->addRoute('PUT', '/items/1', new Handlerhttp_methods_10());
        $app = $app->addRoute('PUT', '/items/1', new Handlerhttp_methods_11());
        $app = $app->addRoute('PUT', '/items/1', new Handlerhttp_methods_12());
        return $app;
    }

    public static function create_json_bodies(): App
    {
        $app = new App();
        $app = $app->addRoute('POST', '/users', new Handlerjson_bodies_1());
        $app = $app->addRoute('POST', '/users', new Handlerjson_bodies_2());
        $app = $app->addRoute('POST', '/users', new Handlerjson_bodies_3());
        $app = $app->addRoute('POST', '/products', new Handlerjson_bodies_4());
        $app = $app->addRoute('POST', '/items', new Handlerjson_bodies_5());
        $app = $app->addRoute('POST', '/users', new Handlerjson_bodies_6());
        $app = $app->addRoute('POST', '/payment', new Handlerjson_bodies_7());
        $app = $app->addRoute('POST', '/payment', new Handlerjson_bodies_8());
        $app = $app->addRoute('POST', '/payment', new Handlerjson_bodies_9());
        $app = $app->addRoute('POST', '/contact', new Handlerjson_bodies_10());
        $app = $app->addRoute('POST', '/contact', new Handlerjson_bodies_11());
        $app = $app->addRoute('POST', '/contact', new Handlerjson_bodies_12());
        $app = $app->addRoute('POST', '/users', new Handlerjson_bodies_13());
        $app = $app->addRoute('POST', '/users', new Handlerjson_bodies_14());
        $app = $app->addRoute('POST', '/api/v1/data', new Handlerjson_bodies_15());
        $app = $app->addRoute('POST', '/api/v1/data', new Handlerjson_bodies_16());
        $app = $app->addRoute('POST', '/config', new Handlerjson_bodies_17());
        $app = $app->addRoute('POST', '/config', new Handlerjson_bodies_18());
        $app = $app->addRoute('POST', '/config', new Handlerjson_bodies_19());
        $app = $app->addRoute('POST', '/billing', new Handlerjson_bodies_20());
        $app = $app->addRoute('POST', '/billing', new Handlerjson_bodies_21());
        $app = $app->addRoute('POST', '/data', new Handlerjson_bodies_22());
        $app = $app->addRoute('POST', '/items/list', new Handlerjson_bodies_23());
        $app = $app->addRoute('POST', '/items/', new Handlerjson_bodies_24());
        $app = $app->addRoute('POST', '/items/?limit=10?limit=10', new Handlerjson_bodies_25());
        $app = $app->addRoute('POST', '/items/', new Handlerjson_bodies_26());
        $app = $app->addRoute('POST', '/events/', new Handlerjson_bodies_27());
        $app = $app->addRoute('POST', '/events/', new Handlerjson_bodies_28());
        $app = $app->addRoute('POST', '/items/nested', new Handlerjson_bodies_29());
        $app = $app->addRoute('POST', '/items/optional-all', new Handlerjson_bodies_30());
        $app = $app->addRoute('POST', '/items/list-validated', new Handlerjson_bodies_31());
        $app = $app->addRoute('POST', '/items/', new Handlerjson_bodies_32());
        $app = $app->addRoute('POST', '/items/', new Handlerjson_bodies_33());
        $app = $app->addRoute('POST', '/items/', new Handlerjson_bodies_34());
        $app = $app->addRoute('POST', '/items/', new Handlerjson_bodies_35());
        $app = $app->addRoute('POST', '/items/nested', new Handlerjson_bodies_36());
        $app = $app->addRoute('POST', '/items/', new Handlerjson_bodies_37());
        $app = $app->addRoute('POST', '/items/validated', new Handlerjson_bodies_38());
        $app = $app->addRoute('POST', '/items/validated', new Handlerjson_bodies_39());
        $app = $app->addRoute('POST', '/items/', new Handlerjson_bodies_40());
        $app = $app->addRoute('PATCH', '/items/1', new Handlerjson_bodies_41());
        $app = $app->addRoute('POST', '/items/', new Handlerjson_bodies_42());
        $app = $app->addRoute('POST', '/items/', new Handlerjson_bodies_43());
        $app = $app->addRoute('POST', '/items/validated', new Handlerjson_bodies_44());
        $app = $app->addRoute('POST', '/items/validated', new Handlerjson_bodies_45());
        $app = $app->addRoute('POST', '/items/validated', new Handlerjson_bodies_46());
        $app = $app->addRoute('POST', '/items/validated', new Handlerjson_bodies_47());
        $app = $app->addRoute('POST', '/items/', new Handlerjson_bodies_48());
        $app = $app->addRoute('POST', '/items/', new Handlerjson_bodies_49());
        return $app;
    }

    public static function create_lifecycle_hooks(): App
    {
        $app = new App();
        $app = $app->addRoute('GET', '/api/test-hook-order', new Handlerlifecycle_hooks_1());
        $app = $app->addRoute('POST', '/api/full-lifecycle', new Handlerlifecycle_hooks_2());
        $app = $app->addRoute('GET', '/api/test-error', new Handlerlifecycle_hooks_3());
        $app = $app->addRoute('GET', '/api/test-on-request', new Handlerlifecycle_hooks_4());
        $app = $app->addRoute('GET', '/api/test-timing', new Handlerlifecycle_hooks_5());
        $app = $app->addRoute('GET', '/api/test-security-headers', new Handlerlifecycle_hooks_6());
        $app = $app->addRoute('GET', '/api/protected-resource-fail', new Handlerlifecycle_hooks_7());
        $app = $app->addRoute('GET', '/api/protected-resource', new Handlerlifecycle_hooks_8());
        $app = $app->addRoute('GET', '/api/admin-only', new Handlerlifecycle_hooks_9());
        $app = $app->addRoute('GET', '/api/admin-only-forbidden', new Handlerlifecycle_hooks_10());
        $app = $app->addRoute('POST', '/api/test-rate-limit-exceeded', new Handlerlifecycle_hooks_11());
        $app = $app->addRoute('POST', '/api/test-rate-limit', new Handlerlifecycle_hooks_12());
        return $app;
    }

    public static function create_multipart(): App
    {
        $app = new App();
        $app = $app->addRoute('POST', '/upload', new Handlermultipart_1());
        $app = $app->addRoute('POST', '/upload', new Handlermultipart_2());
        $app = $app->addRoute('POST', '/upload', new Handlermultipart_3());
        $app = $app->addRoute('POST', '/upload', new Handlermultipart_4());
        $app = $app->addRoute('POST', '/upload', new Handlermultipart_5());
        $app = $app->addRoute('POST', '/upload', new Handlermultipart_6());
        $app = $app->addRoute('POST', '/files/images-only', new Handlermultipart_7());
        $app = $app->addRoute('POST', '/files/upload', new Handlermultipart_8());
        $app = $app->addRoute('POST', '/files/list', new Handlermultipart_9());
        $app = $app->addRoute('POST', '/files/validated', new Handlermultipart_10());
        $app = $app->addRoute('POST', '/', new Handlermultipart_11());
        $app = $app->addRoute('POST', '/', new Handlermultipart_12());
        $app = $app->addRoute('POST', '/', new Handlermultipart_13());
        $app = $app->addRoute('POST', '/files/image', new Handlermultipart_14());
        $app = $app->addRoute('POST', '/', new Handlermultipart_15());
        $app = $app->addRoute('POST', '/', new Handlermultipart_16());
        $app = $app->addRoute('POST', '/', new Handlermultipart_17());
        $app = $app->addRoute('POST', '/files/optional', new Handlermultipart_18());
        $app = $app->addRoute('POST', '/files/optional', new Handlermultipart_19());
        $app = $app->addRoute('POST', '/files/document', new Handlermultipart_20());
        $app = $app->addRoute('POST', '/files/required', new Handlermultipart_21());
        $app = $app->addRoute('POST', '/', new Handlermultipart_22());
        return $app;
    }

    public static function create_path_params(): App
    {
        $app = new App();
        $app = $app->addRoute('GET', '/items/e8b5a51d-11c8-3310-a6ab-367563f20686', new Handlerpath_params_1());
        $app = $app->addRoute('GET', '/items/630eb68f-e0fa-5ecc-887a-7c7a62614681', new Handlerpath_params_2());
        $app = $app->addRoute('GET', '/events/2025-10-30', new Handlerpath_params_3());
        $app = $app->addRoute('GET', '/events/2025-13-45', new Handlerpath_params_4());
        $app = $app->addRoute('GET', '/bookings/2025-10-30T14:30:00Z', new Handlerpath_params_5());
        $app = $app->addRoute('GET', '/delays/P1DT2H30M', new Handlerpath_params_6());
        $app = $app->addRoute('GET', '/prices/19.99', new Handlerpath_params_7());
        $app = $app->addRoute('GET', '/users/alice', new Handlerpath_params_8());
        $app = $app->addRoute('GET', '/users/ab', new Handlerpath_params_9());
        $app = $app->addRoute('GET', '/users/this_username_is_way_too_long_to_be_valid', new Handlerpath_params_10());
        $app = $app->addRoute('GET', '/repos/spikard-labs/spikard-http', new Handlerpath_params_11());
        $app = $app->addRoute('GET', '/repos/invalid@owner', new Handlerpath_params_12());
        $app = $app->addRoute('GET', '/offset/-100', new Handlerpath_params_13());
        $app = $app->addRoute('GET', '/path/bool/True', new Handlerpath_params_14());
        $app = $app->addRoute('GET', '/path/bool/1', new Handlerpath_params_15());
        $app = $app->addRoute('GET', '/date/2023-07-15', new Handlerpath_params_16());
        $app = $app->addRoute('GET', '/models/foo', new Handlerpath_params_17());
        $app = $app->addRoute('GET', '/models/alexnet', new Handlerpath_params_18());
        $app = $app->addRoute('GET', '/path/float/42.5', new Handlerpath_params_19());
        $app = $app->addRoute('GET', '/path/int/foobar', new Handlerpath_params_20());
        $app = $app->addRoute('GET', '/path/int/42', new Handlerpath_params_21());
        $app = $app->addRoute('GET', '/path/param-lt-gt/2', new Handlerpath_params_22());
        $app = $app->addRoute('GET', '/path/param-ge/3', new Handlerpath_params_23());
        $app = $app->addRoute('GET', '/path/param-gt/2', new Handlerpath_params_24());
        $app = $app->addRoute('GET', '/path/param-gt/42', new Handlerpath_params_25());
        $app = $app->addRoute('GET', '/path/param-le/3', new Handlerpath_params_26());
        $app = $app->addRoute('GET', '/path/param-lt/2', new Handlerpath_params_27());
        $app = $app->addRoute('GET', '/1.0/1/abc/c892496f-b1fd-4b91-bdb8-b46f92df1716', new Handlerpath_params_28());
        $app = $app->addRoute('GET', '/type-syntax/items/not-a-uuid', new Handlerpath_params_29());
        $app = $app->addRoute('GET', '/type-syntax/items-count/50', new Handlerpath_params_30());
        $app = $app->addRoute('GET', '/type-syntax/items/550e8400-e29b-41d4-a716-446655440000', new Handlerpath_params_31());
        $app = $app->addRoute('GET', '/type-syntax/users/42', new Handlerpath_params_32());
        $app = $app->addRoute('GET', '/files/home/johndoe/myfile.txt', new Handlerpath_params_33());
        $app = $app->addRoute('GET', '/path/str/foobar', new Handlerpath_params_34());
        $app = $app->addRoute('GET', '/path/param-maxlength/foobar', new Handlerpath_params_35());
        $app = $app->addRoute('GET', '/path/param-minlength/fo', new Handlerpath_params_36());
        $app = $app->addRoute('GET', '/items/ec38df32-ceda-4cfa-9b4a-1aeb94ad551a', new Handlerpath_params_37());
        return $app;
    }

    public static function create_query_params(): App
    {
        $app = new App();
        $app = $app->addRoute('GET', '/items/negative?offset=%22-10%22', new Handlerquery_params_1());
        $app = $app->addRoute('GET', '/stats?threshold=%221.5e-3%22', new Handlerquery_params_2());
        $app = $app->addRoute('GET', '/search?term=%22foo%22', new Handlerquery_params_3());
        $app = $app->addRoute('GET', '/search?term=%22ab%22', new Handlerquery_params_4());
        $app = $app->addRoute('GET', '/search?term=%22this_is_way_too_long%22', new Handlerquery_params_5());
        $app = $app->addRoute('GET', '/subscribe?email=%22user%40example.com%22', new Handlerquery_params_6());
        $app = $app->addRoute('GET', '/subscribe?email=%22invalid-email%22', new Handlerquery_params_7());
        $app = $app->addRoute('GET', '/items?limit=%225%22', new Handlerquery_params_8());
        $app = $app->addRoute('GET', '/items?limit=%220%22', new Handlerquery_params_9());
        $app = $app->addRoute('GET', '/items?offset=%220%22', new Handlerquery_params_10());
        $app = $app->addRoute('GET', '/items?limit=%22100%22', new Handlerquery_params_11());
        $app = $app->addRoute('GET', '/items?limit=%22101%22', new Handlerquery_params_12());
        $app = $app->addRoute('GET', '/items?ids=%221%22&ids=%222%22&ids=%223%22', new Handlerquery_params_13());
        $app = $app->addRoute('GET', '/items?ids=%221%22', new Handlerquery_params_14());
        $app = $app->addRoute('GET', '/items?tags=%22a%22&tags=%22b%22&tags=%22c%22&tags=%22d%22&tags=%22e%22&tags=%22f%22', new Handlerquery_params_15());
        $app = $app->addRoute('GET', '/items?active=%22%22', new Handlerquery_params_16());
        $app = $app->addRoute('GET', '/subscribe?email=%22user%40example.com%22', new Handlerquery_params_17());
        $app = $app->addRoute('GET', '/subscribe?email=%22not-an-email%22', new Handlerquery_params_18());
        $app = $app->addRoute('GET', '/network?ip=%22192.168.1.1%22', new Handlerquery_params_19());
        $app = $app->addRoute('GET', '/network?ip=%22999.999.999.999%22', new Handlerquery_params_20());
        $app = $app->addRoute('GET', '/network/ipv6?ip=%222001%3A0db8%3A85a3%3A0000%3A0000%3A8a2e%3A0370%3A7334%22', new Handlerquery_params_21());
        $app = $app->addRoute('GET', '/redirect?url=%22https%3A%2F%2Fexample.com%2Fpath%3Fquery%3Dvalue%22', new Handlerquery_params_22());
        $app = $app->addRoute('GET', '/redirect?url=%22not%20a%20uri%22', new Handlerquery_params_23());
        $app = $app->addRoute('GET', '/dns?host=%22api.example.com%22', new Handlerquery_params_24());
        $app = $app->addRoute('GET', '/items?quantity=%2215%22', new Handlerquery_params_25());
        $app = $app->addRoute('GET', '/items?quantity=%2217%22', new Handlerquery_params_26());
        $app = $app->addRoute('GET', '/items?ids=%221%22&ids=%222%22&ids=%223%22&ids=%224%22', new Handlerquery_params_27());
        $app = $app->addRoute('GET', '/items?ids=%221%22&ids=%222%22&ids=%222%22&ids=%223%22', new Handlerquery_params_28());
        $app = $app->addRoute('GET', '/items?tags=python|rust|typescript?tags=%22python%7Crust%7Ctypescript%22', new Handlerquery_params_29());
        $app = $app->addRoute('GET', '/items?colors=red;green;blue?colors=%22red%3Bgreen%3Bblue%22', new Handlerquery_params_30());
        $app = $app->addRoute('GET', '/search?keywords=rust%20web%20framework?keywords=%22rust%20web%20framework%22', new Handlerquery_params_31());
        $app = $app->addRoute('GET', '/query/list-default', new Handlerquery_params_32());
        $app = $app->addRoute('GET', '/query/list-default?tags=%22apple%22', new Handlerquery_params_33());
        $app = $app->addRoute('GET', '/query/bool?flag=%221%22', new Handlerquery_params_34());
        $app = $app->addRoute('GET', '/query/bool?flag=%22true%22', new Handlerquery_params_35());
        $app = $app->addRoute('GET', '/query/date?event_date=%222024-01-15%22', new Handlerquery_params_36());
        $app = $app->addRoute('GET', '/query/datetime?timestamp=%222024-01-15T10%3A30%3A00Z%22', new Handlerquery_params_37());
        $app = $app->addRoute('GET', '/query/enum?model=%22vgg16%22', new Handlerquery_params_38());
        $app = $app->addRoute('GET', '/query/enum?model=%22alexnet%22', new Handlerquery_params_39());
        $app = $app->addRoute('GET', '/query/float-ge?price=%220.01%22', new Handlerquery_params_40());
        $app = $app->addRoute('GET', '/query/int-ge?value=%2210%22', new Handlerquery_params_41());
        $app = $app->addRoute('GET', '/query/int-gt?value=%221%22', new Handlerquery_params_42());
        $app = $app->addRoute('GET', '/query/int-le?value=%22100%22', new Handlerquery_params_43());
        $app = $app->addRoute('GET', '/query/int-lt?value=%2249%22', new Handlerquery_params_44());
        $app = $app->addRoute('GET', '/query/int/default', new Handlerquery_params_45());
        $app = $app->addRoute('GET', '/query/int/default?query=50', new Handlerquery_params_46());
        $app = $app->addRoute('GET', '/query/list?device_ids=1&device_ids=2', new Handlerquery_params_47());
        $app = $app->addRoute('GET', '/items/?q=%22foo%22&q=%22bar%22', new Handlerquery_params_48());
        $app = $app->addRoute('GET', '/query/list', new Handlerquery_params_49());
        $app = $app->addRoute('GET', '/query/list-default', new Handlerquery_params_50());
        $app = $app->addRoute('GET', '/query/multi-type?age=%2230%22&name=%22john%22&score=%2295.5%22&active=%22true%22', new Handlerquery_params_51());
        $app = $app->addRoute('GET', '/query/int/optional', new Handlerquery_params_52());
        $app = $app->addRoute('GET', '/query/optional-default', new Handlerquery_params_53());
        $app = $app->addRoute('GET', '/query/optional', new Handlerquery_params_54());
        $app = $app->addRoute('GET', '/query/optional?query=%22baz%22', new Handlerquery_params_55());
        $app = $app->addRoute('GET', '/query/basic?name=%22hello%20world%22', new Handlerquery_params_56());
        $app = $app->addRoute('GET', '/query/basic?name=%22test%26value%3D123%22', new Handlerquery_params_57());
        $app = $app->addRoute('GET', '/test?email=%22x%40test.com%22&special=%22%26%40A.ac%22', new Handlerquery_params_58());
        $app = $app->addRoute('GET', '/query/int?query=%2242.5%22', new Handlerquery_params_59());
        $app = $app->addRoute('GET', '/query/int?query=%22baz%22', new Handlerquery_params_60());
        $app = $app->addRoute('GET', '/query/int', new Handlerquery_params_61());
        $app = $app->addRoute('GET', '/query/int?query=42', new Handlerquery_params_62());
        $app = $app->addRoute('GET', '/query', new Handlerquery_params_63());
        $app = $app->addRoute('GET', '/query?query=%22baz%22', new Handlerquery_params_64());
        $app = $app->addRoute('GET', '/query/str-max-length?name=%22this_is_way_too_long%22', new Handlerquery_params_65());
        $app = $app->addRoute('GET', '/query/str-min-length?name=%22ab%22', new Handlerquery_params_66());
        $app = $app->addRoute('GET', '/query/pattern?code=%22abc123%22', new Handlerquery_params_67());
        $app = $app->addRoute('GET', '/items/?item_query=%22nonregexquery%22', new Handlerquery_params_68());
        $app = $app->addRoute('GET', '/items/?item_query=%22fixedquery%22', new Handlerquery_params_69());
        $app = $app->addRoute('GET', '/query/uuid?item_id=%22not-a-uuid%22', new Handlerquery_params_70());
        $app = $app->addRoute('GET', '/query/uuid?item_id=%22c892496f-b1fd-4b91-bdb8-b46f92df1716%22', new Handlerquery_params_71());
        return $app;
    }

    public static function create_rate_limit(): App
    {
        $app = new App();
        $app = $app->addRoute('GET', '/rate-limit/basic', new Handlerrate_limit_1());
        $app = $app->addRoute('GET', '/rate-limit/exceeded', new Handlerrate_limit_2());
        return $app;
    }

    public static function create_request_id(): App
    {
        $app = new App();
        $app = $app->addRoute('GET', '/request-id/preserved', new Handlerrequest_id_1());
        $app = $app->addRoute('GET', '/request-id/generated', new Handlerrequest_id_2());
        $app = $app->addRoute('GET', '/request-id/disabled', new Handlerrequest_id_3());
        return $app;
    }

    public static function create_request_timeout(): App
    {
        $app = new App();
        $app = $app->addRoute('GET', '/timeouts/fast', new Handlerrequest_timeout_1());
        $app = $app->addRoute('GET', '/timeouts/slow', new Handlerrequest_timeout_2());
        return $app;
    }

    public static function create_scripts(): App
    {
        $app = new App();
        return $app;
    }

    public static function create_static_files(): App
    {
        $app = new App();
        $app = $app->addRoute('GET', '/public/hello.txt', new Handlerstatic_files_1());
        $app = $app->addRoute('GET', '/app/', new Handlerstatic_files_2());
        return $app;
    }

    public static function create_status_codes(): App
    {
        $app = new App();
        $app = $app->addRoute('POST', '/upload', new Handlerstatus_codes_1());
        $app = $app->addRoute('GET', '/status-test/200', new Handlerstatus_codes_2());
        $app = $app->addRoute('POST', '/items/', new Handlerstatus_codes_3());
        $app = $app->addRoute('POST', '/tasks/', new Handlerstatus_codes_4());
        $app = $app->addRoute('DELETE', '/status-test/204', new Handlerstatus_codes_5());
        $app = $app->addRoute('GET', '/files/document.pdf', new Handlerstatus_codes_6());
        $app = $app->addRoute('GET', '/data?skip_template_expansion=true', new Handlerstatus_codes_7());
        $app = $app->addRoute('GET', '/data', new Handlerstatus_codes_8());
        $app = $app->addRoute('TRACE', '/data', new Handlerstatus_codes_9());
        $app = $app->addRoute('GET', '/data', new Handlerstatus_codes_10());
        $app = $app->addRoute('GET', '/old-path', new Handlerstatus_codes_11());
        $app = $app->addRoute('GET', '/temp-redirect', new Handlerstatus_codes_12());
        $app = $app->addRoute('GET', '/status-test/304', new Handlerstatus_codes_13());
        $app = $app->addRoute('POST', '/redirect-post', new Handlerstatus_codes_14());
        $app = $app->addRoute('POST', '/items/', new Handlerstatus_codes_15());
        $app = $app->addRoute('GET', '/users/me', new Handlerstatus_codes_16());
        $app = $app->addRoute('GET', '/admin/users', new Handlerstatus_codes_17());
        $app = $app->addRoute('GET', '/status-test/404', new Handlerstatus_codes_18());
        $app = $app->addRoute('POST', '/slow-endpoint', new Handlerstatus_codes_19());
        $app = $app->addRoute('POST', '/items/', new Handlerstatus_codes_20());
        $app = $app->addRoute('GET', '/api/resource', new Handlerstatus_codes_21());
        $app = $app->addRoute('GET', '/error', new Handlerstatus_codes_22());
        $app = $app->addRoute('GET', '/health', new Handlerstatus_codes_23());
        return $app;
    }

    public static function create_streaming(): App
    {
        $app = new App();
        $app = $app->addRoute('GET', '/stream/logfile', new Handlerstreaming_1());
        $app = $app->addRoute('GET', '/stream/csv-report', new Handlerstreaming_2());
        $app = $app->addRoute('GET', '/stream/json-lines', new Handlerstreaming_3());
        return $app;
    }

    public static function create_url_encoded(): App
    {
        $app = new App();
        $app = $app->addRoute('POST', '/register', new Handlerurl_encoded_1());
        $app = $app->addRoute('POST', '/profile', new Handlerurl_encoded_2());
        $app = $app->addRoute('POST', '/data', new Handlerurl_encoded_3());
        $app = $app->addRoute('POST', '/users', new Handlerurl_encoded_4());
        $app = $app->addRoute('POST', '/accounts', new Handlerurl_encoded_5());
        $app = $app->addRoute('POST', '/products', new Handlerurl_encoded_6());
        $app = $app->addRoute('POST', '/tags', new Handlerurl_encoded_7());
        $app = $app->addRoute('POST', '/subscribe', new Handlerurl_encoded_8());
        $app = $app->addRoute('POST', '/products', new Handlerurl_encoded_9());
        $app = $app->addRoute('POST', '/settings', new Handlerurl_encoded_10());
        $app = $app->addRoute('POST', '/form/', new Handlerurl_encoded_11());
        $app = $app->addRoute('POST', '/form/', new Handlerurl_encoded_12());
        $app = $app->addRoute('POST', '/form/tags', new Handlerurl_encoded_13());
        $app = $app->addRoute('POST', '/form/', new Handlerurl_encoded_14());
        $app = $app->addRoute('POST', '/token', new Handlerurl_encoded_15());
        $app = $app->addRoute('POST', '/register/', new Handlerurl_encoded_16());
        $app = $app->addRoute('POST', '/form/validated', new Handlerurl_encoded_17());
        $app = $app->addRoute('POST', '/login/', new Handlerurl_encoded_18());
        $app = $app->addRoute('POST', '/login/', new Handlerurl_encoded_19());
        $app = $app->addRoute('POST', '/form/', new Handlerurl_encoded_20());
        $app = $app->addRoute('POST', '/form/validated', new Handlerurl_encoded_21());
        $app = $app->addRoute('POST', '/form/validated', new Handlerurl_encoded_22());
        return $app;
    }

    public static function create_validation_errors(): App
    {
        $app = new App();
        $app = $app->addRoute('POST', '/users', new Handlervalidation_errors_1());
        $app = $app->addRoute('POST', '/profiles', new Handlervalidation_errors_2());
        $app = $app->addRoute('POST', '/items/', new Handlervalidation_errors_3());
        $app = $app->addRoute('POST', '/items/', new Handlervalidation_errors_4());
        $app = $app->addRoute('POST', '/items/', new Handlervalidation_errors_5());
        $app = $app->addRoute('POST', '/items/', new Handlervalidation_errors_6());
        $app = $app->addRoute('GET', '/items/?q=test', new Handlervalidation_errors_7());
        $app = $app->addRoute('GET', '/items/not-a-uuid', new Handlervalidation_errors_8());
        $app = $app->addRoute('GET', '/items/?q=test&is_active=maybe', new Handlervalidation_errors_9());
        $app = $app->addRoute('POST', '/items/', new Handlervalidation_errors_10());
        $app = $app->addRoute('GET', '/models/invalid_model', new Handlervalidation_errors_11());
        $app = $app->addRoute('POST', '/items/', new Handlervalidation_errors_12());
        $app = $app->addRoute('POST', '/items/', new Handlervalidation_errors_13());
        $app = $app->addRoute('GET', '/items/', new Handlervalidation_errors_14());
        $app = $app->addRoute('POST', '/items/', new Handlervalidation_errors_15());
        $app = $app->addRoute('POST', '/items/', new Handlervalidation_errors_16());
        $app = $app->addRoute('GET', '/items/?q=test&price=0', new Handlervalidation_errors_17());
        $app = $app->addRoute('GET', '/items/?q=test&limit=101', new Handlervalidation_errors_18());
        $app = $app->addRoute('GET', '/items/?q=test&skip=not_a_number', new Handlervalidation_errors_19());
        $app = $app->addRoute('GET', '/items/?q=this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter', new Handlervalidation_errors_20());
        $app = $app->addRoute('GET', '/items/?q=ab', new Handlervalidation_errors_21());
        $app = $app->addRoute('GET', '/items/?q=invalid!', new Handlervalidation_errors_22());
        return $app;
    }

}

final class Handlerauth_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'The provided API key is not valid', 'status' => 401, 'title' => 'Invalid API key', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
    }
}

final class Handlerauth_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Expected \'X-API-Key\' header or \'api_key\' query parameter with valid API key', 'status' => 401, 'title' => 'Missing API key', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
    }
}

final class Handlerauth_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['data' => 'sensitive information', 'message' => 'Access granted'], 200, []);
    }
}

final class Handlerauth_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['data' => 'sensitive information', 'message' => 'Access granted'], 200, []);
    }
}

final class Handlerauth_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['data' => 'sensitive information', 'message' => 'Access granted'], 200, ['X-API-Key-Deprecated' => 'true']);
    }
}

final class Handlerauth_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['data' => 'sensitive information', 'message' => 'Access granted'], 200, []);
    }
}

final class Handlerauth_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Authorization header must use Bearer scheme: \'Bearer <token>\'', 'status' => 401, 'title' => 'Invalid Authorization header format', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
    }
}

final class Handlerauth_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Token has expired', 'status' => 401, 'title' => 'JWT validation failed', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
    }
}

final class Handlerauth_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Token audience is invalid', 'status' => 401, 'title' => 'JWT validation failed', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
    }
}

final class Handlerauth_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Token signature is invalid', 'status' => 401, 'title' => 'JWT validation failed', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
    }
}

final class Handlerauth_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Expected \'Authorization: Bearer <token>\'', 'status' => 401, 'title' => 'Missing or invalid Authorization header', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
    }
}

final class Handlerauth_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Access granted', 'user_id' => 'user123'], 200, []);
    }
}

final class Handlerauth_13 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Token issuer is invalid, expected \'https://auth.example.com\'', 'status' => 401, 'title' => 'JWT validation failed', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
    }
}

final class Handlerauth_14 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Malformed JWT token: expected 3 parts separated by dots, found 2', 'status' => 401, 'title' => 'Malformed JWT token', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
    }
}

final class Handlerauth_15 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Required claims \'role\' and \'permissions\' missing from JWT', 'status' => 403, 'title' => 'Forbidden', 'type' => 'https://spikard.dev/errors/forbidden'], 403, []);
    }
}

final class Handlerauth_16 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'JWT not valid yet, not before claim is in the future', 'status' => 401, 'title' => 'JWT validation failed', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
    }
}

final class Handlerauth_17 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Access granted', 'user_id' => 'user123'], 200, []);
    }
}

final class Handlerauth_18 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['auth_method' => 'jwt', 'message' => 'Access granted', 'user_id' => 'user123'], 200, []);
    }
}

final class Handlerbackground_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 202, ['content-type' => 'application/json']);
    }
}

final class Handlerbackground_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 202, ['content-type' => 'application/json']);
    }
}

final class Handlerbody_limits_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 413, []);
    }
}

final class Handlerbody_limits_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['accepted' => true, 'note' => 'small'], 200, []);
    }
}

final class Handlercompression_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Compressed payload', 'payload' => 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'], 200, ['content-encoding' => 'gzip', 'vary' => 'Accept-Encoding']);
    }
}

final class Handlercompression_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Small payload', 'payload' => 'tiny'], 200, ['content-encoding' => '<<absent>>']);
    }
}

final class Handlercontent_types_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Unsupported charset \'utf-16\' for JSON. Only UTF-8 is supported.', 'status' => 415, 'title' => 'Unsupported Charset', 'type' => 'https://spikard.dev/errors/unsupported-charset'], 415, []);
    }
}

final class Handlercontent_types_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['name' => 'test'], 201, []);
    }
}

final class Handlercontent_types_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['error' => 'multipart/form-data requires \'boundary\' parameter'], 400, []);
    }
}

final class Handlercontent_types_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Unsupported media type', 'status' => 415, 'title' => 'Unsupported Media Type', 'type' => 'https://spikard.dev/errors/unsupported-media-type'], 415, []);
    }
}

final class Handlercontent_types_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['data' => 'value'], 201, []);
    }
}

final class Handlercontent_types_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['value' => 'test'], 201, []);
    }
}

final class Handlercontent_types_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['name' => 'test'], 201, []);
    }
}

final class Handlercontent_types_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['error' => 'Content-Length header does not match actual body size'], 400, []);
    }
}

final class Handlercontent_types_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Unsupported media type'], 415, []);
    }
}

final class Handlercontent_types_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('binary_data_placeholder', 200, ['content-disposition' => 'attachment; filename=file.bin', 'content-type' => 'application/octet-stream']);
    }
}

final class Handlercontent_types_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('id,name,price
1,Item A,10.0
2,Item B,20.0', 200, ['content-disposition' => 'attachment; filename=data.csv', 'content-type' => 'text/csv; charset=utf-8']);
    }
}

final class Handlercontent_types_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['id' => 1, 'name' => 'Item'], 200, ['content-type' => 'application/json']);
    }
}

final class Handlercontent_types_13 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('<html><body><h1>Hello</h1></body></html>', 200, ['content-type' => 'text/html; charset=utf-8']);
    }
}

final class Handlercontent_types_14 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('jpeg_binary_data', 200, ['content-type' => 'image/jpeg']);
    }
}

final class Handlercontent_types_15 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['name' => 'Item', 'price' => 42.0], 200, ['content-type' => 'application/json']);
    }
}

final class Handlercontent_types_16 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['emoji' => '☕', 'name' => 'Café'], 200, ['content-type' => 'application/json; charset=utf-8']);
    }
}

final class Handlercontent_types_17 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('pdf_binary_data', 200, ['content-type' => 'application/pdf', 'content-disposition' => 'attachment; filename=document.pdf']);
    }
}

final class Handlercontent_types_18 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('png_binary_data', 200, ['content-type' => 'image/png']);
    }
}

final class Handlercontent_types_19 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('Hello, World!', 200, ['content-type' => 'text/plain; charset=utf-8']);
    }
}

final class Handlercontent_types_20 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('<?xml version="1.0"?><item><name>Item</name><price>42.0</price></item>', 200, ['content-type' => 'application/xml']);
    }
}

final class Handlercookies_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 200, []);
    }
}

final class Handlercookies_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 200, []);
    }
}

final class Handlercookies_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 200, []);
    }
}

final class Handlercookies_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 200, []);
    }
}

final class Handlercookies_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['cookie', 'key'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlercookies_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['username' => 'secret'], 200, []);
    }
}

final class Handlercookies_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[A-Z0-9]{8}$'], 'input' => 'invalid-format', 'loc' => ['cookie', 'tracking_id'], 'msg' => 'String should match pattern \'^[A-Z0-9]{8}$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlercookies_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['tracking_id' => 'ABC12345'], 200, []);
    }
}

final class Handlercookies_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 20], 'input' => 'this_cookie_value_is_way_too_long', 'loc' => ['cookie', 'session_id'], 'msg' => 'String should have at most 20 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlercookies_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['token' => 'abc'], 200, []);
    }
}

final class Handlercookies_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => '', 'loc' => ['cookie', 'tracking_id'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlercookies_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['fatebook_tracker' => 'tracker456', 'googall_tracker' => 'ga789', 'session_id' => 'session123'], 200, []);
    }
}

final class Handlercookies_13 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['msg' => 'Create an account first'], 200, []);
    }
}

final class Handlercookies_14 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['ads_id' => null], 200, []);
    }
}

final class Handlercookies_15 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['ads_id' => 'abc123'], 200, []);
    }
}

final class Handlercookies_16 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => '', 'loc' => ['cookie', 'session_id'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlercookies_17 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Cookie deleted'], 200, []);
    }
}

final class Handlercookies_18 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Multiple cookies set'], 200, []);
    }
}

final class Handlercookies_19 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Session cookie set'], 200, []);
    }
}

final class Handlercookies_20 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Cookie set with SameSite=Lax'], 200, []);
    }
}

final class Handlercookies_21 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Cookie set with SameSite=None'], 200, []);
    }
}

final class Handlercookies_22 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Cookie set with SameSite=Strict'], 200, []);
    }
}

final class Handlercookies_23 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Cookie set'], 200, []);
    }
}

final class Handlercookies_24 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Cookie set with domain'], 200, []);
    }
}

final class Handlercookies_25 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Cookie set with path'], 200, []);
    }
}

final class Handlercookies_26 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Come to the dark side, we have cookies'], 200, []);
    }
}

final class Handlercors_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 403, []);
    }
}

final class Handlercors_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 403, []);
    }
}

final class Handlercors_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 204, ['Access-Control-Allow-Origin' => 'https://example.com', 'Access-Control-Allow-Headers' => 'Content-Type', 'Access-Control-Max-Age' => '3600', 'Access-Control-Allow-Methods' => 'POST']);
    }
}

final class Handlercors_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 200, ['Access-Control-Allow-Origin' => 'https://example.com', 'Access-Control-Expose-Headers' => 'X-Total-Count, X-Request-Id', 'X-Total-Count' => '42', 'X-Request-Id' => 'abc123']);
    }
}

final class Handlercors_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['error' => 'Origin \'null\' is not allowed'], 403, []);
    }
}

final class Handlercors_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 204, ['Access-Control-Allow-Private-Network' => 'true', 'Access-Control-Allow-Origin' => 'https://public.example.com', 'Vary' => 'Origin', 'Access-Control-Allow-Methods' => 'GET, POST']);
    }
}

final class Handlercors_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['data' => 'cacheable resource'], 200, ['Cache-Control' => 'public, max-age=3600', 'Access-Control-Allow-Origin' => 'https://app.example.com', 'Vary' => 'Origin']);
    }
}

final class Handlercors_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['data' => 'resource data'], 200, ['Vary' => 'Origin', 'Access-Control-Allow-Origin' => 'https://admin.example.com']);
    }
}

final class Handlercors_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 200, ['Vary' => 'Origin']);
    }
}

final class Handlercors_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 204, ['Vary' => 'Origin', 'Access-Control-Allow-Methods' => 'GET, POST, PUT, PATCH, DELETE', 'Access-Control-Allow-Origin' => 'https://app.example.com', 'Access-Control-Max-Age' => '3600']);
    }
}

final class Handlercors_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 204, ['Access-Control-Allow-Origin' => 'https://app.example.com', 'Access-Control-Max-Age' => '3600', 'Vary' => 'Origin', 'Access-Control-Allow-Headers' => 'Content-Type, X-Custom-Header', 'Access-Control-Allow-Methods' => 'GET, POST, PUT, PATCH, DELETE']);
    }
}

final class Handlercors_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 200, ['Access-Control-Allow-Methods' => 'GET, POST, PUT, DELETE, OPTIONS', 'Access-Control-Max-Age' => '600', 'Access-Control-Allow-Headers' => 'Content-Type, X-Custom-Header', 'Access-Control-Allow-Origin' => 'https://example.com']);
    }
}

final class Handlercors_13 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['data' => 'resource data'], 200, ['Vary' => 'Origin', 'Access-Control-Allow-Origin' => 'https://subdomain.example.com']);
    }
}

final class Handlercors_14 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'CORS request from origin \'https://malicious-site.com\' not allowed'], 403, []);
    }
}

final class Handlercors_15 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Success'], 200, ['Access-Control-Allow-Origin' => 'https://app.example.com', 'Vary' => 'Origin']);
    }
}

final class Handlercors_16 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['data' => 'public'], 200, ['Access-Control-Allow-Origin' => '*']);
    }
}

final class Handlercors_17 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['username' => 'john'], 200, ['Vary' => 'Origin', 'Access-Control-Allow-Origin' => 'https://app.example.com', 'Access-Control-Allow-Credentials' => 'true']);
    }
}

final class Handlercors_18 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['items' => []], 200, ['Vary' => 'Origin', 'Access-Control-Allow-Origin' => 'https://example.com']);
    }
}

final class Handlerdi_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['max_size' => 10, 'pool_status' => 'connected'], 200, []);
    }
}

final class Handlerdi_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Circular dependency detected', 'errors' => [['cycle' => ['service_a', 'service_b', 'service_a'], 'msg' => 'Circular dependency detected in dependency graph', 'type' => 'circular_dependency']], 'status' => 500, 'title' => 'Dependency Resolution Failed', 'type' => 'https://spikard.dev/errors/dependency-error'], 500, []);
    }
}

final class Handlerdi_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['authenticated' => true, 'logged' => true], 200, ['X-Log-Level' => 'debug', 'X-Auth-Mode' => 'strict']);
    }
}

final class Handlerdi_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['timestamp' => '<<present>>'], 200, []);
    }
}

final class Handlerdi_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Required dependency not found', 'errors' => [['dependency_key' => 'non_existent_service', 'msg' => 'Dependency \'non_existent_service\' is not registered', 'type' => 'missing_dependency']], 'status' => 500, 'title' => 'Dependency Resolution Failed', 'type' => 'https://spikard.dev/errors/dependency-error'], 500, []);
    }
}

final class Handlerdi_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['app_name' => 'MyApp', 'context_id' => '<<uuid>>', 'pool_id' => '<<uuid>>'], 200, []);
    }
}

final class Handlerdi_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['session_active' => true], 200, []);
    }
}

final class Handlerdi_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['auth_enabled' => true, 'has_cache' => true, 'has_db' => true], 200, []);
    }
}

final class Handlerdi_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['db_name' => 'PostgreSQL', 'log_level' => 'info'], 200, []);
    }
}

final class Handlerdi_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['first_id' => '<<uuid>>', 'second_id' => '<<same_as:first_id>>'], 200, []);
    }
}

final class Handlerdi_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['cache_status' => 'ready', 'db_status' => 'connected'], 200, []);
    }
}

final class Handlerdi_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['cache_type' => 'Redis', 'pool_type' => 'PostgreSQL'], 200, []);
    }
}

final class Handlerdi_13 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['session_id' => '<<uuid>>', 'status' => 'completed'], 200, []);
    }
}

final class Handlerdi_14 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['mode' => 'test', 'strict' => false], 200, []);
    }
}

final class Handlerdi_15 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['adapter' => 'postgresql', 'user_id' => 42], 200, []);
    }
}

final class Handlerdi_16 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['count' => 1, 'counter_id' => '<<uuid>>'], 200, []);
    }
}

final class Handlerdi_17 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Dependency type mismatch', 'errors' => [['actual_type' => 'string', 'dependency_key' => 'config', 'expected_type' => 'object', 'msg' => 'Dependency \'config\' type mismatch: expected object, got string', 'type' => 'type_mismatch']], 'status' => 500, 'title' => 'Dependency Resolution Failed', 'type' => 'https://spikard.dev/errors/dependency-error'], 500, []);
    }
}

final class Handlerdi_18 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['app_name' => 'SpikardApp', 'max_connections' => 100, 'version' => '1.0.0'], 200, []);
    }
}

final class Handleredge_cases_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['term' => 'café'], 200, []);
    }
}

final class Handleredge_cases_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['term' => 'hi there'], 200, []);
    }
}

final class Handleredge_cases_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['filter' => ''], 200, []);
    }
}

final class Handleredge_cases_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['id' => 9007199254740991], 200, []);
    }
}

final class Handleredge_cases_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['value' => 3.141592653589793], 201, []);
    }
}

final class Handleredge_cases_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['offset' => 0], 201, []);
    }
}

final class Handleredge_cases_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 10001, 'max_length' => 10000], 'loc' => ['body', 'content'], 'msg' => 'String length must not exceed 10000', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handleredge_cases_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['name' => 'café'], 201, []);
    }
}

final class Handleredge_cases_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['text' => 'Hello 👋 World 🌍'], 201, []);
    }
}

final class Handleredge_cases_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['value' => 'file\\u0000.txt'], 'loc' => ['body', 'filename'], 'msg' => 'String contains null byte character', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handleredge_cases_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['value' => 123000], 201, []);
    }
}

final class Handleredge_cases_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['value' => 123], 200, []);
    }
}

final class Handleredge_cases_13 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['error' => 'Request body exceeds maximum nesting depth of 32'], 400, []);
    }
}

final class Handleredge_cases_14 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['items' => ['first', 'third', 'sixth']], 200, []);
    }
}

final class Handleredge_cases_15 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['max_depth' => 10, 'message' => 'Processed deeply nested structure', 'value_found' => 'deep'], 200, []);
    }
}

final class Handleredge_cases_16 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['empty_array_length' => 0, 'empty_object_keys' => 0, 'empty_string_length' => 0, 'explicit_null_is_null' => true, 'false_is_false' => true, 'zero_is_falsy' => true], 200, []);
    }
}

final class Handleredge_cases_17 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['precise_value' => 3.141592653589793, 'sum' => 0.30000000000000004, 'very_large' => 1.7976931348623157e308, 'very_small' => 1e-10], 200, []);
    }
}

final class Handleredge_cases_18 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['large_int' => 9223372036854775807, 'max_safe_int' => 9007199254740991, 'negative_large' => -9223372036854775808], 200, []);
    }
}

final class Handleredge_cases_19 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['backslashes' => 'C:\\\\Users\\\\Path', 'empty_string' => '', 'quotes' => 'He said "hello" and \'goodbye\'', 'special_chars' => '!@#$%^&*()_+-=[]{}|;\':",./<>?', 'tabs_newlines' => 'line1
	line2
line3', 'unicode_escapes' => 'Hello', 'whitespace' => '   '], 200, []);
    }
}

final class Handleredge_cases_20 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['description' => 'Best café in München 🇩🇪', 'emoji_reactions' => '👍❤️😂🎉', 'id' => 1, 'name' => 'Coffee Shop ☕', 'tags' => ['食べ物', '音楽', '💰']], 200, []);
    }
}

final class Handlerheaders_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 200, []);
    }
}

final class Handlerheaders_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^Bearer [A-Za-z0-9-._~+/]+=*$', 'value' => 'Bearer invalid token with spaces'], 'loc' => ['headers', 'authorization'], 'msg' => 'Invalid Bearer token format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerheaders_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^Bearer [A-Za-z0-9-._~+/]+=*$', 'value' => 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9'], 'loc' => ['headers', 'authorization'], 'msg' => 'Invalid Bearer token format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerheaders_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 200, []);
    }
}

final class Handlerheaders_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-f0-9]{32}$', 'value' => 'invalid-key'], 'loc' => ['headers', 'x-api-key'], 'msg' => 'Invalid API key format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerheaders_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['accept' => 'application/json'], 200, []);
    }
}

final class Handlerheaders_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['accept_encoding' => 'gzip, deflate, br'], 200, []);
    }
}

final class Handlerheaders_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['accept_language' => 'en-US,en;q=0.9'], 200, []);
    }
}

final class Handlerheaders_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['headers', 'authorization'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerheaders_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['credentials' => 'foobar', 'scheme' => 'Digest'], 200, []);
    }
}

final class Handlerheaders_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'Other invalidauthorization', 'loc' => ['headers', 'authorization'], 'msg' => 'String should match pattern \'^Digest .+\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerheaders_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['password' => 'password', 'username' => 'username'], 200, []);
    }
}

final class Handlerheaders_13 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['headers', 'authorization'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerheaders_14 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['token' => 'valid_token_123'], 200, []);
    }
}

final class Handlerheaders_15 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['content_type' => 'application/json'], 200, []);
    }
}

final class Handlerheaders_16 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['content_type_lower' => 'application/json', 'content_type_mixed' => 'application/json', 'content_type_upper' => 'application/json'], 200, []);
    }
}

final class Handlerheaders_17 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[0-9]{3,}$'], 'input' => 'invalid-format', 'loc' => ['headers', 'x-request-id'], 'msg' => 'String should match pattern \'^[0-9]{3,}$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerheaders_18 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['x_request_id' => '12345'], 200, []);
    }
}

final class Handlerheaders_19 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 20], 'input' => 'this_is_way_too_long_for_validation', 'loc' => ['headers', 'x-session-id'], 'msg' => 'String should have at most 20 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerheaders_20 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['headers', 'x-token'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerheaders_21 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['x_token' => 'secret123'], 200, []);
    }
}

final class Handlerheaders_22 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['host' => 'example.com:8080'], 200, []);
    }
}

final class Handlerheaders_23 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['x_client_version' => '1.2.3', 'x_request_id' => 'req-12345', 'x_trace_id' => 'trace-abc'], 200, []);
    }
}

final class Handlerheaders_24 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['X-Token values' => ['foo', 'bar']], 200, []);
    }
}

final class Handlerheaders_25 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['strange_header' => null], 200, []);
    }
}

final class Handlerheaders_26 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['origin' => 'https://example.com'], 200, []);
    }
}

final class Handlerheaders_27 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['referer' => 'https://example.com/page'], 200, []);
    }
}

final class Handlerheaders_28 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['User-Agent' => 'Mozilla/5.0 Custom Browser'], 200, []);
    }
}

final class Handlerheaders_29 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['User-Agent' => 'testclient'], 200, []);
    }
}

final class Handlerheaders_30 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['msg' => 'Hello World'], 200, []);
    }
}

final class Handlerheaders_31 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['msg' => 'Hello secret'], 200, []);
    }
}

final class Handlerheaders_32 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['headers', 'x-api-key'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerheaders_33 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['username' => 'secret'], 200, []);
    }
}

final class Handlerhttp_methods_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response([], 200, []);
    }
}

final class Handlerhttp_methods_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response([], 200, []);
    }
}

final class Handlerhttp_methods_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['id' => 1, 'message' => 'Item deleted successfully', 'name' => 'Deleted Item'], 200, []);
    }
}

final class Handlerhttp_methods_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 200, ['Content-Type' => 'application/json', 'Content-Length' => '85']);
    }
}

final class Handlerhttp_methods_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 200, ['Access-Control-Allow-Methods' => 'GET, POST, PUT, DELETE, OPTIONS', 'Access-Control-Max-Age' => '86400', 'Access-Control-Allow-Origin' => 'https://example.com', 'Access-Control-Allow-Headers' => 'Content-Type']);
    }
}

final class Handlerhttp_methods_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['id' => 1, 'in_stock' => true, 'name' => 'Existing Item', 'price' => 79.99], 200, []);
    }
}

final class Handlerhttp_methods_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['id' => 1, 'in_stock' => false, 'name' => 'Updated Name', 'price' => 89.99], 200, []);
    }
}

final class Handlerhttp_methods_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['description' => 'Completely replaced', 'id' => 1, 'in_stock' => true, 'name' => 'Updated Item', 'price' => 99.99], 200, []);
    }
}

final class Handlerhttp_methods_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['id' => 999, 'name' => 'New Item', 'price' => 49.99], 200, []);
    }
}

final class Handlerhttp_methods_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['id' => 1, 'name' => 'Fixed Name', 'price' => 50.0], 200, []);
    }
}

final class Handlerhttp_methods_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => '1', 'loc' => ['body', 'price'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerhttp_methods_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '2 validation errors in request', 'errors' => [['input' => 'X', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short'], ['input' => -10, 'loc' => ['body', 'price'], 'msg' => 'Input should be greater than 0', 'type' => 'greater_than']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 201, []);
    }
}

final class Handlerjson_bodies_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['required' => true], 'loc' => ['body', 'profile', 'email'], 'msg' => 'Field required', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 201, []);
    }
}

final class Handlerjson_bodies_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 201, []);
    }
}

final class Handlerjson_bodies_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 201, []);
    }
}

final class Handlerjson_bodies_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['additional_properties' => false, 'unexpected_field' => 'extra_field'], 'loc' => ['body', 'extra_field'], 'msg' => 'Additional properties are not allowed', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 201, []);
    }
}

final class Handlerjson_bodies_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['matched_schemas' => 2], 'loc' => ['body'], 'msg' => 'Must match exactly one schema (oneOf), but matched 2', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['matched_schemas' => 0], 'loc' => ['body'], 'msg' => 'Must match exactly one schema (oneOf), but matched 0', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 201, []);
    }
}

final class Handlerjson_bodies_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 201, []);
    }
}

final class Handlerjson_bodies_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['matched_schemas' => 0], 'loc' => ['body'], 'msg' => 'Must match at least one schema (anyOf), but matched 0', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_13 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 201, []);
    }
}

final class Handlerjson_bodies_14 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['prohibited_value' => 'admin'], 'loc' => ['body', 'username'], 'msg' => 'Must not match the schema', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_15 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 201, []);
    }
}

final class Handlerjson_bodies_16 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['const' => '1.0', 'value' => '2.0'], 'loc' => ['body', 'version'], 'msg' => 'Value must be exactly \'1.0\'', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_17 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 201, []);
    }
}

final class Handlerjson_bodies_18 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_properties' => 1, 'min_properties' => 2], 'loc' => ['body'], 'msg' => 'Object must have at least 2 properties', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_19 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_properties' => 4, 'max_properties' => 3], 'loc' => ['body'], 'msg' => 'Object must have at most 3 properties', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_20 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 201, []);
    }
}

final class Handlerjson_bodies_21 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['dependency' => 'credit_card', 'required_fields' => ['billing_address']], 'loc' => ['body'], 'msg' => 'When \'credit_card\' is present, \'billing_address\' is required', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_22 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 201, []);
    }
}

final class Handlerjson_bodies_23 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['images' => [['name' => 'Front', 'url' => 'https://example.com/img1.jpg'], ['name' => 'Back', 'url' => 'https://example.com/img2.jpg']], 'name' => 'Product Bundle', 'tags' => ['electronics', 'gadget']], 200, []);
    }
}

final class Handlerjson_bodies_24 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['name' => 'Product', 'ratings' => [4.5, 4.8, 5.0, 4.2], 'tags' => ['electronics', 'gadget', 'new']], 200, []);
    }
}

final class Handlerjson_bodies_25 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['item' => ['name' => 'Item', 'price' => 42.0], 'limit' => 10], 200, []);
    }
}

final class Handlerjson_bodies_26 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['in_stock' => true, 'name' => 'Item', 'price' => 42.0], 200, []);
    }
}

final class Handlerjson_bodies_27 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['event_date' => '2024-03-15', 'name' => 'Conference'], 200, []);
    }
}

final class Handlerjson_bodies_28 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['created_at' => '2024-03-15T10:30:00Z', 'name' => 'Meeting'], 200, []);
    }
}

final class Handlerjson_bodies_29 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['name' => 'Product', 'price' => 100.0, 'seller' => ['address' => ['city' => 'Springfield', 'country' => ['code' => 'US', 'name' => 'USA'], 'street' => '123 Main St'], 'name' => 'John Doe']], 200, []);
    }
}

final class Handlerjson_bodies_30 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['description' => null, 'name' => null, 'price' => null, 'tax' => null], 200, []);
    }
}

final class Handlerjson_bodies_31 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 1], 'input' => [], 'loc' => ['body', 'tags'], 'msg' => 'List should have at least 1 item after validation', 'type' => 'too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_32 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['expected' => '\'electronics\', \'clothing\' or \'books\''], 'input' => 'furniture', 'loc' => ['body', 'category'], 'msg' => 'Input should be \'electronics\', \'clothing\' or \'books\'', 'type' => 'enum']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_33 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['category' => 'electronics', 'name' => 'Item'], 200, []);
    }
}

final class Handlerjson_bodies_34 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['name' => 'Item', 'price' => 42.0], 200, []);
    }
}

final class Handlerjson_bodies_35 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not a number', 'loc' => ['body', 'price'], 'msg' => 'Input should be a valid number', 'type' => 'float_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_36 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['image' => ['name' => 'Product Image', 'url' => 'https://example.com/image.jpg'], 'name' => 'Foo', 'price' => 42.0], 200, []);
    }
}

final class Handlerjson_bodies_37 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['description' => null, 'name' => 'Item', 'price' => 42.0, 'tax' => null], 200, []);
    }
}

final class Handlerjson_bodies_38 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['ge' => 1], 'input' => 0.5, 'loc' => ['body', 'price'], 'msg' => 'Input should be greater than or equal to 1', 'type' => 'greater_than_equal']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_39 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['name' => 'Item', 'price' => 100.0], 200, []);
    }
}

final class Handlerjson_bodies_40 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['description' => null, 'name' => 'Foo', 'price' => 35.4, 'tax' => null], 200, []);
    }
}

final class Handlerjson_bodies_41 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['description' => 'Original description', 'name' => 'Original Item', 'price' => 45.0], 200, []);
    }
}

final class Handlerjson_bodies_42 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => '', 'loc' => ['body', 'name'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_43 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['description' => 'A very nice Item', 'name' => 'Foo', 'price' => 35.4, 'tax' => 3.2], 200, []);
    }
}

final class Handlerjson_bodies_44 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 50], 'input' => 'This is a very long name that exceeds the maximum length', 'loc' => ['body', 'name'], 'msg' => 'String should have at most 50 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_45 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_46 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[A-Z]{3}[0-9]{4}$'], 'input' => 'ABC-123', 'loc' => ['body', 'sku'], 'msg' => 'String should match pattern \'^[A-Z]{3}[0-9]{4}$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_47 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['name' => 'Item', 'sku' => 'ABC1234'], 200, []);
    }
}

final class Handlerjson_bodies_48 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not-a-valid-uuid', 'loc' => ['body', 'item_id'], 'msg' => 'Input should be a valid UUID', 'type' => 'uuid_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerjson_bodies_49 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['item_id' => 'c892496f-b1fd-4b91-bdb8-b46f92df1716', 'name' => 'Item'], 200, []);
    }
}

final class Handlerlifecycle_hooks_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['execution_order' => ['first_hook', 'second_hook', 'third_hook'], 'message' => 'Hooks executed in order'], 200, []);
    }
}

final class Handlerlifecycle_hooks_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['action' => 'update_profile', 'message' => 'Action completed successfully', 'request_id' => '.*', 'user_id' => 'user-123'], 200, ['X-Request-ID' => '.*', 'X-Response-Time' => '.*ms', 'X-Frame-Options' => 'DENY', 'X-Content-Type-Options' => 'nosniff']);
    }
}

final class Handlerlifecycle_hooks_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['error' => 'Internal Server Error', 'error_id' => '.*', 'message' => 'An unexpected error occurred'], 500, ['Content-Type' => 'application/json']);
    }
}

final class Handlerlifecycle_hooks_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['has_request_id' => true, 'message' => 'onRequest hooks executed', 'request_logged' => true], 200, ['X-Request-ID' => '.*']);
    }
}

final class Handlerlifecycle_hooks_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Response with timing info'], 200, ['X-Response-Time' => '.*ms']);
    }
}

final class Handlerlifecycle_hooks_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Response with security headers'], 200, ['X-Frame-Options' => 'DENY', 'X-Content-Type-Options' => 'nosniff', 'X-XSS-Protection' => '1; mode=block', 'Strict-Transport-Security' => 'max-age=31536000; includeSubDomains']);
    }
}

final class Handlerlifecycle_hooks_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['error' => 'Unauthorized', 'message' => 'Invalid or expired authentication token'], 401, []);
    }
}

final class Handlerlifecycle_hooks_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['authenticated' => true, 'message' => 'Access granted', 'user_id' => 'user-123'], 200, []);
    }
}

final class Handlerlifecycle_hooks_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Admin access granted', 'role' => 'admin', 'user_id' => 'admin-456'], 200, []);
    }
}

final class Handlerlifecycle_hooks_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['error' => 'Forbidden', 'message' => 'Admin role required for this endpoint'], 403, []);
    }
}

final class Handlerlifecycle_hooks_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['error' => 'Rate limit exceeded', 'message' => 'Too many requests, please try again later'], 429, ['Retry-After' => '60']);
    }
}

final class Handlerlifecycle_hooks_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Request accepted', 'rate_limit_checked' => true], 200, []);
    }
}

final class Handlermultipart_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 201, []);
    }
}

final class Handlermultipart_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 201, []);
    }
}

final class Handlermultipart_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['declared_mime' => 'image/jpeg', 'detected_type' => 'image/png', 'magic_bytes' => '89504e470d0a1a0a'], 'loc' => ['files', 'image'], 'msg' => 'File type mismatch: MIME type is image/jpeg but magic numbers indicate image/png', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlermultipart_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['declared_mime' => 'image/png', 'detected_type' => 'image/jpeg', 'magic_bytes' => 'ffd8ffe0'], 'loc' => ['files', 'image'], 'msg' => 'File type mismatch: MIME type is image/png but magic numbers indicate image/jpeg', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlermultipart_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 201, []);
    }
}

final class Handlermultipart_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['buffer_size' => 0], 'loc' => ['files', 'file'], 'msg' => 'File buffer is empty', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlermultipart_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 422, []);
    }
}

final class Handlermultipart_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['filename' => 'empty.txt', 'size' => 0], 200, []);
    }
}

final class Handlermultipart_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['filenames' => ['file1.txt', 'file2.txt'], 'total_size' => 35], 200, []);
    }
}

final class Handlermultipart_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'File too large. Maximum size is 1MB'], 413, []);
    }
}

final class Handlermultipart_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['test2' => ['content' => '<file2 content>', 'content_type' => 'text/plain', 'filename' => 'test2.txt', 'headers' => [['content-disposition', 'form-data; name="test2"; filename="test2.txt"'], ['content-type', 'text/plain'], ['x-custom', 'f2']], 'size' => 15]], 200, []);
    }
}

final class Handlermultipart_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['test1' => '<file1 content>'], 200, []);
    }
}

final class Handlermultipart_13 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['some' => 'data'], 200, []);
    }
}

final class Handlermultipart_14 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['content_type' => 'image/jpeg', 'filename' => 'photo.jpg', 'size' => 22], 200, []);
    }
}

final class Handlermultipart_15 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['active' => 'true', 'age' => '25', 'file' => ['content' => 'file data here', 'content_type' => 'text/plain', 'filename' => 'upload.txt', 'size' => 14], 'username' => 'testuser'], 200, []);
    }
}

final class Handlermultipart_16 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['test1' => ['content' => '<file1 content>', 'content_type' => 'text/plain', 'filename' => 'test1.txt', 'size' => 15], 'test2' => ['content' => '<file2 content>', 'content_type' => 'text/plain', 'filename' => 'test2.txt', 'size' => 15]], 200, []);
    }
}

final class Handlermultipart_17 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['files' => [['content' => 'first file', 'content_type' => 'text/plain', 'filename' => 'file1.txt', 'size' => 10], ['content' => 'second file', 'content_type' => 'text/plain', 'filename' => 'file2.txt', 'size' => 11]], 'tags' => ['python', 'rust', 'web']], 200, []);
    }
}

final class Handlermultipart_18 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['file' => null], 200, []);
    }
}

final class Handlermultipart_19 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['content_type' => 'text/plain', 'filename' => 'optional.txt', 'size' => 21], 200, []);
    }
}

final class Handlermultipart_20 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['content_type' => 'application/pdf', 'filename' => 'report.pdf', 'size' => 16], 200, []);
    }
}

final class Handlermultipart_21 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'required', 'loc' => ['body', 'file'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlermultipart_22 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['test' => ['content' => '<file content>', 'content_type' => 'text/plain', 'filename' => 'test.txt', 'size' => 14]], 200, []);
    }
}

final class Handlerpath_params_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['id' => 'e8b5a51d-11c8-3310-a6ab-367563f20686'], 200, []);
    }
}

final class Handlerpath_params_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['id' => '630eb68f-e0fa-5ecc-887a-7c7a62614681'], 200, []);
    }
}

final class Handlerpath_params_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['date' => '2025-10-30'], 200, []);
    }
}

final class Handlerpath_params_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['format' => 'date', 'value' => '2025-13-45'], 'loc' => ['path', 'date'], 'msg' => 'Invalid date format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerpath_params_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['timestamp' => '2025-10-30T14:30:00Z'], 200, []);
    }
}

final class Handlerpath_params_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['duration' => 'P1DT2H30M'], 200, []);
    }
}

final class Handlerpath_params_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['amount' => '19.99'], 200, []);
    }
}

final class Handlerpath_params_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['username' => 'alice'], 200, []);
    }
}

final class Handlerpath_params_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 2, 'min_length' => 3], 'loc' => ['path', 'username'], 'msg' => 'String length must be at least 3', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerpath_params_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 42, 'max_length' => 20], 'loc' => ['path', 'username'], 'msg' => 'String length must not exceed 20', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerpath_params_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['owner' => 'spikard-labs', 'repo' => 'spikard-http'], 200, []);
    }
}

final class Handlerpath_params_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-zA-Z0-9-]+$', 'value' => 'invalid@owner'], 'loc' => ['path', 'owner'], 'msg' => 'String does not match pattern', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerpath_params_13 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['value' => -100], 200, []);
    }
}

final class Handlerpath_params_14 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['item_id' => true], 200, []);
    }
}

final class Handlerpath_params_15 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['item_id' => true], 200, []);
    }
}

final class Handlerpath_params_16 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['date_param' => '2023-07-15'], 200, []);
    }
}

final class Handlerpath_params_17 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['expected' => '\'alexnet\', \'resnet\' or \'lenet\''], 'input' => 'foo', 'loc' => ['path', 'model_name'], 'msg' => 'Input should be \'alexnet\', \'resnet\' or \'lenet\'', 'type' => 'enum']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerpath_params_18 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['model_name' => 'alexnet'], 200, []);
    }
}

final class Handlerpath_params_19 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['item_id' => 42.5], 200, []);
    }
}

final class Handlerpath_params_20 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'foobar', 'loc' => ['path', 'item_id'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'type' => 'int_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerpath_params_21 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['item_id' => 42], 200, []);
    }
}

final class Handlerpath_params_22 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['item_id' => 2], 200, []);
    }
}

final class Handlerpath_params_23 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['item_id' => 3], 200, []);
    }
}

final class Handlerpath_params_24 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['gt' => 3], 'input' => 2, 'loc' => ['path', 'item_id'], 'msg' => 'Input should be greater than 3', 'type' => 'greater_than']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerpath_params_25 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['item_id' => 42], 200, []);
    }
}

final class Handlerpath_params_26 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['item_id' => 3], 200, []);
    }
}

final class Handlerpath_params_27 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['item_id' => 2], 200, []);
    }
}

final class Handlerpath_params_28 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['order_id' => 'c892496f-b1fd-4b91-bdb8-b46f92df1716', 'service_id' => 1, 'user_id' => 'abc', 'version' => 1.0], 200, []);
    }
}

final class Handlerpath_params_29 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not-a-uuid', 'loc' => ['path', 'id'], 'msg' => 'Input should be a valid UUID', 'type' => 'uuid_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerpath_params_30 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['count' => '50'], 200, []);
    }
}

final class Handlerpath_params_31 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['id' => '550e8400-e29b-41d4-a716-446655440000'], 200, []);
    }
}

final class Handlerpath_params_32 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['user_id' => '42'], 200, []);
    }
}

final class Handlerpath_params_33 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['file_path' => 'home/johndoe/myfile.txt'], 200, []);
    }
}

final class Handlerpath_params_34 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['item_id' => 'foobar'], 200, []);
    }
}

final class Handlerpath_params_35 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 3], 'input' => 'foobar', 'loc' => ['path', 'item_id'], 'msg' => 'String should have at most 3 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerpath_params_36 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'fo', 'loc' => ['path', 'item_id'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerpath_params_37 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['item_id' => 'ec38df32-ceda-4cfa-9b4a-1aeb94ad551a'], 200, []);
    }
}

final class Handlerquery_params_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['offset' => -10], 200, []);
    }
}

final class Handlerquery_params_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['threshold' => 0.0015], 200, []);
    }
}

final class Handlerquery_params_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['term' => 'foo'], 200, []);
    }
}

final class Handlerquery_params_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 2, 'min_length' => 3], 'loc' => ['query', 'term'], 'msg' => 'String length must be at least 3', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 21, 'max_length' => 10], 'loc' => ['query', 'term'], 'msg' => 'String length must not exceed 10', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['email' => 'user@example.com'], 200, []);
    }
}

final class Handlerquery_params_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$', 'value' => 'invalid-email'], 'loc' => ['query', 'email'], 'msg' => 'String does not match pattern', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['limit' => 5], 200, []);
    }
}

final class Handlerquery_params_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['exclusive_minimum' => 0, 'value' => 0], 'loc' => ['query', 'limit'], 'msg' => 'Value must be greater than 0', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['offset' => 0], 200, []);
    }
}

final class Handlerquery_params_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['limit' => 100], 200, []);
    }
}

final class Handlerquery_params_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['maximum' => 100, 'value' => 101], 'loc' => ['query', 'limit'], 'msg' => 'Value must not exceed 100', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_13 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['ids' => [1, 2, 3]], 200, []);
    }
}

final class Handlerquery_params_14 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_items' => 1, 'min_items' => 2], 'loc' => ['query', 'ids'], 'msg' => 'Array must contain at least 2 items', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_15 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_items' => 6, 'max_items' => 5], 'loc' => ['query', 'tags'], 'msg' => 'Array must not contain more than 5 items', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_16 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['active' => false], 200, []);
    }
}

final class Handlerquery_params_17 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['email' => 'user@example.com'], 200, []);
    }
}

final class Handlerquery_params_18 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['format' => 'email', 'value' => 'not-an-email'], 'loc' => ['query', 'email'], 'msg' => 'Invalid email format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_19 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['ip' => '192.168.1.1'], 200, []);
    }
}

final class Handlerquery_params_20 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['format' => 'ipv4', 'value' => '999.999.999.999'], 'loc' => ['query', 'ip'], 'msg' => 'Invalid IPv4 address format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_21 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['ip' => '2001:0db8:85a3:0000:0000:8a2e:0370:7334'], 200, []);
    }
}

final class Handlerquery_params_22 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['url' => 'https://example.com/path?query=value'], 200, []);
    }
}

final class Handlerquery_params_23 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['format' => 'uri', 'value' => 'not a uri'], 'loc' => ['query', 'url'], 'msg' => 'Invalid URI format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_24 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['host' => 'api.example.com'], 200, []);
    }
}

final class Handlerquery_params_25 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['quantity' => 15], 200, []);
    }
}

final class Handlerquery_params_26 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['multiple_of' => 5, 'value' => 17], 'loc' => ['query', 'quantity'], 'msg' => 'Value must be a multiple of 5', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_27 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['ids' => [1, 2, 3, 4]], 200, []);
    }
}

final class Handlerquery_params_28 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['duplicate_index' => 2, 'duplicate_value' => 2, 'unique_items' => true], 'loc' => ['query', 'ids'], 'msg' => 'Array items must be unique', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_29 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['tags' => ['python', 'rust', 'typescript']], 200, []);
    }
}

final class Handlerquery_params_30 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['colors' => ['red', 'green', 'blue']], 200, []);
    }
}

final class Handlerquery_params_31 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['keywords' => ['rust', 'web', 'framework']], 200, []);
    }
}

final class Handlerquery_params_32 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response([], 200, []);
    }
}

final class Handlerquery_params_33 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['apple'], 200, []);
    }
}

final class Handlerquery_params_34 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['flag' => true], 200, []);
    }
}

final class Handlerquery_params_35 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['flag' => true], 200, []);
    }
}

final class Handlerquery_params_36 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['event_date' => '2024-01-15'], 200, []);
    }
}

final class Handlerquery_params_37 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['timestamp' => '2024-01-15T10:30:00Z'], 200, []);
    }
}

final class Handlerquery_params_38 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['expected' => '\'alexnet\', \'resnet\' or \'lenet\''], 'input' => 'vgg16', 'loc' => ['query', 'model'], 'msg' => 'Input should be \'alexnet\', \'resnet\' or \'lenet\'', 'type' => 'enum']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_39 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['model' => 'alexnet'], 200, []);
    }
}

final class Handlerquery_params_40 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['price' => 0.01], 200, []);
    }
}

final class Handlerquery_params_41 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['value' => 10], 200, []);
    }
}

final class Handlerquery_params_42 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['value' => 1], 200, []);
    }
}

final class Handlerquery_params_43 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['value' => 100], 200, []);
    }
}

final class Handlerquery_params_44 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['value' => 49], 200, []);
    }
}

final class Handlerquery_params_45 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('foo bar 10', 200, []);
    }
}

final class Handlerquery_params_46 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('foo bar 50', 200, []);
    }
}

final class Handlerquery_params_47 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response([1, 2], 200, []);
    }
}

final class Handlerquery_params_48 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['q' => ['foo', 'bar']], 200, []);
    }
}

final class Handlerquery_params_49 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['query', 'device_ids'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_50 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response([], 200, []);
    }
}

final class Handlerquery_params_51 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['active' => true, 'age' => 30, 'name' => 'john', 'score' => 95.5], 200, []);
    }
}

final class Handlerquery_params_52 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('foo bar None', 200, []);
    }
}

final class Handlerquery_params_53 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['limit' => 10], 200, []);
    }
}

final class Handlerquery_params_54 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('foo bar None', 200, []);
    }
}

final class Handlerquery_params_55 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('foo bar baz', 200, []);
    }
}

final class Handlerquery_params_56 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['name' => 'hello world'], 200, []);
    }
}

final class Handlerquery_params_57 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['name' => 'test&value=123'], 200, []);
    }
}

final class Handlerquery_params_58 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['email' => 'x@test.com', 'special' => '&@A.ac'], 200, []);
    }
}

final class Handlerquery_params_59 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 42.5, 'loc' => ['query', 'query'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'type' => 'int_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_60 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'baz', 'loc' => ['query', 'query'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'type' => 'int_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_61 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['query', 'query'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_62 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('foo bar 42', 200, []);
    }
}

final class Handlerquery_params_63 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['query', 'query'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_64 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('foo bar baz', 200, []);
    }
}

final class Handlerquery_params_65 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 10], 'input' => 'this_is_way_too_long', 'loc' => ['query', 'name'], 'msg' => 'String should have at most 10 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_66 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['query', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_67 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[0-9]{3,}$'], 'input' => 'abc123', 'loc' => ['query', 'code'], 'msg' => 'String should match pattern \'^[0-9]{3,}$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_68 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^fixedquery$'], 'input' => 'nonregexquery', 'loc' => ['query', 'item_query'], 'msg' => 'String should match pattern \'^fixedquery$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_69 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['item_query' => 'fixedquery'], 200, []);
    }
}

final class Handlerquery_params_70 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not-a-uuid', 'loc' => ['query', 'item_id'], 'msg' => 'Input should be a valid UUID', 'type' => 'uuid_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerquery_params_71 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['item_id' => 'c892496f-b1fd-4b91-bdb8-b46f92df1716'], 200, []);
    }
}

final class Handlerrate_limit_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['request' => 'under-limit', 'status' => 'ok'], 200, []);
    }
}

final class Handlerrate_limit_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 429, []);
    }
}

final class Handlerrequest_id_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['echo' => 'trace-123', 'status' => 'preserved'], 200, ['x-request-id' => 'trace-123']);
    }
}

final class Handlerrequest_id_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['status' => 'generated'], 200, ['x-request-id' => '<<uuid>>']);
    }
}

final class Handlerrequest_id_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['status' => 'no-request-id'], 200, ['x-request-id' => '<<absent>>']);
    }
}

final class Handlerrequest_timeout_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['duration' => 'fast', 'status' => 'ok'], 200, []);
    }
}

final class Handlerrequest_timeout_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 408, []);
    }
}

final class Handlerstatic_files_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('Hello from static storage', 200, ['content-type' => 'text/plain', 'cache-control' => 'public, max-age=60']);
    }
}

final class Handlerstatic_files_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('<!doctype html><h1>Welcome</h1>', 200, ['content-type' => 'text/html']);
    }
}

final class Handlerstatus_codes_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['error' => 'Payload Too Large', 'message' => 'Request body size exceeds maximum allowed size of 1024 bytes'], 413, []);
    }
}

final class Handlerstatus_codes_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['id' => 1, 'name' => 'Item 1'], 200, []);
    }
}

final class Handlerstatus_codes_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['id' => 1, 'name' => 'New Item'], 201, []);
    }
}

final class Handlerstatus_codes_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['message' => 'Task accepted for processing', 'task_id' => 'abc123'], 202, []);
    }
}

final class Handlerstatus_codes_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 204, []);
    }
}

final class Handlerstatus_codes_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('binary_data_1024_bytes', 206, ['Content-Type' => 'application/pdf', 'Content-Length' => '1024', 'Accept-Ranges' => 'bytes', 'Content-Range' => 'bytes 0-1023/5000']);
    }
}

final class Handlerstatus_codes_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response([], 200, []);
    }
}

final class Handlerstatus_codes_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['error' => 'Request Header Fields Too Large', 'message' => 'Request headers exceed maximum allowed size of 8192 bytes'], 431, []);
    }
}

final class Handlerstatus_codes_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 405, []);
    }
}

final class Handlerstatus_codes_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['error' => 'Service Unavailable', 'message' => 'The service is temporarily unavailable. Please try again later.'], 503, ['Retry-After' => '60']);
    }
}

final class Handlerstatus_codes_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 301, ['location' => '/new-path']);
    }
}

final class Handlerstatus_codes_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 302, ['location' => '/target-path']);
    }
}

final class Handlerstatus_codes_13 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(null, 304, []);
    }
}

final class Handlerstatus_codes_14 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response([], 307, ['location' => '/target-post']);
    }
}

final class Handlerstatus_codes_15 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Invalid request format'], 400, []);
    }
}

final class Handlerstatus_codes_16 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Not authenticated'], 401, ['www-authenticate' => 'Bearer']);
    }
}

final class Handlerstatus_codes_17 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Not enough permissions'], 403, []);
    }
}

final class Handlerstatus_codes_18 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Item not found'], 404, []);
    }
}

final class Handlerstatus_codes_19 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Request timeout'], 408, ['Connection' => 'close']);
    }
}

final class Handlerstatus_codes_20 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => '', 'loc' => ['body', 'name'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerstatus_codes_21 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Rate limit exceeded. Try again in 60 seconds.'], 429, ['Retry-After' => '60', 'X-RateLimit-Limit' => '100', 'X-RateLimit-Reset' => '1609459200', 'X-RateLimit-Remaining' => '0']);
    }
}

final class Handlerstatus_codes_22 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Internal server error', 'status' => 500, 'title' => 'Internal Server Error', 'type' => 'https://spikard.dev/errors/internal-server-error'], 500, []);
    }
}

final class Handlerstatus_codes_23 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Service temporarily unavailable'], 503, ['retry-after' => '120']);
    }
}

final class Handlerstreaming_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('LOG:\\u0000\\u0001\\u0002\\u0003|TAIL|\\u0007\\n', 200, ['content-type' => 'application/octet-stream']);
    }
}

final class Handlerstreaming_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('id,name,value\\n1,Alice,42\\n2,Bob,7\\n', 200, ['content-type' => 'text/csv']);
    }
}

final class Handlerstreaming_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response('{"index":0,"payload":"alpha"}\\n{"index":1,"payload":"beta"}\\n{"index":2,"payload":"gamma"}\\n', 200, ['content-type' => 'application/x-ndjson']);
    }
}

final class Handlerurl_encoded_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['tags' => ['python', 'rust', 'typescript']], 201, []);
    }
}

final class Handlerurl_encoded_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['user' => ['age' => 30, 'email' => 'john@example.com', 'name' => 'John Doe']], 201, []);
    }
}

final class Handlerurl_encoded_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['contact.email' => 'john@example.com', 'user-name' => 'JohnDoe'], 201, []);
    }
}

final class Handlerurl_encoded_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 2, 'min_length' => 3, 'value' => 'ab'], 'loc' => ['body', 'username'], 'msg' => 'String length must be at least 3', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerurl_encoded_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^ACC-[0-9]{6}$', 'value' => 'INVALID123'], 'loc' => ['body', 'account_id'], 'msg' => 'String does not match pattern \'^ACC-[0-9]{6}$\'', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerurl_encoded_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_value' => 0, 'minimum' => 1], 'loc' => ['body', 'quantity'], 'msg' => 'Value must be at least 1', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerurl_encoded_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_items' => 1, 'min_items' => 2], 'loc' => ['body', 'tags'], 'msg' => 'Array must contain at least 2 items', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerurl_encoded_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['format' => 'email', 'value' => 'not-an-email'], 'loc' => ['body', 'email'], 'msg' => 'Invalid email format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerurl_encoded_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['value' => 'not-a-number'], 'loc' => ['body', 'price'], 'msg' => 'Value is not a valid integer', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerurl_encoded_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['property' => 'unknown_field'], 'loc' => ['body', 'unknown_field'], 'msg' => 'Additional properties are not allowed', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerurl_encoded_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['subscribe' => true, 'username' => 'johndoe'], 200, []);
    }
}

final class Handlerurl_encoded_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['description' => '', 'username' => 'johndoe'], 200, []);
    }
}

final class Handlerurl_encoded_13 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['tags' => ['python', 'fastapi', 'web']], 200, []);
    }
}

final class Handlerurl_encoded_14 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['age' => 30, 'username' => 'johndoe'], 200, []);
    }
}

final class Handlerurl_encoded_15 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['access_token' => 'johndoe', 'token_type' => 'bearer'], 200, []);
    }
}

final class Handlerurl_encoded_16 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['email' => null, 'username' => 'johndoe'], 200, []);
    }
}

final class Handlerurl_encoded_17 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-z0-9_]+$'], 'input' => 'john doe', 'loc' => ['body', 'username'], 'msg' => 'String should match pattern \'^[a-z0-9_]+$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerurl_encoded_18 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => '', 'loc' => ['body', 'username'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerurl_encoded_19 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['username' => 'johndoe'], 200, []);
    }
}

final class Handlerurl_encoded_20 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['description' => 'Test & Development', 'name' => 'John Doe'], 200, []);
    }
}

final class Handlerurl_encoded_21 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 20], 'input' => 'this_is_a_very_long_username_that_exceeds_limit', 'loc' => ['body', 'username'], 'msg' => 'String should have at most 20 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlerurl_encoded_22 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['body', 'username'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_1 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '3 validation errors in request', 'errors' => [['ctx' => ['ge' => 18], 'input' => 15, 'loc' => ['body', 'age'], 'msg' => 'Input should be greater than or equal to 18', 'type' => 'greater_than_equal'], ['ctx' => ['pattern' => '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$'], 'input' => 'invalid-email', 'loc' => ['body', 'email'], 'msg' => 'String should match pattern \'^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$\'', 'type' => 'string_pattern_mismatch'], ['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_2 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$'], 'input' => 'invalid', 'loc' => ['body', 'profile', 'contact', 'email'], 'msg' => 'String should match pattern \'^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_3 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 123, 'loc' => ['body', 'tags', '2'], 'msg' => 'Input should be a valid unknown', 'type' => 'type_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_4 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => ['tag1', 'tag2', 'tag3', 'tag4', 'tag5', 'tag6', 'tag7', 'tag8', 'tag9', 'tag10', 'tag11'], 'loc' => ['body', 'tags'], 'msg' => '["tag1","tag2","tag3","tag4","tag5","tag6","tag7","tag8","tag9","tag10","tag11"] has more than 10 items', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_5 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => [], 'loc' => ['body', 'tags'], 'msg' => '[] has less than 1 item', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_6 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not_a_float', 'loc' => ['body', 'price'], 'msg' => 'Input should be a valid number, unable to parse string as a number', 'type' => 'float_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_7 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['headers', 'x-token'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_8 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not-a-uuid', 'loc' => ['path', 'item_id'], 'msg' => 'Input should be a valid UUID, invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `n` at 0', 'type' => 'uuid_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_9 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'maybe', 'loc' => ['query', 'is_active'], 'msg' => 'Input should be a valid boolean, unable to interpret input', 'type' => 'bool_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_10 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not-a-datetime', 'loc' => ['body', 'created_at'], 'msg' => 'Input should be a valid datetime', 'type' => 'datetime_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_11 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['expected' => '\'alexnet\', \'resnet\' or \'lenet\''], 'input' => 'invalid_model', 'loc' => ['path', 'model_name'], 'msg' => 'Input should be \'alexnet\', \'resnet\' or \'lenet\'', 'type' => 'enum']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_12 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => 'Invalid request format'], 400, []);
    }
}

final class Handlervalidation_errors_13 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => ['name' => 'Item'], 'loc' => ['body', 'price'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_14 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['query', 'q'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_15 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '3 validation errors in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'X', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short'], ['ctx' => ['gt' => 0], 'input' => -10, 'loc' => ['body', 'price'], 'msg' => 'Input should be greater than 0', 'type' => 'greater_than'], ['input' => 'not_a_number', 'loc' => ['body', 'quantity'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'type' => 'int_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_16 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '3 validation errors in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'SF', 'loc' => ['body', 'seller', 'address', 'city'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short'], ['ctx' => ['min_length' => 5], 'input' => '123', 'loc' => ['body', 'seller', 'address', 'zip_code'], 'msg' => 'String should have at least 5 characters', 'type' => 'string_too_short'], ['ctx' => ['min_length' => 3], 'input' => 'Jo', 'loc' => ['body', 'seller', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_17 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['gt' => 0], 'input' => '0', 'loc' => ['query', 'price'], 'msg' => 'Input should be greater than 0', 'type' => 'greater_than']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_18 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['le' => 100], 'input' => '101', 'loc' => ['query', 'limit'], 'msg' => 'Input should be less than or equal to 100', 'type' => 'less_than_equal']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_19 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not_a_number', 'loc' => ['query', 'skip'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'type' => 'int_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_20 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 50], 'input' => 'this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter', 'loc' => ['query', 'q'], 'msg' => 'String should have at most 50 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_21 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['query', 'q'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}

final class Handlervalidation_errors_22 implements HandlerInterface {
    public function handle(Request $request): Response
    {
        return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-zA-Z0-9_-]+$'], 'input' => 'invalid!', 'loc' => ['query', 'q'], 'msg' => 'String should match pattern \'^[a-zA-Z0-9_-]+$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
    }
}
