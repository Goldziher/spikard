/** Schedule work to run after the HTTP response resolves. */
export function run(work: () => void | Promise<void>): void {
	void Promise.resolve().then(() => work());
}
