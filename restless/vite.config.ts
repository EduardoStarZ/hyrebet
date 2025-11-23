import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
  ],
  server: {
    host: '127.0.0.1', // Listen on all network interfaces (for external access)
    port: 5000,    // Set the desired port
    // strictPort: true, // Optional: Exit if port is already in use
  proxy: {
    '/json': { // Adjust this path to match your API endpoint prefix
      target: 'http://127.0.0.1:4000/', // Replace with your backend server URL
      changeOrigin: true,
      logLevel: 'debug'
    }
  }
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
})
