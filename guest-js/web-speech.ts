/**
 * Web Speech API implementation for browser fallback.
 * @module web-speech
 */

import type {
  RecognitionOptions,
  RecognitionResult,
  SynthesisOptions,
  Voice,
  VoiceGender,
  VoiceQuality,
} from "./types";

// Type definitions for Web Speech API
interface SpeechRecognitionErrorEvent extends Event {
  error: string;
  message: string;
}

interface SpeechRecognitionResult {
  isFinal: boolean;
  readonly length: number;
  item(index: number): SpeechRecognitionAlternative;
  [index: number]: SpeechRecognitionAlternative;
}

interface SpeechRecognitionAlternative {
  transcript: string;
  confidence: number;
}

interface SpeechRecognitionResultList {
  readonly length: number;
  item(index: number): SpeechRecognitionResult;
  [index: number]: SpeechRecognitionResult;
}

interface SpeechRecognitionEvent extends Event {
  results: SpeechRecognitionResultList;
  resultIndex: number;
}

interface SpeechRecognition extends EventTarget {
  continuous: boolean;
  interimResults: boolean;
  lang: string;
  maxAlternatives: number;
  onresult: ((event: SpeechRecognitionEvent) => void) | null;
  onerror: ((event: SpeechRecognitionErrorEvent) => void) | null;
  onend: (() => void) | null;
  start(): void;
  stop(): void;
  abort(): void;
}

interface SpeechRecognitionConstructor {
  new (): SpeechRecognition;
}

// Track active sessions
const activeSessions = new Map<string, SpeechRecognition>();
let sessionCounter = 0;

/**
 * Get the SpeechRecognition constructor (handles webkit prefix).
 */
function getSpeechRecognition(): SpeechRecognitionConstructor | null {
  if (typeof window === "undefined") return null;
  const w = window as unknown as {
    SpeechRecognition?: SpeechRecognitionConstructor;
    webkitSpeechRecognition?: SpeechRecognitionConstructor;
  };
  return w.SpeechRecognition ?? w.webkitSpeechRecognition ?? null;
}

/**
 * Perform one-shot speech recognition using Web Speech API.
 */
export async function webRecognize(options?: RecognitionOptions): Promise<RecognitionResult> {
  const SR = getSpeechRecognition();
  if (!SR) {
    throw new Error("Web Speech API not available");
  }

  return new Promise((resolve, reject) => {
    const recognition = new SR();
    recognition.continuous = false;
    recognition.interimResults = false;
    recognition.lang = options?.language ?? "en-US";
    recognition.maxAlternatives = 5;

    recognition.onresult = (event: SpeechRecognitionEvent) => {
      const result = event.results[event.results.length - 1];
      if (result && result.isFinal) {
        const alternatives = [];
        for (let i = 0; i < result.length; i++) {
          const alt = result[i];
          alternatives.push({
            text: alt.transcript,
            confidence: alt.confidence,
          });
        }

        resolve({
          text: result[0]?.transcript ?? "",
          confidence: result[0]?.confidence ?? 0,
          isFinal: true,
          alternatives,
        });
      }
    };

    recognition.onerror = (event: SpeechRecognitionErrorEvent) => {
      reject(new Error(`Speech recognition error: ${event.error}`));
    };

    recognition.onend = () => {
      // If we got here without a result, resolve with empty
    };

    try {
      recognition.start();
    } catch (e) {
      reject(e);
    }
  });
}

/**
 * Start streaming speech recognition using Web Speech API.
 */
export function webStartRecognition(options?: RecognitionOptions): string {
  const SR = getSpeechRecognition();
  if (!SR) {
    throw new Error("Web Speech API not available");
  }

  const sessionId = `web-speech-${++sessionCounter}`;
  const recognition = new SR();
  recognition.continuous = options?.continuous ?? true;
  recognition.interimResults = options?.interimResults ?? true;
  recognition.lang = options?.language ?? "en-US";
  recognition.maxAlternatives = 5;

  activeSessions.set(sessionId, recognition);

  try {
    recognition.start();
  } catch (e) {
    activeSessions.delete(sessionId);
    throw e;
  }

  return sessionId;
}

/**
 * Stop a streaming speech recognition session.
 */
