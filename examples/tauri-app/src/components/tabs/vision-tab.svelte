<script>
   import Icon from '@iconify/svelte';

   let {
      detectLandmarks = $bindable(),
      classifyAttributes = $bindable(),
      maxClassifications = $bindable(),
      ocrResult,
      ocrImage,
      isProcessingOcr,
      barcodeResults,
      barcodeImage,
      isProcessingBarcode,
      faceResults,
      faceImage,
      isProcessingFaces,
      classificationResults,
      classificationImage,
      isProcessingClassification,
      handleOcrImage,
      handleBarcodeImage,
      handleFaceImage,
      handleClassificationImage,
   } = $props();

   function formatPercent(value) {
      return `${(value * 100).toFixed(0)}%`;
   }

   function formatPoint(point) {
      return `(${formatPercent(point.x)}, ${formatPercent(point.y)})`;
   }

   function formatBoundingBox(box) {
      return (
         `Position ${formatPercent(box.x)}, ${formatPercent(box.y)} ` +
         `- size ${formatPercent(box.width)} x ${formatPercent(box.height)}`
      );
   }
</script>

<div class='tool-grid'>
   <section class='panel'>
      <div class='panel-header'>
         <Icon icon='lucide:file-search' />
         <div>
            <h2>Text recognition</h2>
            <p>Extract text blocks from a still image and inspect the recognized output.</p>
         </div>
      </div>

      <div class='upload-zone'>
         <input id='ocr-upload' type='file' accept='image/*' onchange={handleOcrImage} />
         <label for='ocr-upload'>
            <Icon icon='lucide:upload-cloud' />
            <span>Upload image for OCR</span>
         </label>
      </div>

      {#if ocrImage}
         <div class='preview-card'>
            <img src={ocrImage} alt='OCR input preview' />
         </div>
      {/if}

      {#if isProcessingOcr}
         <p class='status-note'>Processing image...</p>
      {:else if ocrResult}
         <div class='result-card'>
            <div class='result-header'>
               <h3>Extracted text</h3>
               <span>{ocrResult.blocks?.length || 0} block(s)</span>
            </div>
            <pre>{ocrResult.text || '(No text found)'}</pre>
         </div>
      {/if}
   </section>

   <section class='panel'>
      <div class='panel-header'>
         <Icon icon='lucide:scan-barcode' />
         <div>
            <h2>Barcode detection</h2>
            <p>Decode QR codes and other supported barcode formats from a still image.</p>
         </div>
      </div>

      <div class='upload-zone'>
         <input id='barcode-upload' type='file' accept='image/*' onchange={handleBarcodeImage} />
         <label for='barcode-upload'>
            <Icon icon='lucide:barcode' />
            <span>Upload image for scanning</span>
         </label>
      </div>

      {#if barcodeImage}
         <div class='preview-card'>
            <img src={barcodeImage} alt='Barcode input preview' />
         </div>
      {/if}

      {#if isProcessingBarcode}
         <p class='status-note'>Scanning for barcodes...</p>
      {:else if barcodeResults.length > 0}
         <div class='result-card'>
            <div class='result-header'>
               <h3>Detected barcodes</h3>
               <span>{barcodeResults.length} result(s)</span>
            </div>

            <div class='stack-list'>
               {#each barcodeResults as barcode}
                  <article class='stack-item'>
                     <div class='stack-item-header'>
                        <strong>{barcode.format}</strong>
                        <span>{formatBoundingBox(barcode.boundingBox)}</span>
                     </div>
                     <code>{barcode.rawValue}</code>
                  </article>
               {/each}
            </div>
         </div>
      {:else if barcodeImage}
         <p class='empty-note'>No barcodes were detected in the uploaded image.</p>
      {/if}
   </section>

   <section class='panel'>
      <div class='panel-header'>
         <Icon icon='lucide:user-square-2' />
         <div>
            <h2>Face detection</h2>
            <p>Inspect pose, bounds, landmarks, and classified attributes for each face.</p>
         </div>
      </div>

      <div class='toggle-row'>
         <label class='toggle-pill'>
            <input type='checkbox' bind:checked={detectLandmarks} />
            <span>Detect landmarks</span>
         </label>

         <label class='toggle-pill'>
            <input type='checkbox' bind:checked={classifyAttributes} />
            <span>Classify attributes</span>
         </label>
      </div>

      <div class='upload-zone'>
         <input id='face-upload' type='file' accept='image/*' onchange={handleFaceImage} />
         <label for='face-upload'>
            <Icon icon='lucide:camera' />
            <span>Upload image for face detection</span>
         </label>
      </div>

      {#if faceImage}
         <div class='preview-card'>
            <img src={faceImage} alt='Face detection input preview' />
         </div>
      {/if}

      {#if isProcessingFaces}
         <p class='status-note'>Detecting faces...</p>
      {:else if faceResults.length > 0}
         <div class='stack-list'>
            {#each faceResults as face, index}
               <article class='face-card'>
                  <div class='stack-item-header'>
                     <strong>Face {index + 1}</strong>
                     <span>{formatBoundingBox(face.boundingBox)}</span>
                  </div>

                  <div class='metric-row'>
                     {#if face.rollAngle != null}
                        <span>Roll: {face.rollAngle.toFixed(1)}deg</span>
                     {/if}

                     {#if face.yawAngle != null}
                        <span>Yaw: {face.yawAngle.toFixed(1)}deg</span>
                     {/if}
                  </div>

                  {#if face.landmarks}
                     <div class='detail-block'>
                        <h4>Landmarks</h4>
                        <div class='chip-list'>
                           {#if face.landmarks.leftEye}
                              <span>Left eye {formatPoint(face.landmarks.leftEye)}</span>
                           {/if}
                           {#if face.landmarks.rightEye}
                              <span>Right eye {formatPoint(face.landmarks.rightEye)}</span>
                           {/if}
                           {#if face.landmarks.nose}
                              <span>Nose {formatPoint(face.landmarks.nose)}</span>
                           {/if}
                           {#if face.landmarks.mouthLeft}
                              <span>Mouth left {formatPoint(face.landmarks.mouthLeft)}</span>
                           {/if}
                           {#if face.landmarks.mouthRight}
                              <span>Mouth right {formatPoint(face.landmarks.mouthRight)}</span>
                           {/if}
                        </div>
                     </div>
                  {/if}

                  {#if face.attributes}
                     <div class='detail-block'>
                        <h4>Attributes</h4>

                        {#if face.attributes.smilingProbability != null}
                           <div class='meter'>
                              <div class='meter-header'>
                                 <span>Smiling</span>
                                 <strong>{formatPercent(face.attributes.smilingProbability)}</strong>
                              </div>
                              <div class='meter-track'>
                                 <div class='meter-fill' style={`width: ${formatPercent(face.attributes.smilingProbability)}`}></div>
                              </div>
                           </div>
                        {/if}

                        {#if face.attributes.leftEyeOpenProbability != null}
                           <div class='meter'>
                              <div class='meter-header'>
                                 <span>Left eye open</span>
                                 <strong>{formatPercent(face.attributes.leftEyeOpenProbability)}</strong>
                              </div>
                              <div class='meter-track'>
                                 <div class='meter-fill' style={`width: ${formatPercent(face.attributes.leftEyeOpenProbability)}`}></div>
                              </div>
                           </div>
                        {/if}

                        {#if face.attributes.rightEyeOpenProbability != null}
                           <div class='meter'>
                              <div class='meter-header'>
                                 <span>Right eye open</span>
                                 <strong>{formatPercent(face.attributes.rightEyeOpenProbability)}</strong>
                              </div>
                              <div class='meter-track'>
                                 <div class='meter-fill' style={`width: ${formatPercent(face.attributes.rightEyeOpenProbability)}`}></div>
                              </div>
                           </div>
                        {/if}
                     </div>
                  {/if}
               </article>
            {/each}
         </div>
      {:else if faceImage}
         <p class='empty-note'>No faces were detected in the uploaded image.</p>
      {/if}
   </section>

   <section class='panel'>
      <div class='panel-header'>
         <Icon icon='lucide:image' />
         <div>
            <h2>Image classification</h2>
            <p>Return ranked labels for objects and scenes present in the uploaded image.</p>
         </div>
      </div>

      <div class='field'>
         <label for='max-results'>Max results: {maxClassifications}</label>
         <input id='max-results' type='range' bind:value={maxClassifications} min='1' max='20' step='1' />
      </div>

      <div class='upload-zone'>
         <input
            id='classification-upload'
            type='file'
            accept='image/*'
            onchange={handleClassificationImage}
         />
         <label for='classification-upload'>
            <Icon icon='lucide:tag' />
            <span>Upload image to classify</span>
         </label>
      </div>

      {#if classificationImage}
         <div class='preview-card'>
            <img src={classificationImage} alt='Classification input preview' />
         </div>
      {/if}

      {#if isProcessingClassification}
         <p class='status-note'>Classifying image...</p>
      {:else if classificationResults.length > 0}
         <div class='result-card'>
            <div class='result-header'>
               <h3>Classification results</h3>
               <span>{classificationResults.length} label(s)</span>
            </div>

            <div class='stack-list compact'>
               {#each classificationResults as classification}
                  <div class='classification-row'>
                     <div class='classification-copy'>
                        <strong>{classification.identifier}</strong>
                        <span>{formatPercent(classification.confidence)}</span>
                     </div>
                     <div class='meter-track'>
                        <div class='meter-fill' style={`width: ${formatPercent(classification.confidence)}`}></div>
                     </div>
                  </div>
               {/each}
            </div>
         </div>
      {:else if classificationImage}
         <p class='empty-note'>No classifications were returned for the uploaded image.</p>
      {/if}
   </section>
</div>

<style lang='scss'>
   .tool-grid {
      display: grid;
      gap: 1rem;
      grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr));
   }

   .panel,
   .result-card,
   .stack-item,
   .face-card {
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-lg);
      background: oklch(100% 0 0 / 5%);
      box-shadow: var(--shadow-md);
   }

   .panel {
      display: flex;
      flex-direction: column;
      gap: 1rem;
      padding: 1.3rem;
   }

   .panel-header {
      display: flex;
      gap: 0.9rem;
      align-items: flex-start;

      :global(svg) {
         color: var(--accent);
         flex-shrink: 0;
         font-size: 1.2rem;
         margin-top: 0.15rem;
      }

      h2,
      p {
         margin: 0;
      }

      p {
         color: var(--text-muted);
         margin-top: 0.3rem;
      }
   }

   .field {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      label {
         color: var(--text-muted);
         font-size: 0.86rem;
      }
   }

   .upload-zone {
      input {
         display: none;
      }

      label {
         display: flex;
         align-items: center;
         justify-content: center;
         gap: 0.65rem;
         border: 1px dashed oklch(100% 0 0 / 12%);
         border-radius: var(--radius-md);
         background: oklch(9% 0.01 260 / 62%);
         color: var(--text-muted);
         cursor: pointer;
         padding: 1rem;
         transition: border-color 0.2s ease, color 0.2s ease;

         &:hover {
            border-color: oklch(72% 0.2 220 / 35%);
            color: var(--text);
         }
      }
   }

   .toggle-row {
      display: flex;
      flex-wrap: wrap;
      gap: 0.75rem;
   }

   .toggle-pill {
      display: inline-flex;
      align-items: center;
      gap: 0.5rem;
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: 999px;
      background: oklch(9% 0.01 260 / 62%);
      color: var(--text);
      padding: 0.55rem 0.9rem;

      input {
         margin: 0;
      }
   }

   .preview-card {
      overflow: hidden;
      border: 1px solid oklch(100% 0 0 / 8%);
      border-radius: var(--radius-md);
      background: oklch(9% 0.01 260 / 72%);

      img {
         display: block;
         width: 100%;
         max-height: 13rem;
         object-fit: contain;
         background: oklch(7% 0.01 260);
      }
   }

   .status-note,
   .empty-note {
      margin: 0;
      color: var(--text-muted);
      font-size: 0.9rem;
   }

   .result-card {
      padding: 1rem;

      pre {
         margin: 0;
         white-space: pre-wrap;
         word-break: break-word;
         font-family: var(--font-mono);
         line-height: 1.6;
      }
   }

   .result-header,
   .stack-item-header,
   .meter-header,
   .classification-copy {
      display: flex;
      flex-wrap: wrap;
      align-items: center;
      justify-content: space-between;
      gap: 0.75rem;
   }

   .result-header {
      margin-bottom: 0.8rem;

      h3,
      span {
         margin: 0;
      }

      span {
         color: var(--text-muted);
         font-size: 0.82rem;
      }
   }

   .stack-list {
      display: flex;
      flex-direction: column;
      gap: 0.75rem;

      &.compact {
         gap: 1rem;
      }
   }

   .stack-item,
   .face-card {
      padding: 0.9rem;
   }

   .stack-item {
      code {
         display: block;
         margin-top: 0.65rem;
         color: var(--accent);
         font-family: var(--font-mono);
         overflow-wrap: anywhere;
      }
   }

   .stack-item-header span,
   .metric-row,
   .classification-copy span {
      color: var(--text-muted);
      font-size: 0.82rem;
   }

   .metric-row,
   .chip-list {
      display: flex;
      flex-wrap: wrap;
      gap: 0.5rem;
      margin-top: 0.75rem;
   }

   .chip-list span {
      border-radius: 999px;
      background: oklch(100% 0 0 / 6%);
      padding: 0.35rem 0.6rem;
      font-size: 0.8rem;
   }

   .detail-block {
      margin-top: 0.9rem;

      h4 {
         margin: 0 0 0.55rem;
         font-size: 0.9rem;
      }
   }

   .meter {
      display: flex;
      flex-direction: column;
      gap: 0.45rem;

      + .meter {
         margin-top: 0.7rem;
      }
   }

   .meter-track {
      overflow: hidden;
      border-radius: 999px;
      background: oklch(100% 0 0 / 6%);
      height: 0.55rem;
   }

   .meter-fill {
      height: 100%;
      border-radius: inherit;
      background: linear-gradient(135deg, var(--accent), oklch(72% 0.2 220));
   }

   .classification-row {
      display: flex;
      flex-direction: column;
      gap: 0.45rem;
   }
</style>
