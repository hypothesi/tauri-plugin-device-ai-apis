package com.hypothesi.device_ai_apis

import android.Manifest
import android.app.Activity
import android.content.Intent
import android.content.pm.PackageManager
import android.graphics.BitmapFactory
import android.graphics.Rect
import android.os.Bundle
import android.speech.RecognitionListener
import android.speech.RecognizerIntent
import android.speech.SpeechRecognizer
import android.speech.tts.TextToSpeech
import android.speech.tts.Voice
import android.util.Base64
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import com.google.mlkit.vision.barcode.BarcodeScanning
import com.google.mlkit.vision.barcode.common.Barcode
import com.google.mlkit.vision.common.InputImage
import com.google.mlkit.vision.face.FaceDetection
import com.google.mlkit.vision.face.FaceDetectorOptions
import com.google.mlkit.vision.face.FaceLandmark
import com.google.mlkit.vision.label.ImageLabeling
import com.google.mlkit.vision.label.defaults.ImageLabelerOptions
import com.google.mlkit.vision.text.TextRecognition
import com.google.mlkit.vision.text.latin.TextRecognizerOptions
import com.google.mlkit.nl.languageid.LanguageIdentification
import org.json.JSONArray
import org.json.JSONObject
import java.io.File
import java.util.Locale

// Argument classes
@InvokeArg
class RecognitionOptionsArgs {
    var language: String? = null
    var continuous: Boolean = false
    var interimResults: Boolean = false
}

@InvokeArg
class SpeechRecognizeArgs {
    var options: RecognitionOptionsArgs? = null
}

@InvokeArg
class SynthesisOptionsArgs {
    var voice: String? = null
    var rate: Float? = null
    var pitch: Float? = null
    var volume: Float? = null
}

@InvokeArg
class SpeechSynthesizeArgs {
    var text: String = ""
    var options: SynthesisOptionsArgs? = null
}

@InvokeArg
class ImageSourceArgs {
    var base64: String? = null
    var filePath: String? = null
    var bytes: ByteArray? = null
}

@InvokeArg
class OcrOptionsArgs {
    var languages: Array<String>? = null
    var recognitionLevel: String? = null
}

@InvokeArg
class VisionRecognizeTextArgs {
    var image: ImageSourceArgs? = null
    var options: OcrOptionsArgs? = null
}

@InvokeArg
class BarcodeOptionsArgs {
    var formats: Array<String>? = null
}

@InvokeArg
class VisionDetectBarcodesArgs {
    var image: ImageSourceArgs? = null
    var options: BarcodeOptionsArgs? = null
}

@InvokeArg
class FaceOptionsArgs {
    var detectLandmarks: Boolean = false
    var classifyAttributes: Boolean = false
}

@InvokeArg
class VisionDetectFacesArgs {
    var image: ImageSourceArgs? = null
    var options: FaceOptionsArgs? = null
}

@InvokeArg
class ClassificationOptionsArgs {
    var maxResults: Int? = null
    var minConfidence: Float? = null
}

@InvokeArg
class VisionClassifyImageArgs {
    var image: ImageSourceArgs? = null
    var options: ClassificationOptionsArgs? = null
}

@InvokeArg
class TextIdentifyLanguageArgs {
    var text: String = ""
}

@InvokeArg
class TextTranslateArgs {
    var text: String = ""
    var from: String = ""
    var to: String = ""
}

@TauriPlugin
class DeviceAiPlugin(private val activity: Activity) : Plugin(activity) {
    private var speechRecognizer: SpeechRecognizer? = null
    private var textToSpeech: TextToSpeech? = null
    private var ttsInitialized = false

    companion object {
        private const val RECORD_AUDIO_REQUEST_CODE = 1001
    }

    override fun load(webView: android.webkit.WebView) {
        super.load(webView)

        // Initialize TTS
        textToSpeech = TextToSpeech(activity) { status ->
            ttsInitialized = status == TextToSpeech.SUCCESS
        }
    }

    // MARK: - Capabilities

