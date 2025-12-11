<?php

declare(strict_types=1);

namespace Spikard\Tests;

use InvalidArgumentException;
use PHPUnit\Framework\TestCase;
use Spikard\Http\JsonRpcMethodInfo;

final class JsonRpcMethodInfoTest extends TestCase
{
    public function testToArrayRoundTrip(): void
    {
        $info = new JsonRpcMethodInfo(
            methodName: 'user.create',
            description: 'Create a user',
            paramsSchema: ['type' => 'object'],
            resultSchema: ['type' => 'object'],
            deprecated: true,
            tags: ['users', 'beta'],
        );

        $array = $info->toArray();

        self::assertSame('user.create', $array['method_name']);
        self::assertSame('Create a user', $array['description']);
        self::assertSame(['type' => 'object'], $array['params_schema']);
        self::assertSame(['type' => 'object'], $array['result_schema']);
        self::assertTrue($array['deprecated']);
        self::assertSame(['users', 'beta'], $array['tags']);

        $roundTrip = JsonRpcMethodInfo::fromArray($array);
        self::assertSame('user.create', $roundTrip->methodName);
        self::assertSame('Create a user', $roundTrip->description);
        self::assertSame(['type' => 'object'], $roundTrip->paramsSchema);
        self::assertSame(['type' => 'object'], $roundTrip->resultSchema);
        self::assertTrue($roundTrip->deprecated);
        self::assertSame(['users', 'beta'], $roundTrip->tags);
    }

    public function testRejectsEmptyMethodName(): void
    {
        $this->expectException(InvalidArgumentException::class);
        new JsonRpcMethodInfo('');
    }

    public function testRejectsInvalidMethodNameCharacters(): void
    {
        $this->expectException(InvalidArgumentException::class);
        new JsonRpcMethodInfo('bad name!');
    }

    public function testFromArrayValidatesTypes(): void
    {
        $this->expectException(\TypeError::class);
        JsonRpcMethodInfo::fromArray([
            'method_name' => ['not-a-string'],
        ]);
    }
}
