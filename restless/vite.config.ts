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
    '/api': { // Adjust this path to match your API endpoint prefix
      target: 'http://127.0.0.1:4000/', // Replace with your backend server URL
      changeOrigin: true,
      rewrite: (path) => path.replace(/^\/api/, '')
    },
    '/auth': {
      target: 'http://127.0.0.1:3000/',
      changeOrigin: true,
      rewrite: (path) => path.replace(/^\/auth/, '')
    },
    '/static': {
      target: 'http://127.0.0.1:8080/',
      changeOrigin: true,
      rewrite: (path) => path.replace(/^\/auth/, '')
    }
  }
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
})
