//! Live speech-recognition control state machine.
//!
//! Shared by the macOS (Rust/objc) live-microphone path and mirrored by the iOS
//! Swift implementation (`ios/Sources/DeviceAiPlugin.swift`). Pure logic, no
//! platform dependencies, so the decision matrix is unit-testable on any host
//! (Linux CI included).
//!
//! The platform shell owns the recognizer/audio engine and drives this machine
//! on a short tick (condvar wait / repeating timer). Each tick the shell passes
//! the current session state and receives the next action to perform.
//!
//! NOTE: constants here and in the Swift mirror must stay in sync — update both
//! together (see `SpeechLiveConstants` in DeviceAiPlugin.swift).

use std::time::Duration;

/// End-of-utterance silence: no new recognition result for this long while
/// some text has been seen → send `endAudio` so the recognizer emits a final
/// result.
pub const SILENCE_LIMIT: Duration = Duration::from_millis(1200);

/// Hard budget without any recognized text → give up with a "no speech"
/// error rather than waiting the full session.
pub const NO_SPEECH_LIMIT: Duration = Duration::from_secs(8);

/// Hard cap on total session length for continuous speech → finalize with
/// `endAudio` so the utterance so far is returned instead of lost.
pub const HARD_CAP: Duration = Duration::from_secs(30);

/// Grace after `endAudio` was sent: if no final result arrives within this
/// window, fall back to the best accumulated text (if any) or time out.
pub const FINAL_GRACE: Duration = Duration::from_secs(10);

/// Action the platform shell should take after evaluating the current state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveAction {
    /// Nothing to do — keep the session running and tick again later.
    Continue,
    /// Send `endAudio` (once — the shell tracks `end_audio_sent`).
    EndAudio,
    /// Abort the session: no speech was detected within [`NO_SPEECH_LIMIT`].
    CancelNoSpeech,
    /// Abort the session: no final result within [`FINAL_GRACE`] after
    /// `endAudio`, and no usable text was accumulated.
    CancelTimeout,
}

/// Immutable snapshot of the live session evaluated on each tick.
///
/// All timestamps are relative to the session start (`Duration` since start),
/// which keeps the struct trivially constructible in tests.
#[derive(Debug, Clone, Copy)]
pub struct LiveState {
    /// Whether any non-empty recognition result has been seen so far.
    pub have_text: bool,
    /// Time since session start when the last recognition result arrived.
    /// `None` when no result has arrived yet.
    pub last_change: Option<Duration>,
    /// Time since session start of this tick (`now - started`).
    pub elapsed: Duration,
    /// Whether `endAudio` has already been sent for this session.
    pub end_audio_sent: bool,
    /// Time since session start when `endAudio` was sent. `None` if not sent.
    pub end_audio_at: Option<Duration>,
}

impl LiveState {
    fn silence_since(&self) -> Option<Duration> {
        self.last_change.map(|c| self.elapsed.saturating_sub(c))
    }
}