    @Command
    fun getCapabilities(invoke: Invoke) {
        val speechRecognitionAvailable = SpeechRecognizer.isRecognitionAvailable(activity)

        val result = JSObject()

        // Speech Recognition
        val speechRecognition = JSObject()
        speechRecognition.put("available", speechRecognitionAvailable)
        speechRecognition.put("onDevice", false) // Android speech recognition typically uses cloud
        speechRecognition.put("requiresPermission", true)
        result.put("speechRecognition", speechRecognition)

        // Speech Synthesis
        val speechSynthesis = JSObject()
        speechSynthesis.put("available", true)
        speechSynthesis.put("onDevice", true)
        speechSynthesis.put("requiresPermission", false)
        result.put("speechSynthesis", speechSynthesis)

        // Text Recognition (ML Kit)
        val textRecognition = JSObject()
        textRecognition.put("available", true)
        textRecognition.put("onDevice", true)
        textRecognition.put("requiresPermission", false)
        result.put("textRecognition", textRecognition)

        // Barcode Detection (ML Kit)
        val barcodeDetection = JSObject()
        barcodeDetection.put("available", true)
        barcodeDetection.put("onDevice", true)
        barcodeDetection.put("requiresPermission", false)
        result.put("barcodeDetection", barcodeDetection)

        // Face Detection (ML Kit)
        val faceDetection = JSObject()
        faceDetection.put("available", true)
        faceDetection.put("onDevice", true)
        faceDetection.put("requiresPermission", false)
        result.put("faceDetection", faceDetection)

        // Image Classification (ML Kit)
        val imageClassification = JSObject()
        imageClassification.put("available", true)
        imageClassification.put("onDevice", true)
        imageClassification.put("requiresPermission", false)
        result.put("imageClassification", imageClassification)

        // Language Identification (ML Kit)
        val languageIdentification = JSObject()
        languageIdentification.put("available", true)
        languageIdentification.put("onDevice", true)
        languageIdentification.put("requiresPermission", false)
        result.put("languageIdentification", languageIdentification)

        // Translation
        val translation = JSObject()
        translation.put("available", false) // Not yet implemented
        translation.put("onDevice", true)
        translation.put("requiresPermission", false)
        result.put("translation", translation)

        invoke.resolve(result)
    }

    // MARK: - Speech Recognition

    @Command
    fun speechRecognize(invoke: Invoke) {
        val args = invoke.parseArgs(SpeechRecognizeArgs::class.java)

        if (!SpeechRecognizer.isRecognitionAvailable(activity)) {
            invoke.reject("Speech recognition not available on this device")
            return
        }

        // Check permission
        if (ContextCompat.checkSelfPermission(activity, Manifest.permission.RECORD_AUDIO)
            != PackageManager.PERMISSION_GRANTED) {
            ActivityCompat.requestPermissions(
                activity,
                arrayOf(Manifest.permission.RECORD_AUDIO),
                RECORD_AUDIO_REQUEST_CODE
            )
            invoke.reject("PERMISSION_REQUIRED", "Microphone permission required")
            return
        }

        activity.runOnUiThread {
            performSpeechRecognition(invoke, args.options)
        }
    }

