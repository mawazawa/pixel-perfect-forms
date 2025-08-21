//
// ██████╗██╗   ██╗    ██╗    ██╗ ██████╗ ██████╗ ██╗  ██╗███████╗██████╗ 
//██╔════╝██║   ██║    ██║    ██║██╔═══██╗██╔══██╗██║ ██╔╝██╔════╝██╔══██╗
//██║     ██║   ██║    ██║ █╗ ██║██║   ██║██████╔╝█████╔╝ █████╗  ██████╔╝
//██║     ╚██╗ ██╔╝    ██║███╗██║██║   ██║██╔══██╗██╔═██╗ ██╔══╝  ██╔══██╗
//╚██████╗ ╚████╔╝     ╚███╔███╔╝╚██████╔╝██║  ██║██║  ██╗███████╗██║  ██║
// ╚═════╝  ╚═══╝       ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝
//                                     app/static/workers/cv-worker.js

/**
 * Computer Vision Worker for FL-100 Template Matching and Auto-Alignment
 * 
 * This worker handles:
 * - Template matching for form field detection
 * - Automatic correction of misaligned overlays
 * - SSIM/PSNR quality assessment
 * - Background processing to avoid blocking UI
 */

// Import OpenCV.js for computer vision operations
importScripts('https://docs.opencv.org/4.8.0/opencv.js');

class CVWorker {
    constructor() {
        this.isInitialized = false;
        this.templates = new Map();
        this.alignmentThreshold = 0.8; // Template matching threshold
        this.maxAlignment = 50; // Maximum pixels to auto-correct
    }

    /**
     * Initialize OpenCV and prepare worker
     */
    async initialize() {
        return new Promise((resolve) => {
            if (typeof cv !== 'undefined') {
                this.isInitialized = true;
                resolve(true);
            } else {
                // Wait for OpenCV to load
                const checkCV = () => {
                    if (typeof cv !== 'undefined') {
                        this.isInitialized = true;
                        resolve(true);
                    } else {
                        setTimeout(checkCV, 100);
                    }
                };
                checkCV();
            }
        });
    }

    /**
     * Process template matching for form field detection
     * @param {Object} data - Contains image data and template info
     */
    processTemplateMatching(data) {
        if (!this.isInitialized) {
            throw new Error('CV Worker not initialized');
        }

        const { imageData, templateData, fieldId, threshold = this.alignmentThreshold } = data;

        try {
            // Convert image data to OpenCV Mat
            const src = this.imageDataToMat(imageData);
            const template = this.imageDataToMat(templateData);

            // Convert to grayscale for better matching
            const srcGray = new cv.Mat();
            const templateGray = new cv.Mat();
            cv.cvtColor(src, srcGray, cv.COLOR_RGBA2GRAY);
            cv.cvtColor(template, templateGray, cv.COLOR_RGBA2GRAY);

            // Perform template matching
            const result = new cv.Mat();
            cv.matchTemplate(srcGray, templateGray, result, cv.TM_CCOEFF_NORMED);

            // Find the best match location
            const minMaxLoc = cv.minMaxLoc(result);
            const confidence = minMaxLoc.maxVal;
            const matchLocation = minMaxLoc.maxLoc;

            // Calculate alignment correction
            const correction = {
                x: 0,
                y: 0,
                confidence,
                applied: false
            };

            if (confidence >= threshold) {
                // Calculate the offset needed for alignment
                const centerX = matchLocation.x + template.cols / 2;
                const centerY = matchLocation.y + template.rows / 2;
                
                // Expected center (assuming current position is roughly correct)
                const expectedX = src.cols / 2;
                const expectedY = src.rows / 2;
                
                correction.x = centerX - expectedX;
                correction.y = centerY - expectedY;
                
                // Only apply correction if within reasonable bounds
                if (Math.abs(correction.x) <= this.maxAlignment && 
                    Math.abs(correction.y) <= this.maxAlignment) {
                    correction.applied = true;
                }
            }

            // Clean up OpenCV Mats
            src.delete();
            template.delete();
            srcGray.delete();
            templateGray.delete();
            result.delete();

            return {
                fieldId,
                correction,
                success: true
            };

        } catch (error) {
            return {
                fieldId,
                correction: { x: 0, y: 0, confidence: 0, applied: false },
                error: error.message,
                success: false
            };
        }
    }

