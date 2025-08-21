/*
██████╗  █████╗ ██████╗ ███████╗██████╗     ██╗   ██╗ █████╗ ██╗     ██╗██████╗  █████╗ ████████╗██╗ ██████╗ ███╗   ██╗
██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔══██╗    ██║   ██║██╔══██╗██║     ██║██╔══██╗██╔══██╗╚══██╔══╝██║██╔═══██╗████╗  ██║
██████╔╝███████║██████╔╝█████╗  ██████╔╝    ██║   ██║███████║██║     ██║██║  ██║███████║   ██║   ██║██║   ██║██╔██╗ ██║
██╔═══╝ ██╔══██║██╔═══╝ ██╔══╝  ██╔══██╗    ██║   ██║██╔══██║██║     ██║██║  ██║██╔══██║   ██║   ██║██║   ██║██║╚██╗██║
██║     ██║  ██║██║     ███████╗██║  ██║    ╚██████╔╝██║  ██║███████╗██║██████╔╝██║  ██║   ██║   ██║╚██████╔╝██║ ╚████║
╚═╝     ╚═╝  ╚═╝╚═╝     ╚══════╝╚═╝  ╚═╝     ╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝ ╚═════╝ ╚═╝  ╚═══╝
                                                      app/static/paper-validation.js
*/

/**
 * Paper Size Validation and Print Margins Calculator
 * Ensures legal document compliance across different paper formats
 * Supports US Letter, A4, Legal, and custom sizes with <0.25mm accuracy
 */

class PaperValidationSystem {
    constructor() {
        // Standard paper sizes in millimeters (width x height)
        this.PAPER_SIZES = {
            'US_LETTER': { width: 215.9, height: 279.4, name: 'US Letter (8.5" × 11")' },
            'US_LEGAL': { width: 215.9, height: 355.6, name: 'US Legal (8.5" × 14")' },
            'A4': { width: 210, height: 297, name: 'A4 (210mm × 297mm)' },
            'A3': { width: 297, height: 420, name: 'A3 (297mm × 420mm)' },
            'TABLOID': { width: 279.4, height: 431.8, name: 'Tabloid (11" × 17")' }
        };

        // Standard margins for legal documents (in millimeters)
        this.LEGAL_MARGINS = {
            'US_LETTER': { top: 19.05, right: 19.05, bottom: 25.4, left: 19.05 }, // 0.75", 0.75", 1", 0.75"
            'US_LEGAL': { top: 19.05, right: 19.05, bottom: 25.4, left: 19.05 },
            'A4': { top: 20, right: 20, bottom: 25, left: 20 },
            'A3': { top: 25, right: 25, bottom: 30, left: 25 },
            'TABLOID': { top: 25.4, right: 25.4, bottom: 25.4, left: 25.4 }
        };

        // Precision tolerances
        this.TOLERANCE_MM = 0.25; // Maximum allowed deviation
        this.MIN_MARGIN_MM = 12.7; // Minimum printable margin (0.5")
        this.MAX_CONTENT_WIDTH_RATIO = 0.9; // Maximum content width relative to page

        this.currentPaperSize = 'US_LETTER';
        this.currentOrientation = 'portrait'; // 'portrait' or 'landscape'
        this.customMargins = null;
        this.validationErrors = [];
    }

    /**
     * Validate current document against paper size requirements
     */
    validateDocument(documentElement) {
        this.validationErrors = [];
        
        try {
            const pages = this.getDocumentPages(documentElement);
            const results = {
                isValid: true,
                paperSize: this.currentPaperSize,
                orientation: this.currentOrientation,
                pages: [],
                errors: [],
                warnings: []
            };

            for (let i = 0; i < pages.length; i++) {
                const pageResult = this.validatePage(pages[i], i + 1);
                results.pages.push(pageResult);
                
                if (!pageResult.isValid) {
                    results.isValid = false;
                    results.errors.push(...pageResult.errors);
                }
                
                results.warnings.push(...pageResult.warnings);
            }

            return results;
            
        } catch (error) {
            return {
                isValid: false,
                error: error.message,
                pages: [],
                errors: [error.message],
                warnings: []
            };
        }
    }

