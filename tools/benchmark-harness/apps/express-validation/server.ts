#!/usr/bin/env node
/**
 * Express comparison server for benchmarking (TypeScript)
 *
 * Implements all 18 workload endpoints with Zod validation.
 * Uses Express middleware pattern for request validation.
 */

import express, { type Express, type NextFunction, type Request, type RequestHandler, type Response } from "express";
import { type ZodSchema, z } from "zod";

const app: Express = express();

app.use(express.json());
app.use(express.urlencoded({ extended: true }));

const SmallPayloadSchema = z.object({
	name: z.string(),
	description: z.string(),
	price: z.number(),
	tax: z.number(),
});

const ImageSchema = z.object({
	url: z.string(),
	name: z.string(),
});

const MediumPayloadSchema = z.object({
	name: z.string(),
	price: z.number(),
	image: ImageSchema,
});

const CountrySchema = z.object({
	name: z.string(),
	code: z.string(),
});

const AddressSchema = z.object({
	street: z.string(),
	city: z.string(),
	country: CountrySchema,
});

const SellerSchema = z.object({
	name: z.string(),
	address: AddressSchema,
});

const LargePayloadSchema = z.object({
	name: z.string(),
	price: z.number(),
	seller: SellerSchema,
});

const VeryLargePayloadSchema = z.object({
	name: z.string(),
	tags: z.array(z.string()),
	images: z.array(ImageSchema),
});

const IntParamSchema = z.object({
	id: z.string().transform((val) => {
		const num = Number.parseInt(val, 10);
		if (Number.isNaN(num)) {
			throw new Error("Invalid integer");
		}
		return num;
	}),
});

const UuidParamSchema = z.object({
	uuid: z.string().uuid(),
});

const DateParamSchema = z.object({
	date: z.string().date(),
});

type SmallPayload = z.infer<typeof SmallPayloadSchema>;
type MediumPayload = z.infer<typeof MediumPayloadSchema>;
type LargePayload = z.infer<typeof LargePayloadSchema>;
type VeryLargePayload = z.infer<typeof VeryLargePayloadSchema>;

interface FileResponse {
	files_received: number;
	total_bytes: number;
}

interface IdResponse {
	id: string | number;
}

interface MultipleIdResponse {
	user_id: string;
	post_id: string;
}

interface DeepPathResponse {
	org: string;
	team: string;
	project: string;
	resource: string;
	id: string;
}

interface UuidResponse {
	uuid: string;
}

interface DateResponse {
	date: string;
}

interface HealthResponse {
	status: string;
}

function validateBody<T>(schema: ZodSchema<T>): RequestHandler {
	return (req: Request, res: Response, next: NextFunction): void => {
		try {
			req.body = schema.parse(req.body) as T;
			next();
		} catch (error) {
			if (error instanceof z.ZodError) {
				res.status(400).json({
					error: "Validation failed",
					details: error.issues,
				});
			} else {
				res.status(400).json({
					error: "Validation failed",
					details: String(error),
				});
			}
		}
	};
}

function validateParams<T>(schema: ZodSchema<T>): RequestHandler {
	return (req: Request, res: Response, next: NextFunction): void => {
		try {
			req.params = schema.parse(req.params) as T & Record<string, string>;
			next();
		} catch (error) {
			if (error instanceof z.ZodError) {
				res.status(400).json({
					error: "Validation failed",
					details: error.issues,
				});
			} else {
				res.status(400).json({
					error: "Validation failed",
					details: String(error),
				});
			}
		}
	};
}

app.post(
	"/json/small",
	validateBody(SmallPayloadSchema),
	(req: Request<unknown, SmallPayload, SmallPayload>, res: Response<SmallPayload>): void => {
		res.json(req.body);
	},
);

app.post(
	"/json/medium",
	validateBody(MediumPayloadSchema),
	(req: Request<unknown, MediumPayload, MediumPayload>, res: Response<MediumPayload>): void => {
		res.json(req.body);
	},
);

