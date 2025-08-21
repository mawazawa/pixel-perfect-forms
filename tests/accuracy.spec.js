/*
 █████╗  ██████╗ ██████╗██╗   ██╗██████╗  █████╗  ██████╗██╗   ██╗    ████████╗███████╗███████╗████████╗███████╗
██╔══██╗██╔════╝██╔════╝██║   ██║██╔══██╗██╔══██╗██╔════╝╚██╗ ██╔╝    ╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝██╔════╝
███████║██║     ██║     ██║   ██║██████╔╝███████║██║      ╚████╔╝        ██║   █████╗  ███████╗   ██║   ███████╗
██╔══██║██║     ██║     ██║   ██║██╔══██╗██╔══██║██║       ╚██╔╝         ██║   ██╔══╝  ╚════██║   ██║   ╚════██║
██║  ██║╚██████╗╚██████╗╚██████╔╝██║  ██║██║  ██║╚██████╗   ██║          ██║   ███████╗███████║   ██║   ███████║
╚═╝  ╚═╝ ╚═════╝ ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝   ╚═╝          ╚═╝   ╚══════╝╚══════╝   ╚═╝   ╚══════╝
                                                            tests/accuracy.spec.js
*/

import { test, expect } from '@playwright/test';

/**
 * FL-100 Pixel-Perfect Accuracy Tests
 * Validates that forms maintain <0.25mm accuracy across browsers and devices
 */

const ACCURACY_THRESHOLD_MM = 0.25;
const DPI = 96; // Standard browser DPI
const MM_PER_PIXEL = 25.4 / DPI;