    private fun performSpeechRecognition(invoke: Invoke, options: RecognitionOptionsArgs?) {
        speechRecognizer = SpeechRecognizer.createSpeechRecognizer(activity)

        val intent = Intent(RecognizerIntent.ACTION_RECOGNIZE_SPEECH).apply {
            putExtra(RecognizerIntent.EXTRA_LANGUAGE_MODEL, RecognizerIntent.LANGUAGE_MODEL_FREE_FORM)
            putExtra(RecognizerIntent.EXTRA_LANGUAGE, options?.language ?: Locale.getDefault().toString())
            putExtra(RecognizerIntent.EXTRA_PARTIAL_RESULTS, options?.interimResults ?: false)
            putExtra(RecognizerIntent.EXTRA_MAX_RESULTS, 5)
        }

        speechRecognizer?.setRecognitionListener(object : RecognitionListener {
            override fun onReadyForSpeech(params: Bundle?) {}
            override fun onBeginningOfSpeech() {}
            override fun onRmsChanged(rmsdB: Float) {}
            override fun onBufferReceived(buffer: ByteArray?) {}
            override fun onEndOfSpeech() {}

            override fun onError(error: Int) {
                val errorMessage = when (error) {
                    SpeechRecognizer.ERROR_AUDIO -> "Audio recording error"
                    SpeechRecognizer.ERROR_CLIENT -> "Client side error"
                    SpeechRecognizer.ERROR_INSUFFICIENT_PERMISSIONS -> "Insufficient permissions"
                    SpeechRecognizer.ERROR_NETWORK -> "Network error"
                    SpeechRecognizer.ERROR_NETWORK_TIMEOUT -> "Network timeout"
                    SpeechRecognizer.ERROR_NO_MATCH -> "No speech match"
                    SpeechRecognizer.ERROR_RECOGNIZER_BUSY -> "Recognition service busy"
                    SpeechRecognizer.ERROR_SERVER -> "Server error"
                    SpeechRecognizer.ERROR_SPEECH_TIMEOUT -> "No speech input"
                    else -> "Unknown error"
                }
                invoke.reject("SPEECH_RECOGNITION_FAILED", errorMessage)
                speechRecognizer?.destroy()
                speechRecognizer = null
            }

            override fun onResults(results: Bundle?) {
                val matches = results?.getStringArrayList(SpeechRecognizer.RESULTS_RECOGNITION)
                val confidenceScores = results?.getFloatArray(SpeechRecognizer.CONFIDENCE_SCORES)

                if (matches.isNullOrEmpty()) {
                    invoke.reject("NO_SPEECH_DETECTED", "No speech was detected")
                    return
                }

                val result = JSObject()
                result.put("text", matches[0])
                result.put("confidence", confidenceScores?.getOrNull(0) ?: 0.9f)
                result.put("isFinal", true)

                val alternatives = JSONArray()
                for (i in 1 until minOf(matches.size, 4)) {
                    val alt = JSONObject()
                    alt.put("text", matches[i])
                    alt.put("confidence", confidenceScores?.getOrNull(i) ?: 0.8f)
                    alternatives.put(alt)
                }
                result.put("alternatives", alternatives)

                invoke.resolve(result)
                speechRecognizer?.destroy()
                speechRecognizer = null
            }

            override fun onPartialResults(partialResults: Bundle?) {}
            override fun onEvent(eventType: Int, params: Bundle?) {}
        })

        speechRecognizer?.startListening(intent)
    }

    @Command
    fun speechRecognizeStart(invoke: Invoke) {
        invoke.reject("FEATURE_NOT_AVAILABLE", "Streaming speech recognition not yet implemented")
    }

    @Command
    fun speechRecognizeStop(invoke: Invoke) {
        invoke.reject("FEATURE_NOT_AVAILABLE", "Streaming speech recognition not yet implemented")
    }

    // MARK: - Speech Synthesis

    @Command
    fun speechSynthesize(invoke: Invoke) {
        val args = invoke.parseArgs(SpeechSynthesizeArgs::class.java)

        if (!ttsInitialized) {
            invoke.reject("SPEECH_SYNTHESIS_FAILED", "Text-to-speech not initialized")
            return
        }

        val tts = textToSpeech ?: run {
            invoke.reject("SPEECH_SYNTHESIS_FAILED", "Text-to-speech not available")
            return
        }

        // Set voice if specified
        args.options?.voice?.let { voiceId ->
            val voice = tts.voices?.find { it.name == voiceId }
            if (voice != null) {
                tts.voice = voice
            }
        }

        // Set speech rate
        args.options?.rate?.let { rate ->
            tts.setSpeechRate(rate)
        }

        // Set pitch
        args.options?.pitch?.let { pitch ->
            tts.setPitch(pitch)
        }

        val result = tts.speak(args.text, TextToSpeech.QUEUE_FLUSH, null, "utterance_${System.currentTimeMillis()}")

        if (result == TextToSpeech.SUCCESS) {
            invoke.resolve(JSObject())
        } else {
            invoke.reject("SPEECH_SYNTHESIS_FAILED", "Failed to speak text")
        }
    }

