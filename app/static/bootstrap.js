/*
 ██████╗ ██╗   ██╗████████╗██████╗  ██████╗  ██████╗  ██████╗     ██████╗  ██████╗  ██████╗ ██████╗  ██████╗ 
██╔═══██╗██║   ██║╚══██╔══╝██╔══██╗██╔═══██╗██╔════╝ ██╔═══██╗    ██╔══██╗██╔═══██╗██╔════╝██╔═══██╗██╔═══██╗
██║   ██║██║   ██║   ██║   ██████╔╝██║   ██║██║  ███╗██║   ██║    ██████╔╝██║   ██║██║     ██║   ██║██║   ██║
██║▄▄ ██║██║   ██║   ██║   ██╔═══╝ ██║   ██║██║   ██║██║   ██║    ██╔══██╗██║   ██║██║     ██║   ██║██║   ██║
╚██████╔╝╚██████╔╝   ██║   ██║     ╚██████╔╝╚██████╔╝╚██████╔╝    ██║  ██║╚██████╔╝╚██████╗╚██████╔╝╚██████╔╝
 ╚══▀▀═╝  ╚═════╝    ╚═╝   ╚═╝      ╚═════╝  ╚═════╝  ╚═════╝     ╚═╝  ╚═╝ ╚═════╝  ╚═════╝ ╚═════╝  ╚═════╝ 
                                                                                          app/static/bootstrap.js
*/

// Minimal progressive-enhancement bootstrap for worklets and workers

// CSS Paint Worklet (Houdini)
if (globalThis.CSS && 'paintWorklet' in CSS) {
  // Load a simple grid painter for future rulers/grids
  CSS.paintWorklet.addModule('static/worklets/paint-grid.js').catch(console.warn);
}

// Start a placeholder worker for OffscreenCanvas-based tasks
try {
  const worker = new Worker('static/workers/diff-worker.js', { type: 'module' });
  worker.postMessage({ type: 'init' });
} catch (e) {
  console.warn('Worker init failed:', e);
}

// Initialize Export and Print Systems
(async function initializeExportSystems() {
  try {
    // Load export and print systems
    await Promise.all([
      import('./export-system.js').catch(() => {
        return import('static/export-system.js');
      }),
      import('./paper-validation.js').catch(() => {
        return import('static/paper-validation.js');
      }),
      import('./print-preview.js').catch(() => {
        return import('static/print-preview.js');
      })
    ]);

    // Initialize systems after WASM loads
    document.addEventListener('TrunkApplicationStarted', async () => {
      console.log('🖨️ Initializing export and print systems...');
      
      // Initialize export system
      if (window.exportSystem) {
        const initialized = await window.exportSystem.initialize();
        if (initialized) {
          console.log('✅ Export system ready');
        } else {
          console.warn('⚠️ Export system initialization failed');
        }
      }

      // Set default paper size
      if (window.paperValidation) {
        window.paperValidation.setPaperSize('US_LETTER', 'portrait');
        console.log('✅ Paper validation system ready');
      }

      // Initialize print preview
      if (window.printPreview) {
        console.log('✅ Print preview system ready');
      }

      // Add export buttons to UI if present
      addExportButtons();
    });

  } catch (error) {
    console.warn('Export systems initialization failed:', error);
  }
})();

// Add export and print buttons to the UI
function addExportButtons() {
  const toolbar = document.querySelector('.proofing-toolbar') || 
                 document.querySelector('.document-toolbar');
  
  if (!toolbar) return;

  // Create export section
  const exportSection = document.createElement('div');
  exportSection.className = 'toolbar-section';
  exportSection.innerHTML = `
    <h3>Export & Print</h3>
    <div class="export-controls">
      <button id="exportPDF" class="export-button">📄 PDF</button>
      <button id="exportPNG" class="export-button">🖼️ PNG</button>
      <button id="exportSVG" class="export-button">📐 SVG</button>
      <button id="printPreview" class="export-button">🖨️ Preview</button>
      <button id="validatePaper" class="export-button">📏 Validate</button>
    </div>
  `;

  toolbar.appendChild(exportSection);

  // Add event listeners
  setupExportEventListeners();
}

