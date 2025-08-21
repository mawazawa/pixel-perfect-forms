/*
██████╗ ███████╗██████╗ ███████╗ ██████╗ ██████╗ ███╗   ███╗ █████╗ ███╗   ██╗ ██████╗███████╗
██╔══██╗██╔════╝██╔══██╗██╔════╝██╔═══██╗██╔══██╗████╗ ████║██╔══██╗████╗  ██║██╔════╝██╔════╝
██████╔╝█████╗  ██████╔╝█████╗  ██║   ██║██████╔╝██╔████╔██║███████║██╔██╗ ██║██║     █████╗  
██╔═══╝ ██╔══╝  ██╔══██╗██╔══╝  ██║   ██║██╔══██╗██║╚██╔╝██║██╔══██║██║╚██╗██║██║     ██╔══╝  
██║     ███████╗██║  ██║██║     ╚██████╔╝██║  ██║██║ ╚═╝ ██║██║  ██║██║ ╚████║╚██████╗███████╗
╚═╝     ╚══════╝╚═╝  ╚═╝╚═╝      ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝╚══════╝
                                                   scripts/analyze-performance.js
*/

const fs = require('fs');
const path = require('path');

/**
 * FL-100 Performance Analysis Script
 * Analyzes bundle sizes, load times, and performance metrics
 */

class PerformanceAnalyzer {
    constructor() {
        this.distPath = path.join(__dirname, '../app/dist');
        this.reportPath = path.join(__dirname, '../performance-report.json');
        this.thresholds = {
            bundleSize: 5 * 1024 * 1024, // 5MB
            wasmSize: 3 * 1024 * 1024,   // 3MB
            jsSize: 2 * 1024 * 1024,     // 2MB
            cssSize: 500 * 1024,         // 500KB
            loadTime: 3000,              // 3s
            firstContentfulPaint: 2000,  // 2s
            largestContentfulPaint: 3000 // 3s
        };
    }

    /**
     * Analyze WASM bundle sizes
     */
    analyzeBundleSizes() {
        console.log('\n=== Bundle Size Analysis ===');
        
        if (!fs.existsSync(this.distPath)) {
            console.error('❌ Dist directory not found. Run build first.');
            return false;
        }

        const files = fs.readdirSync(this.distPath);
        const analysis = {
            wasm: { size: 0, files: [] },
            js: { size: 0, files: [] },
            css: { size: 0, files: [] },
            other: { size: 0, files: [] },
            total: 0
        };

        files.forEach(file => {
            const filePath = path.join(this.distPath, file);
            const stats = fs.statSync(filePath);
            
            if (stats.isFile()) {
                const ext = path.extname(file).toLowerCase();
                const size = stats.size;
                
                switch (ext) {
                    case '.wasm':
                        analysis.wasm.size += size;
                        analysis.wasm.files.push({ name: file, size });
                        break;
                    case '.js':
                        analysis.js.size += size;
                        analysis.js.files.push({ name: file, size });
                        break;
                    case '.css':
                        analysis.css.size += size;
                        analysis.css.files.push({ name: file, size });
                        break;
                    default:
                        analysis.other.size += size;
                        analysis.other.files.push({ name: file, size });
                }
                
                analysis.total += size;
            }
        });

        // Report findings
        this.reportBundleSize('WASM', analysis.wasm, this.thresholds.wasmSize);
        this.reportBundleSize('JavaScript', analysis.js, this.thresholds.jsSize);
        this.reportBundleSize('CSS', analysis.css, this.thresholds.cssSize);
        this.reportBundleSize('Other', analysis.other, Infinity);
        
        console.log(`\n📦 Total Bundle Size: ${this.formatBytes(analysis.total)}`);
        
        // Check against threshold
        if (analysis.total > this.thresholds.bundleSize) {
            console.log(`⚠️  Exceeds ${this.formatBytes(this.thresholds.bundleSize)} threshold`);
            return false;
        } else {
            console.log(`✅ Within ${this.formatBytes(this.thresholds.bundleSize)} threshold`);
        }

        return analysis;
    }

    /**
     * Report bundle size for a category
     */
    reportBundleSize(category, data, threshold) {
        if (data.files.length === 0) return;

        const status = data.size <= threshold ? '✅' : '⚠️';
        console.log(`\n${status} ${category}: ${this.formatBytes(data.size)}`);
        
        data.files.forEach(file => {
            console.log(`  - ${file.name}: ${this.formatBytes(file.size)}`);
        });
        
        if (data.size > threshold) {
            console.log(`  Exceeds ${this.formatBytes(threshold)} threshold`);
        }
    }

