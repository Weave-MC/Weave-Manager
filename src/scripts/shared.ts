import type {MinecraftProcess} from "./types";

export async function showProcessInfo(mcProcess: MinecraftProcess) {

}

export async function createLaunchProfile(mcProcess: MinecraftProcess) {

}

export function checkVerticalOverflow(element: HTMLElement): boolean {
    return element.scrollHeight > element.clientHeight
}

/**
 * https://stackoverflow.com/a/42543908
 */
export function getScrollParent(el: HTMLElement, includeHidden: boolean) {
    let style = getComputedStyle(el)
    const excludedStaticParent = style.position === "absolute"
    const overflowRegex = includeHidden ? /(auto|scroll|hidden)/ : /(auto|scroll)/

    if (style.position === "fixed") return document.body
    for (let parent = el; (parent = parent.parentElement);) {
        style = getComputedStyle(parent)
        if (excludedStaticParent && style.position === "static")
            continue

        if (overflowRegex.test(style.overflow + style.overflowX + style.overflowY)) return parent
    }

    return document.body
}