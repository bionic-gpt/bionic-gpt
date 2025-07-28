import { FrameElement } from "@hotwired/turbo";

// Store the interval so we don't create multiple instances
let refreshInterval: number | null = null;

// If we have any elements with the class 'processing' then
// kick off a timer that will refresh the parent turbo frame 
export const refreshFrame = () => {
    const itemsCurrentlyProcessing = document.querySelectorAll('.processing')
    if(itemsCurrentlyProcessing.length > 0) {
        const item = itemsCurrentlyProcessing[0]

        let closestParentTurboFrame = item.closest('turbo-frame')

        if(closestParentTurboFrame instanceof FrameElement) {
            if (refreshInterval === null) {
                refreshInterval = window.setInterval(() => {
                    if(closestParentTurboFrame instanceof FrameElement) {
                        closestParentTurboFrame.reload()
                        if(document.querySelectorAll('.processing').length == 0) {
                            if (refreshInterval !== null) {
                                clearInterval(refreshInterval)
                                refreshInterval = null
                            }
                        }
                    }
                }, 5000);
            }
        }
    }
}
