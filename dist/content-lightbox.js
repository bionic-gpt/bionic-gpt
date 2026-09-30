(function () {
  function initContentLightbox() {
    if (window.__contentLightboxReady) return;
    window.__contentLightboxReady = true;

    const imageOverlay = document.createElement("div");
    imageOverlay.className = "image-lightbox";
    imageOverlay.setAttribute("aria-hidden", "true");

    const overlayImg = document.createElement("img");
    overlayImg.alt = "";
    imageOverlay.appendChild(overlayImg);
    document.body.appendChild(imageOverlay);

    const codeOverlay = document.createElement("div");
    codeOverlay.className = "code-lightbox";
    codeOverlay.setAttribute("aria-hidden", "true");

    const codePanel = document.createElement("div");
    codePanel.className = "code-lightbox__panel";

    const codeToolbar = document.createElement("div");
    codeToolbar.className = "code-lightbox__toolbar";

    const zoomOutButton = document.createElement("button");
    zoomOutButton.type = "button";
    zoomOutButton.className = "code-lightbox__button";
    zoomOutButton.textContent = "A-";

    const zoomInButton = document.createElement("button");
    zoomInButton.type = "button";
    zoomInButton.className = "code-lightbox__button";
    zoomInButton.textContent = "A+";

    const closeButton = document.createElement("button");
    closeButton.type = "button";
    closeButton.className = "code-lightbox__button";
    closeButton.textContent = "Close";

    const codeContent = document.createElement("div");
    codeContent.className = "code-lightbox__content";

    codeToolbar.appendChild(zoomOutButton);
    codeToolbar.appendChild(zoomInButton);
    codeToolbar.appendChild(closeButton);
    codePanel.appendChild(codeToolbar);
    codePanel.appendChild(codeContent);
    codeOverlay.appendChild(codePanel);
    document.body.appendChild(codeOverlay);

    const contentOverlay = document.createElement("div");
    contentOverlay.className = "content-lightbox";
    contentOverlay.setAttribute("aria-hidden", "true");
    contentOverlay.setAttribute("role", "dialog");
    contentOverlay.setAttribute("aria-modal", "true");
    contentOverlay.setAttribute("aria-label", "Expanded content");

    const contentPanel = document.createElement("div");
    contentPanel.className = "content-lightbox__panel";

    const contentToolbar = document.createElement("div");
    contentToolbar.className = "content-lightbox__toolbar";

    const contentCloseButton = document.createElement("button");
    contentCloseButton.type = "button";
    contentCloseButton.className = "code-lightbox__button";
    contentCloseButton.textContent = "Close";

    const contentBody = document.createElement("div");
    contentBody.className = "content-lightbox__content";

    contentToolbar.appendChild(contentCloseButton);
    contentPanel.appendChild(contentToolbar);
    contentPanel.appendChild(contentBody);
    contentOverlay.appendChild(contentPanel);
    document.body.appendChild(contentOverlay);

    let codeZoom = 1;
    let contentTrigger = null;

    function setCodeZoom(nextZoom) {
      codeZoom = Math.max(0.75, Math.min(2.25, nextZoom));
      codePanel.style.setProperty("--code-lightbox-font-scale", String(codeZoom));
    }

    function closeImageLightbox() {
      imageOverlay.classList.remove("is-open");
      imageOverlay.setAttribute("aria-hidden", "true");
      overlayImg.removeAttribute("src");
      if (
        !codeOverlay.classList.contains("is-open") &&
        !contentOverlay.classList.contains("is-open")
      ) {
        document.body.style.overflow = "";
      }
    }

    function openImageLightbox(src, alt) {
      overlayImg.src = src;
      overlayImg.alt = alt || "";
      imageOverlay.classList.add("is-open");
      imageOverlay.setAttribute("aria-hidden", "false");
      document.body.style.overflow = "hidden";
    }

    function closeCodeLightbox() {
      codeOverlay.classList.remove("is-open");
      codeOverlay.setAttribute("aria-hidden", "true");
      codeContent.replaceChildren();
      setCodeZoom(1);
      if (
        !imageOverlay.classList.contains("is-open") &&
        !contentOverlay.classList.contains("is-open")
      ) {
        document.body.style.overflow = "";
      }
    }

    function openCodeLightbox(preNode) {
      const clone = preNode.cloneNode(true);
      clone.querySelectorAll(".code-copy-btn").forEach((node) => node.remove());
      codeContent.replaceChildren(clone);
      setCodeZoom(1.25);
      codeOverlay.classList.add("is-open");
      codeOverlay.setAttribute("aria-hidden", "false");
      document.body.style.overflow = "hidden";
    }

    function closeContentLightbox() {
      contentOverlay.classList.remove("is-open");
      contentOverlay.setAttribute("aria-hidden", "true");
      contentBody.replaceChildren();
      if (
        !imageOverlay.classList.contains("is-open") &&
        !codeOverlay.classList.contains("is-open")
      ) {
        document.body.style.overflow = "";
      }
      if (contentTrigger instanceof HTMLElement) {
        contentTrigger.focus();
      }
      contentTrigger = null;
    }

    function openContentLightbox(sourceNode) {
      const clone = sourceNode.cloneNode(true);
      clone.removeAttribute("data-content-lightbox");
      clone.removeAttribute("tabindex");
      clone.removeAttribute("role");
      clone.removeAttribute("aria-haspopup");
      clone.removeAttribute("aria-label");
      contentTrigger = sourceNode;
      contentBody.replaceChildren(clone);
      contentOverlay.classList.add("is-open");
      contentOverlay.setAttribute("aria-hidden", "false");
      document.body.style.overflow = "hidden";
      contentCloseButton.focus();
    }

    zoomInButton.addEventListener("click", function (event) {
      event.stopPropagation();
      setCodeZoom(codeZoom + 0.15);
    });

    zoomOutButton.addEventListener("click", function (event) {
      event.stopPropagation();
      setCodeZoom(codeZoom - 0.15);
    });

    closeButton.addEventListener("click", function (event) {
      event.stopPropagation();
      closeCodeLightbox();
    });

    contentCloseButton.addEventListener("click", function (event) {
      event.stopPropagation();
      closeContentLightbox();
    });

    document.addEventListener("click", function (event) {
      const target = event.target;
      if (!(target instanceof Element)) return;

      if (target.closest(".code-copy-btn")) return;

      const expandableContent = target.closest("[data-content-lightbox]");
      if (expandableContent && !target.closest(".content-lightbox")) {
        openContentLightbox(expandableContent);
        return;
      }

      const pre = target.closest("pre");
      if (pre && pre.closest("article, .prose") && !pre.closest(".code-lightbox")) {
        openCodeLightbox(pre);
        return;
      }

      if (!(target instanceof HTMLImageElement)) return;
      if (!target.closest("article, .prose")) return;
      if (target.closest(".image-lightbox")) return;
      if (!target.src) return;
      openImageLightbox(target.src, target.alt);
    });

    imageOverlay.addEventListener("click", function () {
      closeImageLightbox();
    });

    codeOverlay.addEventListener("click", function (event) {
      if (event.target === codeOverlay) {
        closeCodeLightbox();
      }
    });

    codePanel.addEventListener("click", function (event) {
      event.stopPropagation();
    });

    contentOverlay.addEventListener("click", function (event) {
      if (event.target === contentOverlay) {
        closeContentLightbox();
      }
    });

    contentPanel.addEventListener("click", function (event) {
      event.stopPropagation();
    });

    document.addEventListener("keydown", function (event) {
      const target = event.target;
      if (
        target instanceof Element &&
        target.matches("[data-content-lightbox]") &&
        (event.key === "Enter" || event.key === " ")
      ) {
        event.preventDefault();
        openContentLightbox(target);
        return;
      }
      if (event.key === "Escape" && imageOverlay.classList.contains("is-open")) {
        closeImageLightbox();
      }
      if (event.key === "Escape" && codeOverlay.classList.contains("is-open")) {
        closeCodeLightbox();
      }
      if (event.key === "Escape" && contentOverlay.classList.contains("is-open")) {
        closeContentLightbox();
      }
    });
  }

  if (document.readyState !== "loading") {
    initContentLightbox();
  } else {
    document.addEventListener("DOMContentLoaded", initContentLightbox);
  }
})();
