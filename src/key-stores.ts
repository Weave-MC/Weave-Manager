import {readable} from 'svelte/store'

export const shiftDown = readable(false, set => {
    const controller = new AbortController();
    document.addEventListener('keydown', e => set(e.shiftKey), { signal: controller.signal })
    document.addEventListener('keyup', e => set(e.shiftKey), { signal: controller.signal })
    return () => controller.abort()
})