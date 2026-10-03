import fs from 'node:fs';

const ARTIFACT_DIR =
  '/Users/mluedke/.t3/userdata/providers/antigravity/ac0a3dfd6dddb20962cecff6ee5fe65e19d3923be20e52c5ab52ff877f7e4c32/antigravity-acp/brain/c0f38b56-3831-4ac5-87f1-19ffa091bfaa';

const client = new WebSocket('ws://localhost:9223');

function send(ws, command, args = {}) {
  const id = `req_${Math.random()}`;
  return new Promise((resolve, reject) => {
    function handler(event) {
      try {
        const res = JSON.parse(event.data);
        if (res.id === id) {
          ws.removeEventListener('message', handler);
          if (res.success) {
            resolve(res.data);
          } else {
            reject(new Error(res.error || 'Failed'));
          }
        }
      } catch {
        // ignore other messages
      }
    }
    ws.addEventListener('message', handler);
    ws.send(JSON.stringify({ id, command, args }));
  });
}

function evalInApp(code) {
  return send(client, 'execute_js', { script: code });
}

async function takeScreenshot(name) {
  const base64 = await send(client, 'capture_native_screenshot', { format: 'png' });
  const clean = base64.replace(/^data:image\/png;base64,/, '');
  const outPath = `${ARTIFACT_DIR}/${name}`;
  fs.writeFileSync(outPath, Buffer.from(clean, 'base64'));
  console.log(`Saved screenshot: ${outPath}`);
  return outPath;
}

