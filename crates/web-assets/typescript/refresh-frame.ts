import { FrameElement } from "@hotwired/turbo";

// If we have any elements with the class 'processing' then
// kick off a timer that will refresh the parent turbo frame 
export const refreshFrame = () => {
    const itemsCurrentlyProcessing = document.querySelectorAll('.processing')
    if(itemsCurrentlyProcessing.length > 0) {
        const item = itemsCurrentlyProcessing[0]

        let clostestParentTurboFrame = item.closest('turbo-frame')

        if(clostestParentTurboFrame instanceof FrameElement) {
            const intId = setInterval(() => {
                if(clostestParentTurboFrame instanceof FrameElement) {
                    clostestParentTurboFrame.reload()
                    if(document.querySelectorAll('.processing').length == 0) {
                        clearInterval(intId)
                    }
                }
            }, 1000);
        }
    }
}
