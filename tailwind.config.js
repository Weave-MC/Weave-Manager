/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.svelte"],
  theme: {
    colors: {
      'crust': 'rgb(var(--color-crust)',
      'base': 'rgb(var(--color-base)',
      'surface0': 'rgb(var(--color-surface0)',
      'surface1': 'rgb(var(--color-surface1)',
      'overlay0': 'rgb(var(--color-overlay0)',
      'overlay1': 'rgb(var(--color-overlay1)',
      'text': 'rgb(var(--color-text)',
      'red': 'rgb(var(--color-red)',
      'green': 'rgb(var(--color-green)',
      'pink': 'rgb(var(--color-pink)',
      'blue': 'rgb(var(--color-blue)',
      'orange': 'rgb(var(--color-orange)',
    }
  },
  safelist: [
    {
      pattern: /bg-,+/
    },
    'mocha',
    'macchiato',
    'frappe',
    'latte'
  ],
  plugins: [
      require('@tailwindcss/forms'),
      // require('@catppuccin/tailwindcss')({
      //   prefix: false,
      //   defaultFlavour: 'mocha'
      // })
  ],
}