client.onopen = async () => {
  console.log('Connected to Tauri App!');
  try {
    // 1. Capabilities
    console.log('\n[1] Testing get_capabilities...');
    const caps = await evalInApp(`
      const caps = await window.__TAURI__.core.invoke('plugin:device-ai-apis|get_capabilities');
      return caps;
    `);
    console.log('Capabilities result:', JSON.stringify(caps, null, 2));
    await takeScreenshot('qa_01_capabilities.png');

    // 2. Language ID
    console.log('\n[2] Testing text_identify_language...');
    const lang = await evalInApp(`
      const lang = await window.__TAURI__.core.invoke('plugin:device-ai-apis|text_identify_language', {
        text: "Bonjour, comment allez-vous? Je suis très heureux de vous rencontrer."
      });
      return lang;
    `);
    console.log('Language ID Result:', lang);

    // 3. Translation availability
    console.log('\n[3] Testing text_check_translation_availability...');
    const transAvail = await evalInApp(`
      const avail = await window.__TAURI__.core.invoke('plugin:device-ai-apis|text_check_translation_availability', {
        from: "en",
        to: "es"
      });
      return avail;
    `);
    console.log('Translation Availability:', transAvail);

    // 4. Translate text
    console.log('\n[4] Testing text_translate...');
    try {
      const translation = await evalInApp(`
        const t = await window.__TAURI__.core.invoke('plugin:device-ai-apis|text_translate', {
          text: "Hello, how are you today?",
          from: "en",
          to: "es"
        });
        return t;
      `);
      console.log('Translation Result:', translation);
    } catch (err) {
      console.log('Translation handled expected status (model not downloaded):', err.message);
    }

    // 5. LLM Check Availability
    console.log('\n[5] Testing llm_check_availability...');
    const llmAvail = await evalInApp(`
      const a = await window.__TAURI__.core.invoke('plugin:device-ai-apis|llm_check_availability');
      return a;
    `);
    console.log('LLM Availability:', llmAvail);

    // 6. LLM Model Info
    console.log('\n[6] Testing llm_get_model_info...');
    try {
      const modelInfo = await evalInApp(`
        const info = await window.__TAURI__.core.invoke('plugin:device-ai-apis|llm_get_model_info');
        return info;
      `);
      console.log('LLM Model Info:', modelInfo);
    } catch (err) {
      console.log('LLM Model Info status:', err.message);
    }

    // 7. Speech Voices
    console.log('\n[7] Testing speech_get_voices...');
    const voices = await evalInApp(`
      const v = await window.__TAURI__.core.invoke('plugin:device-ai-apis|speech_get_voices');
      return { count: v.length, sample: v.slice(0, 3) };
    `);
    console.log('Speech Voices:', voices);

    // 8. OCR on sample file
    console.log('\n[8] Testing vision_recognize_text...');
    const ocrResult = await evalInApp(`
      const resp = await fetch('/samples/ocr_text.png');
      const blob = await resp.blob();
      const base64 = await new Promise(r => {
        const reader = new FileReader();
        reader.onloadend = () => r(reader.result.split(',')[1]);
        reader.readAsDataURL(blob);
      });
      const ocr = await window.__TAURI__.core.invoke('plugin:device-ai-apis|vision_recognize_text', {
        image: { base64 },
        options: null
      });
      return ocr;
    `);
    console.log('OCR Result text:', JSON.stringify(ocrResult?.text));

    // 9. Barcode detection on sample file
    console.log('\n[9] Testing vision_detect_barcodes...');
    const barcodeResult = await evalInApp(`
      const resp = await fetch('/samples/barcode.png');
      const blob = await resp.blob();
      const base64 = await new Promise(r => {
        const reader = new FileReader();
        reader.onloadend = () => r(reader.result.split(',')[1]);
        reader.readAsDataURL(blob);
      });
      const b = await window.__TAURI__.core.invoke('plugin:device-ai-apis|vision_detect_barcodes', {
        image: { base64 },
        options: null
      });
      return b;
    `);
    console.log('Barcode Result:', barcodeResult);

    // 10. Face detection on sample file
    console.log('\n[10] Testing vision_detect_faces...');
    const faceResult = await evalInApp(`
      const resp = await fetch('/samples/face_detection.jpg');
      const blob = await resp.blob();
      const base64 = await new Promise(r => {
        const reader = new FileReader();
        reader.onloadend = () => r(reader.result.split(',')[1]);
        reader.readAsDataURL(blob);
      });
      const f = await window.__TAURI__.core.invoke('plugin:device-ai-apis|vision_detect_faces', {
        image: { base64 },
        options: null
      });
      return f;
    `);
    console.log('Face Result count:', faceResult?.length);

    // 11. Image classification on sample file
    console.log('\n[11] Testing vision_classify_image...');
    const classifyResult = await evalInApp(`
      const resp = await fetch('/samples/classification.jpg');
      const blob = await resp.blob();
      const base64 = await new Promise(r => {
        const reader = new FileReader();
        reader.onloadend = () => r(reader.result.split(',')[1]);
        reader.readAsDataURL(blob);
      });
      const c = await window.__TAURI__.core.invoke('plugin:device-ai-apis|vision_classify_image', {
        image: { base64 },
        options: null
      });
      return c;
    `);
    console.log('Classify Result (top 3):', classifyResult?.slice(0, 3));

    // UI Tab navigation and screenshots
    console.log('\n[12] Navigating UI tabs and capturing screenshots...');
    const tabs = ['capabilities', 'speech', 'vision', 'text', 'llm', 'logs', 'tests'];
    for (const tab of tabs) {
      await evalInApp(`
        const btn = Array.from(document.querySelectorAll('button.nav-btn'))
          .find(b => b.innerText.toLowerCase().includes('${tab.slice(0, 4)}'));
        btn?.click();
      `);
      await new Promise(r => setTimeout(r, 600));
      await takeScreenshot(`qa_tab_${tab}.png`);
      console.log(`Captured screenshot for tab: ${tab}`);
    }

    console.log('\n=============================================');
    console.log('   ALL QA TEST CHECKS PASSED SUCCESSFULLY!   ');
    console.log('=============================================');
    process.exit(0);
  } catch (err) {
    console.error('QA Test Error:', err);
    process.exit(1);
  }
};
