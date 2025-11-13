import { backgroundRun } from "../index.js";

/** Schedule work to run after the HTTP response resolves. */
export function run(work: () => void | Promise<void>): void {
	const task = async () => {
		await work();
	};
	backgroundRun(task);
}