    @Command
    fun speechGetVoices(invoke: Invoke) {
        val tts = textToSpeech

        if (!ttsInitialized || tts == null) {
            invoke.reject("SPEECH_SYNTHESIS_FAILED", "Text-to-speech not initialized")
            return
        }

        val voices = tts.voices ?: emptySet()
        val voiceArray = JSONArray()

        for (voice in voices) {
            val voiceObj = JSONObject()
            voiceObj.put("id", voice.name)
            voiceObj.put("name", voice.name)
            voiceObj.put("language", voice.locale.toLanguageTag())
            voiceObj.put("isDefault", voice == tts.defaultVoice)
            voiceObj.put("quality", if (voice.quality >= Voice.QUALITY_HIGH) "enhanced" else "default")
            voiceArray.put(voiceObj)
        }

        invoke.resolve(voiceArray)
    }

    // MARK: - Vision

    @Command
    fun visionRecognizeText(invoke: Invoke) {
        val args = invoke.parseArgs(VisionRecognizeTextArgs::class.java)

        val inputImage = loadInputImage(args.image)
        if (inputImage == null) {
            invoke.reject("INVALID_IMAGE", "Failed to load image")
            return
        }
        val imageWidth = inputImage.width.toFloat().coerceAtLeast(1f)
        val imageHeight = inputImage.height.toFloat().coerceAtLeast(1f)

        val recognizer = TextRecognition.getClient(TextRecognizerOptions.DEFAULT_OPTIONS)

        recognizer.process(inputImage)
            .addOnSuccessListener { visionText ->
                val blocks = JSONArray()

                for (block in visionText.textBlocks) {
                    val blockObj = JSONObject()
                    blockObj.put("text", block.text)
                    blockObj.put(
                        "boundingBox",
                        rectToNormalizedJson(block.boundingBox, imageWidth, imageHeight),
                    )
                    blockObj.put("confidence", block.cornerPoints?.let { 0.9f } ?: 0.8f)

                    val lines = JSONArray()
                    for (line in block.lines) {
                        val lineObj = JSONObject()
                        lineObj.put("text", line.text)
                        lineObj.put(
                            "boundingBox",
                            rectToNormalizedJson(line.boundingBox, imageWidth, imageHeight),
                        )
                        lines.put(lineObj)
                    }
                    blockObj.put("lines", lines)
                    blocks.put(blockObj)
                }

                val result = JSObject()
                result.put("text", visionText.text)
                result.put("blocks", blocks)
                invoke.resolve(result)
            }
            .addOnFailureListener { e ->
                invoke.reject("TEXT_RECOGNITION_FAILED", e.message ?: "Unknown error")
            }
    }

    @Command
    fun visionDetectBarcodes(invoke: Invoke) {
        val args = invoke.parseArgs(VisionDetectBarcodesArgs::class.java)

        val inputImage = loadInputImage(args.image)
        if (inputImage == null) {
            invoke.reject("INVALID_IMAGE", "Failed to load image")
            return
        }
        val imageWidth = inputImage.width.toFloat().coerceAtLeast(1f)
        val imageHeight = inputImage.height.toFloat().coerceAtLeast(1f)

        val scanner = BarcodeScanning.getClient()

        scanner.process(inputImage)
            .addOnSuccessListener { barcodes ->
                val barcodeArray = JSONArray()

                for (barcode in barcodes) {
                    val barcodeObj = JSONObject()
                    barcodeObj.put("format", barcodeFormatToString(barcode.format))
                    barcodeObj.put("rawValue", barcode.rawValue ?: "")
                    barcodeObj.put(
                        "boundingBox",
                        rectToNormalizedJson(barcode.boundingBox, imageWidth, imageHeight),
                    )
                    barcodeArray.put(barcodeObj)
                }

                invoke.resolve(barcodeArray)
            }
            .addOnFailureListener { e ->
                invoke.reject("BARCODE_DETECTION_FAILED", e.message ?: "Unknown error")
            }
    }

