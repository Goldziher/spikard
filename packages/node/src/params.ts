/**
 * Parameter types for Spikard route handlers
 *
 * Generic types that allow for type-safe extraction and validation
 * of query parameters, path parameters, and request bodies using Zod schemas.
 */

/**
 * Query parameter with type-safe extraction
 *
 * Generic type that allows Zod schema introspection for runtime validation.
 *
 * @example
 * ```typescript
 * import { z } from 'zod';
 *
 * function handler(limit: Query<z.infer<typeof z.string()>>) {
 *   // limit is typed as string
 * }
 * ```
 */
export type Query<T> = T;

/**
 * Path parameter with type-safe extraction
 *
 * Generic type that allows Zod schema introspection for runtime validation.
 *
 * @example
 * ```typescript
 * import { z } from 'zod';
 *
 * function handler(id: Path<number>) {
 *   // id is typed as number
 * }
 * ```
 */
export type Path<T> = T;

/**
 * Request body with type-safe extraction and validation
 *
 * Generic type that allows Zod schema introspection for runtime validation.
 *
 * @example
 * ```typescript
 * import { z } from 'zod';
 *
 * const UserSchema = z.object({
 *   name: z.string(),
 *   email: z.string().email(),
 * });
 *
 * function handler(body: Body<z.infer<typeof UserSchema>>) {
 *   // body is typed as { name: string, email: string }
 * }
 * ```
 */
export type Body<T> = T;

/**
 * Query parameter default value helper
 *
 * Used to specify default values for optional query parameters.
 *
 * @example
 * ```typescript
 * function handler(limit: Query<string | undefined> = QueryDefault("10")) {
 *   // limit defaults to "10" if not provided
 * }
 * ```
 */
export function QueryDefault<T>(value: T): T {
	return value;
}
