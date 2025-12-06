import { backgroundRun } from "../index";

/** Schedule work to run after the HTTP response resolves. */
export function run(work: () => void | Promise<void>): void {
	const task = async (): Promise<undefined> => {
		await work();
		return undefined;
	};
	backgroundRun(task);
}
