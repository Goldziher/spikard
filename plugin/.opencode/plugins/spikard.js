// AI-RULEZ :: GENERATED FILE — DO NOT EDIT
// Content-Hash: blake3:a7c106308b3345f76ab1d6073ef3256b596d1da1282310da3f25de7c8d7301fc
// Source-Hash: blake3:84f6dbbe893d49479fb99571e00f7caaad8c748add1fe1e87aa99532bca9d9eb
// Schema-Version: v1

/**
 * OpenCode v2 adapter for spikard.
 *
 * To add OpenCode-specific tools or hooks:
 *
 * 1. Create .ai-rulez/opencode/index.js.
 * 2. Default-export { id, setup(ctx) } from that source file. OpenCode v2 does not
 *    run v1 plugins (an exported function returning hooks).
 * 3. If the plugin bundles skills, commands, or agents, call
 *    registerBundledContent(ctx) from ../ai-rulez-content.js in setup.
 * 4. Run ai-rulez generate --plugin --dry-run, then ai-rulez generate --plugin.
 *
 * Keep shared skills, commands, agents, and MCP configuration in their normal
 * .ai-rulez sources. Validate all external input and never interpolate untrusted
 * values into shell commands.
 */
import { registerBundledContent } from "../ai-rulez-content.js"

/** @type {import("@opencode/plugin").Plugin.Plugin} */
export default {
  id: "spikard",
  async setup(ctx) {
    await registerBundledContent(ctx)
  },
}
