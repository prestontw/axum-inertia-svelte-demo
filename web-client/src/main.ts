import './app.css'
import { createInertiaApp, type ResolvedComponent } from '@inertiajs/svelte'
import { mount } from 'svelte'

createInertiaApp({
  resolve: name => {
    const pages: {[page: string]: ResolvedComponent} = import.meta.glob('./Pages/**/*.svelte', { eager: true })
    return pages[`./Pages/${name}.svelte`]
  },
  setup({ el, App, props }) {
    mount(App, { target: el!, props })
  },
})

