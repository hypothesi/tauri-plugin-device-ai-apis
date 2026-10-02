/**
 * Normalize the many error formats the demo can receive into a displayable string.
 */
export function getErrorMessage(error) {
  if (!error) {
    return "Unknown error";
  }

  if (typeof error === "object" && typeof error.message === "string") {
    return error.message;
  }

  if (typeof error === "string") {
    return error;
  }

  return String(error);
}

/**
 * Convert a Blob or File into a base64 string without the data URL prefix.
 */
export function blobToBase64(blob) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();

    reader.onload = () => {
      if (typeof reader.result !== "string") {
        reject(new Error("Could not read blob as a data URL."));
        return;
      }

      const [, base64 = ""] = reader.result.split(",");

      resolve(base64);
    };
    reader.onerror = () => {
      reject(reader.error ?? new Error("Failed to read the selected file."));
    };
    reader.readAsDataURL(blob);
  });
}

/**
 * Convert a File into a data URL for preview rendering.
 */
export function fileToDataUrl(file) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();

    reader.onload = () => {
      if (typeof reader.result !== "string") {
        reject(new Error("Could not build a preview for the selected file."));
        return;
      }

      resolve(reader.result);
    };
    reader.onerror = () => {
      reject(reader.error ?? new Error("Failed to read the selected file."));
    };
    reader.readAsDataURL(file);
  });
}
