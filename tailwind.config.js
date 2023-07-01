/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.svelte"],
  theme: {
    colors: {
      'crust': 'rgb(var(--color-crust)',
      'base': 'rgb(var(--color-base)',
      'surface': 'rgb(var(--color-surface)',
      'overlay': 'rgb(var(--color-overlay)',
      'text': 'rgb(var(--color-text)',
      'disabled': 'rgb(var(--color-disabled)',
      'enabled': 'rgb(var(--color-enabled)'
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

