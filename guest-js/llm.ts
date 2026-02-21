/**
 * On-device language model API.
 *
 * Provides text generation, streaming, multi-turn sessions, summarization,
 * and text rewriting using platform-native on-device language models
 * (Apple FoundationModels on macOS/iOS, Phi Silica on Windows).
 *
 * @module llm
 */

import { invoke, Channel } from "@tauri-apps/api/core";
import type {
  LlmAvailability,
  LlmModelInfo,
  LlmGenerateOptions,
  LlmGenerateResult,
  LlmStreamEvent,
  LlmSessionOptions,
  LlmSummarizeOptions,
  LlmSummarizeResult,
  LlmRewriteOptions,
  LlmRewriteResult,
} from "./types";
import { isTauri } from "./platform";

/**
 * Check if an on-device language model is available.
 *
 * @returns Availability status with reason if unavailable.
 *
 * @example
 * ```typescript
 * const status = await checkAvailability();
 * if (status.available) {
 *   console.log('LLM is available');
 * } else {
 *   console.log('LLM unavailable:', status.reason);
 * }
 * ```
 */
export async function checkAvailability(): Promise<LlmAvailability> {
  if (!isTauri()) {
    return { available: false, reason: "Not running in Tauri" };
  }
  return invoke<LlmAvailability>("plugin:device-ai-apis|llm_check_availability");
}

/**
 * Get information about the on-device language model.
 *
 * @returns Model metadata including capabilities and context window size.
 * @throws If no language model is available.
 */
export async function getModelInfo(): Promise<LlmModelInfo> {
  if (!isTauri()) {
    throw new Error("LLM model info requires Tauri runtime");
  }
  return invoke<LlmModelInfo>("plugin:device-ai-apis|llm_get_model_info");
}

/**
 * Generate text using the on-device language model.
 *
 * @param options Generation options including prompt and parameters.
 * @returns The generated text with metadata.
 * @throws If the model is unavailable or generation fails.
 *
 * @example
 * ```typescript
 * const result = await generate({
 *   prompt: 'Explain quantum computing in one paragraph.',
 *   temperature: 0.7,
 *   maxTokens: 256,
 * });
 * console.log(result.content);
 * ```
 */
export async function generate(options: LlmGenerateOptions): Promise<LlmGenerateResult> {
  if (!isTauri()) {
    throw new Error("LLM generation requires Tauri runtime");
  }
  return invoke<LlmGenerateResult>("plugin:device-ai-apis|llm_generate", { options });
}

/**
 * Stream text generation from the on-device language model.
 *
 * Tokens are delivered incrementally via the callback as they are generated.
 *
 * @param options Generation options including prompt and parameters.
 * @param onEvent Callback invoked for each streaming event (delta, done, error).
 * @throws If the model is unavailable or streaming fails.
 *
 * @example
 * ```typescript
 * let fullText = '';
 * await generateStream(
 *   { prompt: 'Write a haiku about coding.' },
 *   (event) => {
 *     if (event.type === 'delta') {
 *       fullText += event.content;
 *     } else if (event.type === 'done') {
 *       console.log('Complete:', event.content);
 *     }
 *   },
 * );
 * ```
 */
export async function generateStream(
  options: LlmGenerateOptions,
  onEvent: (event: LlmStreamEvent) => void,
): Promise<void> {
  if (!isTauri()) {
    throw new Error("LLM streaming requires Tauri runtime");
  }
  const channel = new Channel<LlmStreamEvent>();
  channel.onmessage = onEvent;
  return invoke<void>("plugin:device-ai-apis|llm_generate_stream", {
    options,
    onEvent: channel,
  });
}

/**
 * Create a multi-turn conversation session.
 *
 * Sessions maintain context across multiple messages, enabling
 * back-and-forth conversation with the language model.
 *
 * @param options Session options including optional system prompt.
 * @returns A session ID for use with {@link sessionSend} and {@link destroySession}.
 * @throws If the model is unavailable.
 *
 * @example
 * ```typescript
 * const sessionId = await createSession({
 *   systemPrompt: 'You are a helpful coding assistant.',
 *   temperature: 0.5,
 * });
 * ```
 */
