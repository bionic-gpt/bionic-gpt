(function () {
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

  let codeZoom = 1;

  function setCodeZoom(nextZoom) {
    codeZoom = Math.max(0.75, Math.min(2.25, nextZoom));
    codePanel.style.setProperty("--code-lightbox-font-scale", String(codeZoom));
  }

  function closeImageLightbox() {
    imageOverlay.classList.remove("is-open");
    imageOverlay.setAttribute("aria-hidden", "true");
    overlayImg.removeAttribute("src");
    if (!codeOverlay.classList.contains("is-open")) {
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
    if (!imageOverlay.classList.contains("is-open")) {
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

  document.addEventListener("click", function (event) {
    const target = event.target;
    if (!(target instanceof Element)) return;

    if (target.closest(".code-copy-btn")) return;

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

  document.addEventListener("keydown", function (event) {
    if (event.key === "Escape" && imageOverlay.classList.contains("is-open")) {
      closeImageLightbox();
    }
    if (event.key === "Escape" && codeOverlay.classList.contains("is-open")) {
      closeCodeLightbox();
    }
  });
})();