app.post(
	"/json/large",
	validateBody(LargePayloadSchema),
	(req: Request<unknown, LargePayload, LargePayload>, res: Response<LargePayload>): void => {
		res.json(req.body);
	},
);

app.post(
	"/json/very-large",
	validateBody(VeryLargePayloadSchema),
	(req: Request<unknown, VeryLargePayload, VeryLargePayload>, res: Response<VeryLargePayload>): void => {
		res.json(req.body);
	},
);

app.post("/multipart/small", (_req: Request, res: Response<FileResponse>): void => {
	res.json({ files_received: 1, total_bytes: 1024 });
});

app.post("/multipart/medium", (_req: Request, res: Response<FileResponse>): void => {
	res.json({ files_received: 2, total_bytes: 10240 });
});

app.post("/multipart/large", (_req: Request, res: Response<FileResponse>): void => {
	res.json({ files_received: 5, total_bytes: 102400 });
});

app.post("/urlencoded/simple", (req: Request, res: Response<Record<string, unknown>>): void => {
	res.json(req.body ?? {});
});

app.post("/urlencoded/complex", (req: Request, res: Response<Record<string, unknown>>): void => {
	res.json(req.body ?? {});
});

app.get("/path/simple/:id", (req: Request<{ id: string }>, res: Response<IdResponse>): void => {
	res.json({ id: req.params.id });
});

app.get(
	"/path/multiple/:user_id/:post_id",
	(req: Request<{ user_id: string; post_id: string }>, res: Response<MultipleIdResponse>): void => {
		res.json({
			user_id: req.params.user_id,
			post_id: req.params.post_id,
		});
	},
);

app.get(
	"/path/deep/:org/:team/:project/:resource/:id",
	(
		req: Request<{ org: string; team: string; project: string; resource: string; id: string }>,
		res: Response<DeepPathResponse>,
	): void => {
		res.json({
			org: req.params.org,
			team: req.params.team,
			project: req.params.project,
			resource: req.params.resource,
			id: req.params.id,
		});
	},
);

app.get(
	"/path/int/:id",
	validateParams(IntParamSchema),
	(req: Request<{ id: number }>, res: Response<IdResponse>): void => {
		res.json({ id: req.params.id });
	},
);

app.get(
	"/path/uuid/:uuid",
	validateParams(UuidParamSchema),
	(req: Request<{ uuid: string }>, res: Response<UuidResponse>): void => {
		res.json({ uuid: req.params.uuid });
	},
);

app.get(
	"/path/date/:date",
	validateParams(DateParamSchema),
	(req: Request<{ date: string }>, res: Response<DateResponse>): void => {
		res.json({ date: req.params.date });
	},
);

app.get("/query/few", (req: Request, res: Response<Record<string, unknown>>): void => {
	res.json(req.query ?? {});
});

app.get("/query/medium", (req: Request, res: Response<Record<string, unknown>>): void => {
	res.json(req.query ?? {});
});

app.get("/query/many", (req: Request, res: Response<Record<string, unknown>>): void => {
	res.json(req.query ?? {});
});

app.get("/health", (_req: Request, res: Response<HealthResponse>): void => {
	res.json({ status: "ok" });
});

app.get("/", (_req: Request, res: Response<HealthResponse>): void => {
	res.json({ status: "ok" });
});

function resolvePort(defaultPort = 8000): number {
	for (const arg of process.argv.slice(2)) {
		const parsed = Number.parseInt(arg, 10);
		if (Number.isFinite(parsed) && parsed >= 0 && parsed < 65536) {
			return parsed;
		}
	}

	const envPort = process.env.PORT ? Number.parseInt(process.env.PORT, 10) : Number.NaN;
	if (Number.isFinite(envPort) && envPort >= 0 && envPort < 65536) {
		return envPort;
	}

	return defaultPort;
}

const port = resolvePort();

app.listen(port, "0.0.0.0", (): void => {
	console.error(`[express] Starting server on port ${port}`);
});