    /**
     * Calculate SSIM (Structural Similarity Index) for quality assessment
     * @param {Object} data - Contains two image data objects to compare
     */
    calculateSSIM(data) {
        if (!this.isInitialized) {
            throw new Error('CV Worker not initialized');
        }

        const { imageData1, imageData2 } = data;

        try {
            const img1 = this.imageDataToMat(imageData1);
            const img2 = this.imageDataToMat(imageData2);

            // Convert to grayscale
            const gray1 = new cv.Mat();
            const gray2 = new cv.Mat();
            cv.cvtColor(img1, gray1, cv.COLOR_RGBA2GRAY);
            cv.cvtColor(img2, gray2, cv.COLOR_RGBA2GRAY);

            // Convert to float for calculations
            const float1 = new cv.Mat();
            const float2 = new cv.Mat();
            gray1.convertTo(float1, cv.CV_32F);
            gray2.convertTo(float2, cv.CV_32F);

            // Calculate means
            const mean1 = cv.mean(float1)[0];
            const mean2 = cv.mean(float2)[0];

            // Calculate variances and covariance (simplified SSIM)
            const diff1 = new cv.Mat();
            const diff2 = new cv.Mat();
            const mul = new cv.Mat();
            
            cv.subtract(float1, new cv.Scalar(mean1), diff1);
            cv.subtract(float2, new cv.Scalar(mean2), diff2);
            cv.multiply(diff1, diff2, mul);

            const var1 = cv.mean(cv.multiply(diff1, diff1))[0];
            const var2 = cv.mean(cv.multiply(diff2, diff2))[0];
            const covar = cv.mean(mul)[0];

            // SSIM calculation constants
            const c1 = 0.01 * 0.01;
            const c2 = 0.03 * 0.03;

            const ssim = ((2 * mean1 * mean2 + c1) * (2 * covar + c2)) /
                        ((mean1 * mean1 + mean2 * mean2 + c1) * (var1 + var2 + c2));

            // Clean up
            img1.delete();
            img2.delete();
            gray1.delete();
            gray2.delete();
            float1.delete();
            float2.delete();
            diff1.delete();
            diff2.delete();
            mul.delete();

            return {
                ssim,
                quality: ssim > 0.9 ? 'excellent' : 
                        ssim > 0.7 ? 'good' : 
                        ssim > 0.5 ? 'fair' : 'poor',
                success: true
            };

        } catch (error) {
            return {
                ssim: 0,
                quality: 'error',
                error: error.message,
                success: false
            };
        }
    }

    /**
     * Detect form field boundaries using edge detection
     * @param {Object} data - Contains image data and detection parameters
     */
    detectFieldBoundaries(data) {
        if (!this.isInitialized) {
            throw new Error('CV Worker not initialized');
        }

        const { imageData, threshold1 = 50, threshold2 = 150 } = data;

        try {
            const src = this.imageDataToMat(imageData);
            const gray = new cv.Mat();
            const edges = new cv.Mat();
            const contours = new cv.MatVector();
            const hierarchy = new cv.Mat();

            // Convert to grayscale
            cv.cvtColor(src, gray, cv.COLOR_RGBA2GRAY);

            // Apply Gaussian blur to reduce noise
            const blurred = new cv.Mat();
            cv.GaussianBlur(gray, blurred, new cv.Size(5, 5), 0);

            // Detect edges using Canny
            cv.Canny(blurred, edges, threshold1, threshold2);

            // Find contours
            cv.findContours(edges, contours, hierarchy, cv.RETR_EXTERNAL, cv.CHAIN_APPROX_SIMPLE);

            // Process contours to find rectangular form fields
            const fields = [];
            for (let i = 0; i < contours.size(); i++) {
                const contour = contours.get(i);
                const area = cv.contourArea(contour);
                
                // Filter by area (form fields should be reasonably sized)
                if (area > 100 && area < 10000) {
                    const rect = cv.boundingRect(contour);
                    
                    // Check aspect ratio (form fields are typically rectangular)
                    const aspectRatio = rect.width / rect.height;
                    if (aspectRatio > 0.5 && aspectRatio < 10) {
                        fields.push({
                            x: rect.x,
                            y: rect.y,
                            width: rect.width,
                            height: rect.height,
                            area,
                            aspectRatio
                        });
                    }
                }
                contour.delete();
            }

            // Clean up
            src.delete();
            gray.delete();
            edges.delete();
            blurred.delete();
            contours.delete();
            hierarchy.delete();

            return {
                fields,
                count: fields.length,
                success: true
            };

        } catch (error) {
            return {
                fields: [],
                count: 0,
                error: error.message,
                success: false
            };
        }
    }

    /**
     * Convert ImageData to OpenCV Mat
     * @param {ImageData} imageData - Browser ImageData object
     * @returns {cv.Mat} OpenCV Mat object
     */
    imageDataToMat(imageData) {
        return cv.matFromImageData(imageData);
    }

    /**
     * Convert OpenCV Mat to ImageData
     * @param {cv.Mat} mat - OpenCV Mat object
     * @returns {ImageData} Browser ImageData object
     */
    matToImageData(mat) {
        const canvas = new OffscreenCanvas(mat.cols, mat.rows);
        cv.imshow(canvas, mat);
        const ctx = canvas.getContext('2d');
        return ctx.getImageData(0, 0, mat.cols, mat.rows);
    }
}

// Initialize worker
const cvWorker = new CVWorker();
let initPromise = null;

// Message handler
self.addEventListener('message', async (event) => {
    const { type, data, id } = event.data;

    try {
        // Initialize if needed
        if (!initPromise) {
            initPromise = cvWorker.initialize();
        }
        await initPromise;

        let result;

        switch (type) {
            case 'template-match':
                result = cvWorker.processTemplateMatching(data);
                break;

            case 'calculate-ssim':
                result = cvWorker.calculateSSIM(data);
                break;

            case 'detect-fields':
                result = cvWorker.detectFieldBoundaries(data);
                break;

            case 'ping':
                result = { pong: true, initialized: cvWorker.isInitialized };
                break;

            default:
                throw new Error(`Unknown message type: ${type}`);
        }

        self.postMessage({
            type: 'result',
            id,
            data: result
        });

    } catch (error) {
        self.postMessage({
            type: 'error',
            id,
            error: error.message,
            data: null
        });
    }
});

// Handle worker errors
self.addEventListener('error', (error) => {
    self.postMessage({
        type: 'worker-error',
        error: error.message
    });
});

// Signal worker is ready
self.postMessage({
    type: 'ready',
    message: 'CV Worker initialized'
});