// Setup event listeners for export buttons
function setupExportEventListeners() {
  // PDF Export
  document.getElementById('exportPDF')?.addEventListener('click', async () => {
    if (window.exportSystem) {
      const result = await window.exportSystem.exportToPDF('main-document', {
        filename: 'FL-100-form.pdf',
        includeFields: true,
        quality: 'high'
      });
      
      if (result.success) {
        showNotification('PDF exported successfully', 'success');
      } else {
        showNotification('PDF export failed: ' + result.error, 'error');
      }
    }
  });

  // PNG Export
  document.getElementById('exportPNG')?.addEventListener('click', async () => {
    if (window.exportSystem) {
      const result = await window.exportSystem.exportToPNG('main-document', {
        filename: 'FL-100-form.png',
        dpi: 300,
        quality: 1.0
      });
      
      if (result.success) {
        showNotification('PNG exported successfully', 'success');
      } else {
        showNotification('PNG export failed: ' + result.error, 'error');
      }
    }
  });

  // SVG Export
  document.getElementById('exportSVG')?.addEventListener('click', async () => {
    if (window.exportSystem) {
      const result = await window.exportSystem.exportToSVG('main-document', {
        filename: 'FL-100-form.svg',
        includeStyles: true,
        embedFonts: true
      });
      
      if (result.success) {
        showNotification('SVG exported successfully', 'success');
      } else {
        showNotification('SVG export failed: ' + result.error, 'error');
      }
    }
  });

  // Print Preview
  document.getElementById('printPreview')?.addEventListener('click', async () => {
    if (window.printPreview) {
      const result = await window.printPreview.openPrintPreview('main-document', {
        enableToolbar: true,
        autoValidate: true,
        showAccuracyMetrics: true
      });
      
      if (!result.success) {
        showNotification('Failed to open print preview: ' + result.error, 'error');
      }
    }
  });

  // Paper Validation
  document.getElementById('validatePaper')?.addEventListener('click', () => {
    if (window.paperValidation) {
      const documentElement = document.querySelector('.document-manager') || 
                             document.querySelector('.proofing-ui');
      
      if (documentElement) {
        const validation = window.paperValidation.validateDocument(documentElement);
        
        if (validation.isValid) {
          showNotification('Document validation passed ✅', 'success');
        } else {
          const errors = validation.errors.join(', ');
          showNotification('Validation issues: ' + errors, 'warning');
        }
      }
    }
  });
}

// Show notification to user
function showNotification(message, type = 'info') {
  const notification = document.createElement('div');
  notification.className = `notification notification-${type}`;
  notification.style.cssText = `
    position: fixed;
    top: 20px;
    right: 20px;
    padding: 12px 20px;
    border-radius: 6px;
    color: white;
    font-weight: 500;
    z-index: 10000;
    max-width: 400px;
    box-shadow: 0 4px 12px rgba(0,0,0,0.2);
    transition: all 0.3s ease;
  `;
  
  // Set background color based on type
  switch (type) {
    case 'success':
      notification.style.backgroundColor = '#28a745';
      break;
    case 'error':
      notification.style.backgroundColor = '#dc3545';
      break;
    case 'warning':
      notification.style.backgroundColor = '#ffc107';
      notification.style.color = '#212529';
      break;
    default:
      notification.style.backgroundColor = '#007bff';
  }
  
  notification.textContent = message;
  document.body.appendChild(notification);
  
  // Auto-remove after 5 seconds
  setTimeout(() => {
    notification.style.opacity = '0';
    notification.style.transform = 'translateX(100%)';
    setTimeout(() => {
      document.body.removeChild(notification);
    }, 300);
  }, 5000);
}
