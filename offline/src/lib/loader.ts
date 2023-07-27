import type { Writable } from "svelte/store"
import Spinner from "./Spinner.svelte"

export const loader = (node: Element, loading: Writable<boolean>) => {
    let Spin: Spinner | undefined

    const unsubscribe = loading.subscribe(loading => {
        if (loading) {
            Spin = new Spinner({
                target: node,
                intro: true,
            })
        } else {
            if (Spin) {
                Spin?.$destroy?.()
                Spin = undefined
            }
        }
    })
}