    /**
     * Analyze Lighthouse performance report
     */
    analyzeLighthouseReport() {
        console.log('\n=== Lighthouse Performance Analysis ===');
        
        if (!fs.existsSync(this.reportPath)) {
            console.log('📄 Lighthouse report not found. Run performance test first.');
            return null;
        }

        try {
            const report = JSON.parse(fs.readFileSync(this.reportPath, 'utf8'));
            const audits = report.audits;
            
            // Core Web Vitals
            this.reportMetric('First Contentful Paint', 
                audits['first-contentful-paint'], 
                this.thresholds.firstContentfulPaint);
                
            this.reportMetric('Largest Contentful Paint', 
                audits['largest-contentful-paint'], 
                this.thresholds.largestContentfulPaint);
                
            this.reportMetric('Cumulative Layout Shift', 
                audits['cumulative-layout-shift'], 
                0.1);
                
            this.reportMetric('Total Blocking Time', 
                audits['total-blocking-time'], 
                300);

            // Performance Score
            const performanceScore = Math.round(report.categories.performance.score * 100);
            console.log(`\n🎯 Performance Score: ${performanceScore}/100`);
            
            if (performanceScore >= 90) {
                console.log('✅ Excellent performance');
            } else if (performanceScore >= 70) {
                console.log('⚠️  Good performance - room for improvement');
            } else {
                console.log('❌ Poor performance - needs optimization');
            }

            return report;
            
        } catch (error) {
            console.error('❌ Failed to parse Lighthouse report:', error.message);
            return null;
        }
    }

    /**
     * Report a performance metric
     */
    reportMetric(name, audit, threshold) {
        if (!audit) return;

        const value = audit.numericValue;
        const unit = audit.numericUnit || 'ms';
        const status = value <= threshold ? '✅' : '❌';
        
        console.log(`${status} ${name}: ${value.toFixed(1)}${unit} (threshold: ${threshold}${unit})`);
    }

    /**
     * Analyze WASM optimization opportunities
     */
    analyzeWasmOptimization() {
        console.log('\n=== WASM Optimization Analysis ===');
        
        const wasmFiles = fs.readdirSync(this.distPath)
            .filter(file => file.endsWith('.wasm'))
            .map(file => ({
                name: file,
                path: path.join(this.distPath, file),
                size: fs.statSync(path.join(this.distPath, file)).size
            }));

        if (wasmFiles.length === 0) {
            console.log('❌ No WASM files found');
            return;
        }

        wasmFiles.forEach(file => {
            console.log(`\n📄 ${file.name}: ${this.formatBytes(file.size)}`);
            
            // Suggest optimizations based on size
            if (file.size > 2 * 1024 * 1024) {
                console.log('💡 Optimization suggestions:');
                console.log('  - Enable wee_alloc for smaller memory footprint');
                console.log('  - Use wasm-opt for size optimization');
                console.log('  - Consider code splitting or lazy loading');
                console.log('  - Remove debug symbols in release builds');
            } else if (file.size > 1024 * 1024) {
                console.log('💡 Consider using wasm-opt for additional size reduction');
            } else {
                console.log('✅ WASM size is optimal');
            }
        });
    }

    /**
     * Generate performance recommendations
     */
    generateRecommendations(bundleAnalysis, lighthouseReport) {
        console.log('\n=== Performance Recommendations ===');
        
        const recommendations = [];

        // Bundle size recommendations
        if (bundleAnalysis && bundleAnalysis.total > this.thresholds.bundleSize) {
            recommendations.push('🔧 Reduce bundle size through code splitting');
            recommendations.push('🔧 Implement lazy loading for non-critical components');
        }

        if (bundleAnalysis && bundleAnalysis.wasm.size > this.thresholds.wasmSize) {
            recommendations.push('🔧 Optimize WASM with wasm-opt tool');
            recommendations.push('🔧 Consider async WASM loading');
        }

        // Performance recommendations
        if (lighthouseReport) {
            const performanceScore = Math.round(lighthouseReport.categories.performance.score * 100);
            
            if (performanceScore < 90) {
                recommendations.push('🔧 Optimize critical rendering path');
                recommendations.push('🔧 Implement service worker for caching');
                recommendations.push('🔧 Compress static assets');
            }
        }

        if (recommendations.length === 0) {
            console.log('🎉 No performance issues detected - excellent work!');
        } else {
            recommendations.forEach(rec => console.log(rec));
        }
    }

    /**
     * Format bytes to human readable
     */
    formatBytes(bytes) {
        if (bytes === 0) return '0 B';
        
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    }

    /**
     * Run complete analysis
     */
    run() {
        console.log('🔍 FL-100 Performance Analysis Starting...\n');
        
        const bundleAnalysis = this.analyzeBundleSizes();
        const lighthouseReport = this.analyzeLighthouseReport();
        
        this.analyzeWasmOptimization();
        this.generateRecommendations(bundleAnalysis, lighthouseReport);
        
        console.log('\n✨ Analysis complete!\n');
        
        // Return success/failure for CI
        return bundleAnalysis !== false;
    }
}

// Run analysis if called directly
if (require.main === module) {
    const analyzer = new PerformanceAnalyzer();
    const success = analyzer.run();
    process.exit(success ? 0 : 1);
}

module.exports = PerformanceAnalyzer;