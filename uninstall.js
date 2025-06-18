#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

function cleanupHooks() {
  console.log('🧹 Cleaning up rusky hooks...');
  
  try {
    // .rusky 디렉토리가 있는지 확인
    const ruskyDir = path.join(process.cwd(), '.rusky');
    if (fs.existsSync(ruskyDir)) {
      console.log('Found .rusky directory. You may want to run "rusky uninstall" to clean up git hooks.');
    }
    
    console.log('✅ rusky uninstalled successfully!');
    console.log('💡 To completely remove git hooks, run "rusky uninstall" before uninstalling the package.');
    
  } catch (error) {
    console.error('⚠️  Warning: Could not clean up hooks:', error.message);
  }
}

if (require.main === module) {
  cleanupHooks();
} 