/// Decides the next action for a live recognition session.
///
/// Decision order (first match wins):
/// 1. `endAudio` sent and the final-result grace expired → keep the session
///    result (see [`LiveOutcome`]) — the shell must not keep waiting; if text
///    exists it resolves with it, otherwise it cancels with a timeout.
/// 2. Silence after speech (≥ [`SILENCE_LIMIT`]) and `endAudio` not yet sent →
///    [`LiveAction::EndAudio`].
/// 3. No speech at all within [`NO_SPEECH_LIMIT`] → [`LiveAction::CancelNoSpeech`].
/// 4. Session reached [`HARD_CAP`] and `endAudio` not yet sent →
///    [`LiveAction::EndAudio`] (finalize long continuous speech).
/// 5. Otherwise → [`LiveAction::Continue`].
pub fn next_action(state: &LiveState) -> LiveAction {
    // 1. Final-result grace after endAudio.
    if state.end_audio_sent {
        let sent_at = state.end_audio_at.unwrap_or(Duration::ZERO);
        if state.elapsed.saturating_sub(sent_at) >= FINAL_GRACE {
            return LiveAction::CancelTimeout;
        }
        return LiveAction::Continue;
    }

    // 2. End of utterance: silence after some text was seen.
    if state.have_text {
        if let Some(silence) = state.silence_since() {
            if silence >= SILENCE_LIMIT {
                return LiveAction::EndAudio;
            }
        }
    }

    // 3. No speech detected at all within the no-speech budget.
    if !state.have_text && state.elapsed >= NO_SPEECH_LIMIT {
        return LiveAction::CancelNoSpeech;
    }

    // 4. Hard cap: continuous speech past the session budget → finalize.
    if state.elapsed >= HARD_CAP {
        return LiveAction::EndAudio;
    }

    LiveAction::Continue
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state(have_text: bool, last_change: Option<Duration>, elapsed: Duration) -> LiveState {
        LiveState {
            have_text,
            last_change,
            elapsed,
            end_audio_sent: false,
            end_audio_at: None,
        }
    }

    fn sent(at: Duration, elapsed: Duration, have_text: bool) -> LiveState {
        LiveState {
            have_text,
            last_change: Some(at),
            elapsed,
            end_audio_sent: true,
            end_audio_at: Some(at),
        }
    }

    #[test]
    fn fresh_session_continues() {
        assert_eq!(
            next_action(&state(false, None, Duration::ZERO)),
            LiveAction::Continue
        );
    }

    #[test]
    fn active_speech_continues() {
        // Text is arriving continuously — silence below the limit.
        let s = state(
            true,
            Some(Duration::from_millis(500)),
            Duration::from_millis(900),
        );
        assert_eq!(next_action(&s), LiveAction::Continue);
    }

    #[test]
    fn silence_after_speech_ends_audio() {
        // Last result 1.3s ago with text present → EndAudio.
        let s = state(
            true,
            Some(Duration::from_millis(1000)),
            Duration::from_millis(2300),
        );
        assert_eq!(next_action(&s), LiveAction::EndAudio);
    }

    #[test]
    fn silence_below_limit_continues() {
        // 1.1s of silence — just under the 1.2s limit.
        let s = state(
            true,
            Some(Duration::from_millis(100)),
            Duration::from_millis(1200),
        );
        assert_eq!(next_action(&s), LiveAction::Continue);
    }

    #[test]
    fn no_speech_within_budget_cancels() {
        let s = state(false, None, Duration::from_secs(8));
        assert_eq!(next_action(&s), LiveAction::CancelNoSpeech);
    }

    #[test]
    fn no_speech_before_budget_continues() {
        let s = state(false, None, Duration::from_secs(7));
        assert_eq!(next_action(&s), LiveAction::Continue);
    }

    #[test]
    fn hard_cap_finalizes_speech() {
        // Continuous speech for 30s (results keep arriving, silence never
        // reaches the limit) → EndAudio at the hard cap.
        let s = state(
            true,
            Some(Duration::from_millis(29_800)),
            Duration::from_secs(30),
        );
        assert_eq!(next_action(&s), LiveAction::EndAudio);
    }

    #[test]
    fn hard_cap_without_text_is_no_speech() {
        // No text flag with 30s elapsed: rule 3 (no-speech budget of 8s)
        // fires long before the hard cap, so the session is cancelled as
        // "no speech" — the hard cap is only a fallback for empty
        // transcriptions that still count as `last_change`.
        let s = state(
            false,
            Some(Duration::from_millis(29_900)),
            Duration::from_secs(30),
        );
        assert_eq!(next_action(&s), LiveAction::CancelNoSpeech);
    }

    #[test]
    fn grace_after_end_audio_continues_then_cancels() {
        // endAudio sent at 5s, tick at 6s → still within grace.
        let within = sent(Duration::from_secs(5), Duration::from_secs(6), true);
        assert_eq!(next_action(&within), LiveAction::Continue);

        // Tick at 15.1s → grace (10s) expired → timeout.
        let expired = sent(Duration::from_secs(5), Duration::from_millis(15_100), true);
        assert_eq!(next_action(&expired), LiveAction::CancelTimeout);
    }

    #[test]
    fn end_audio_never_sent_twice() {
        // Once endAudio is sent, silence/no-speech/hard-cap rules are inert:
        // only the grace rule applies.
        let s = sent(Duration::from_secs(2), Duration::from_secs(3), true);
        assert_eq!(next_action(&s), LiveAction::Continue);
    }

    #[test]
    fn exact_boundaries() {
        // Exactly at the silence limit → EndAudio (>= comparison).
        let s = state(true, Some(Duration::ZERO), SILENCE_LIMIT);
        assert_eq!(next_action(&s), LiveAction::EndAudio);

        // Exactly at grace expiry → CancelTimeout.
        let s = sent(Duration::from_secs(2), Duration::from_secs(12), true);
        assert_eq!(next_action(&s), LiveAction::CancelTimeout);
    }
}