export async function webStopRecognition(sessionId: string): Promise<RecognitionResult> {
  const recognition = activeSessions.get(sessionId);
  if (!recognition) {
    throw new Error(`Session not found: ${sessionId}`);
  }

  return new Promise((resolve, reject) => {
    let lastResult: RecognitionResult = {
      text: "",
      confidence: 0,
      isFinal: true,
      alternatives: [],
    };

    recognition.onresult = (event: SpeechRecognitionEvent) => {
      const result = event.results[event.results.length - 1];
      if (result) {
        const alternatives = [];
        for (let i = 0; i < result.length; i++) {
          const alt = result[i];
          alternatives.push({
            text: alt.transcript,
            confidence: alt.confidence,
          });
        }
        lastResult = {
          text: result[0]?.transcript ?? "",
          confidence: result[0]?.confidence ?? 0,
          isFinal: result.isFinal,
          alternatives,
        };
      }
    };

    recognition.onerror = (event: SpeechRecognitionErrorEvent) => {
      activeSessions.delete(sessionId);
      reject(new Error(`Speech recognition error: ${event.error}`));
    };

    recognition.onend = () => {
      activeSessions.delete(sessionId);
      resolve(lastResult);
    };

    try {
      recognition.stop();
    } catch (e) {
      activeSessions.delete(sessionId);
      reject(e);
    }
  });
}

/**
 * Synthesize and speak text using Web Speech API.
 */
export async function webSynthesize(text: string, options?: SynthesisOptions): Promise<void> {
  if (typeof window === "undefined" || !("speechSynthesis" in window)) {
    throw new Error("Web Speech Synthesis API not available");
  }

  return new Promise((resolve, reject) => {
    const utterance = new SpeechSynthesisUtterance(text);

    if (options?.voice) {
      const voices = window.speechSynthesis.getVoices();
      const voice = voices.find((v) => v.voiceURI === options.voice);
      if (voice) {
        utterance.voice = voice;
        utterance.lang = voice.lang;
      }
    }
    if (options?.rate !== undefined) {
      // Web Speech API rate: 0.1-10, our API: 0.5-2.0
      utterance.rate = options.rate;
    }
    if (options?.pitch !== undefined) {
      // Web Speech API pitch: 0-2, our API: 0.5-2.0
      utterance.pitch = options.pitch;
    }
    if (options?.volume !== undefined) {
      // Web Speech API volume: 0-1
      utterance.volume = options.volume;
    }

    utterance.onend = () => resolve();
    utterance.onerror = (event) => reject(new Error(`Speech synthesis error: ${event.error}`));

    window.speechSynthesis.speak(utterance);
  });
}

/**
 * Get available voices using Web Speech API.
 */
export async function webGetVoices(): Promise<Voice[]> {
  if (typeof window === "undefined" || !("speechSynthesis" in window)) {
    throw new Error("Web Speech Synthesis API not available");
  }

  // Voices might not be loaded immediately
  return new Promise((resolve) => {
    const getVoicesSync = () => {
      const voices = window.speechSynthesis.getVoices();
      return voices.map(
        (voice): Voice => ({
          id: voice.voiceURI,
          name: voice.name,
          language: voice.lang,
          isDefault: voice.default,
          quality: voice.localService ? ("default" as VoiceQuality) : ("enhanced" as VoiceQuality),
          gender: guessGender(voice.name),
        }),
      );
    };

    const voices = getVoicesSync();
    if (voices.length > 0) {
      resolve(voices);
      return;
    }

    // Wait for voices to load
    window.speechSynthesis.onvoiceschanged = () => {
      resolve(getVoicesSync());
    };

    // Fallback timeout
    setTimeout(() => resolve(getVoicesSync()), 100);
  });
}

/**
 * Try to guess gender from voice name.
 */
function guessGender(name: string): VoiceGender {
  const lowerName = name.toLowerCase();
  if (
    lowerName.includes("female") ||
    lowerName.includes("woman") ||
    lowerName.includes("samantha") ||
    lowerName.includes("karen") ||
    lowerName.includes("victoria") ||
    lowerName.includes("kate") ||
    lowerName.includes("moira") ||
    lowerName.includes("fiona")
  ) {
    return "female";
  }
  if (
    lowerName.includes("male") ||
    lowerName.includes("man") ||
    lowerName.includes("daniel") ||
    lowerName.includes("alex") ||
    lowerName.includes("fred") ||
    lowerName.includes("tom")
  ) {
    return "male";
  }
  return "neutral";
}