    @Command
    fun visionDetectFaces(invoke: Invoke) {
        val args = invoke.parseArgs(VisionDetectFacesArgs::class.java)

        val inputImage = loadInputImage(args.image)
        if (inputImage == null) {
            invoke.reject("INVALID_IMAGE", "Failed to load image")
            return
        }
        val imageWidth = inputImage.width.toFloat().coerceAtLeast(1f)
        val imageHeight = inputImage.height.toFloat().coerceAtLeast(1f)

        val detectLandmarks = args.options?.detectLandmarks ?: false
        val classifyAttributes = args.options?.classifyAttributes ?: false

        val optionsBuilder = FaceDetectorOptions.Builder()
            .setPerformanceMode(FaceDetectorOptions.PERFORMANCE_MODE_ACCURATE)

        if (detectLandmarks) {
            optionsBuilder.setLandmarkMode(FaceDetectorOptions.LANDMARK_MODE_ALL)
        }

        if (classifyAttributes) {
            optionsBuilder.setClassificationMode(FaceDetectorOptions.CLASSIFICATION_MODE_ALL)
        }

        val detector = FaceDetection.getClient(optionsBuilder.build())

        detector.process(inputImage)
            .addOnSuccessListener { faces ->
                val faceArray = JSONArray()

                for (face in faces) {
                    val faceObj = JSONObject()
                    faceObj.put(
                        "boundingBox",
                        rectToNormalizedJson(face.boundingBox, imageWidth, imageHeight),
                    )

                    if (detectLandmarks) {
                        val landmarks = JSONObject()
                        face.getLandmark(FaceLandmark.LEFT_EYE)?.let {
                            landmarks.put("leftEye", pointToJson(it.position.x, it.position.y))
                        }
                        face.getLandmark(FaceLandmark.RIGHT_EYE)?.let {
                            landmarks.put("rightEye", pointToJson(it.position.x, it.position.y))
                        }
                        face.getLandmark(FaceLandmark.NOSE_BASE)?.let {
                            landmarks.put("nose", pointToJson(it.position.x, it.position.y))
                        }
                        face.getLandmark(FaceLandmark.MOUTH_LEFT)?.let {
                            landmarks.put("mouthLeft", pointToJson(it.position.x, it.position.y))
                        }
                        face.getLandmark(FaceLandmark.MOUTH_RIGHT)?.let {
                            landmarks.put("mouthRight", pointToJson(it.position.x, it.position.y))
                        }
                        faceObj.put("landmarks", landmarks)
                    }

                    if (classifyAttributes) {
                        val attributes = JSONObject()
                        if (face.smilingProbability != null) {
                            attributes.put("smilingProbability", face.smilingProbability)
                        }
                        if (face.leftEyeOpenProbability != null) {
                            attributes.put("leftEyeOpenProbability", face.leftEyeOpenProbability)
                        }
                        if (face.rightEyeOpenProbability != null) {
                            attributes.put("rightEyeOpenProbability", face.rightEyeOpenProbability)
                        }
                        faceObj.put("attributes", attributes)
                    }

                    faceObj.put("rollAngle", face.headEulerAngleZ)
                    faceObj.put("yawAngle", face.headEulerAngleY)

                    faceArray.put(faceObj)
                }

                invoke.resolve(faceArray)
            }
            .addOnFailureListener { e ->
                invoke.reject("FACE_DETECTION_FAILED", e.message ?: "Unknown error")
            }
    }

    @Command
    fun visionClassifyImage(invoke: Invoke) {
        val args = invoke.parseArgs(VisionClassifyImageArgs::class.java)

        val inputImage = loadInputImage(args.image)
        if (inputImage == null) {
            invoke.reject("INVALID_IMAGE", "Failed to load image")
            return
        }

        val maxResults = args.options?.maxResults ?: 10
        val minConfidence = args.options?.minConfidence ?: 0.5f

        val options = ImageLabelerOptions.Builder()
            .setConfidenceThreshold(minConfidence)
            .build()

        val labeler = ImageLabeling.getClient(options)

        labeler.process(inputImage)
            .addOnSuccessListener { labels ->
                val classificationArray = JSONArray()

                for (label in labels.take(maxResults)) {
                    val classificationObj = JSONObject()
                    classificationObj.put("identifier", label.text)
                    classificationObj.put("confidence", label.confidence)
                    classificationArray.put(classificationObj)
                }

                invoke.resolve(classificationArray)
            }
            .addOnFailureListener { e ->
                invoke.reject("IMAGE_CLASSIFICATION_FAILED", e.message ?: "Unknown error")
            }
    }

    // MARK: - Vision Helpers