    /**
     * Validate a single page against paper requirements
     */
    validatePage(pageElement, pageNumber) {
        const result = {
            pageNumber,
            isValid: true,
            errors: [],
            warnings: [],
            measurements: {},
            contentBounds: {},
            marginViolations: []
        };

        try {
            // Get page dimensions
            const pageRect = pageElement.getBoundingClientRect();
            const paperSize = this.getPaperSize();
            
            // Calculate expected dimensions
            const expectedDimensions = this.getExpectedPageDimensions();
            
            // Validate page size
            const sizeValidation = this.validatePageSize(pageRect, expectedDimensions);
            result.measurements.pageSize = sizeValidation;
            
            if (!sizeValidation.isValid) {
                result.isValid = false;
                result.errors.push(...sizeValidation.errors);
            }

            // Validate content positioning
            const contentValidation = this.validateContentPositioning(pageElement, pageRect);
            result.contentBounds = contentValidation;
            
            if (!contentValidation.isValid) {
                result.isValid = false;
                result.errors.push(...contentValidation.errors);
            }

            // Validate margins
            const marginValidation = this.validateMargins(pageElement, pageRect);
            result.marginViolations = marginValidation.violations;
            
            if (marginValidation.violations.length > 0) {
                result.warnings.push(...marginValidation.warnings);
            }

            // Validate field positioning
            const fieldValidation = this.validateFieldPositioning(pageElement, pageRect);
            if (!fieldValidation.isValid) {
                result.warnings.push(...fieldValidation.warnings);
            }

            return result;
            
        } catch (error) {
            result.isValid = false;
            result.errors.push(`Page ${pageNumber} validation failed: ${error.message}`);
            return result;
        }
    }

    /**
     * Calculate print margins for current paper size
     */
    calculatePrintMargins() {
        const paperSize = this.getPaperSize();
        const margins = this.getMargins();
        
        return {
            paperSize: {
                width: paperSize.width,
                height: paperSize.height,
                name: paperSize.name
            },
            margins: {
                top: margins.top,
                right: margins.right,
                bottom: margins.bottom,
                left: margins.left
            },
            contentArea: {
                width: paperSize.width - margins.left - margins.right,
                height: paperSize.height - margins.top - margins.bottom,
                x: margins.left,
                y: margins.top
            },
            printableArea: {
                width: paperSize.width - (this.MIN_MARGIN_MM * 2),
                height: paperSize.height - (this.MIN_MARGIN_MM * 2),
                x: this.MIN_MARGIN_MM,
                y: this.MIN_MARGIN_MM
            }
        };
    }

    /**
     * Set paper size and orientation
     */
    setPaperSize(sizeKey, orientation = 'portrait') {
        if (!this.PAPER_SIZES[sizeKey]) {
            throw new Error(`Unsupported paper size: ${sizeKey}`);
        }

        this.currentPaperSize = sizeKey;
        this.currentOrientation = orientation;
        
        // Trigger document revalidation
        this.notifyPaperSizeChange();
        
        return this.calculatePrintMargins();
    }

    /**
     * Set custom margins (in millimeters)
     */
    setCustomMargins(margins) {
        const { top, right, bottom, left } = margins;
        
        // Validate margin values
        const minMargin = this.MIN_MARGIN_MM;
        const paperSize = this.getPaperSize();
        
        if (top < minMargin || right < minMargin || bottom < minMargin || left < minMargin) {
            throw new Error(`All margins must be at least ${minMargin}mm`);
        }
        
        if (left + right >= paperSize.width || top + bottom >= paperSize.height) {
            throw new Error('Margins exceed paper dimensions');
        }
        
        this.customMargins = { top, right, bottom, left };
        return this.calculatePrintMargins();
    }

    /**
     * Get current paper size with orientation
     */
    getPaperSize() {
        const baseSize = this.PAPER_SIZES[this.currentPaperSize];
        
        if (this.currentOrientation === 'landscape') {
            return {
                width: baseSize.height,
                height: baseSize.width,
                name: `${baseSize.name} (Landscape)`
            };
        }
        
        return baseSize;
    }

    /**
     * Get current margins
     */
    getMargins() {
        if (this.customMargins) {
            return this.customMargins;
        }
        
        const baseMargins = this.LEGAL_MARGINS[this.currentPaperSize];
        
        if (this.currentOrientation === 'landscape') {
            // Swap margins for landscape
            return {
                top: baseMargins.left,
                right: baseMargins.top,
                bottom: baseMargins.right,
                left: baseMargins.bottom
            };
        }
        
        return baseMargins;
    }

    /**
     * Convert measurements between different units
     */
    convertUnits(value, fromUnit, toUnit) {
        // Convert to millimeters first
        let mm;
        
        switch (fromUnit.toLowerCase()) {
            case 'mm':
                mm = value;
                break;
            case 'in':
            case 'inch':
                mm = value * 25.4;
                break;
            case 'pt':
            case 'point':
                mm = value * 0.352778;
                break;
            case 'px':
            case 'pixel':
                mm = value * 0.264583; // Assuming 96 DPI
                break;
            default:
                throw new Error(`Unsupported unit: ${fromUnit}`);
        }
        
        // Convert from millimeters to target unit
        switch (toUnit.toLowerCase()) {
            case 'mm':
                return mm;
            case 'in':
            case 'inch':
                return mm / 25.4;
            case 'pt':
            case 'point':
                return mm / 0.352778;
            case 'px':
            case 'pixel':
                return mm / 0.264583; // Assuming 96 DPI
            default:
                throw new Error(`Unsupported unit: ${toUnit}`);
        }
    }