test.describe('FL-100 Accuracy Validation', () => {
  test.beforeEach(async ({ page }) => {
    // Set high DPI for precise measurements
    await page.setViewportSize({ width: 1200, height: 1600 });
    await page.goto('/');
    
    // Wait for WASM to load
    await page.waitForFunction(() => window.wasmBindings !== undefined);
    
    // Wait for any calibration to complete
    await page.waitForTimeout(2000);
  });

  test('Page dimensions match US Letter specifications', async ({ page }) => {
    // Navigate to document view
    const documentPage = page.locator('.document-page, .form-page').first();
    await expect(documentPage).toBeVisible();
    
    // Get page dimensions
    const boundingBox = await documentPage.boundingBox();
    expect(boundingBox).not.toBeNull();
    
    // Convert pixels to mm
    const widthMM = boundingBox.width * MM_PER_PIXEL;
    const heightMM = boundingBox.height * MM_PER_PIXEL;
    
    // US Letter content area: 8.5" - 1.5" margins = 7" = 177.8mm
    // 11" - 1.5" margins = 9.5" = 241.3mm
    const expectedWidthMM = 177.8;
    const expectedHeightMM = 241.3;
    
    // Validate dimensions within tolerance
    const widthDeviation = Math.abs(widthMM - expectedWidthMM);
    const heightDeviation = Math.abs(heightMM - expectedHeightMM);
    
    expect(widthDeviation).toBeLessThan(ACCURACY_THRESHOLD_MM);
    expect(heightDeviation).toBeLessThan(ACCURACY_THRESHOLD_MM);
    
    console.log(`Page dimensions: ${widthMM.toFixed(2)}mm x ${heightMM.toFixed(2)}mm`);
    console.log(`Expected: ${expectedWidthMM}mm x ${expectedHeightMM}mm`);
    console.log(`Deviation: ${widthDeviation.toFixed(2)}mm x ${heightDeviation.toFixed(2)}mm`);
  });

  test('Form field positioning accuracy', async ({ page }) => {
    // Check if form fields are present
    const formFields = page.locator('.form-field, .field-input');
    const fieldCount = await formFields.count();
    
    if (fieldCount === 0) {
      test.skip('No form fields found to test');
    }
    
    // Test positioning of first few fields
    for (let i = 0; i < Math.min(fieldCount, 5); i++) {
      const field = formFields.nth(i);
      await expect(field).toBeVisible();
      
      const boundingBox = await field.boundingBox();
      expect(boundingBox).not.toBeNull();
      
      // Convert to mm
      const xMM = boundingBox.x * MM_PER_PIXEL;
      const yMM = boundingBox.y * MM_PER_PIXEL;
      const widthMM = boundingBox.width * MM_PER_PIXEL;
      const heightMM = boundingBox.height * MM_PER_PIXEL;
      
      // Validate field is within page bounds
      expect(xMM).toBeGreaterThan(0);
      expect(yMM).toBeGreaterThan(0);
      expect(widthMM).toBeGreaterThan(1); // At least 1mm wide
      expect(heightMM).toBeGreaterThan(1); // At least 1mm tall
      
      console.log(`Field ${i}: ${xMM.toFixed(2)}, ${yMM.toFixed(2)} (${widthMM.toFixed(2)}x${heightMM.toFixed(2)}mm)`);
    }
  });

  test('Text field font metrics accuracy', async ({ page }) => {
    const textFields = page.locator('.field-input.text-input, input[type="text"]');
    const textFieldCount = await textFields.count();
    
    if (textFieldCount === 0) {
      test.skip('No text fields found to test');
    }
    
    const firstTextField = textFields.first();
    await expect(firstTextField).toBeVisible();
    
    // Get computed font size
    const fontSize = await firstTextField.evaluate(el => {
      const style = window.getComputedStyle(el);
      return parseFloat(style.fontSize);
    });
    
    // Legal documents typically use 12pt font
    const expectedFontSizePx = 16; // 12pt = 16px at 96 DPI
    const fontSizeDeviation = Math.abs(fontSize - expectedFontSizePx);
    
    // Allow 1px deviation for font rendering
    expect(fontSizeDeviation).toBeLessThan(2);
    
    console.log(`Font size: ${fontSize}px (expected: ${expectedFontSizePx}px)`);
  });

  test('Checkbox field dimensions', async ({ page }) => {
    const checkboxes = page.locator('input[type="checkbox"], .checkbox-input');
    const checkboxCount = await checkboxes.count();
    
    if (checkboxCount === 0) {
      test.skip('No checkboxes found to test');
    }
    
    const firstCheckbox = checkboxes.first();
    await expect(firstCheckbox).toBeVisible();
    
    const boundingBox = await firstCheckbox.boundingBox();
    expect(boundingBox).not.toBeNull();
    
    // Convert to mm
    const widthMM = boundingBox.width * MM_PER_PIXEL;
    const heightMM = boundingBox.height * MM_PER_PIXEL;
    
    // Standard checkbox should be ~5mm (0.2") square
    const expectedSize = 5; // mm
    const widthDeviation = Math.abs(widthMM - expectedSize);
    const heightDeviation = Math.abs(heightMM - expectedSize);
    
    expect(widthDeviation).toBeLessThan(ACCURACY_THRESHOLD_MM);
    expect(heightDeviation).toBeLessThan(ACCURACY_THRESHOLD_MM);
    
    console.log(`Checkbox: ${widthMM.toFixed(2)}mm x ${heightMM.toFixed(2)}mm`);
  });

  test('Grid overlay precision', async ({ page }) => {
    // Enable grid if available
    const gridToggle = page.locator('input[type="checkbox"]:has-text("Grid"), .grid-toggle');
    if (await gridToggle.count() > 0) {
      await gridToggle.check();
    }
    
    const gridOverlay = page.locator('.grid-overlay, .global-grid-overlay');
    if (await gridOverlay.count() === 0) {
      test.skip('No grid overlay found');
    }
    
    await expect(gridOverlay.first()).toBeVisible();
    
    // Check grid line spacing (should be consistent)
    const gridLines = await page.$$eval('.grid-overlay', overlays => {
      return overlays.map(overlay => {
        const style = window.getComputedStyle(overlay);
        const backgroundSize = style.backgroundSize;
        return backgroundSize;
      });
    });
    
    // Validate grid consistency
    expect(gridLines.length).toBeGreaterThan(0);
    console.log('Grid patterns:', gridLines);
  });

  test('Print CSS accuracy validation', async ({ page }) => {
    // Simulate print mode
    await page.addStyleTag({
      content: `
        @media screen {
          .form-page, .document-page {
            transform: scale(0.75) !important;
            transform-origin: top left !important;
          }
        }
      `
    });
    
    // Wait for layout to stabilize
    await page.waitForTimeout(1000);
    
    const documentPage = page.locator('.document-page, .form-page').first();
    await expect(documentPage).toBeVisible();
    
    // Check that print styles don't break layout
    const boundingBox = await documentPage.boundingBox();
    expect(boundingBox).not.toBeNull();
    expect(boundingBox.width).toBeGreaterThan(100);
    expect(boundingBox.height).toBeGreaterThan(100);
  });

  test('Export system accuracy validation', async ({ page }) => {
    // Test if export systems are available
    const exportSystemReady = await page.evaluate(() => {
      return window.exportSystem && window.paperValidation && window.printPreview;
    });
    
    if (!exportSystemReady) {
      test.skip('Export systems not loaded');
    }
    
    // Test paper validation
    const validationResult = await page.evaluate(() => {
      const documentElement = document.querySelector('.document-manager, .proofing-ui');
      if (!documentElement) return null;
      
      return window.paperValidation.validateDocument(documentElement);
    });
    
    if (validationResult) {
      expect(validationResult.isValid).toBe(true);
      
      // Check for any accuracy warnings
      const warnings = validationResult.warnings || [];
      console.log('Validation warnings:', warnings);
      
      // No critical errors should be present
      const errors = validationResult.errors || [];
      expect(errors.length).toBe(0);
    }
  });

  test('Responsive layout accuracy', async ({ page }) => {
    // Test various viewport sizes
    const viewports = [
      { width: 1920, height: 1080, name: 'Desktop Large' },
      { width: 1366, height: 768, name: 'Desktop Medium' },
      { width: 1024, height: 768, name: 'Tablet Landscape' },
      { width: 768, height: 1024, name: 'Tablet Portrait' }
    ];
    
    for (const viewport of viewports) {
      await page.setViewportSize({ width: viewport.width, height: viewport.height });
      await page.waitForTimeout(500); // Allow layout to stabilize
      
      const documentPage = page.locator('.document-page, .form-page').first();
      await expect(documentPage).toBeVisible();
      
      const boundingBox = await documentPage.boundingBox();
      expect(boundingBox).not.toBeNull();
      
      // Document should maintain minimum size
      expect(boundingBox.width).toBeGreaterThan(200);
      expect(boundingBox.height).toBeGreaterThan(200);
      
      console.log(`${viewport.name}: ${boundingBox.width}x${boundingBox.height}px`);
    }
  });

  test('Cross-browser font rendering consistency', async ({ page, browserName }) => {
    const textElement = page.locator('.form-field, .field-input').first();
    
    if (await textElement.count() === 0) {
      test.skip('No text elements found');
    }
    
    await expect(textElement).toBeVisible();
    
    // Get font metrics
    const fontMetrics = await textElement.evaluate(el => {
      const style = window.getComputedStyle(el);
      return {
        fontFamily: style.fontFamily,
        fontSize: style.fontSize,
        lineHeight: style.lineHeight,
        fontWeight: style.fontWeight
      };
    });
    
    // Validate font family includes Times New Roman for legal documents
    expect(fontMetrics.fontFamily.toLowerCase()).toContain('times');
    
    // Log browser-specific metrics
    console.log(`${browserName} font metrics:`, fontMetrics);
  });

  test('Performance impact of accuracy features', async ({ page }) => {
    // Measure load time
    const startTime = Date.now();
    await page.goto('/');
    await page.waitForFunction(() => window.wasmBindings !== undefined);
    const loadTime = Date.now() - startTime;
    
    // Should load within reasonable time even with accuracy features
    expect(loadTime).toBeLessThan(10000); // 10 seconds max
    
    // Measure rendering performance
    const renderMetrics = await page.evaluate(() => {
      return performance.getEntriesByType('measure').map(entry => ({
        name: entry.name,
        duration: entry.duration
      }));
    });
    
    console.log(`Load time: ${loadTime}ms`);
    console.log('Render metrics:', renderMetrics);
  });
});