export async function createSession(options?: LlmSessionOptions): Promise<string> {
  if (!isTauri()) {
    throw new Error("LLM sessions require Tauri runtime");
  }
  return invoke<string>("plugin:device-ai-apis|llm_create_session", {
    options: options ?? {},
  });
}

/**
 * Send a message in an existing session and get a response.
 *
 * @param sessionId The session ID from {@link createSession}.
 * @param message The user message to send.
 * @returns The model's response.
 * @throws If the session is not found or generation fails.
 */
export async function sessionSend(
  sessionId: string,
  message: string,
): Promise<LlmGenerateResult> {
  if (!isTauri()) {
    throw new Error("LLM sessions require Tauri runtime");
  }
  return invoke<LlmGenerateResult>("plugin:device-ai-apis|llm_session_send", {
    sessionId,
    message,
  });
}

/**
 * Send a message in an existing session and stream the response.
 *
 * @param sessionId The session ID from {@link createSession}.
 * @param message The user message to send.
 * @param onEvent Callback invoked for each streaming event.
 * @throws If the session is not found or streaming fails.
 */
export async function sessionSendStream(
  sessionId: string,
  message: string,
  onEvent: (event: LlmStreamEvent) => void,
): Promise<void> {
  if (!isTauri()) {
    throw new Error("LLM sessions require Tauri runtime");
  }
  const channel = new Channel<LlmStreamEvent>();
  channel.onmessage = onEvent;
  return invoke<void>("plugin:device-ai-apis|llm_session_send_stream", {
    sessionId,
    message,
    onEvent: channel,
  });
}

/**
 * Destroy a session and free its resources.
 *
 * @param sessionId The session ID to destroy.
 * @throws If the session is not found.
 */
export async function destroySession(sessionId: string): Promise<void> {
  if (!isTauri()) {
    throw new Error("LLM sessions require Tauri runtime");
  }
  return invoke<void>("plugin:device-ai-apis|llm_destroy_session", { sessionId });
}

/**
 * Summarize text using the on-device language model.
 *
 * Uses platform-native text intelligence APIs when available
 * (Windows TextSummarizer), or prompted generation on other platforms.
 *
 * @param options Summarization options including the text to summarize.
 * @returns The summary with model metadata.
 * @throws If the model is unavailable or summarization fails.
 *
 * @example
 * ```typescript
 * const result = await summarize({
 *   text: 'A very long article...',
 * });
 * console.log(result.summary);
 * ```
 */
export async function summarize(
  options: LlmSummarizeOptions,
): Promise<LlmSummarizeResult> {
  if (!isTauri()) {
    throw new Error("LLM summarization requires Tauri runtime");
  }
  return invoke<LlmSummarizeResult>("plugin:device-ai-apis|llm_summarize", { options });
}

/**
 * Rewrite text with a specified tone using the on-device language model.
 *
 * Uses platform-native text intelligence APIs when available
 * (Windows TextRewriter), or prompted generation on other platforms.
 *
 * @param options Rewrite options including text and desired tone.
 * @returns The rewritten text with model metadata.
 * @throws If the model is unavailable or rewriting fails.
 *
 * @example
 * ```typescript
 * const result = await rewrite({
 *   text: 'hey wanna grab lunch tmrw?',
 *   tone: 'formal',
 * });
 * console.log(result.rewrittenText);
 * ```
 */
export async function rewrite(options: LlmRewriteOptions): Promise<LlmRewriteResult> {
  if (!isTauri()) {
    throw new Error("LLM rewriting requires Tauri runtime");
  }
  return invoke<LlmRewriteResult>("plugin:device-ai-apis|llm_rewrite", { options });
}