    private fun loadInputImage(source: ImageSourceArgs?): InputImage? {
        if (source == null) return null

        source.base64?.let { base64 ->
            try {
                val bytes = Base64.decode(base64, Base64.DEFAULT)
                val bitmap = BitmapFactory.decodeByteArray(bytes, 0, bytes.size)
                return InputImage.fromBitmap(bitmap, 0)
            } catch (e: Exception) {
                return null
            }
        }

        source.filePath?.let { filePath ->
            try {
                val bitmap = BitmapFactory.decodeFile(filePath)
                return InputImage.fromBitmap(bitmap, 0)
            } catch (e: Exception) {
                return null
            }
        }

        source.bytes?.let { bytes ->
            try {
                val bitmap = BitmapFactory.decodeByteArray(bytes, 0, bytes.size)
                return InputImage.fromBitmap(bitmap, 0)
            } catch (e: Exception) {
                return null
            }
        }

        return null
    }

    private fun rectToNormalizedJson(rect: Rect?, imageWidth: Float, imageHeight: Float): JSONObject {
        val obj = JSONObject()
        if (rect != null) {
            obj.put("x", rect.left.toFloat() / imageWidth)
            obj.put("y", rect.top.toFloat() / imageHeight)
            obj.put("width", rect.width().toFloat() / imageWidth)
            obj.put("height", rect.height().toFloat() / imageHeight)
        }
        return obj
    }

    private fun pointToJson(x: Float, y: Float): JSONObject {
        val obj = JSONObject()
        obj.put("x", x)
        obj.put("y", y)
        return obj
    }

    private fun barcodeFormatToString(format: Int): String {
        return when (format) {
            Barcode.FORMAT_QR_CODE -> "qr_code"
            Barcode.FORMAT_AZTEC -> "aztec"
            Barcode.FORMAT_CODE_128 -> "code_128"
            Barcode.FORMAT_CODE_39 -> "code_39"
            Barcode.FORMAT_CODE_93 -> "code_93"
            Barcode.FORMAT_CODABAR -> "codabar"
            Barcode.FORMAT_DATA_MATRIX -> "data_matrix"
            Barcode.FORMAT_EAN_13 -> "ean_13"
            Barcode.FORMAT_EAN_8 -> "ean_8"
            Barcode.FORMAT_ITF -> "itf"
            Barcode.FORMAT_PDF417 -> "pdf417"
            Barcode.FORMAT_UPC_A -> "upc_a"
            Barcode.FORMAT_UPC_E -> "upc_e"
            else -> "qr_code"
        }
    }

    // MARK: - Text Processing

    @Command
    fun textIdentifyLanguage(invoke: Invoke) {
        val args = invoke.parseArgs(TextIdentifyLanguageArgs::class.java)

        val languageIdentifier = LanguageIdentification.getClient()

        languageIdentifier.identifyPossibleLanguages(args.text)
            .addOnSuccessListener { identifiedLanguages ->
                if (identifiedLanguages.isEmpty()) {
                    invoke.reject("LANGUAGE_IDENTIFICATION_FAILED", "Could not identify language")
                    return@addOnSuccessListener
                }

                val topLanguage = identifiedLanguages.first()
                val alternatives = JSONArray()

                for (lang in identifiedLanguages.drop(1).take(4)) {
                    val altObj = JSONObject()
                    altObj.put("language", lang.languageTag)
                    altObj.put("confidence", lang.confidence)
                    alternatives.put(altObj)
                }

                val result = JSObject()
                result.put("language", topLanguage.languageTag)
                result.put("confidence", topLanguage.confidence)
                result.put("alternatives", alternatives)
                invoke.resolve(result)
            }
            .addOnFailureListener { e ->
                invoke.reject("LANGUAGE_IDENTIFICATION_FAILED", e.message ?: "Unknown error")
            }
    }

    @Command
    fun textTranslate(invoke: Invoke) {
        invoke.reject("FEATURE_NOT_AVAILABLE", "Translation not yet implemented - requires ML Kit Translation model download")
    }

    override fun onDestroy() {
        super.onDestroy()
        speechRecognizer?.destroy()
        textToSpeech?.shutdown()
    }
}