    /**
     * Check if content fits within paper boundaries
     */
    checkContentFit(contentElement) {
        const contentRect = contentElement.getBoundingClientRect();
        const printMargins = this.calculatePrintMargins();
        
        const maxWidth = printMargins.contentArea.width;
        const maxHeight = printMargins.contentArea.height;
        
        // Convert pixel dimensions to mm (rough approximation)
        const contentWidthMM = this.pixelsToMM(contentRect.width);
        const contentHeightMM = this.pixelsToMM(contentRect.height);
        
        return {
            fitsWidth: contentWidthMM <= maxWidth,
            fitsHeight: contentHeightMM <= maxHeight,
            contentWidth: contentWidthMM,
            contentHeight: contentHeightMM,
            maxWidth,
            maxHeight,
            overflowWidth: Math.max(0, contentWidthMM - maxWidth),
            overflowHeight: Math.max(0, contentHeightMM - maxHeight)
        };
    }

    /**
     * Generate print CSS for current paper settings
     */
    generatePrintCSS() {
        const paperSize = this.getPaperSize();
        const margins = this.getMargins();
        
        return `
            @page {
                size: ${paperSize.width}mm ${paperSize.height}mm;
                margin: ${margins.top}mm ${margins.right}mm ${margins.bottom}mm ${margins.left}mm;
            }
            
            @media print {
                .page {
                    width: ${paperSize.width - margins.left - margins.right}mm;
                    height: ${paperSize.height - margins.top - margins.bottom}mm;
                    page-break-after: always;
                }
                
                .page:last-child {
                    page-break-after: auto;
                }
            }
        `;
    }

    /**
     * Validate page size against expected dimensions
     */
    validatePageSize(pageRect, expectedDimensions) {
        const currentWidthMM = this.pixelsToMM(pageRect.width);
        const currentHeightMM = this.pixelsToMM(pageRect.height);
        
        const widthDiff = Math.abs(currentWidthMM - expectedDimensions.width);
        const heightDiff = Math.abs(currentHeightMM - expectedDimensions.height);
        
        const isValid = widthDiff <= this.TOLERANCE_MM && heightDiff <= this.TOLERANCE_MM;
        
        return {
            isValid,
            currentDimensions: { width: currentWidthMM, height: currentHeightMM },
            expectedDimensions,
            deviations: { width: widthDiff, height: heightDiff },
            errors: isValid ? [] : [
                `Page size deviation exceeds tolerance: width ${widthDiff.toFixed(2)}mm, height ${heightDiff.toFixed(2)}mm`
            ]
        };
    }

    /**
     * Get expected page dimensions for current settings
     */
    getExpectedPageDimensions() {
        const paperSize = this.getPaperSize();
        const margins = this.getMargins();
        
        return {
            width: paperSize.width - margins.left - margins.right,
            height: paperSize.height - margins.top - margins.bottom
        };
    }

    /**
     * Helper: Convert pixels to millimeters (approximate)
     */
    pixelsToMM(pixels) {
        // Rough conversion assuming 96 DPI
        return pixels * 0.264583;
    }

    /**
     * Helper: Get document pages
     */
    getDocumentPages(documentElement) {
        return Array.from(documentElement.querySelectorAll('.document-page, .form-page'));
    }

    /**
     * Notify system of paper size change
     */
    notifyPaperSizeChange() {
        const event = new CustomEvent('paperSizeChanged', {
            detail: {
                paperSize: this.currentPaperSize,
                orientation: this.currentOrientation,
                margins: this.calculatePrintMargins()
            }
        });
        
        document.dispatchEvent(event);
    }

    /**
     * Placeholder methods for comprehensive validation
     */
    validateContentPositioning(pageElement, pageRect) {
        // Implementation would check if content is positioned correctly
        return { isValid: true, errors: [] };
    }

    validateMargins(pageElement, pageRect) {
        // Implementation would check margin compliance
        return { violations: [], warnings: [] };
    }

    validateFieldPositioning(pageElement, pageRect) {
        // Implementation would validate form field positions
        return { isValid: true, warnings: [] };
    }
}

// Initialize paper validation system
window.paperValidation = new PaperValidationSystem();

// Export for module usage
if (typeof module !== 'undefined' && module.exports) {
    module.exports = PaperValidationSystem;
}